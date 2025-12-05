use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fixed_resample::ReadStatus;
use spc::spc_assembler::*;
use spc::spc_emulator::*;
use spc::spc_file_parser::*;
use spc::types::*;
use std::env;
use std::fmt::Error;
use std::num::NonZero;

/// バイナリをディスアセンブル
fn naive_disassemble(ram: &[u8]) {
    let mut pc = 0x100;

    while pc < ram.len() {
        let (opcode, len) = parse_opcode(&ram[pc..]);
        println!("{:#06X}: {:?}", pc, opcode);
        pc += len as usize;
    }
}

/// 実行してみる
fn naive_execution(register: &SPCRegister, ram: &[u8], dsp_register: &[u8; 128]) {
    const CLOCK_TICK_CYCLE_64KHZ: u64 = 384;
    let mut emu = SPCEmulator::new(&register, ram, dsp_register);
    let mut total_cycle = 0u64;
    let mut next_tick_cycle = CLOCK_TICK_CYCLE_64KHZ;
    loop {
        let cycle = emu.execute_step();
        total_cycle = total_cycle.wrapping_add(cycle as u64);
        if total_cycle >= next_tick_cycle {
            emu.clock_tick_64k_hz();
            next_tick_cycle = next_tick_cycle.wrapping_add(CLOCK_TICK_CYCLE_64KHZ);
        }
    }
}

/// 再生してみる
fn naive_play(
    register: &SPCRegister,
    ram: &[u8],
    dsp_register: &[u8; 128],
) -> Result<(), Box<dyn std::error::Error>> {
    const NUM_CHANNELS: usize = 2;
    const CLOCK_TICK_CYCLE_64KHZ: u64 = 384; /* 64KHz周期のクロックサイクル SPCのマスタークロックを64Kで割って得られる = 24576000 / 64000 */
    const NORMALIZED_CONST: f32 = 1.0 / 32768.0;

    // cpalの初期化
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let stream_config: cpal::StreamConfig = device.default_output_config().unwrap().into();
    let sampling_rate = stream_config.sample_rate.0;

    // リサンプラ初期化 32k -> デバイスの出力レート変換となるように
    let (mut prod, mut cons) = fixed_resample::resampling_channel::<f32, NUM_CHANNELS>(
        NonZero::new(NUM_CHANNELS).unwrap(),
        32000,
        sampling_rate,
        Default::default(),
    );

    // SPCエミュレータ初期化
    let mut emu = SPCEmulator::new(&register, ram, dsp_register);
    let mut total_cycle = 0u64;
    let mut next_tick_cycle = CLOCK_TICK_CYCLE_64KHZ;

    // 再生ストリーム作成
    let mut tmp_buffer = vec![0.0; 2048 * NUM_CHANNELS];
    let stream = device
        .build_output_stream(
            &stream_config,
            move |buffer: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // レート変換比を信じ、バッファが一定量埋まるまで出力させる
                let mut nsamples = prod.available_frames();
                while nsamples > 1024 {
                    let cycle = emu.execute_step();
                    total_cycle = total_cycle.wrapping_add(cycle as u64);
                    if total_cycle >= next_tick_cycle {
                        next_tick_cycle = next_tick_cycle.wrapping_add(CLOCK_TICK_CYCLE_64KHZ);
                        if let Some(out) = emu.clock_tick_64k_hz() {
                            let fout = [
                                (out[0] as f32) * NORMALIZED_CONST,
                                (out[1] as f32) * NORMALIZED_CONST,
                            ];
                            prod.push_interleaved(&fout);
                            nsamples = prod.available_frames();
                        }
                    }
                }

                // リサンプラー出力の取り出し
                let frames = buffer.len() / NUM_CHANNELS;
                let status = cons.read_interleaved(&mut tmp_buffer[..frames * NUM_CHANNELS]);
                if let ReadStatus::UnderflowOccurred { .. } = status {
                    eprintln!("input stream fell behind: try increasing channel latency");
                }

                buffer.fill(0.0);
                for ch in 0..NUM_CHANNELS {
                    for (out_chunk, in_chunk) in buffer
                        .chunks_exact_mut(NUM_CHANNELS)
                        .zip(tmp_buffer.chunks_exact(NUM_CHANNELS))
                    {
                        out_chunk[ch] = in_chunk[ch];
                    }
                }
            },
            |err| eprintln!("[SPC] {err}"),
            None,
        )
        .unwrap();

    // 再生開始
    stream.play()?;
    loop {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 引数が合っていないときは説明を表示
    if args.len() != 2 {
        println!("Usage: {} SPC_FILE", args[0]);
        return Err(Box::new(Error));
    }

    // データ読み込み
    let data = std::fs::read(&args[1])?;
    if let Some(spcfile) = parse_spc_file(&data) {
        println!(
            "Info: {} \n\
            SPC Register PC: {:#X} A: {:#X} X: {:#X} Y: {:#X} PSW: {:#X} SP: {:#X} \n\
            Music Title: {} \n\
            Game Title: {} \n\
            Creator: {} \n\
            Comment: {} \n\
            Generate Date: {}/{}/{} \n\
            Music Duration: {} (sec) \n\
            Fadeout Time: {} (msec) \n\
            Composer: {}",
            std::str::from_utf8(&spcfile.header.info).unwrap(),
            spcfile.header.spc_register.pc,
            spcfile.header.spc_register.a,
            spcfile.header.spc_register.x,
            spcfile.header.spc_register.y,
            spcfile.header.spc_register.psw,
            spcfile.header.spc_register.sp,
            std::str::from_utf8(&spcfile.header.music_title)
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(&spcfile.header.game_title)
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(&spcfile.header.creator)
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(&spcfile.header.comment)
                .unwrap()
                .trim_end_matches('\0'),
            spcfile.header.generate_date,
            spcfile.header.generate_month,
            spcfile.header.generate_year,
            spcfile.header.duration,
            spcfile.header.fadeout_time,
            std::str::from_utf8(&spcfile.header.composer)
                .unwrap()
                .trim_end_matches('\0'),
        );
        let _ = naive_play(
            &spcfile.header.spc_register,
            &spcfile.ram,
            &spcfile.dsp_register,
        );
    }

    Ok(())
}

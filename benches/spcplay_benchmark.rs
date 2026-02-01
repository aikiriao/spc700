use criterion::{criterion_group, criterion_main, Criterion};
use spc700::spc::*;
use spc700::spc_file::*;

// 64KHz周期のクロックサイクル SPCのクロック(1.024MHz)を64KHzで割って得られる = 1024000 / 64000
const CLOCK_TICK_CYCLE_64KHZ: u32 = 16;

pub fn spcplay_benchmark(c: &mut Criterion) {
    c.bench_function("SPC file decode", |b| {
        b.iter(|| {
            // SPCファイル読み込み
            let data = std::fs::read("./benches/data/Eyes on Me! [2 = Original] [1 = OG Echo].spc")
                .unwrap();
            let spc_file = parse_spc_file(&data).unwrap();

            // 演奏時間いっぱいまで出力計算
            let mut spc: spc700::spc::SPC<spc700::sdsp::SDSP> = SPC::new(
                &spc_file.header.spc_register,
                &spc_file.ram,
                &spc_file.dsp_register,
            );
            let mut cycle_count = 0;
            let mut tick64khz_count = 0;
            while tick64khz_count < spc_file.header.duration as u64 * 64000 {
                cycle_count += spc.execute_step() as u32;
                if cycle_count >= CLOCK_TICK_CYCLE_64KHZ {
                    cycle_count -= CLOCK_TICK_CYCLE_64KHZ;
                    let _ = spc.clock_tick_64k_hz();
                    tick64khz_count += 1;
                }
            }
        })
    });
}

criterion_group!(benches, spcplay_benchmark);
criterion_main!(benches);

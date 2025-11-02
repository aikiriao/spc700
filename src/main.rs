use spc::spc_file_parser::*;
use spc::spc_assembler::*;
use std::env;
use std::fmt::Error;

/// バイナリをディスアセンブル
fn naive_disassemble(ram: &[u8]) {
    let mut pc = 0x100;

    while pc < ram.len() {
        let (opcode, len) = parse_opcode(&ram[pc..]);
        println!("{:#06X}: {:?}", pc, opcode);
        pc += len as usize;
    }
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
        naive_disassemble(&spcfile.ram);
    }

    Ok(())
}

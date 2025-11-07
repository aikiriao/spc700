use crate::spc_assembler::*;
use crate::spc_dsp::*;
use crate::types::*;

/// ネガティブフラグ
const PSW_FLAG_N: u8 = 1 << 7;
/// オーバーフローフラグ
const PSW_FLAG_V: u8 = 1 << 6;
/// ダイレクトページフラグ
const PSW_FLAG_P: u8 = 1 << 5;
/// ハーフキャリーフラグ
const PSW_FLAG_H: u8 = 1 << 3;
/// ゼロフラグ
const PSW_FLAG_Z: u8 = 1 << 1;
/// キャリーフラグ
const PSW_FLAG_C: u8 = 1 << 0;
/// スタックのベースアドレス
const STACK_BASE_ADDRESS: usize = 0x100;
/// テストレジスタアドレス
const TEST_ADDRESS: usize = 0x00F0;
/// コントロールレジスタアドレス
const CONTROL_ADDRESS: usize = 0x00F1;
/// DSPレジスタアドレス
const DSPADDR_ADDRESS: usize = 0x00F2;
/// DSPデータアドレス
const DSPDATA_ADDRESS: usize = 0x00F3;
/// CPUポートのベースアドレス
const CPUIO0_ADDRESS: usize = 0x00F4;
const CPUIO1_ADDRESS: usize = 0x00F5;
const CPUIO2_ADDRESS: usize = 0x00F6;
const CPUIO3_ADDRESS: usize = 0x00F7;
/// タイマーターゲットのベースアドレス
const T0TARGET_ADDRESS: usize = 0x00FA;
const T1TARGET_ADDRESS: usize = 0x00FB;
const T2TARGET_ADDRESS: usize = 0x00FC;
/// タイマーカウントのベースアドレス
const T0OUT_ADDRESS: usize = 0x00FD;
const T1OUT_ADDRESS: usize = 0x00FE;
const T2OUT_ADDRESS: usize = 0x00FF;

/// エミュレータ
pub struct SPCEmulator {
    reg: SPCRegister,
    dsp: SPCDSP,
    ram: [u8; 65536],
    tick_count: u64,
    timer_enable: [bool; 3],
    timer_count: [u8; 3],
}

/// メモリビットのアドレスとビット位置を取得
fn get_address_bit(address_bit: u16) -> (u8, usize) {
    let bit_pos = ((address_bit >> 13) & 0x07) as u8;
    let address = ((address_bit >> 0) & 0x1F) as usize;
    (bit_pos, address)
}

/// 加算時のハーフキャリーを判定
fn check_half_carry_add_u8(a: u8, b: u8) -> bool {
    (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
}

/// 減算時のハーフキャリーを判定
fn check_half_carry_sub_u8(a: u8, b: u8) -> bool {
    ((a & 0xF) as i16 - (b & 0xF) as i16) < 0
}

/// 加算時のハーフキャリーを判定
fn check_half_carry_add_u16(a: u16, b: u16) -> bool {
    (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
}

/// 減算時のハーフキャリーを判定
fn check_half_carry_sub_u16(a: u16, b: u16) -> bool {
    ((a & 0xF) as i32 - (b & 0xF) as i32) < 0
}

impl SPCEmulator {
    pub fn new(reg: &SPCRegister, ram: &[u8]) -> SPCEmulator {
        let mut emu = Self {
            reg: reg.clone(),
            ram: [0; 65536],
            dsp: SPCDSP::new(),
            tick_count: 0,
            timer_enable: [false; 3],
            timer_count: [0; 3],
        };
        emu.ram.copy_from_slice(ram);

        // TODO: ramの内容からエミュレータをセットアップ
        emu.write_ram_u8(CONTROL_ADDRESS, ram[CONTROL_ADDRESS]);

        emu
    }

    /// ステップ実行
    pub fn execute_step(&mut self) -> u8 {
        let (opcode, len) = parse_opcode(&self.ram[(self.reg.pc as usize)..]);
        println!(
            "{:#06X}: {:02X?} {:X?} {:X?}",
            self.reg.pc,
            self.ram[(self.reg.pc as usize)..((self.reg.pc + len) as usize)].to_vec(),
            opcode,
            self.reg
        );
        self.reg.pc += len;
        self.execute_opcode(&opcode)
    }

    /// クロックカウンタの更新
    fn countup_clock(&mut self, id: usize) {
        let target = self.ram[T0TARGET_ADDRESS + id];
        self.timer_count[id] = self.timer_count[id].overflowing_add(1).0;
        if self.timer_count[id] >= target {
            let mut counter = self.ram[T0OUT_ADDRESS + id];
            self.timer_count[id] = 0;
            counter += 1;
            self.ram[T0OUT_ADDRESS + id] = counter & 0x0F;
        }
    }

    /// クロックティック
    pub fn clock_tick_64k_hz(&mut self) {
        self.tick_count = self.tick_count.overflowing_add(1).0;
        // 8kHzタイマー
        if self.tick_count % 8 == 0 {
            if self.timer_enable[0] {
                self.countup_clock(0);
            }
            if self.timer_enable[1] {
                self.countup_clock(1);
            }
        }
        // 64kHzタイマー
        if self.timer_enable[2] {
            self.countup_clock(2);
        }
        // 32kHz周期で出力サンプル計算
        if self.tick_count % 2 == 0 {
            let _ = self.dsp.compute_sample(&self.ram);
        }
    }

    /// タイマーに関するレジスタの書き込み処理
    fn write_timer_register(&mut self, value: u8) {
        for id in 0..=2 {
            let id_bit = 1 << id;
            if (value & id_bit) != 0 {
                self.timer_enable[id] = true;
                if (self.ram[CONTROL_ADDRESS] & id_bit) == 0 {
                    self.timer_count[id] = 0;
                    self.ram[T0OUT_ADDRESS + id] = 0;
                }
            } else {
                self.timer_enable[id] = false;
            }
        }
    }

    /// RAMへの書き込み
    fn write_ram_u8(&mut self, address: usize, value: u8) {
        // CPUレジスタへの書き込み
        if (address >= TEST_ADDRESS) && (address <= T2OUT_ADDRESS) {
            match address {
                TEST_ADDRESS => {
                    panic!("CANNOT write to test register!!");
                }
                CONTROL_ADDRESS => {
                    self.write_timer_register(value);
                    // TODO: IPL ROM enable, Clear CPUIO ports
                }
                DSPADDR_ADDRESS => {
                    // 何もしないがアドレスをラッチすべき？
                }
                DSPDATA_ADDRESS => {
                    self.dsp
                        .write_dsp_register(self.ram[DSPADDR_ADDRESS], value);
                }
                T0OUT_ADDRESS | T1OUT_ADDRESS | T2OUT_ADDRESS => {
                    panic!("CANNOT write to TxOUT register!!");
                }
                _ => {}
            }
        }
        self.ram[address] = value;
        // println!("W: 0x{:04X} <- {:02X}", address, value);
    }

    /// RAMからの読み込み
    fn read_ram_u8(&self, address: usize) -> u8 {
        // println!("R: 0x{:04X} -> {:02X}", address, self.ram[address]);
        // CPUレジスタからの読み込み
        if (address >= TEST_ADDRESS) && (address <= DSPDATA_ADDRESS) {
            match address {
                TEST_ADDRESS => {
                    panic!("CANNOT read from test register!!");
                }
                CONTROL_ADDRESS => {
                    // 何もしない
                }
                DSPADDR_ADDRESS => {
                    // 何もしないがアドレスをラッチすべき？
                }
                DSPDATA_ADDRESS => {
                    return self.dsp.read_dsp_register(address as u8);
                }
                _ => {}
            }
        }
        self.ram[address]
    }

    /// RAMからの読み込み（デバッグのため関数化）
    fn read_ram_u16(&self, address: usize) -> u16 {
        // println!(
        // "R16: 0x{:04X} -> {:04X}",
        // address,
        // ((self.ram[address + 1] as u16) << 8) | self.ram[address] as u16
        // );
        ((self.ram[address + 1] as u16) << 8) | self.ram[address] as u16
    }

    /// ダイレクトページのアドレスを取得
    fn get_direct_page_address(&self, direct_page: u8) -> usize {
        if self.test_psw_flag(PSW_FLAG_P) {
            0x100usize + direct_page as usize
        } else {
            direct_page as usize
        }
    }

    /// ダイレクトページインデックス間接アドレスを取得
    fn get_direct_page_x_indexed_indirect_address(&self, direct_page: u8) -> usize {
        let dp_address = self.get_direct_page_address(direct_page) + self.reg.x as usize;
        self.read_ram_u16(dp_address) as usize
    }

    /// ダイレクトページ関接インデックスアドレスを取得
    fn get_direct_page_indirect_y_indexed_address(&self, direct_page: u8) -> usize {
        let dp_address = self.get_direct_page_address(direct_page);
        let address = self.read_ram_u16(dp_address);
        (address + (self.reg.y as u16)) as usize
    }

    /// PSWの各フラグが立っているか検査
    fn test_psw_flag(&self, flag: u8) -> bool {
        (self.reg.psw & flag) != 0
    }

    /// 条件conditionに依存し、PSWの各フラグのset/resetを実行
    fn set_psw_flag(&mut self, flag: u8, condition: bool) {
        self.reg.psw = if condition {
            self.reg.psw | flag
        } else {
            self.reg.psw & !flag
        };
    }

    /// スタックにデータをPUSH
    fn push_stack(&mut self, value: u8) {
        self.write_ram_u8(STACK_BASE_ADDRESS + self.reg.sp as usize, value);
        self.reg.sp -= 1;
    }

    /// スタックからデータをPOP
    fn pop_stack(&mut self) -> u8 {
        self.reg.sp += 1;
        self.read_ram_u8(STACK_BASE_ADDRESS + self.reg.sp as usize)
    }

    /// オペコードを実行
    fn execute_opcode(&mut self, opcode: &SPCOpcode) -> u8 {
        match opcode {
            SPCOpcode::NOP => {
                // 何もしない
                2
            }
            // データ転送命令
            SPCOpcode::MOV { oprand } => self.execute_mov(oprand),
            SPCOpcode::MOVW { oprand } => match oprand {
                SPCOprand::DirectPageToYA { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    self.reg.y = self.read_ram_u8(address + 0);
                    self.reg.a = self.read_ram_u8(address + 1);
                    self.set_psw_flag(PSW_FLAG_N, (self.reg.y >> 7) != 0);
                    self.set_psw_flag(PSW_FLAG_N, (self.reg.y == 0) && (self.reg.a == 0));
                    5
                }
                SPCOprand::YAToDirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    self.write_ram_u8(address + 0, self.reg.y);
                    self.write_ram_u8(address + 1, self.reg.a);
                    4
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::XCN => {
                let ret = (self.reg.x >> 4) | (self.reg.x << 4);
                self.reg.x = ret;
                self.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                5
            }
            // 算術演算命令
            SPCOpcode::ADC { oprand } => self.execute_adc(oprand),
            SPCOpcode::ADDW { oprand } => match oprand {
                SPCOprand::DirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let wval = self.read_ram_u16(address);
                    let ya = ((self.reg.y as u16) << 8) | self.reg.a as u16;
                    let (ret, arith_overflow) = ya.overflowing_add(wval);
                    let sign_overflow =
                        ((ya & 0x8000) == (wval & 0x8000)) && ((ya & 0x8000) != (ret & 0x8000));
                    let half_carry = check_half_carry_add_u16(ya, wval);
                    self.reg.y = (ret >> 8) as u8 & 0xFF;
                    self.reg.a = (ret >> 0) as u8 & 0xFF;
                    // フラグ更新
                    self.set_psw_flag(PSW_FLAG_N, (ret >> 15) != 0);
                    self.set_psw_flag(PSW_FLAG_H, half_carry);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    if arith_overflow {
                        self.set_psw_flag(PSW_FLAG_V, false);
                        self.set_psw_flag(PSW_FLAG_C, true);
                    } else if sign_overflow {
                        self.set_psw_flag(PSW_FLAG_V, true);
                        self.set_psw_flag(PSW_FLAG_C, false);
                    } else {
                        self.set_psw_flag(PSW_FLAG_V, false);
                        self.set_psw_flag(PSW_FLAG_C, false);
                    }
                    5
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::DEC { oprand } => self.execute_dec(oprand),
            SPCOpcode::DECW { oprand } => match oprand {
                SPCOprand::DirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let mut wval = self.read_ram_u16(address);
                    wval = wval.overflowing_sub(1).0;
                    self.write_ram_u8(address + 0, ((wval >> 8) & 0xFF) as u8);
                    self.write_ram_u8(address + 1, ((wval >> 0) & 0xFF) as u8);
                    self.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, wval == 0);
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::DIV => {
                let ya = ((self.reg.y as u16) << 8) | self.reg.a as u16;
                let quot = ya / (self.reg.x as u16);
                let rem = ya % (self.reg.x as u16);

                if quot <= 0xFF {
                    self.reg.a = quot as u8;
                } else {
                    self.reg.a = (quot & 0xFF) as u8;
                }
                self.reg.y = rem as u8;

                self.set_psw_flag(PSW_FLAG_N, (quot >> 8) != 0);
                self.set_psw_flag(PSW_FLAG_V, quot > 0xFF);
                self.set_psw_flag(PSW_FLAG_H, (self.reg.y & 0xF) >= (self.reg.x & 0xF));
                self.set_psw_flag(PSW_FLAG_Z, quot == 0);
                12
            }
            SPCOpcode::INC { oprand } => self.execute_inc(oprand),
            SPCOpcode::INCW { oprand } => match oprand {
                SPCOprand::DirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let mut wval = self.read_ram_u16(address);
                    wval = wval.overflowing_add(1).0;
                    self.write_ram_u8(address + 0, ((wval >> 8) & 0xFF) as u8);
                    self.write_ram_u8(address + 1, ((wval >> 0) & 0xFF) as u8);
                    self.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, wval == 0);
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::MUL => {
                let mul = (self.reg.y as i16) * (self.reg.a as i16);
                self.reg.y = ((mul << 8) & 0xFF) as u8;
                self.reg.a = ((mul << 0) & 0xFF) as u8;
                self.set_psw_flag(PSW_FLAG_N, (mul >> 15) != 0);
                self.set_psw_flag(PSW_FLAG_Z, self.reg.y == 0);
                9
            }
            SPCOpcode::SBC { oprand } => self.execute_sbc(oprand),
            SPCOpcode::SUBW { oprand } => match oprand {
                SPCOprand::DirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let wval = self.read_ram_u16(address);
                    let ya = ((self.reg.y as u16) << 8) | self.reg.a as u16;
                    let (ret, arith_overflow) = ya.overflowing_sub(wval);
                    let sign_overflow =
                        ((ya & 0x8000) != (wval & 0x8000)) && ((ya & 0x8000) != (ret & 0x8000));
                    let half_carry = check_half_carry_sub_u16(ya, wval);
                    self.reg.y = (ret >> 8) as u8 & 0xFF;
                    self.reg.a = (ret >> 0) as u8 & 0xFF;
                    // フラグ更新
                    self.set_psw_flag(PSW_FLAG_N, (ret >> 15) != 0);
                    self.set_psw_flag(PSW_FLAG_H, half_carry);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    if !arith_overflow {
                        self.set_psw_flag(PSW_FLAG_V, false);
                        self.set_psw_flag(PSW_FLAG_C, true);
                    } else if sign_overflow {
                        self.set_psw_flag(PSW_FLAG_V, true);
                        self.set_psw_flag(PSW_FLAG_C, false);
                    } else {
                        self.set_psw_flag(PSW_FLAG_V, false);
                        self.set_psw_flag(PSW_FLAG_C, false);
                    }
                    5
                }
                _ => panic!("Invalid oprand!"),
            },
            // スタック操作命令
            SPCOpcode::PUSH { oprand } => {
                match oprand {
                    SPCOprand::Accumulator => {
                        self.push_stack(self.reg.a);
                    }
                    SPCOprand::XIndexRegister => {
                        self.push_stack(self.reg.x);
                    }
                    SPCOprand::YIndexRegister => {
                        self.push_stack(self.reg.y);
                    }
                    SPCOprand::ProgramStatusWord => {
                        self.push_stack(self.reg.psw);
                    }
                    _ => panic!("Invalid oprand!"),
                }
                4
            }
            SPCOpcode::POP { oprand } => {
                match oprand {
                    SPCOprand::Accumulator => {
                        self.reg.a = self.pop_stack();
                    }
                    SPCOprand::XIndexRegister => {
                        self.reg.x = self.pop_stack();
                    }
                    SPCOprand::YIndexRegister => {
                        self.reg.y = self.pop_stack();
                    }
                    SPCOprand::ProgramStatusWord => {
                        self.reg.psw = self.pop_stack();
                    }
                    _ => panic!("Invalid oprand!"),
                }
                4
            }
            // 論理演算命令
            SPCOpcode::AND { oprand } => self.execute_and(oprand),
            SPCOpcode::ASL { oprand } => self.execute_asl(oprand),
            SPCOpcode::EOR { oprand } => self.execute_eor(oprand),
            SPCOpcode::LSR { oprand } => self.execute_lsr(oprand),
            SPCOpcode::OR { oprand } => self.execute_or(oprand),
            SPCOpcode::ROL { oprand } => self.execute_rol(oprand),
            SPCOpcode::ROR { oprand } => self.execute_ror(oprand),
            // ビット操作命令
            SPCOpcode::AND1 { oprand } => self.execute_and1(oprand),
            SPCOpcode::CLR1 { bit, oprand } => match oprand {
                SPCOprand::DirectPageBit { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let memval = self.read_ram_u8(address);
                    self.write_ram_u8(address, memval & !(1 << (*bit)));
                    4
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::EOR1 { oprand } => match oprand {
                SPCOprand::AbsoluteBit { address_bit } => {
                    let (bit_pos, address) = get_address_bit(*address_bit);
                    let memval = self.read_ram_u8(address);
                    let ret = (self.reg.psw & PSW_FLAG_C) ^ ((memval >> bit_pos) & 0x1);
                    self.set_psw_flag(PSW_FLAG_C, ret != 0);
                    5
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::OR1 { oprand } => self.execute_or1(oprand),
            SPCOpcode::MOV1 { oprand } => match oprand {
                SPCOprand::AbsoluteMemoryBitToCarrayFlag { address_bit } => {
                    let (bit_pos, address) = get_address_bit(*address_bit);
                    let memval = self.read_ram_u8(address);
                    self.set_psw_flag(PSW_FLAG_C, ((memval >> bit_pos) & 0x1) != 0);
                    4
                }
                SPCOprand::CarrayFlagToAbsoluteMemoryBit { address_bit } => {
                    let (bit_pos, address) = get_address_bit(*address_bit);
                    let mask = (self.reg.psw & PSW_FLAG_C) << bit_pos;
                    let memval = self.read_ram_u8(address);
                    self.write_ram_u8(
                        address,
                        if mask != 0 {
                            memval | mask
                        } else {
                            memval & !mask
                        },
                    );
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::NOT1 { oprand } => match oprand {
                SPCOprand::AbsoluteBit { address_bit } => {
                    let (bit_pos, address) = get_address_bit(*address_bit);
                    let memval = self.read_ram_u8(address);
                    self.write_ram_u8(address, memval ^ (1 << bit_pos));
                    5
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::SET1 { bit, oprand } => match oprand {
                SPCOprand::DirectPageBit { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let memval = self.read_ram_u8(address);
                    self.write_ram_u8(address, memval | (1 << (*bit)));
                    4
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::TSET1 { oprand } => match oprand {
                SPCOprand::Absolute { address } => {
                    let addr = *address as usize;
                    let memval = self.read_ram_u8(addr);
                    let or = self.reg.a | memval;
                    let and = self.reg.a & memval;
                    self.write_ram_u8(addr, or);
                    self.set_psw_flag(PSW_FLAG_N, (or & PSW_FLAG_N) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, and == 0);
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::TCLR1 { oprand } => match oprand {
                SPCOprand::Absolute { address } => {
                    let memval = self.read_ram_u8(*address as usize);
                    let ret = memval & !self.reg.a;
                    self.write_ram_u8(*address as usize, ret);
                    self.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            // 比較命令
            SPCOpcode::CMP { oprand } => self.execute_cmp(oprand),
            SPCOpcode::CMPW { oprand } => match oprand {
                SPCOprand::DirectPage { direct_page } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let wval = self.read_ram_u16(address) as i32;
                    let ya = ((self.reg.y as i32) << 8) | self.reg.a as i32;
                    let ret = ya - wval;
                    // フラグ更新
                    self.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N as i32) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    self.set_psw_flag(PSW_FLAG_C, ret >= 0);
                    4
                }
                _ => panic!("Invalid oprand!"),
            },
            // フラグ操作命令
            SPCOpcode::CLRC => {
                self.set_psw_flag(PSW_FLAG_C, false);
                2
            }
            SPCOpcode::CLRP => {
                self.set_psw_flag(PSW_FLAG_P, false);
                2
            }
            SPCOpcode::CLRV => {
                self.set_psw_flag(PSW_FLAG_V, false);
                self.set_psw_flag(PSW_FLAG_H, false);
                2
            }
            SPCOpcode::NOTC => {
                self.set_psw_flag(PSW_FLAG_C, !self.test_psw_flag(PSW_FLAG_C));
                3
            }
            SPCOpcode::SETC => {
                self.set_psw_flag(PSW_FLAG_C, true);
                2
            }
            SPCOpcode::SETP => {
                self.set_psw_flag(PSW_FLAG_P, true);
                2
            }
            // 分岐命令
            SPCOpcode::BBC { bit, oprand } => match oprand {
                SPCOprand::DirectPageBitPCRelative {
                    direct_page,
                    pc_relative,
                } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let memval = self.read_ram_u8(address);
                    if memval & (1 << (*bit)) == 0 {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        7
                    } else {
                        5
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BBS { bit, oprand } => match oprand {
                SPCOprand::DirectPageBitPCRelative {
                    direct_page,
                    pc_relative,
                } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let memval = self.read_ram_u8(address);
                    if memval & (1 << (*bit)) != 0 {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        7
                    } else {
                        5
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BCC { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if !self.test_psw_flag(PSW_FLAG_C) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BCS { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if self.test_psw_flag(PSW_FLAG_C) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BEQ { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if self.test_psw_flag(PSW_FLAG_Z) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BMI { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if self.test_psw_flag(PSW_FLAG_N) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BNE { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if !self.test_psw_flag(PSW_FLAG_Z) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BPL { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if !self.test_psw_flag(PSW_FLAG_N) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BRA { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                    4
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BVC { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if !self.test_psw_flag(PSW_FLAG_V) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::BVS { oprand } => match oprand {
                SPCOprand::PCRelative { pc_relative } => {
                    if self.test_psw_flag(PSW_FLAG_V) {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        4
                    } else {
                        2
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::CBNE { oprand } => match oprand {
                SPCOprand::DirectPagePCRelative {
                    direct_page,
                    pc_relative,
                } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let memval = self.read_ram_u8(address);
                    if self.reg.a != memval {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        7
                    } else {
                        5
                    }
                }
                SPCOprand::DirectPageXPCRelative {
                    direct_page,
                    pc_relative,
                } => {
                    let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                    let memval = self.read_ram_u8(address);
                    if self.reg.a != memval {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        8
                    } else {
                        6
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::DBNZ { oprand } => match oprand {
                SPCOprand::DirectPagePCRelative {
                    direct_page,
                    pc_relative,
                } => {
                    let address = self.get_direct_page_address(*direct_page);
                    let mut memval = self.read_ram_u8(address);
                    memval = memval.overflowing_sub(1).0;
                    self.write_ram_u8(address, memval);
                    if memval != 0 {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        6
                    } else {
                        4
                    }
                }
                SPCOprand::YPCRelative { pc_relative } => {
                    self.reg.y = self.reg.y.overflowing_sub(1).0;
                    if self.reg.y != 0 {
                        self.reg.pc = (self.reg.pc as i32 + *pc_relative as i32) as u16;
                        7
                    } else {
                        5
                    }
                }
                _ => panic!("Invalid oprand!"),
            },
            // ジャンプ命令
            SPCOpcode::CALL { oprand } => match oprand {
                SPCOprand::Absolute { address } => {
                    self.push_stack(((self.reg.pc >> 8) & 0xFF) as u8);
                    self.push_stack(((self.reg.pc >> 0) & 0xFF) as u8);
                    self.reg.pc = *address;
                    8
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::JMP { oprand } => match oprand {
                SPCOprand::Absolute { address } => {
                    self.reg.pc = *address;
                    3
                }
                SPCOprand::AbsoluteXIndirect { address } => {
                    let addr = (*address + self.reg.x as u16) as usize;
                    let jmp_pc = self.read_ram_u16(addr);
                    self.reg.pc = jmp_pc;
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::PCALL { oprand } => match oprand {
                SPCOprand::PageAddress { address } => {
                    self.push_stack(((self.reg.pc >> 8) & 0xFF) as u8);
                    self.push_stack(((self.reg.pc >> 0) & 0xFF) as u8);
                    self.reg.pc = 0xFF00u16 | *address as u16;
                    6
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::TCALL { table_index } => {
                let address = 0xFFDEusize - (*table_index * 2) as usize;
                let jmp_pc = self.read_ram_u16(address);
                self.push_stack(((self.reg.pc >> 8) & 0xFF) as u8);
                self.push_stack(((self.reg.pc >> 0) & 0xFF) as u8);
                self.reg.pc = jmp_pc;
                8
            }
            SPCOpcode::RET => {
                let low = self.pop_stack() as u16;
                let high = self.pop_stack() as u16;
                self.reg.pc = (high << 8) | low;
                5
            }
            // 十進補正命令
            SPCOpcode::DAA { oprand } => match oprand {
                SPCOprand::Accumulator => {
                    let mut ret = self.reg.a;
                    let mut carry = self.test_psw_flag(PSW_FLAG_C);
                    // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を足す
                    if self.test_psw_flag(PSW_FLAG_H) || (ret & 0x0F) >= 0xA {
                        (ret, carry) = ret.overflowing_add(0x06);
                    }
                    // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を足す
                    if !self.test_psw_flag(PSW_FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                        (ret, carry) = ret.overflowing_add(0x60);
                    }
                    // 最上位ビットにキャリーフラグをセットする
                    ret = if self.test_psw_flag(PSW_FLAG_C) {
                        ret | 0x80
                    } else {
                        ret & 0x7F
                    };
                    self.reg.a = ret;
                    self.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    self.set_psw_flag(PSW_FLAG_C, carry);
                    3
                }
                _ => panic!("Invalid oprand!"),
            },
            SPCOpcode::DAS { oprand } => match oprand {
                SPCOprand::Accumulator => {
                    let mut ret = self.reg.a;
                    let mut carry = self.test_psw_flag(PSW_FLAG_C);
                    // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を引く
                    if self.test_psw_flag(PSW_FLAG_H) || (ret & 0x0F) >= 0xA {
                        (ret, carry) = ret.overflowing_sub(0x06);
                    }
                    // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を引く
                    if !self.test_psw_flag(PSW_FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                        (ret, carry) = ret.overflowing_sub(0x60);
                    }
                    // 最上位ビットにキャリーフラグをセットする
                    ret = if self.test_psw_flag(PSW_FLAG_C) {
                        ret | 0x80
                    } else {
                        ret & 0x7F
                    };
                    self.reg.a = ret;
                    self.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                    self.set_psw_flag(PSW_FLAG_Z, ret == 0);
                    self.set_psw_flag(PSW_FLAG_C, carry);
                    3
                }
                _ => panic!("Invalid oprand!"),
            },
            // 割り込み命令
            SPCOpcode::EI => {
                panic!("This emulator does not support EI instruction!");
            }
            SPCOpcode::DI => {
                panic!("This emulator does not support DI instruction!");
            }
            SPCOpcode::BRK => {
                panic!("This emulator does not support BRK instruction!");
            }
            SPCOpcode::RETI => {
                panic!("This emulator does not support RETI instruction!");
            }
            // その他の命令
            SPCOpcode::SLEEP => {
                panic!("This emulator does not support SLEEP instruction!");
            }
            SPCOpcode::STOP => {
                panic!("This emulator does not support STOP instruction!");
            }
        }
    }

    /// MOV命令の実行
    fn execute_mov(&mut self, oprand: &SPCOprand) -> u8 {
        let val;
        let cycle;

        // オペランドに応じて代入値と代入先を切り替え
        match oprand {
            SPCOprand::ImmediateToA { immediate } => {
                val = *immediate;
                self.reg.a = val;
                cycle = 2;
            }
            SPCOprand::IndirectToA => {
                val = self.read_ram_u8(self.reg.x as usize);
                self.reg.a = val;
                cycle = 3;
            }
            SPCOprand::IndirectAutoIncrementToA => {
                val = self.read_ram_u8(self.reg.x as usize);
                self.reg.a = val;
                self.reg.x = self.reg.x.overflowing_add(1).0;
                cycle = 4;
            }
            SPCOprand::DirectPageToA { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.read_ram_u8(address);
                self.reg.a = val;
                cycle = 3;
            }
            SPCOprand::DirectPageXToA { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                val = self.read_ram_u8(address);
                self.reg.a = val;
                cycle = 4;
            }
            SPCOprand::AbsoluteToA { address } => {
                val = self.read_ram_u8(*address as usize);
                self.reg.a = val;
                cycle = 4;
            }
            SPCOprand::AbsoluteX { address } => {
                val = self.read_ram_u8((*address + self.reg.x as u16) as usize);
                self.reg.a = val;
                cycle = 5;
            }
            SPCOprand::AbsoluteY { address } => {
                val = self.read_ram_u8((*address + self.reg.y as u16) as usize);
                self.reg.a = val;
                cycle = 5;
            }
            SPCOprand::DirectPageXIndirect { direct_page } => {
                let address = self.get_direct_page_x_indexed_indirect_address(*direct_page);
                val = self.read_ram_u8(address);
                self.reg.a = val;
                cycle = 6;
            }
            SPCOprand::DirectPageIndirectY { direct_page } => {
                let address = self.get_direct_page_indirect_y_indexed_address(*direct_page);
                val = self.read_ram_u8(address);
                self.reg.a = val;
                cycle = 6;
            }
            SPCOprand::ImmediateToX { immediate } => {
                val = *immediate;
                self.reg.x = val;
                cycle = 2;
            }
            SPCOprand::DirectPageToX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.read_ram_u8(address);
                self.reg.x = val;
                cycle = 3;
            }
            SPCOprand::DirectPageYToX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.y as usize;
                val = self.read_ram_u8(address);
                self.reg.x = val;
                cycle = 4;
            }
            SPCOprand::AbsoluteToX { address } => {
                val = self.read_ram_u8(*address as usize);
                self.reg.x = val;
                cycle = 4;
            }
            SPCOprand::ImmediateToY { immediate } => {
                val = *immediate;
                self.reg.y = val;
                cycle = 2;
            }
            SPCOprand::DirectPageToY { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.read_ram_u8(address);
                self.reg.y = val;
                cycle = 3;
            }
            SPCOprand::DirectPageXToY { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                val = self.read_ram_u8(address);
                self.reg.y = val;
                cycle = 4;
            }
            SPCOprand::AbsoluteToY { address } => {
                val = self.read_ram_u8(*address as usize);
                self.reg.y = val;
                cycle = 4;
            }
            SPCOprand::AToIndirect => {
                let address = self.get_direct_page_address(self.reg.x);
                val = self.reg.a;
                self.write_ram_u8(address, val);
                cycle = 4;
            }
            SPCOprand::AToIndirectAutoIncrement => {
                let address = self.get_direct_page_address(self.reg.x);
                val = self.reg.a;
                self.write_ram_u8(address, val);
                self.reg.x += 1;
                cycle = 4;
            }
            SPCOprand::AToDirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.reg.a;
                self.write_ram_u8(address, val);
                cycle = 4;
            }
            SPCOprand::AToDirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                val = self.reg.a;
                self.write_ram_u8(address, val);
                cycle = 5;
            }
            SPCOprand::AToAbsolute { address } => {
                val = self.reg.a;
                self.write_ram_u8(*address as usize, val);
                cycle = 5;
            }
            SPCOprand::AToAbsoluteX { address } => {
                val = self.reg.a;
                self.write_ram_u8((*address + (self.reg.x as u16)) as usize, val);
                cycle = 6;
            }
            SPCOprand::AToAbsoluteY { address } => {
                val = self.reg.a;
                self.write_ram_u8((*address + (self.reg.y as u16)) as usize, val);
                cycle = 6;
            }
            SPCOprand::AToDirectPageXIndirect { direct_page } => {
                let address = self.get_direct_page_x_indexed_indirect_address(*direct_page);
                val = self.reg.a;
                self.write_ram_u8(address, val);
                cycle = 7;
            }
            SPCOprand::AToDirectPageIndirectY { direct_page } => {
                let address = self.get_direct_page_indirect_y_indexed_address(*direct_page);
                val = self.reg.a;
                self.write_ram_u8(address, val);
                cycle = 7;
            }
            SPCOprand::XToDirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.reg.x;
                self.write_ram_u8(address, val);
                cycle = 4;
            }
            SPCOprand::XToDirectPageY { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.y as usize;
                val = self.reg.x;
                self.write_ram_u8(address, val);
                cycle = 5;
            }
            SPCOprand::XToAbsolute { address } => {
                val = self.reg.x;
                self.write_ram_u8(*address as usize, val);
                cycle = 5;
            }
            SPCOprand::YToDirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                val = self.reg.y;
                self.write_ram_u8(address, val);
                cycle = 4;
            }
            SPCOprand::YToDirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                val = self.reg.y;
                self.write_ram_u8(address, val);
                cycle = 5;
            }
            SPCOprand::YToAbsolute { address } => {
                val = self.reg.y;
                self.write_ram_u8(*address as usize, val);
                cycle = 5;
            }
            SPCOprand::XToA => {
                val = self.reg.x;
                self.reg.a = val;
                cycle = 2;
            }
            SPCOprand::YToA => {
                val = self.reg.y;
                self.reg.a = val;
                cycle = 2;
            }
            SPCOprand::AToX => {
                val = self.reg.a;
                self.reg.x = val;
                cycle = 2;
            }
            SPCOprand::AToY => {
                val = self.reg.a;
                self.reg.y = val;
                cycle = 2;
            }
            SPCOprand::StackPointerToX => {
                val = self.reg.sp;
                self.reg.x = val;
                cycle = 2;
            }
            SPCOprand::XToStackPointer => {
                val = self.reg.x;
                self.reg.sp = val;
                cycle = 2;
            }
            SPCOprand::DirectPageToDirectPage {
                direct_page_dst,
                direct_page_src,
            } => {
                let dst_address = self.get_direct_page_address(*direct_page_dst);
                let src_address = self.get_direct_page_address(*direct_page_src);
                val = self.read_ram_u8(src_address);
                self.write_ram_u8(dst_address, val);
                cycle = 5;
            }
            SPCOprand::ImmediateToDirectPage {
                direct_page,
                immediate,
            } => {
                let address = self.get_direct_page_address(*direct_page);
                val = *immediate;
                self.write_ram_u8(address, val);
                cycle = 5;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (val & PSW_FLAG_N) != 0);
        self.set_psw_flag(PSW_FLAG_Z, val == 0);

        cycle
    }

    /// OR命令の実行
    fn execute_or(&mut self, oprand: &SPCOprand) -> u8 {
        fn or(a: u8, b: u8) -> u8 {
            a | b
        }
        self.execute_binary_logical_operation(oprand, or)
    }

    /// AND命令の実行
    fn execute_and(&mut self, oprand: &SPCOprand) -> u8 {
        fn and(a: u8, b: u8) -> u8 {
            a & b
        }
        self.execute_binary_logical_operation(oprand, and)
    }

    /// AND命令の実行
    fn execute_eor(&mut self, oprand: &SPCOprand) -> u8 {
        fn eor(a: u8, b: u8) -> u8 {
            a ^ b
        }
        self.execute_binary_logical_operation(oprand, eor)
    }

    /// 2項論理演算の実行
    fn execute_binary_logical_operation(&mut self, oprand: &SPCOprand, op: fn(u8, u8) -> u8) -> u8 {
        let ret;
        let cycle;

        match oprand {
            SPCOprand::Immediate { immediate } => {
                ret = op(self.reg.a, *immediate);
                self.reg.a = ret;
                cycle = 2;
            }
            SPCOprand::IndirectPage => {
                let memval = self.read_ram_u8(self.reg.x as usize);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 3;
            }
            SPCOprand::DirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 3;
            }
            SPCOprand::DirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 4;
            }
            SPCOprand::Absolute { address } => {
                let memval = self.read_ram_u8(*address as usize);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 4;
            }
            SPCOprand::AbsoluteX { address } => {
                let memval = self.read_ram_u8((*address + self.reg.x as u16) as usize);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 5;
            }
            SPCOprand::AbsoluteY { address } => {
                let memval = self.read_ram_u8((*address + self.reg.y as u16) as usize);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 5;
            }
            SPCOprand::DirectPageXIndirect { direct_page } => {
                let address = self.get_direct_page_x_indexed_indirect_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 6;
            }
            SPCOprand::DirectPageIndirectY { direct_page } => {
                let address = self.get_direct_page_indirect_y_indexed_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.a, memval);
                self.reg.a = ret;
                cycle = 6;
            }
            SPCOprand::IndirectPageToIndirectPage => {
                let dst_address = self.get_direct_page_address(self.reg.x);
                let src_address = self.get_direct_page_address(self.reg.y);
                let dst_memval = self.read_ram_u8(dst_address);
                let src_memval = self.read_ram_u8(src_address);
                ret = op(dst_memval, src_memval);
                self.write_ram_u8(dst_address, ret);
                cycle = 5;
            }
            SPCOprand::DirectPageToDirectPage {
                direct_page_dst,
                direct_page_src,
            } => {
                let dst_address = self.get_direct_page_address(*direct_page_dst);
                let src_address = self.get_direct_page_address(*direct_page_src);
                let dst_memval = self.read_ram_u8(dst_address);
                let src_memval = self.read_ram_u8(src_address);
                ret = op(dst_memval, src_memval);
                self.write_ram_u8(dst_address, ret);
                cycle = 6;
            }
            SPCOprand::ImmediateToDirectPage {
                direct_page,
                immediate,
            } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = op(memval, *immediate);
                self.write_ram_u8(address, ret);
                cycle = 5;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
        self.set_psw_flag(PSW_FLAG_Z, ret == 0);

        cycle
    }

    /// ASL命令の実行
    fn execute_asl(&mut self, oprand: &SPCOprand) -> u8 {
        fn asl(a: u8) -> u8 {
            // NOTE: 最上位ビットはキャリーフラグに入る（よくある算術左シフトと異なる）
            a << 1
        }
        self.execute_unary_bit_opration(oprand, asl)
    }

    /// ROL命令の実行
    fn execute_rol(&mut self, oprand: &SPCOprand) -> u8 {
        fn rol(a: u8) -> u8 {
            let msb = a >> 7;
            (a << 1) | msb
        }
        self.execute_unary_bit_opration(oprand, rol)
    }

    /// ROR命令の実行
    fn execute_ror(&mut self, oprand: &SPCOprand) -> u8 {
        fn ror(a: u8) -> u8 {
            let lsb = a & 1;
            (a >> 1) | (lsb << 7)
        }
        self.execute_unary_bit_opration(oprand, ror)
    }

    /// LSR命令の実行
    fn execute_lsr(&mut self, oprand: &SPCOprand) -> u8 {
        fn lsr(a: u8) -> u8 {
            a >> 1
        }
        self.execute_unary_bit_opration(oprand, lsr)
    }

    /// 単項ビット演算命令の実行
    fn execute_unary_bit_opration(&mut self, oprand: &SPCOprand, op: fn(u8) -> u8) -> u8 {
        let ret;
        let prev_msb;
        let cycle;

        match oprand {
            SPCOprand::Accumulator => {
                prev_msb = self.reg.a & 0x80;
                ret = op(self.reg.a);
                self.reg.a = ret;
                cycle = 2;
            }
            SPCOprand::DirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                prev_msb = memval & 0x80;
                ret = op(memval);
                self.write_ram_u8(address, ret);
                cycle = 4;
            }
            SPCOprand::DirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                let memval = self.read_ram_u8(address);
                prev_msb = memval & 0x80;
                ret = op(memval);
                self.write_ram_u8(address, ret);
                cycle = 5;
            }
            SPCOprand::Absolute { address } => {
                let addr = *address as usize;
                let memval = self.read_ram_u8(addr);
                prev_msb = memval & 0x80;
                ret = op(memval);
                self.write_ram_u8(addr, ret);
                cycle = 5;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
        self.set_psw_flag(PSW_FLAG_Z, ret == 0);
        self.set_psw_flag(PSW_FLAG_C, prev_msb != 0);

        cycle
    }

    /// OR1命令の実行
    fn execute_or1(&mut self, oprand: &SPCOprand) -> u8 {
        fn or(a: u8, b: u8) -> bool {
            (a | b) != 0
        }
        self.execute_bit_operation_with_carry(oprand, or);
        5
    }

    /// AND1命令の実行
    fn execute_and1(&mut self, oprand: &SPCOprand) -> u8 {
        fn and(a: u8, b: u8) -> bool {
            (a & b) != 0
        }
        self.execute_bit_operation_with_carry(oprand, and);
        4
    }

    /// キャリーフラグとのビット演算の実行
    fn execute_bit_operation_with_carry(&mut self, oprand: &SPCOprand, op: fn(u8, u8) -> bool) {
        let ret;

        match oprand {
            SPCOprand::AbsoluteBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.psw & PSW_FLAG_C, (memval >> bit_pos) & 0x1);
            }
            SPCOprand::AbsoluteInverseBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let memval = self.read_ram_u8(address);
                ret = op(self.reg.psw & PSW_FLAG_C, !((memval >> bit_pos) & 0x1));
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_C, ret);
    }

    /// INC命令の実行
    fn execute_inc(&mut self, oprand: &SPCOprand) -> u8 {
        fn inc(a: u8) -> u8 {
            a.overflowing_add(1).0
        }
        self.execute_inc_dec(oprand, inc)
    }

    /// DEC命令の実行
    fn execute_dec(&mut self, oprand: &SPCOprand) -> u8 {
        fn dec(a: u8) -> u8 {
            a.overflowing_sub(1).0
        }
        self.execute_inc_dec(oprand, dec)
    }

    /// INC/DEC命令の実行
    fn execute_inc_dec(&mut self, oprand: &SPCOprand, op: fn(u8) -> u8) -> u8 {
        let ret;
        let cycle;

        match oprand {
            SPCOprand::Accumulator => {
                ret = op(self.reg.a);
                self.reg.a = ret;
                cycle = 2;
            }
            SPCOprand::DirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = op(memval);
                self.write_ram_u8(address, ret);
                cycle = 4;
            }
            SPCOprand::DirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                let memval = self.read_ram_u8(address);
                ret = op(memval);
                self.write_ram_u8(address, ret);
                cycle = 5;
            }
            SPCOprand::Absolute { address } => {
                let memval = self.read_ram_u8(*address as usize);
                ret = op(memval);
                self.write_ram_u8(*address as usize, ret);
                cycle = 5;
            }
            SPCOprand::XIndexRegister => {
                ret = op(self.reg.x);
                self.reg.x = ret;
                cycle = 2;
            }
            SPCOprand::YIndexRegister => {
                ret = op(self.reg.y);
                self.reg.y = ret;
                cycle = 2;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
        self.set_psw_flag(PSW_FLAG_Z, ret == 0);

        cycle
    }

    /// CMP命令の実行
    fn execute_cmp(&mut self, oprand: &SPCOprand) -> u8 {
        let ret;
        let cycle;

        match oprand {
            SPCOprand::Immediate { immediate } => {
                ret = self.reg.a as i16 - *immediate as i16;
                cycle = 2;
            }
            SPCOprand::IndirectPage => {
                let memval = self.read_ram_u8(self.reg.x as usize);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 3;
            }
            SPCOprand::DirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 3;
            }
            SPCOprand::DirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                let memval = self.read_ram_u8(address);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 4;
            }
            SPCOprand::Absolute { address } => {
                let memval = self.read_ram_u8(*address as usize);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 4;
            }
            SPCOprand::AbsoluteX { address } => {
                let addr = *address + self.reg.x as u16;
                let memval = self.read_ram_u8(addr as usize);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 5;
            }
            SPCOprand::AbsoluteY { address } => {
                let addr = *address + self.reg.y as u16;
                let memval = self.read_ram_u8(addr as usize);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 5;
            }
            SPCOprand::DirectPageXIndirect { direct_page } => {
                let address = self.get_direct_page_x_indexed_indirect_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 6;
            }
            SPCOprand::DirectPageIndirectY { direct_page } => {
                let address = self.get_direct_page_indirect_y_indexed_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = self.reg.a as i16 - memval as i16;
                cycle = 6;
            }
            SPCOprand::IndirectPageToIndirectPage => {
                let address1 = self.get_direct_page_address(self.reg.x);
                let address2 = self.get_direct_page_address(self.reg.y);
                let memval1 = self.read_ram_u8(address1);
                let memval2 = self.read_ram_u8(address2);
                ret = memval1 as i16 - memval2 as i16;
                cycle = 6;
            }
            SPCOprand::DirectPageToDirectPage {
                direct_page_dst,
                direct_page_src,
            } => {
                let address1 = self.get_direct_page_address(*direct_page_dst);
                let address2 = self.get_direct_page_address(*direct_page_src);
                let memval1 = self.read_ram_u8(address1);
                let memval2 = self.read_ram_u8(address2);
                ret = memval1 as i16 - memval2 as i16;
                cycle = 6;
            }
            SPCOprand::ImmediateToDirectPage {
                direct_page,
                immediate,
            } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = memval as i16 - *immediate as i16;
                cycle = 5;
            }
            SPCOprand::ImmediateToX { immediate } => {
                ret = self.reg.x as i16 - *immediate as i16;
                cycle = 2;
            }
            SPCOprand::DirectPageToX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = self.reg.x as i16 - memval as i16;
                cycle = 3;
            }
            SPCOprand::AbsoluteToX { address } => {
                let memval = self.read_ram_u8(*address as usize);
                ret = self.reg.x as i16 - memval as i16;
                cycle = 4;
            }
            SPCOprand::ImmediateToY { immediate } => {
                ret = self.reg.y as i16 - *immediate as i16;
                cycle = 2;
            }
            SPCOprand::DirectPageToY { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                ret = self.reg.y as i16 - memval as i16;
                cycle = 3;
            }
            SPCOprand::AbsoluteToY { address } => {
                let memval = self.read_ram_u8(*address as usize);
                ret = self.reg.y as i16 - memval as i16;
                cycle = 4;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N as i16) != 0);
        self.set_psw_flag(PSW_FLAG_Z, ret == 0);
        self.set_psw_flag(PSW_FLAG_C, ret >= 0);

        cycle
    }

    /// ADC命令の実行
    fn execute_adc(&mut self, oprand: &SPCOprand) -> u8 {
        fn add(a: u8, b: u8, carry: bool) -> (u8, bool, bool, bool) {
            let mut ret = (a as u16) + (b as u16);
            if carry {
                ret += 1;
            }
            (
                (ret & 0xFF) as u8,
                (ret & 0x100) != 0,
                ((a & 0x80) == (b & 0x80)) && (((a & 0x80) as u16) != (ret & 0x80)),
                check_half_carry_add_u8(a, b),
            )
        }
        self.execute_adc_sbc(oprand, add)
    }

    /// SBC命令の実行
    fn execute_sbc(&mut self, oprand: &SPCOprand) -> u8 {
        fn sub(a: u8, b: u8, carry: bool) -> (u8, bool, bool, bool) {
            let mut ret = (a as i16) - (b as i16);
            if !carry {
                ret -= 1;
            }
            (
                (ret & 0xFF) as u8,
                (ret & 0x100) != 0,
                ((a & 0x80) != (b & 0x80)) && (((a & 0x80) as i16) != (ret & 0x80)),
                check_half_carry_sub_u8(a, b),
            )
        }
        self.execute_adc_sbc(oprand, sub)
    }

    /// ADC/SBC命令の実行共通ルーチン
    fn execute_adc_sbc(
        &mut self,
        oprand: &SPCOprand,
        op: fn(u8, u8, bool) -> (u8, bool, bool, bool),
    ) -> u8 {
        let ret;
        let arith_overflow;
        let sign_overflow;
        let half_carry;
        let cycle;

        match oprand {
            SPCOprand::Immediate { immediate } => {
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, *immediate, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 2;
            }
            SPCOprand::IndirectPage => {
                let memval = self.read_ram_u8(self.reg.x as usize);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 3;
            }
            SPCOprand::DirectPage { direct_page } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 3;
            }
            SPCOprand::DirectPageX { direct_page } => {
                let address = self.get_direct_page_address(*direct_page) + self.reg.x as usize;
                let memval = self.read_ram_u8(address);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 4;
            }
            SPCOprand::Absolute { address } => {
                let memval = self.read_ram_u8(*address as usize);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 4;
            }
            SPCOprand::AbsoluteX { address } => {
                let addr = *address + self.reg.x as u16;
                let memval = self.read_ram_u8(addr as usize);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 5;
            }
            SPCOprand::AbsoluteY { address } => {
                let addr = *address + self.reg.y as u16;
                let memval = self.read_ram_u8(addr as usize);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 5;
            }
            SPCOprand::DirectPageXIndirect { direct_page } => {
                let address = self.get_direct_page_x_indexed_indirect_address(*direct_page);
                let memval = self.read_ram_u8(address);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 6;
            }
            SPCOprand::DirectPageIndirectY { direct_page } => {
                let address = self.get_direct_page_indirect_y_indexed_address(*direct_page);
                let memval = self.read_ram_u8(address);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(self.reg.a, memval, self.test_psw_flag(PSW_FLAG_C));
                self.reg.a = ret;
                cycle = 6;
            }
            SPCOprand::IndirectPageToIndirectPage => {
                let address1 = self.get_direct_page_address(self.reg.x);
                let address2 = self.get_direct_page_address(self.reg.y);
                let memval1 = self.read_ram_u8(address1);
                let memval2 = self.read_ram_u8(address2);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(memval1, memval2, self.test_psw_flag(PSW_FLAG_C));
                self.write_ram_u8(address1, ret);
                cycle = 5;
            }
            SPCOprand::DirectPageToDirectPage {
                direct_page_dst,
                direct_page_src,
            } => {
                let address_dst = self.get_direct_page_address(*direct_page_dst);
                let address_src = self.get_direct_page_address(*direct_page_src);
                let memval_dst = self.read_ram_u8(address_dst);
                let memval_src = self.read_ram_u8(address_src);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(memval_dst, memval_src, self.test_psw_flag(PSW_FLAG_C));
                self.write_ram_u8(address_dst, ret);
                cycle = 6;
            }
            SPCOprand::ImmediateToDirectPage {
                direct_page,
                immediate,
            } => {
                let address = self.get_direct_page_address(*direct_page);
                let memval = self.read_ram_u8(address);
                (ret, arith_overflow, sign_overflow, half_carry) =
                    op(memval, *immediate, self.test_psw_flag(PSW_FLAG_C));
                self.write_ram_u8(address, ret);
                cycle = 5;
            }
            _ => panic!("Invalid oprand!"),
        }

        // フラグ更新
        self.set_psw_flag(PSW_FLAG_N, (self.reg.a & PSW_FLAG_N) != 0);
        self.set_psw_flag(PSW_FLAG_H, half_carry);
        self.set_psw_flag(PSW_FLAG_Z, ret == 0);
        if arith_overflow {
            self.set_psw_flag(PSW_FLAG_V, false);
            self.set_psw_flag(PSW_FLAG_C, true);
        } else if sign_overflow {
            self.set_psw_flag(PSW_FLAG_V, true);
            self.set_psw_flag(PSW_FLAG_C, false);
        } else {
            self.set_psw_flag(PSW_FLAG_V, false);
            self.set_psw_flag(PSW_FLAG_C, false);
        }

        cycle
    }
}

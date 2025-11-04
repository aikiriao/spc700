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

/// RAMへの書き込み（デバッグするため関数化）
fn write_ram_u8(ram: &mut [u8], address: usize, value: u8) {
    ram[address] = value;
    println!("W: 0x{:04X} <- {:02X}", address, value);
}

/// RAMからの読み込み（デバッグのため関数化）
fn read_ram_u8(ram: &mut [u8], address: usize) -> u8 {
    println!("R: 0x{:04X} -> {:02X}", address, ram[address]);
    ram[address]
}

impl SPCRegister {
    /// スタックのベースアドレス
    const STACK_BASE_ADDRESS: usize = 0x100usize;

    /// ダイレクトページのアドレスを取得
    fn get_direct_page_address(&self, direct_page: u8) -> usize {
        if self.test_psw_flag(PSW_FLAG_P) {
            0x100usize + direct_page as usize
        } else {
            direct_page as usize
        }
    }

    /// ダイレクトページインデックス間接アドレスを取得
    fn get_direct_page_x_indexed_indirect_address(&self, ram: &mut [u8], direct_page: u8) -> usize {
        let dp_address = self.get_direct_page_address(direct_page) + self.x as usize;
        make_u16_from_u8(&ram[dp_address..(dp_address + 2)]) as usize
    }

    /// ダイレクトページ関接インデックスアドレスを取得
    fn get_direct_page_indirect_y_indexed_address(&self, ram: &mut [u8], direct_page: u8) -> usize {
        let dp_address = self.get_direct_page_address(direct_page);
        let address = make_u16_from_u8(&ram[dp_address..(dp_address + 2)]);
        (address + (self.y as u16)) as usize
    }

    /// フラグが立っているか検査
    fn test_psw_flag(&self, flag: u8) -> bool {
        (self.psw & flag) != 0
    }

    /// 条件conditionに依存し、pswのflagのset/resetを実行
    fn set_psw_flag(&mut self, flag: u8, condition: bool) {
        self.psw = if condition {
            self.psw | flag
        } else {
            self.psw & !flag
        };
    }

    /// スタックにデータをPUSH
    fn push_stack(&mut self, ram: &mut [u8], value: u8) {
        write_ram_u8(ram, Self::STACK_BASE_ADDRESS + self.sp as usize, value);
        self.sp -= 1;
    }

    /// スタックからデータをPOP
    fn pop_stack(&mut self, ram: &mut [u8]) -> u8 {
        self.sp += 1;
        ram[Self::STACK_BASE_ADDRESS + self.sp as usize]
    }
}

/// MOV命令の実行
fn execute_mov(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    let val;

    // オペランドに応じて代入値と代入先を切り替え
    match oprand {
        SPCOprand::ImmediateToA { immediate } => {
            val = *immediate;
            register.a = val;
        }
        SPCOprand::IndirectToA => {
            val = read_ram_u8(ram, register.x as usize);
            register.a = val;
        }
        SPCOprand::IndirectAutoIncrementToA => {
            val = read_ram_u8(ram, register.x as usize);
            register.a = val;
            register.x = register.x.overflowing_add(1).0;
        }
        SPCOprand::DirectPageToA { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = read_ram_u8(ram, address);
            register.a = val;
        }
        SPCOprand::DirectPageXToA { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = read_ram_u8(ram, address);
            register.a = val;
        }
        SPCOprand::AbsoluteToA { address } => {
            val = read_ram_u8(ram, *address as usize);
            register.a = val;
        }
        SPCOprand::AbsoluteX { address } => {
            val = read_ram_u8(ram, (*address + register.x as u16) as usize);
            register.a = val;
        }
        SPCOprand::AbsoluteY { address } => {
            val = read_ram_u8(ram, (*address + register.y as u16) as usize);
            register.a = val;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            val = read_ram_u8(ram, address);
            register.a = val;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            val = read_ram_u8(ram, address);
            register.a = val;
        }
        SPCOprand::ImmediateToX { immediate } => {
            val = *immediate;
            register.x = val;
        }
        SPCOprand::DirectPageToX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = read_ram_u8(ram, address);
            register.x = val;
        }
        SPCOprand::DirectPageYToX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.y as usize;
            val = read_ram_u8(ram, address);
            register.x = val;
        }
        SPCOprand::AbsoluteToX { address } => {
            val = read_ram_u8(ram, *address as usize);
            register.x = val;
        }
        SPCOprand::ImmediateToY { immediate } => {
            val = *immediate;
            register.y = val;
        }
        SPCOprand::DirectPageToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = read_ram_u8(ram, address);
            register.y = val;
        }
        SPCOprand::DirectPageXToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = read_ram_u8(ram, address);
            register.y = val;
        }
        SPCOprand::AbsoluteToY { address } => {
            val = read_ram_u8(ram, *address as usize);
            register.y = val;
        }
        SPCOprand::AToIndirect => {
            let address = register.get_direct_page_address(register.x);
            val = register.a;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::AToIndirectAutoIncrement => {
            let address = register.get_direct_page_address(register.x);
            val = register.a;
            write_ram_u8(ram, address, val);
            register.x += 1;
        }
        SPCOprand::AToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.a;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::AToDirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = register.a;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::AToAbsolute { address } => {
            val = register.a;
            write_ram_u8(ram, *address as usize, val);
        }
        SPCOprand::AToAbsoluteX { address } => {
            val = register.a;
            write_ram_u8(ram, (*address + (register.x as u16)) as usize, val);
        }
        SPCOprand::AToAbsoluteY { address } => {
            val = register.a;
            write_ram_u8(ram, (*address + (register.y as u16)) as usize, val);
        }
        SPCOprand::AToDirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            val = register.a;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::AToDirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            val = register.a;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::XToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.x;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::XToDirectPageY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.y as usize;
            val = register.x;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::XToAbsolute { address } => {
            val = register.x;
            write_ram_u8(ram, *address as usize, val);
        }
        SPCOprand::YToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.y;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::YToDirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = register.y;
            write_ram_u8(ram, address, val);
        }
        SPCOprand::YToAbsolute { address } => {
            val = register.y;
            write_ram_u8(ram, *address as usize, val);
        }
        SPCOprand::XToA => {
            val = register.x;
            register.a = val;
        }
        SPCOprand::YToA => {
            val = register.y;
            register.a = val;
        }
        SPCOprand::AToX => {
            val = register.a;
            register.x = val;
        }
        SPCOprand::AToY => {
            val = register.a;
            register.y = val;
        }
        SPCOprand::StackPointerToX => {
            val = register.sp;
            register.x = val;
        }
        SPCOprand::XToStackPointer => {
            val = register.x;
            register.sp = val;
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page_dst,
            direct_page_src,
        } => {
            let dst_address = register.get_direct_page_address(*direct_page_dst);
            let src_address = register.get_direct_page_address(*direct_page_src);
            val = read_ram_u8(ram, src_address);
            write_ram_u8(ram, dst_address, val);
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            val = *immediate;
            write_ram_u8(ram, address, val);
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (val & PSW_FLAG_N) != 0);
    register.set_psw_flag(PSW_FLAG_Z, val == 0);
}

/// OR命令の実行
fn execute_or(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn or(a: u8, b: u8) -> u8 {
        a | b
    }
    execute_binary_logical_operation(register, ram, oprand, or);
}

/// AND命令の実行
fn execute_and(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn and(a: u8, b: u8) -> u8 {
        a & b
    }
    execute_binary_logical_operation(register, ram, oprand, and);
}

/// AND命令の実行
fn execute_eor(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn eor(a: u8, b: u8) -> u8 {
        a ^ b
    }
    execute_binary_logical_operation(register, ram, oprand, eor);
}

/// 2項論理演算の実行
fn execute_binary_logical_operation(
    register: &mut SPCRegister,
    ram: &mut [u8],
    oprand: &SPCOprand,
    op: fn(u8, u8) -> u8,
) {
    let ret;

    match oprand {
        SPCOprand::Immediate { immediate } => {
            ret = op(register.a, *immediate);
            register.a = ret;
        }
        SPCOprand::IndirectPage => {
            ret = op(register.a, ram[register.x as usize]);
            register.a = ret;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = op(register.a, memval);
            register.a = ret;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            let memval = read_ram_u8(ram, address);
            ret = op(register.a, memval);
            register.a = ret;
        }
        SPCOprand::Absolute { address } => {
            ret = op(register.a, ram[*address as usize]);
            register.a = ret;
        }
        SPCOprand::AbsoluteX { address } => {
            ret = op(register.a, ram[(*address + register.x as u16) as usize]);
            register.a = ret;
        }
        SPCOprand::AbsoluteY { address } => {
            ret = op(register.a, ram[(*address + register.y as u16) as usize]);
            register.a = ret;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            ret = op(register.a, memval);
            register.a = ret;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            ret = op(register.a, memval);
            register.a = ret;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let dst_address = register.get_direct_page_address(register.x);
            let src_address = register.get_direct_page_address(register.y);
            ret = op(ram[dst_address], ram[src_address]);
            write_ram_u8(ram, dst_address, ret);
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page_dst,
            direct_page_src,
        } => {
            let dst_address = register.get_direct_page_address(*direct_page_dst);
            let src_address = register.get_direct_page_address(*direct_page_src);
            ret = op(ram[dst_address], ram[src_address]);
            write_ram_u8(ram, dst_address, ret);
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = op(memval, *immediate);
            write_ram_u8(ram, address, ret);
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
    register.set_psw_flag(PSW_FLAG_Z, ret == 0);
}

/// ASL命令の実行
fn execute_asl(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn asl(a: u8) -> u8 {
        // NOTE: 最上位ビットはキャリーフラグに入る
        a << 1
    }
    execute_unary_bit_opration(register, ram, oprand, asl);
}

/// ROL命令の実行
fn execute_rol(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn rol(a: u8) -> u8 {
        let msb = a >> 7;
        (a << 1) | msb
    }
    execute_unary_bit_opration(register, ram, oprand, rol);
}

/// ROR命令の実行
fn execute_ror(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn ror(a: u8) -> u8 {
        let lsb = a & 1;
        (a >> 1) | (lsb << 7)
    }
    execute_unary_bit_opration(register, ram, oprand, ror);
}

/// LSR命令の実行
fn execute_lsr(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn lsr(a: u8) -> u8 {
        a >> 1
    }
    execute_unary_bit_opration(register, ram, oprand, lsr);
}

/// 単項ビット演算命令の実行
fn execute_unary_bit_opration(
    register: &mut SPCRegister,
    ram: &mut [u8],
    oprand: &SPCOprand,
    op: fn(u8) -> u8,
) {
    let ret;
    let prev_msb;

    match oprand {
        SPCOprand::Accumulator => {
            prev_msb = register.a & 0x80;
            ret = op(register.a);
            register.a = ret;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            prev_msb = memval & 0x80;
            ret = op(memval);
            write_ram_u8(ram, address, ret);
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            let memval = read_ram_u8(ram, address);
            prev_msb = memval & 0x80;
            ret = op(memval);
            write_ram_u8(ram, address, ret);
        }
        SPCOprand::Absolute { address } => {
            let addr = *address as usize;
            let memval = read_ram_u8(ram, addr);
            prev_msb = memval & 0x80;
            ret = op(memval);
            write_ram_u8(ram, addr, ret);
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
    register.set_psw_flag(PSW_FLAG_Z, ret == 0);
    register.set_psw_flag(PSW_FLAG_C, prev_msb != 0);
}

/// メモリビットのアドレスとビット位置を取得
fn get_address_bit(address_bit: u16) -> (u8, usize) {
    let bit_pos = ((address_bit >> 13) & 0x07) as u8;
    let address = ((address_bit >> 0) & 0x1F) as usize;
    (bit_pos, address)
}

/// OR1命令の実行
fn execute_or1(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn or(a: u8, b: u8) -> bool {
        (a | b) != 0
    }
    execute_bit_operation_with_carry(register, ram, oprand, or);
}

/// AND1命令の実行
fn execute_and1(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn and(a: u8, b: u8) -> bool {
        (a & b) != 0
    }
    execute_bit_operation_with_carry(register, ram, oprand, and);
}

/// キャリーフラグとのビット演算の実行
fn execute_bit_operation_with_carry(
    register: &mut SPCRegister,
    ram: &mut [u8],
    oprand: &SPCOprand,
    op: fn(u8, u8) -> bool,
) {
    let ret;

    match oprand {
        SPCOprand::AbsoluteBit { address_bit } => {
            let (bit_pos, address) = get_address_bit(*address_bit);
            let memval = read_ram_u8(ram, address);
            ret = op(register.psw & PSW_FLAG_C, (memval >> bit_pos) & 0x1);
        }
        SPCOprand::AbsoluteInverseBit { address_bit } => {
            let (bit_pos, address) = get_address_bit(*address_bit);
            let memval = read_ram_u8(ram, address);
            ret = op(register.psw & PSW_FLAG_C, !((memval >> bit_pos) & 0x1));
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_C, ret);
}

/// INC命令の実行
fn execute_inc(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn inc(a: u8) -> u8 {
        a.overflowing_add(1).0
    }
    execute_inc_dec(register, ram, oprand, inc);
}

/// DEC命令の実行
fn execute_dec(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn dec(a: u8) -> u8 {
        a.overflowing_sub(1).0
    }
    execute_inc_dec(register, ram, oprand, dec);
}

/// INC/DEC命令の実行
fn execute_inc_dec(
    register: &mut SPCRegister,
    ram: &mut [u8],
    oprand: &SPCOprand,
    op: fn(u8) -> u8,
) {
    let ret;

    match oprand {
        SPCOprand::Accumulator => {
            ret = op(register.a);
            register.a = ret;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = op(memval);
            write_ram_u8(ram, address, ret);
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            let memval = read_ram_u8(ram, address);
            ret = op(memval);
            write_ram_u8(ram, address, ret);
        }
        SPCOprand::Absolute { address } => {
            ret = op(ram[*address as usize]);
            write_ram_u8(ram, *address as usize, ret);
        }
        SPCOprand::XIndexRegister => {
            ret = op(register.x);
            register.x = ret;
        }
        SPCOprand::YIndexRegister => {
            ret = op(register.y);
            register.y = ret;
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N) != 0);
    register.set_psw_flag(PSW_FLAG_Z, ret == 0);
}

/// CMP命令の実行
fn execute_cmp(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    let ret;

    match oprand {
        SPCOprand::Immediate { immediate } => {
            ret = register.a as i16 - *immediate as i16;
        }
        SPCOprand::IndirectPage => {
            ret = register.a as i16 - ram[register.x as usize] as i16;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            let memval = read_ram_u8(ram, address);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::Absolute { address } => {
            ret = register.a as i16 - ram[*address as usize] as i16;
        }
        SPCOprand::AbsoluteX { address } => {
            let addr = *address + register.x as u16;
            let memval = read_ram_u8(ram, addr as usize);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::AbsoluteY { address } => {
            let addr = *address + register.y as u16;
            let memval = read_ram_u8(ram, addr as usize);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            ret = register.a as i16 - memval as i16;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let address1 = register.get_direct_page_address(register.x);
            let address2 = register.get_direct_page_address(register.y);
            let memval1 = read_ram_u8(ram, address1);
            let memval2 = read_ram_u8(ram, address2);
            ret = memval1 as i16 - memval2 as i16;
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page_dst,
            direct_page_src,
        } => {
            let address1 = register.get_direct_page_address(*direct_page_dst);
            let address2 = register.get_direct_page_address(*direct_page_src);
            let memval1 = read_ram_u8(ram, address1);
            let memval2 = read_ram_u8(ram, address2);
            ret = memval1 as i16 - memval2 as i16;
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = memval as i16 - *immediate as i16;
        }
        SPCOprand::ImmediateToX { immediate } => {
            ret = register.x as i16 - *immediate as i16;
        }
        SPCOprand::DirectPageToX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = register.x as i16 - memval as i16;
        }
        SPCOprand::AbsoluteToX { address } => {
            ret = register.x as i16 - ram[*address as usize] as i16;
        }
        SPCOprand::ImmediateToY { immediate } => {
            ret = register.y as i16 - *immediate as i16;
        }
        SPCOprand::DirectPageToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            ret = register.y as i16 - memval as i16;
        }
        SPCOprand::AbsoluteToY { address } => {
            ret = register.y as i16 - ram[*address as usize] as i16;
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N as i16) != 0);
    register.set_psw_flag(PSW_FLAG_Z, ret == 0);
    register.set_psw_flag(PSW_FLAG_C, ret >= 0);
}

/// ADC命令の実行
fn execute_adc(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
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
    execute_adc_sbc(register, ram, oprand, add);
}

/// SBC命令の実行
fn execute_sbc(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn sub(a: u8, b: u8, carry: bool) -> (u8, bool, bool, bool) {
        let mut ret = (a as i16) - (b as i16);
        if !carry {
            ret += 1;
        }
        (
            (ret & 0xFF) as u8,
            (ret & 0x100) != 0,
            ((a & 0x80) != (b & 0x80)) && (((a & 0x80) as i16) != (ret & 0x80)),
            check_half_carry_sub_u8(a, b),
        )
    }
    execute_adc_sbc(register, ram, oprand, sub);
}

/// ADC/SBC命令の実行共通ルーチン
fn execute_adc_sbc(
    register: &mut SPCRegister,
    ram: &mut [u8],
    oprand: &SPCOprand,
    op: fn(u8, u8, bool) -> (u8, bool, bool, bool),
) {
    let ret;
    let arith_overflow;
    let sign_overflow;
    let half_carry;

    match oprand {
        SPCOprand::Immediate { immediate } => {
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, *immediate, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::IndirectPage => {
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                register.a,
                ram[register.x as usize],
                register.test_psw_flag(PSW_FLAG_C),
            );
            register.a = ret;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            let memval = read_ram_u8(ram, address);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::Absolute { address } => {
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                register.a,
                ram[*address as usize],
                register.test_psw_flag(PSW_FLAG_C),
            );
            register.a = ret;
        }
        SPCOprand::AbsoluteX { address } => {
            let addr = *address + register.x as u16;
            let memval = read_ram_u8(ram, addr as usize);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::AbsoluteY { address } => {
            let addr = *address + register.y as u16;
            let memval = read_ram_u8(ram, addr as usize);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            let memval = read_ram_u8(ram, address);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, memval, register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let address1 = register.get_direct_page_address(register.x);
            let address2 = register.get_direct_page_address(register.y);
            let memval1 = read_ram_u8(ram, address1);
            let memval2 = read_ram_u8(ram, address2);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(memval1, memval2, register.test_psw_flag(PSW_FLAG_C));
            write_ram_u8(ram, address1, ret);
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page_dst,
            direct_page_src,
        } => {
            let address_dst = register.get_direct_page_address(*direct_page_dst);
            let address_src = register.get_direct_page_address(*direct_page_src);
            let memval_dst = read_ram_u8(ram, address_dst);
            let memval_src = read_ram_u8(ram, address_src);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(memval_dst, memval_src, register.test_psw_flag(PSW_FLAG_C));
            write_ram_u8(ram, address_dst, ret);
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            let memval = read_ram_u8(ram, address);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(memval, *immediate, register.test_psw_flag(PSW_FLAG_C));
            write_ram_u8(ram, address, ret);
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_N, (register.a & PSW_FLAG_N) != 0);
    register.set_psw_flag(PSW_FLAG_H, half_carry);
    register.set_psw_flag(PSW_FLAG_Z, ret == 0);
    if arith_overflow {
        register.set_psw_flag(PSW_FLAG_V, false);
        register.set_psw_flag(PSW_FLAG_C, true);
    } else if sign_overflow {
        register.set_psw_flag(PSW_FLAG_V, true);
        register.set_psw_flag(PSW_FLAG_C, false);
    } else {
        register.set_psw_flag(PSW_FLAG_V, false);
        register.set_psw_flag(PSW_FLAG_C, false);
    }
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

/// オペコードを実行
pub fn execute_opcode(register: &mut SPCRegister, ram: &mut [u8], opcode: &SPCOpcode) {
    match opcode {
        SPCOpcode::NOP => {
            // 何もしない
        }
        // データ転送命令
        SPCOpcode::MOV { oprand } => execute_mov(register, ram, oprand),
        SPCOpcode::MOVW { oprand } => match oprand {
            SPCOprand::DirectPageToYA { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                register.y = read_ram_u8(ram, address + 0);
                register.a = read_ram_u8(ram, address + 1);
                register.set_psw_flag(PSW_FLAG_N, (register.y >> 7) != 0);
                register.set_psw_flag(PSW_FLAG_N, (register.y == 0) && (register.a == 0));
            }
            SPCOprand::YAToDirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                write_ram_u8(ram, address + 0, register.y);
                write_ram_u8(ram, address + 1, register.a);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::XCN => {
            let ret = (register.x >> 4) | (register.x << 4);
            register.x = ret;
            register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
            register.set_psw_flag(PSW_FLAG_Z, ret == 0);
        }
        // 算術演算命令
        SPCOpcode::ADC { oprand } => execute_adc(register, ram, oprand),
        SPCOpcode::ADDW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let wval = make_u16_from_u8(&ram[address..(address + 2)]);
                let ya = ((register.y as u16) << 8) | register.a as u16;
                let (ret, arith_overflow) = ya.overflowing_add(wval);
                let sign_overflow =
                    ((ya & 0x8000) == (wval & 0x8000)) && ((ya & 0x8000) != (ret & 0x8000));
                let half_carry = check_half_carry_add_u16(ya, wval);
                register.y = (ret >> 8) as u8 & 0xFF;
                register.a = (ret >> 0) as u8 & 0xFF;
                // フラグ更新
                register.set_psw_flag(PSW_FLAG_N, (ret >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_H, half_carry);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
                if arith_overflow {
                    register.set_psw_flag(PSW_FLAG_V, false);
                    register.set_psw_flag(PSW_FLAG_C, true);
                } else if sign_overflow {
                    register.set_psw_flag(PSW_FLAG_V, true);
                    register.set_psw_flag(PSW_FLAG_C, false);
                } else {
                    register.set_psw_flag(PSW_FLAG_V, false);
                    register.set_psw_flag(PSW_FLAG_C, false);
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DEC { oprand } => execute_dec(register, ram, oprand),
        SPCOpcode::DECW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let mut wval = make_u16_from_u8(&ram[address..(address + 2)]);
                wval = wval.overflowing_sub(1).0;
                write_ram_u8(ram, address + 0, ((wval >> 8) & 0xFF) as u8);
                write_ram_u8(ram, address + 1, ((wval >> 0) & 0xFF) as u8);
                register.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_Z, wval == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DIV => {
            let ya = ((register.y as u16) << 8) | register.a as u16;
            let quot = ya / (register.x as u16);
            let rem = ya % (register.x as u16);

            if quot <= 0xFF {
                register.a = quot as u8;
            } else {
                register.a = (quot & 0xFF) as u8;
            }
            register.y = rem as u8;

            register.set_psw_flag(PSW_FLAG_N, (quot >> 8) != 0);
            register.set_psw_flag(PSW_FLAG_V, quot > 0xFF);
            register.set_psw_flag(PSW_FLAG_H, (register.y & 0xF) >= (register.x & 0xF));
            register.set_psw_flag(PSW_FLAG_Z, quot == 0);
        }
        SPCOpcode::INC { oprand } => execute_inc(register, ram, oprand),
        SPCOpcode::INCW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let mut wval = make_u16_from_u8(&ram[address..(address + 2)]);
                wval = wval.overflowing_add(1).0;
                write_ram_u8(ram, address + 0, ((wval >> 8) & 0xFF) as u8);
                write_ram_u8(ram, address + 1, ((wval >> 0) & 0xFF) as u8);
                register.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_Z, wval == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::MUL => {
            let mul = (register.y as i16) * (register.a as i16);
            register.y = ((mul << 8) & 0xFF) as u8;
            register.a = ((mul << 0) & 0xFF) as u8;
            register.set_psw_flag(PSW_FLAG_N, (mul >> 15) != 0);
            register.set_psw_flag(PSW_FLAG_Z, register.y == 0);
        }
        SPCOpcode::SBC { oprand } => execute_sbc(register, ram, oprand),
        SPCOpcode::SUBW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let wval = make_u16_from_u8(&ram[address..(address + 2)]);
                let ya = ((register.y as u16) << 8) | register.a as u16;
                let (ret, arith_overflow) = ya.overflowing_sub(wval);
                let sign_overflow =
                    ((ya & 0x8000) != (wval & 0x8000)) && ((ya & 0x8000) != (ret & 0x8000));
                let half_carry = check_half_carry_sub_u16(ya, wval);
                register.y = (ret >> 8) as u8 & 0xFF;
                register.a = (ret >> 0) as u8 & 0xFF;
                // フラグ更新
                register.set_psw_flag(PSW_FLAG_N, (ret >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_H, half_carry);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
                if !arith_overflow {
                    register.set_psw_flag(PSW_FLAG_V, false);
                    register.set_psw_flag(PSW_FLAG_C, true);
                } else if sign_overflow {
                    register.set_psw_flag(PSW_FLAG_V, true);
                    register.set_psw_flag(PSW_FLAG_C, false);
                } else {
                    register.set_psw_flag(PSW_FLAG_V, false);
                    register.set_psw_flag(PSW_FLAG_C, false);
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        // スタック操作命令
        SPCOpcode::PUSH { oprand } => match oprand {
            SPCOprand::Accumulator => {
                register.push_stack(ram, register.a);
            }
            SPCOprand::XIndexRegister => {
                register.push_stack(ram, register.x);
            }
            SPCOprand::YIndexRegister => {
                register.push_stack(ram, register.y);
            }
            SPCOprand::ProgramStatusWord => {
                register.push_stack(ram, register.psw);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::POP { oprand } => match oprand {
            SPCOprand::Accumulator => {
                register.a = register.pop_stack(ram);
            }
            SPCOprand::XIndexRegister => {
                register.x = register.pop_stack(ram);
            }
            SPCOprand::YIndexRegister => {
                register.y = register.pop_stack(ram);
            }
            SPCOprand::ProgramStatusWord => {
                register.psw = register.pop_stack(ram);
            }
            _ => panic!("Invalid oprand!"),
        },
        // 論理演算命令
        SPCOpcode::AND { oprand } => execute_and(register, ram, oprand),
        SPCOpcode::ASL { oprand } => execute_asl(register, ram, oprand),
        SPCOpcode::EOR { oprand } => execute_eor(register, ram, oprand),
        SPCOpcode::LSR { oprand } => execute_lsr(register, ram, oprand),
        SPCOpcode::OR { oprand } => execute_or(register, ram, oprand),
        SPCOpcode::ROL { oprand } => execute_rol(register, ram, oprand),
        SPCOpcode::ROR { oprand } => execute_ror(register, ram, oprand),
        // ビット操作命令
        SPCOpcode::AND1 { oprand } => execute_and1(register, ram, oprand),
        SPCOpcode::CLR1 { bit, oprand } => match oprand {
            SPCOprand::DirectPageBit { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let memval = read_ram_u8(ram, address);
                write_ram_u8(ram, address, memval & !(1 << (*bit)));
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::EOR1 { oprand } => match oprand {
            SPCOprand::AbsoluteBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let memval = read_ram_u8(ram, address);
                let ret = (register.psw & PSW_FLAG_C) ^ ((memval >> bit_pos) & 0x1);
                register.set_psw_flag(PSW_FLAG_C, ret != 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::OR1 { oprand } => execute_or1(register, ram, oprand),
        SPCOpcode::MOV1 { oprand } => match oprand {
            SPCOprand::AbsoluteMemoryBitToCarrayFlag { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let memval = read_ram_u8(ram, address);
                register.set_psw_flag(PSW_FLAG_C, ((memval >> bit_pos) & 0x1) != 0);
            }
            SPCOprand::CarrayFlagToAbsoluteMemoryBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let mask = (register.psw & PSW_FLAG_C) << bit_pos;
                let memval = read_ram_u8(ram, address);
                write_ram_u8(
                    ram,
                    address,
                    if mask != 0 {
                        memval | mask
                    } else {
                        memval & !mask
                    },
                );
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::NOT1 { oprand } => match oprand {
            SPCOprand::AbsoluteBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let memval = read_ram_u8(ram, address);
                write_ram_u8(ram, address, memval ^ (1 << bit_pos));
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::SET1 { bit, oprand } => match oprand {
            SPCOprand::DirectPageBit { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let memval = read_ram_u8(ram, address);
                write_ram_u8(ram, address, memval | (1 << (*bit)));
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::TSET1 { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                let addr = *address as usize;
                let memval = read_ram_u8(ram, addr);
                let or = register.a | memval;
                let and = register.a & memval;
                write_ram_u8(ram, addr, or);
                register.set_psw_flag(PSW_FLAG_N, (or & PSW_FLAG_N) != 0);
                register.set_psw_flag(PSW_FLAG_Z, and == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::TCLR1 { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                let memval = read_ram_u8(ram, *address as usize);
                let ret = memval & !register.a;
                write_ram_u8(ram, *address as usize, ret);
                register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        // 比較命令
        SPCOpcode::CMP { oprand } => execute_cmp(register, ram, oprand),
        SPCOpcode::CMPW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let wval = make_u16_from_u8(&ram[address..(address + 2)]) as i32;
                let ya = ((register.y as i32) << 8) | register.a as i32;
                let ret = ya - wval;
                // フラグ更新
                register.set_psw_flag(PSW_FLAG_N, (ret & PSW_FLAG_N as i32) != 0);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
                register.set_psw_flag(PSW_FLAG_C, ret >= 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        // フラグ操作命令
        SPCOpcode::CLRC => {
            register.set_psw_flag(PSW_FLAG_C, false);
        }
        SPCOpcode::CLRP => {
            register.set_psw_flag(PSW_FLAG_P, false);
        }
        SPCOpcode::CLRV => {
            register.set_psw_flag(PSW_FLAG_V, false);
            register.set_psw_flag(PSW_FLAG_H, false);
        }
        SPCOpcode::NOTC => {
            register.set_psw_flag(PSW_FLAG_C, !register.test_psw_flag(PSW_FLAG_C));
        }
        SPCOpcode::SETC => {
            register.set_psw_flag(PSW_FLAG_C, true);
        }
        SPCOpcode::SETP => {
            register.set_psw_flag(PSW_FLAG_P, true);
        }
        // 分岐命令
        SPCOpcode::BBC { bit, oprand } => match oprand {
            SPCOprand::DirectPageBitPCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page);
                let memval = read_ram_u8(ram, address);
                if memval & (1 << (*bit)) == 0 {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BBS { bit, oprand } => match oprand {
            SPCOprand::DirectPageBitPCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page);
                let memval = read_ram_u8(ram, address);
                if memval & (1 << (*bit)) != 0 {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BCC { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_C) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BCS { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_C) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BEQ { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_Z) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BMI { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_N) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BNE { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_Z) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BPL { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_Z) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BRA { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BVC { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_V) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BVS { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_V) {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::CBNE { oprand } => match oprand {
            SPCOprand::DirectPagePCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page);
                let memval = read_ram_u8(ram, address);
                if register.a != memval {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            SPCOprand::DirectPageXPCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page) + register.x as usize;
                let memval = read_ram_u8(ram, address);
                if register.a != memval {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DBNZ { oprand } => match oprand {
            SPCOprand::DirectPagePCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page);
                let mut memval = read_ram_u8(ram, address);
                memval = memval.overflowing_sub(1).0;
                write_ram_u8(ram, address, memval);
                if memval != 0 {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            SPCOprand::YPCRelative { pc_relative } => {
                register.y = register.y.overflowing_sub(1).0;
                if register.y != 0 {
                    register.pc = (register.pc as i32 + *pc_relative as i32) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        // ジャンプ命令
        SPCOpcode::CALL { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                register.push_stack(ram, ((register.pc >> 8) & 0xFF) as u8);
                register.push_stack(ram, ((register.pc >> 0) & 0xFF) as u8);
                register.pc = *address;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::JMP { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                register.pc = *address;
            }
            SPCOprand::AbsoluteXIndirect { address } => {
                let addr = (*address + register.x as u16) as usize;
                let jmp_pc = make_u16_from_u8(&ram[addr..(addr + 2)]);
                register.pc = jmp_pc;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::PCALL { oprand } => match oprand {
            SPCOprand::PageAddress { address } => {
                register.push_stack(ram, ((register.pc >> 8) & 0xFF) as u8);
                register.push_stack(ram, ((register.pc >> 0) & 0xFF) as u8);
                register.pc = 0xFF00u16 | *address as u16;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::TCALL { table_index } => {
            let address = 0xFFDEusize - (*table_index * 2) as usize;
            let jmp_pc = make_u16_from_u8(&ram[address..(address + 2)]);
            register.push_stack(ram, ((register.pc >> 8) & 0xFF) as u8);
            register.push_stack(ram, ((register.pc >> 0) & 0xFF) as u8);
            register.pc = jmp_pc;
        }
        SPCOpcode::RET => {
            let low = register.pop_stack(ram) as u16;
            let high = register.pop_stack(ram) as u16;
            register.pc = (high << 8) | low;
        }
        // 十進補正命令
        SPCOpcode::DAA { oprand } => match oprand {
            SPCOprand::Accumulator => {
                let mut ret = register.a;
                let mut carry = register.test_psw_flag(PSW_FLAG_C);
                // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を足す
                if register.test_psw_flag(PSW_FLAG_H) || (ret & 0x0F) >= 0xA {
                    (ret, carry) = ret.overflowing_add(0x06);
                }
                // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を足す
                if !register.test_psw_flag(PSW_FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                    (ret, carry) = ret.overflowing_add(0x60);
                }
                // 最上位ビットにキャリーフラグをセットする
                ret = if register.test_psw_flag(PSW_FLAG_C) {
                    ret | 0x80
                } else {
                    ret & 0x7F
                };
                register.a = ret;
                register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
                register.set_psw_flag(PSW_FLAG_C, carry);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DAS { oprand } => match oprand {
            SPCOprand::Accumulator => {
                let mut ret = register.a;
                let mut carry = register.test_psw_flag(PSW_FLAG_C);
                // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を引く
                if register.test_psw_flag(PSW_FLAG_H) || (ret & 0x0F) >= 0xA {
                    (ret, carry) = ret.overflowing_sub(0x06);
                }
                // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を引く
                if !register.test_psw_flag(PSW_FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                    (ret, carry) = ret.overflowing_sub(0x60);
                }
                // 最上位ビットにキャリーフラグをセットする
                ret = if register.test_psw_flag(PSW_FLAG_C) {
                    ret | 0x80
                } else {
                    ret & 0x7F
                };
                register.a = ret;
                register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
                register.set_psw_flag(PSW_FLAG_C, carry);
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

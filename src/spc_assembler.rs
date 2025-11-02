use crate::types::*;

/// オペコード長チェック付き命令生成マクロ
macro_rules! create_opcode_with_length_check {
    ($ram:expr, $opcode:expr, $length:expr) => {{
        if $ram.len() < $length {
            panic!("Insufficient instruction length: {}", $ram[0]);
        }
        ($opcode, $length)
    }};
}

/// RAMからオペコードを解釈
pub fn parse_opcode(ram: &[u8]) -> (SPCOpcode, u16) {
    match ram[0] {
        0x00 => create_opcode_with_length_check!(ram, SPCOpcode::NOP, 1),
        0x01 | 0x11 | 0x21 | 0x31 | 0x41 | 0x51 | 0x61 | 0x71 | 0x81 | 0x91 | 0xA1 | 0xB1
        | 0xC1 | 0xD1 | 0xE1 | 0xF1 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::TCALL {
                table_index: (ram[0] >> 4),
            },
            1
        ),
        0x02 | 0x22 | 0x42 | 0x62 | 0x82 | 0xA2 | 0xC2 | 0xE2 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SET1 {
                bit: (ram[0] >> 5),
                oprand: SPCOprand::DirectPageBit {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x03 | 0x23 | 0x43 | 0x63 | 0x83 | 0xA3 | 0xC3 | 0xE3 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BBS {
                bit: (ram[0] >> 5),
                oprand: SPCOprand::DirectPageBitPCRelative {
                    direct_page: ram[1],
                    pc_relative: ram[2] as i8,
                },
            },
            3
        ),
        0x04 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x05 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x06 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::IndirectPage,
            },
            1
        ),
        0x07 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x08 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::Immediate { immediate: ram[1] },
            },
            2
        ),
        0x09 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                },
            },
            3
        ),
        0x14 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x15 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x16 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x17 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x18 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                },
            },
            3
        ),
        0x19 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::IndirectPageToIndirectPage
            },
            1
        ),
        0x0A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR1 {
                oprand: SPCOprand::AbsoluteBit {
                    address_bit: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x2A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR1 {
                oprand: SPCOprand::AbsoluteInverseBit {
                    address_bit: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x0B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ASL {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x0C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ASL {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x1B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ASL {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x1C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ASL {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0x0D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::PUSH {
                oprand: SPCOprand::ProgramStatusWord
            },
            1
        ),
        0x2D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::PUSH {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0x4D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::PUSH {
                oprand: SPCOprand::XIndexRegister
            },
            1
        ),
        0x6D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::PUSH {
                oprand: SPCOprand::YIndexRegister
            },
            1
        ),
        0x0E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::TSET1 {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x0F => create_opcode_with_length_check!(ram, SPCOpcode::BRK, 1),
        0x10 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BPL {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0x12 | 0x32 | 0x52 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CLR1 {
                bit: (ram[0] >> 5),
                oprand: SPCOprand::DirectPageBit {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x13 | 0x33 | 0x53 | 0x73 | 0x93 | 0xB3 | 0xD3 | 0xF3 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BBC {
                bit: (ram[0] >> 5),
                oprand: SPCOprand::DirectPageBitPCRelative {
                    direct_page: ram[1],
                    pc_relative: ram[2] as i8,
                },
            },
            3
        ),
        0x1A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DECW {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x1D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::XIndexRegister,
            },
            1
        ),
        0x8B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x8C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x9B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x9C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0xDC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DEC {
                oprand: SPCOprand::YIndexRegister,
            },
            1
        ),
        0x1E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::AbsoluteToX {
                    address: make_u16_from_u8(&ram[1..3]),
                },
            },
            3
        ),
        0x3E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageToX {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x5E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::AbsoluteToY {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x64 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x65 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x66 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::IndirectPage
            },
            1
        ),
        0x67 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x68 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::Immediate { immediate: ram[1] }
            },
            2
        ),
        0x69 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2],
                }
            },
            3
        ),
        0x74 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x75 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x76 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x77 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x78 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                },
            },
            3
        ),
        0x79 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::IndirectPageToIndirectPage
            },
            1
        ),
        0x7E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::DirectPageToY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xAD => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::ImmediateToY { immediate: ram[1] }
            },
            2
        ),
        0xC8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMP {
                oprand: SPCOprand::ImmediateToX { immediate: ram[1] }
            },
            2
        ),
        0x1F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::JMP {
                oprand: SPCOprand::AbsoluteXIndirect {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x5F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::JMP {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x20 => create_opcode_with_length_check!(ram, SPCOpcode::CLRP, 1),
        0x24 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x25 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x26 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::IndirectPage
            },
            1
        ),
        0x27 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x28 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::Immediate { immediate: ram[1] }
            },
            2
        ),
        0x29 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                }
            },
            3
        ),
        0x34 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x35 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x36 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x37 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x38 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                }
            },
            3
        ),
        0x39 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND {
                oprand: SPCOprand::IndirectPageToIndirectPage
            },
            1
        ),
        0x2B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROL {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x2C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROL {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x3B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROL {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x3C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROL {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0x2E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CBNE {
                oprand: SPCOprand::DirectPagePCRelative {
                    direct_page: ram[1],
                    pc_relative: ram[2] as i8
                }
            },
            3
        ),
        0xDE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CBNE {
                oprand: SPCOprand::DirectPageXPCRelative {
                    direct_page: ram[1],
                    pc_relative: ram[2] as i8,
                }
            },
            3
        ),
        0x2F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BRA {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8,
                }
            },
            2
        ),
        0x30 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BMI {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8,
                }
            },
            2
        ),
        0x3A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INCW {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1],
                }
            },
            2
        ),
        0x3D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::XIndexRegister
            },
            1
        ),
        0xAB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xAC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xBB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xBC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0xFC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::INC {
                oprand: SPCOprand::YIndexRegister
            },
            1
        ),
        0x3F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CALL {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x40 => create_opcode_with_length_check!(ram, SPCOpcode::SETP, 1),
        0x44 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x45 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x46 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::IndirectPage
            },
            1
        ),
        0x47 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x48 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::Immediate { immediate: ram[1] }
            },
            2
        ),
        0x49 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                }
            },
            3
        ),
        0x54 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1],
                }
            },
            2
        ),
        0x55 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x56 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3]),
                }
            },
            3
        ),
        0x57 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x58 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                }
            },
            3
        ),
        0x59 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR {
                oprand: SPCOprand::IndirectPageToIndirectPage
            },
            1
        ),
        0x4A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND1 {
                oprand: SPCOprand::AbsoluteBit {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x6A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::AND1 {
                oprand: SPCOprand::AbsoluteInverseBit {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x4B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::LSR {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x4C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::LSR {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x5B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::LSR {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x5C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::LSR {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0x4E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::TCLR1 {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x4F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::PCALL {
                oprand: SPCOprand::PageAddress { address: ram[1] }
            },
            2
        ),
        0x50 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BVC {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0x5A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::CMPW {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x5D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToX
            },
            1
        ),
        0x7D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::XToA
            },
            1
        ),
        0x8D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::ImmediateToY { immediate: ram[1] }
            },
            2
        ),
        0x8F => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                }
            },
            3
        ),
        0x9D => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::StackPointerToX
            },
            1
        ),
        0xAF => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToIndirectAutoIncrement
            },
            1
        ),
        0xBD => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::XToStackPointer
            },
            1
        ),
        0xBF => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::IndirectAutoIncrementToA
            },
            1
        ),
        0xC4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToDirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xC5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToAbsolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xC6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToIndirect
            },
            1
        ),
        0xC7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToDirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xC9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::XToAbsolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xCB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::YToDirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xCC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::YToAbsolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xCD => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::ImmediateToX { immediate: ram[1] }
            },
            2
        ),
        0xD4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToDirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xD5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToAbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xD6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToAbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xD7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToDirectPageIndirectY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xD8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::XToDirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xD9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::XToDirectPageY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xDB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xDD => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::YToA
            },
            1
        ),
        0xE4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageToA {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xE5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AbsoluteToA {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xE6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::IndirectToA
            },
            1
        ),
        0xE7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xE8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::ImmediateToA { immediate: ram[1] }
            },
            2
        ),
        0xE9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AbsoluteToX {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xEB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageToY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xEC => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AbsoluteToY {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xF4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageXToA {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xF5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xF6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xF7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xF8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageToX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xF9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageYToX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xFA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                }
            },
            3
        ),
        0xFB => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::DirectPageXToY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xFD => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV {
                oprand: SPCOprand::AToY
            },
            1
        ),
        0x60 => create_opcode_with_length_check!(ram, SPCOpcode::CLRC, 1),
        0x6B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROR {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x6C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROR {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x7B => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROR {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x7C => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ROR {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0x6E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DBNZ {
                oprand: SPCOprand::DirectPagePCRelative {
                    direct_page: ram[1],
                    pc_relative: ram[2] as i8
                }
            },
            3
        ),
        0xFE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DBNZ {
                oprand: SPCOprand::YPCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0x6F => create_opcode_with_length_check!(ram, SPCOpcode::RET, 1),
        0x70 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BVS {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0x7A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADDW {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x7F => create_opcode_with_length_check!(ram, SPCOpcode::RETI, 1),
        0x80 => create_opcode_with_length_check!(ram, SPCOpcode::SETC, 1),
        0x84 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x85 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x86 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::Indirect,
            },
            1
        ),
        0x87 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x88 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::Immediate { immediate: ram[1] },
            },
            2
        ),
        0x89 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                },
            },
            3
        ),
        0x94 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x95 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x96 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                },
            },
            3
        ),
        0x97 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                },
            },
            2
        ),
        0x98 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                }
            },
            3
        ),
        0x99 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::ADC {
                oprand: SPCOprand::IndirectToIndirect
            },
            1
        ),
        0x8A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::EOR1 {
                oprand: SPCOprand::AbsoluteBit {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0x8E => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0xAE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::XIndexRegister
            },
            1
        ),
        0xCE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::YIndexRegister
            },
            1
        ),
        0xEE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::ProgramStatusWord
            },
            1
        ),
        0x90 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BCC {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0x9A => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SUBW {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0x9E => create_opcode_with_length_check!(ram, SPCOpcode::DIV, 1),
        0x9F => create_opcode_with_length_check!(ram, SPCOpcode::XCN, 1),
        0xA0 => create_opcode_with_length_check!(ram, SPCOpcode::EI, 1),
        0xA4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::DirectPage {
                    direct_page: ram[1]
                }
            },
            1
        ),
        0xA5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::Absolute {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xA6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::IndirectToA
            },
            1
        ),
        0xA7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::DirectPageXIndirect {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xA8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::Immediate { immediate: ram[1] }
            },
            2
        ),
        0xA9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::DirectPageToDirectPage {
                    direct_page1: ram[1],
                    direct_page2: ram[2]
                }
            },
            3
        ),
        0xB4 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::DirectPageX {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xB5 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::AbsoluteX {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xB6 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::AbsoluteY {
                    address: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xB7 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xB8 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::ImmediateToDirectPage {
                    direct_page: ram[1],
                    immediate: ram[2]
                }
            },
            3
        ),
        0xB9 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::SBC {
                oprand: SPCOprand::IndirectToIndirect
            },
            1
        ),
        0xAA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV1 {
                oprand: SPCOprand::AbsoluteMemoryBitToCarrayFlag {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xCA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOV1 {
                oprand: SPCOprand::CarrayFlagToAbsoluteMemoryBit {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xB0 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BCS {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0xBA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOVW {
                oprand: SPCOprand::DirectPageToAY {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xDA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOVW {
                oprand: SPCOprand::AYToDirectPage {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xBE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DAS {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0xC0 => create_opcode_with_length_check!(ram, SPCOpcode::DI, 1),
        0xCF => create_opcode_with_length_check!(ram, SPCOpcode::MUL, 1),
        0xD0 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BNE {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0xDF => create_opcode_with_length_check!(
            ram,
            SPCOpcode::DAA {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0xE0 => create_opcode_with_length_check!(ram, SPCOpcode::CLRV, 1),
        0xEA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BNE {
                oprand: SPCOprand::AbsoluteBit {
                    address_bit: make_u16_from_u8(&ram[1..3])
                }
            },
            3
        ),
        0xED => create_opcode_with_length_check!(ram, SPCOpcode::NOTC, 1),
        0xEF => create_opcode_with_length_check!(ram, SPCOpcode::SLEEP, 1),
        0xF0 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::BEQ {
                oprand: SPCOprand::PCRelative {
                    pc_relative: ram[1] as i8
                }
            },
            2
        ),
        0xFF => create_opcode_with_length_check!(ram, SPCOpcode::STOP, 1),
    }
}

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

impl SPCRegister {
    /// スタックのベースアドレス
    const STACK_BASE_ADDRESS: usize = 0x100usize;

    /// ダイレクトページのアドレスを取得
    fn get_direct_page_address(&self, direct_page: u8) -> usize {
        if self.test_psw_flag(PSW_FLAG_H) {
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
        ram[Self::STACK_BASE_ADDRESS + self.sp as usize] = value;
        self.sp += 1;
    }

    /// スタックからデータをPOP
    fn pop_stack(&mut self, ram: &mut [u8]) -> u8 {
        self.sp -= 1;
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
            val = ram[register.x as usize];
            register.a = val;
        }
        SPCOprand::IndirectAutoIncrementToA => {
            val = ram[register.x as usize];
            register.a = val;
            register.x += 1;
        }
        SPCOprand::DirectPageToA { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = ram[address];
            register.a = val;
        }
        SPCOprand::DirectPageXToA { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = ram[address];
            register.a = val;
        }
        SPCOprand::AbsoluteToA { address } => {
            val = ram[*address as usize];
            register.a = val;
        }
        SPCOprand::AbsoluteX { address } => {
            val = ram[(*address + register.x as u16) as usize];
            register.a = val;
        }
        SPCOprand::AbsoluteY { address } => {
            val = ram[(*address + register.y as u16) as usize];
            register.a = val;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            val = ram[address];
            register.a = val;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            val = ram[address];
            register.a = val;
        }
        SPCOprand::ImmediateToX { immediate } => {
            val = *immediate;
            register.x = val;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = ram[address];
            register.x = val;
        }
        SPCOprand::DirectPageYToX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.y as usize;
            val = ram[address];
            register.x = val;
        }
        SPCOprand::AbsoluteToX { address } => {
            val = ram[*address as usize];
            register.x = val;
        }
        SPCOprand::ImmediateToY { immediate } => {
            val = *immediate;
            register.y = val;
        }
        SPCOprand::DirectPageToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = ram[address];
            register.y = val;
        }
        SPCOprand::DirectPageXToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = ram[address];
            register.y = val;
        }
        SPCOprand::AbsoluteToY { address } => {
            val = ram[*address as usize];
            register.y = val;
        }
        SPCOprand::AToIndirect => {
            let address = register.get_direct_page_address(register.x);
            val = register.a;
            ram[address] = val;
        }
        SPCOprand::AToIndirectAutoIncrement => {
            let address = register.get_direct_page_address(register.x);
            val = register.a;
            ram[address] = val;
            register.x += 1;
        }
        SPCOprand::AToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.a;
            ram[address] = val;
        }
        SPCOprand::AToDirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = register.a;
            ram[address] = val;
        }
        SPCOprand::AToAbsolute { address } => {
            val = register.a;
            ram[*address as usize] = val;
        }
        SPCOprand::AToAbsoluteX { address } => {
            val = register.a;
            ram[(*address + (register.x as u16)) as usize] = val;
        }
        SPCOprand::AToAbsoluteY { address } => {
            val = register.a;
            ram[(*address + (register.y as u16)) as usize] = val;
        }
        SPCOprand::AToDirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            val = register.a;
            ram[address] = val;
        }
        SPCOprand::AToDirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            val = register.a;
            ram[address] = val;
        }
        SPCOprand::XToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.x;
            ram[address] = val;
        }
        SPCOprand::XToDirectPageY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.y as usize;
            val = register.x;
            ram[address] = val;
        }
        SPCOprand::XToAbsolute { address } => {
            val = register.x;
            ram[*address as usize] = val;
        }
        SPCOprand::YToDirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            val = register.y;
            ram[address] = val;
        }
        SPCOprand::YToDirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            val = register.y;
            ram[address] = val;
        }
        SPCOprand::YToAbsolute { address } => {
            val = register.y;
            ram[*address as usize] = val;
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
            direct_page1,
            direct_page2,
        } => {
            let dst_address = register.get_direct_page_address(*direct_page1);
            let src_address = register.get_direct_page_address(*direct_page2);
            val = ram[src_address];
            ram[dst_address] = val;
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            val = *immediate;
            ram[address] = val;
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
            ret = op(register.a, ram[address]);
            register.a = ret;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            ret = op(register.a, ram[address]);
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
            ret = op(register.a, ram[address]);
            register.a = ret;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            ret = op(register.a, ram[address]);
            register.a = ret;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let dst_address = register.get_direct_page_address(register.x);
            let src_address = register.get_direct_page_address(register.y);
            ret = op(ram[dst_address], ram[src_address]);
            ram[dst_address] = ret;
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page1,
            direct_page2,
        } => {
            let dst_address = register.get_direct_page_address(*direct_page1);
            let src_address = register.get_direct_page_address(*direct_page2);
            ret = op(ram[dst_address], ram[src_address]);
            ram[dst_address] = ret;
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            ret = op(ram[address], *immediate);
            ram[address] = ret;
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
        a << 1
    }
    execute_unary_bit_opration(register, ram, oprand, asl);
}

/// ROL命令の実行
fn execute_rol(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn rol(a: u8) -> u8 {
        let msb = a >> 7;
        a << 1 | msb
    }
    execute_unary_bit_opration(register, ram, oprand, rol);
}

/// ROR命令の実行
fn execute_ror(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn ror(a: u8) -> u8 {
        let lsb = a & 1;
        a >> 1 | lsb << 7
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
            prev_msb = (register.a >> 7) & 0x1;
            register.a = op(register.a);
            ret = register.a;
        }
        SPCOprand::DirectPage { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            prev_msb = (ram[address] >> 7) & 0x1;
            ram[address] = op(ram[address]);
            ret = ram[address];
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            prev_msb = (ram[address] >> 7) & 0x1;
            ram[address] = op(ram[address]);
            ret = ram[address];
        }
        SPCOprand::Absolute { address } => {
            let addr = *address as usize;
            prev_msb = (ram[addr] >> 7) & 0x1;
            ram[addr] = op(ram[addr]);
            ret = ram[addr];
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
            ret = op(register.psw & PSW_FLAG_C, (ram[address] >> bit_pos) & 0x1);
        }
        SPCOprand::AbsoluteInverseBit { address_bit } => {
            let (bit_pos, address) = get_address_bit(*address_bit);
            ret = op(
                register.psw & PSW_FLAG_C,
                !((ram[address] >> bit_pos) & 0x1),
            );
        }
        _ => panic!("Invalid oprand!"),
    }

    // フラグ更新
    register.set_psw_flag(PSW_FLAG_C, ret);
}

/// INC命令の実行
fn execute_inc(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn inc(a: u8) -> u8 {
        a + 1
    }
    execute_inc_dec(register, ram, oprand, inc);
}

/// DEC命令の実行
fn execute_dec(register: &mut SPCRegister, ram: &mut [u8], oprand: &SPCOprand) {
    fn dec(a: u8) -> u8 {
        a - 1
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
            ret = op(ram[address]);
            ram[address] = ret;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            ret = op(ram[address]);
            ram[address] = ret;
        }
        SPCOprand::Absolute { address } => {
            ret = op(ram[*address as usize]);
            ram[*address as usize] = ret;
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
            ret = register.a as i16 - ram[address] as i16;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            ret = register.a as i16 - ram[address] as i16;
        }
        SPCOprand::Absolute { address } => {
            ret = register.a as i16 - ram[*address as usize] as i16;
        }
        SPCOprand::AbsoluteX { address } => {
            let addr = *address + register.x as u16;
            ret = register.a as i16 - ram[addr as usize] as i16;
        }
        SPCOprand::AbsoluteY { address } => {
            let addr = *address + register.y as u16;
            ret = register.a as i16 - ram[addr as usize] as i16;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            ret = register.a as i16 - ram[address] as i16;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            ret = register.a as i16 - ram[address] as i16;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let address1 = register.get_direct_page_address(register.x);
            let address2 = register.get_direct_page_address(register.y);
            ret = ram[address1] as i16 - ram[address2] as i16;
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page1,
            direct_page2,
        } => {
            let address1 = register.get_direct_page_address(*direct_page1);
            let address2 = register.get_direct_page_address(*direct_page2);
            ret = ram[address1] as i16 - ram[address2] as i16;
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            ret = ram[address] as i16 - *immediate as i16;
        }
        SPCOprand::ImmediateToX { immediate } => {
            ret = register.x as i16 - *immediate as i16;
        }
        SPCOprand::DirectPageToX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            ret = register.x as i16 - ram[address] as i16;
        }
        SPCOprand::AbsoluteToX { address } => {
            ret = register.x as i16 - ram[*address as usize] as i16;
        }
        SPCOprand::ImmediateToY { immediate } => {
            ret = register.y as i16 - *immediate as i16;
        }
        SPCOprand::DirectPageToY { direct_page } => {
            let address = register.get_direct_page_address(*direct_page);
            ret = register.y as i16 - ram[address] as i16;
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
        let mut ret = (a as u16) - (b as u16);
        if !carry {
            ret += 1;
        }
        (
            (ret & 0xFF) as u8,
            (ret & 0x100) != 0,
            ((a & 0x80) != (b & 0x80)) && (((a & 0x80) as u16) != (ret & 0x80)),
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
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, ram[address], register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::DirectPageX { direct_page } => {
            let address = register.get_direct_page_address(*direct_page) + register.x as usize;
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, ram[address], register.test_psw_flag(PSW_FLAG_C));
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
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                register.a,
                ram[addr as usize],
                register.test_psw_flag(PSW_FLAG_C),
            );
            register.a = ret;
        }
        SPCOprand::AbsoluteY { address } => {
            let addr = *address + register.y as u16;
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                register.a,
                ram[addr as usize],
                register.test_psw_flag(PSW_FLAG_C),
            );
            register.a = ret;
        }
        SPCOprand::DirectPageXIndirect { direct_page } => {
            let address = register.get_direct_page_x_indexed_indirect_address(ram, *direct_page);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, ram[address], register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::DirectPageIndirectY { direct_page } => {
            let address = register.get_direct_page_indirect_y_indexed_address(ram, *direct_page);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(register.a, ram[address], register.test_psw_flag(PSW_FLAG_C));
            register.a = ret;
        }
        SPCOprand::IndirectPageToIndirectPage => {
            let address1 = register.get_direct_page_address(register.x);
            let address2 = register.get_direct_page_address(register.y);
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                ram[address1],
                ram[address2],
                register.test_psw_flag(PSW_FLAG_C),
            );
            ram[address1] = ret;
        }
        SPCOprand::DirectPageToDirectPage {
            direct_page1,
            direct_page2,
        } => {
            let address1 = register.get_direct_page_address(*direct_page1);
            let address2 = register.get_direct_page_address(*direct_page2);
            (ret, arith_overflow, sign_overflow, half_carry) = op(
                ram[address1],
                ram[address2],
                register.test_psw_flag(PSW_FLAG_C),
            );
            ram[address1] = ret;
        }
        SPCOprand::ImmediateToDirectPage {
            direct_page,
            immediate,
        } => {
            let address = register.get_direct_page_address(*direct_page);
            (ret, arith_overflow, sign_overflow, half_carry) =
                op(ram[address], *immediate, register.test_psw_flag(PSW_FLAG_C));
            ram[address] = ret;
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
        SPCOpcode::XCN => {
            let ret = (register.x >> 4) | (register.x << 4);
            register.x = ret;
            register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
            register.set_psw_flag(PSW_FLAG_Z, ret == 0);
        }
        // ジャンプ命令
        SPCOpcode::TCALL { table_index } => {
            let address = 0xFFC0usize + (*table_index * 2) as usize;
            let jmp_pc = make_u16_from_u8(&ram[address..(address + 2)]);
            register.push_stack(ram, ((register.pc >> 8) & 0xFF) as u8);
            register.push_stack(ram, ((register.pc >> 0) & 0xFF) as u8);
            register.pc = jmp_pc;
        }
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
        SPCOpcode::CALL { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                register.push_stack(ram, ((register.pc >> 8) & 0xFF) as u8);
                register.push_stack(ram, ((register.pc >> 0) & 0xFF) as u8);
                register.pc = *address;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::RET => {
            let low = register.pop_stack(ram) as u16;
            let high = register.pop_stack(ram) as u16;
            register.pc = (high << 8) | low;
        }
        // ビット操作命令
        SPCOpcode::SET1 { bit, oprand } => match oprand {
            SPCOprand::DirectPageBit { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                ram[address] |= 1 << (*bit);
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
        SPCOpcode::OR1 { oprand } => execute_or1(register, ram, oprand),
        SPCOpcode::MOV1 { oprand } => match oprand {
            SPCOprand::AbsoluteMemoryBitToCarrayFlag { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                register.set_psw_flag(PSW_FLAG_C, ((ram[address] >> bit_pos) & 0x1) != 0);
            }
            SPCOprand::CarrayFlagToAbsoluteMemoryBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let mask = (register.psw & PSW_FLAG_C) << bit_pos;
                ram[address] = if mask != 0 {
                    ram[address] | mask
                } else {
                    ram[address] & !mask
                };
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::AND1 { oprand } => execute_and1(register, ram, oprand),
        SPCOpcode::EOR1 { oprand } => match oprand {
            SPCOprand::AbsoluteBit { address_bit } => {
                let (bit_pos, address) = get_address_bit(*address_bit);
                let ret = (register.psw & PSW_FLAG_C) ^ ((ram[address] >> bit_pos) & 0x1);
                register.set_psw_flag(PSW_FLAG_C, ret != 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::TCLR1 { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                let ret = ram[*address as usize] & !register.a;
                ram[*address as usize] = ret;
                register.set_psw_flag(PSW_FLAG_N, (ret >> 7) != 0);
                register.set_psw_flag(PSW_FLAG_Z, ret == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        // 分岐命令
        SPCOpcode::BBC { bit, oprand } => match oprand {
            SPCOprand::DirectPageBitPCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page);
                if ram[address] & (1 << (*bit)) == 0 {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
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
                if ram[address] & (1 << (*bit)) != 0 {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BCC { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_C) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BPL { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_Z) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
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
                if register.a != ram[address] {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            SPCOprand::DirectPageXPCRelative {
                direct_page,
                pc_relative,
            } => {
                let address = register.get_direct_page_address(*direct_page) + register.x as usize;
                if register.a != ram[address] {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BRA { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BMI { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_N) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BVC { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_V) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BVS { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_V) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
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
                ram[address] -= 1;
                if ram[address] != 0 {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            SPCOprand::YPCRelative { pc_relative } => {
                register.y -= 1;
                if register.y != 0 {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BCS { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if register.test_psw_flag(PSW_FLAG_C) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::BNE { oprand } => match oprand {
            SPCOprand::PCRelative { pc_relative } => {
                if !register.test_psw_flag(PSW_FLAG_Z) {
                    register.pc = (register.pc as i16 + *pc_relative as i16) as u16;
                }
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
        SPCOpcode::TSET1 { oprand } => match oprand {
            SPCOprand::Absolute { address } => {
                let addr = *address as usize;
                let or = register.a | ram[addr];
                let and = register.a & ram[addr];
                ram[addr] = or;
                register.set_psw_flag(PSW_FLAG_N, (or & PSW_FLAG_N) != 0);
                register.set_psw_flag(PSW_FLAG_Z, and == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::CLR1 { bit, oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                ram[address] &= !(1 << (*bit));
            }
            _ => panic!("Invalid oprand!"),
        },
        // 算術演算命令
        SPCOpcode::ADC { oprand } => execute_adc(register, ram, oprand),
        SPCOpcode::SBC { oprand } => execute_sbc(register, ram, oprand),
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
        SPCOpcode::DECW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let mut wval = make_u16_from_u8(&ram[address..(address + 2)]);
                wval -= 1;
                ram[address + 0] = ((wval >> 8) & 0xFF) as u8;
                ram[address + 1] = ((wval >> 0) & 0xFF) as u8;
                register.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_Z, wval == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::INCW { oprand } => match oprand {
            SPCOprand::DirectPage { direct_page } => {
                let address = register.get_direct_page_address(*direct_page);
                let mut wval = make_u16_from_u8(&ram[address..(address + 2)]);
                wval += 1;
                ram[address + 0] = ((wval >> 8) & 0xFF) as u8;
                ram[address + 1] = ((wval >> 0) & 0xFF) as u8;
                register.set_psw_flag(PSW_FLAG_N, (wval >> 15) != 0);
                register.set_psw_flag(PSW_FLAG_Z, wval == 0);
            }
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DEC { oprand } => execute_dec(register, ram, oprand),
        SPCOpcode::INC { oprand } => execute_inc(register, ram, oprand),
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
        SPCOpcode::CLRP => {
            register.set_psw_flag(PSW_FLAG_P, false);
        }
        SPCOpcode::SETP => {
            register.set_psw_flag(PSW_FLAG_P, true);
        }
        SPCOpcode::CLRC => {
            register.set_psw_flag(PSW_FLAG_C, false);
        }
        SPCOpcode::SETC => {
            register.set_psw_flag(PSW_FLAG_C, true);
        }
        SPCOpcode::DAS { oprand } => match oprand {
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::MUL => {}
        SPCOpcode::MOVW { oprand } => match oprand {
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::DAA { oprand } => match oprand {
            _ => panic!("Invalid oprand!"),
        },
        SPCOpcode::CLRV => {}
        SPCOpcode::NOTC => {}
        SPCOpcode::BEQ { oprand } => match oprand {
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

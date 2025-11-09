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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                oprand: SPCOprand::DirectPageIndirectY {
                    direct_page: ram[1],
                },
            },
            2
        ),
        0x18 => create_opcode_with_length_check!(
            ram,
            SPCOpcode::OR {
                oprand: SPCOprand::ImmediateToDirectPage {
                    immediate: ram[1],
                    direct_page: ram[2],
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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                oprand: SPCOprand::YToDirectPageX {
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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                oprand: SPCOprand::IndirectPage,
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
                    direct_page_src: ram[1], // !NOTICE!
                    direct_page_dst: ram[2], // !NOTICE!
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                oprand: SPCOprand::ProgramStatusWord
            },
            1
        ),
        0xAE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::Accumulator
            },
            1
        ),
        0xCE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::XIndexRegister
            },
            1
        ),
        0xEE => create_opcode_with_length_check!(
            ram,
            SPCOpcode::POP {
                oprand: SPCOprand::YIndexRegister
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
            2
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
                    direct_page_dst: ram[1], // !NOTICE! Oprand maybe LE.
                    direct_page_src: ram[2]  // !NOTICE! Oprand maybe LE.
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
                    immediate: ram[1],
                    direct_page: ram[2],
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
                oprand: SPCOprand::DirectPageToYA {
                    direct_page: ram[1]
                }
            },
            2
        ),
        0xDA => create_opcode_with_length_check!(
            ram,
            SPCOpcode::MOVW {
                oprand: SPCOprand::YAToDirectPage {
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
            SPCOpcode::NOT1 {
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

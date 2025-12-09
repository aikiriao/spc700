/// SPCレジスタ
#[derive(Debug, Clone)]
pub struct SPCRegister {
    /// A（アキュムレータ）
    pub a: u8,
    /// X（インデックスレジスタ）
    pub x: u8,
    /// Y（インデックスレジスタ）
    pub y: u8,
    /// SP（スタックポインタ）
    pub sp: u8,
    /// PC（プログラムカウンタ）
    pub pc: u16,
    /// PSW（プログラムステータスワード）
    pub psw: u8,
}

/// SPCオペランド
#[derive(Debug)]
pub enum SPCOprand {
    Accumulator,
    XIndexRegister,
    YIndexRegister,
    ProgramStatusWord,
    RelativeAddress { address: i8 },
    DirectPage { direct_page: u8 },
    DirectPageX { direct_page: u8 },
    Absolute { address: u16 },
    DirectPageXIndirect { direct_page: u8 },
    DirectPageIndirectY { direct_page: u8 },
    IndirectPageToIndirectPage,
    DirectPageToDirectPage { direct_page_src: u8, direct_page_dst: u8 },
    ImmediateToDirectPage { direct_page: u8, immediate: u8 },
    ImmediateToX { immediate: u8 },
    AbsoluteBit { address_bit: u16 },
    AbsoluteInverseBit { address_bit: u16 },
    DirectPagePCRelative { direct_page: u8, pc_relative: i8 },
    DirectPageXPCRelative { direct_page: u8, pc_relative: i8 },
    PCRelative { pc_relative: i8 },
    PageAddress { address: u8 },
    DirectPageToX { direct_page: u8 },
    DirectPageYToX { direct_page: u8 },
    AbsoluteToX { address: u16 },
    ImmediateToY { immediate: u8 },
    DirectPageToY { direct_page: u8 },
    DirectPageXtoY { direct_page: u8 },
    AbsoluteToY { address: u16 },
    AToIndirect,
    AToIndirectAutoIncrement,
    AToDirectPage { direct_page: u8 },
    AToDirectPageX { direct_page: u8 },
    AToAbsolute { address: u16 },
    AToAbsoluteX { address: u16 },
    AToAbsoluteY { address: u16 },
    AToDirectPageXIndirect { direct_page: u8 },
    AToDirectPageIndirectY { direct_page: u8 },
    XToDirectPage { direct_page: u8 },
    XToDirectPageY { direct_page: u8 },
    XToAbsolute { address: u16 },
    YToDirectPage { direct_page: u8 },
    YToDirectPageX { direct_page: u8 },
    YToAbsolute { address: u16 },
    XToA,
    YToA,
    AToX,
    AToY,
    YToX,
    StackPointerToX,
    XToStackPointer,
    AbsoluteMemoryBitToCarrayFlag { address_bit: u16 },
    CarrayFlagToAbsoluteMemoryBit { address_bit: u16 },
    DirectPageToYA { direct_page: u8 },
    YAToDirectPage { direct_page: u8 },
    DirectPageYPCRelative { direct_page: u8 },
    DirectPageBit { direct_page: u8 },
    DirectPageBitPCRelative { direct_page: u8, pc_relative: i8 },
    IndirectPage,
    Immediate { immediate: u8 },
    AbsoluteXIndirect { address: u16 },
    AbsoluteX { address: u16 },
    AbsoluteY { address: u16 },
    IndirectAutoIncrementToA,
    DirectPageToA { direct_page: u8 },
    AbsoluteToA { address: u16 },
    IndirectToA,
    DirectPageXToA { direct_page: u8 },
    DirectPageXToY { direct_page: u8 },
    YPCRelative { pc_relative: i8 },
    ImmediateToA { immediate: u8 },
}

/// SPCオペコード
#[derive(Debug)]
pub enum SPCOpcode {
    /// NOP
    NOP,
    /// TCALL (Table Call)
    TCALL { table_index: u8 },
    /// SET1
    SET1 { bit: u8, oprand: SPCOprand },
    /// BBS (Branch on Bit Set)
    BBS { bit: u8, oprand: SPCOprand },
    /// OR (Logical OR with Memory)
    OR { oprand: SPCOprand },
    /// OR1 (Logical OR Carry Flag and Memory Bit)
    OR1 { oprand: SPCOprand },
    /// ASL (Arithmetic Left Shift Memory)
    ASL { oprand: SPCOprand },
    /// PUSH
    PUSH { oprand: SPCOprand },
    /// TSET1 (Test and set memory bits)
    TSET1 { oprand: SPCOprand },
    /// BRK (Trigger a software interrupt)
    BRK,
    /// BPL (Branch if Plus)
    BPL { oprand: SPCOprand },
    /// CLR1
    CLR1 { bit: u8, oprand: SPCOprand },
    /// BBC (Branch if Memory Bit is Cleared)
    BBC { bit: u8, oprand: SPCOprand },
    /// DECW
    DECW { oprand: SPCOprand },
    /// DEC (Decrement Memory)
    DEC { oprand: SPCOprand },
    /// CMP (Compare)
    CMP { oprand: SPCOprand },
    /// JMP (Address Jump)
    JMP { oprand: SPCOprand },
    /// CLRP (Clear Direct Page Flag)
    CLRP,
    /// AND (Logical AND with Memory)
    AND { oprand: SPCOprand },
    /// ROL (Rotate Memory Left)
    ROL { oprand: SPCOprand },
    /// CBNE (Compare and Branch if not Equal)
    CBNE { oprand: SPCOprand },
    /// BRA (Branch Always)
    BRA { oprand: SPCOprand },
    /// BMI (Branch if Minus)
    BMI { oprand: SPCOprand },
    /// INCW (Increment 16bit Memory)
    INCW { oprand: SPCOprand },
    /// CALL (Subroutine Call)
    CALL { oprand: SPCOprand },
    /// SETP (Set Direct Page Flag)
    SETP,
    /// EOR (Logical Exclusive OR with Memory)
    EOR { oprand: SPCOprand },
    /// AND1 (Logical AND Carry Flag and Memory Bit)
    AND1 { oprand: SPCOprand },
    /// LSR (Logical Right Shift Memory)
    LSR { oprand: SPCOprand },
    /// TCLR1 (Tests and Clears Memory Bits using Accumulator)
    TCLR1 { oprand: SPCOprand },
    /// PCALL (Page Call)
    PCALL { oprand: SPCOprand },
    /// BVC (Branch if Overflow Flag is Cleared)
    BVC { oprand: SPCOprand },
    /// CMPW (Compare 16bit Value)
    CMPW { oprand: SPCOprand },
    /// MOV (Byte Move)
    MOV { oprand: SPCOprand },
    /// CLRC (Clear Carry Flag)
    CLRC,
    /// ROR (Rotate Memory Right)
    ROR { oprand: SPCOprand },
    /// DBNZ (Decrement and Branch if not Zero)
    DBNZ { oprand: SPCOprand },
    /// BVS (Return from Subroutine)
    RET,
    /// BVS (Branch if Overflow Flag is Set)
    BVS { oprand: SPCOprand },
    /// BVS (Add 16bit Memory to Accumulator)
    ADDW { oprand: SPCOprand },
    /// RETI (Recurn from Interrupt)
    RETI,
    /// SETC (Set Carry Flag)
    SETC,
    /// ADC (Add Memory to Accumulator with Carry)
    ADC { oprand: SPCOprand },
    /// EOR1 (Exclusive OR Carry Flag and Memory Bit)
    EOR1 { oprand: SPCOprand },
    /// POP (Pop Register from Stack)
    POP { oprand: SPCOprand },
    /// BCC (Branch if Carry is Cleared / Branch Less Than)
    BCC { oprand: SPCOprand },
    /// SUBW (Subtract 16bit Memory from Accumulator)
    SUBW { oprand: SPCOprand },
    /// DIV (Divide Y Register Accumulator pair by X Register)
    DIV,
    /// XCN (Exchange Nibble in Accumulator)
    XCN,
    /// EI (Enable Interrupt)
    EI,
    /// SBC (Subtract Memory from Accumulator with Carry)
    SBC { oprand: SPCOprand },
    /// MOV1 (Data Move Carry Flag between Memory Bit)
    MOV1 { oprand: SPCOprand },
    /// INC (Increment Memory)
    INC { oprand: SPCOprand },
    /// BCS (Branch if Carry is Set/ Branch Greater Than or Equal)
    BCS { oprand: SPCOprand },
    /// DAS (Decimal Adjust for Subtruction)
    DAS { oprand: SPCOprand },
    /// DI (Disable Interrupt)
    DI,
    /// MUL (Multiple Accumulator and Y Register)
    MUL,
    /// BNE (Branch if Not Equal)
    BNE { oprand: SPCOprand },
    /// MOVW (Word Move)
    MOVW { oprand: SPCOprand },
    /// DAA (Decimal Adjust for Addition)
    DAA { oprand: SPCOprand },
    /// CLRV (Clear Overflow Flag)
    CLRV,
    /// NOT1 (Inverse Memory Bit)
    NOT1 { oprand: SPCOprand },
    /// NOTC (NOT Carry Flag)
    NOTC,
    /// SLEEP (Sleep the Processor)
    SLEEP,
    /// BEQ (Branch if Equal)
    BEQ { oprand: SPCOprand },
    /// STOP (Stop the Processor)
    STOP,
}

/// メモリ上にあるデータから16bitデータを読みだす
pub fn make_u16_from_u8(data: &[u8]) -> u16 {
    assert_eq!(data.len(), 2);
    ((data[1] as u16) << 8) | data[0] as u16
}

/// SPCのDSPトレイト
pub trait SPCDSP {
    /// レジスタの初期化
    fn initialize(&mut self, ram: &mut [u8], dsp_register: &[u8; 128]);
    /// レジスタに書き込み
    fn write_register(&mut self, ram: &[u8], address: u8, value: u8);
    /// レジスタから読み出し
    fn read_register(&self, ram: &[u8], address: u8) -> u8;
    /// 定期処理
    fn tick(&mut self, ram: &mut [u8]) -> [i16; 2];
}

/// DSPレジスタアドレス
pub const MVOLL_ADDRESS: u8 = 0x0C;
pub const MVOLR_ADDRESS: u8 = 0x1C;
pub const EVOLL_ADDRESS: u8 = 0x2C;
pub const EVOLR_ADDRESS: u8 = 0x3C;
pub const KON_ADDRESS: u8 = 0x4C;
pub const KOFF_ADDRESS: u8 = 0x5C;
pub const FLG_ADDRESS: u8 = 0x6C;
pub const ENDX_ADDRESS: u8 = 0x7C;
pub const EFB_ADDRESS: u8 = 0x0D;
pub const PMON_ADDRESS: u8 = 0x2D;
pub const NON_ADDRESS: u8 = 0x3D;
pub const EON_ADDRESS: u8 = 0x4D;
pub const DIR_ADDRESS: u8 = 0x5D;
pub const ESA_ADDRESS: u8 = 0x6D;
pub const EDL_ADDRESS: u8 = 0x7D;
pub const FIR0_ADDRESS: u8 = 0x0F;
pub const FIR1_ADDRESS: u8 = 0x1F;
pub const FIR2_ADDRESS: u8 = 0x2F;
pub const FIR3_ADDRESS: u8 = 0x3F;
pub const FIR4_ADDRESS: u8 = 0x4F;
pub const FIR5_ADDRESS: u8 = 0x5F;
pub const FIR6_ADDRESS: u8 = 0x6F;
pub const FIR7_ADDRESS: u8 = 0x7F;
pub const V0VOLL_ADDRESS: u8 = 0x00;
pub const V0VOLR_ADDRESS: u8 = 0x01;
pub const V0PITCHL_ADDRESS: u8 = 0x02;
pub const V0PITCHH_ADDRESS: u8 = 0x03;
pub const V0SRCN_ADDRESS: u8 = 0x04;
pub const V0ADSR1_ADDRESS: u8 = 0x05;
pub const V0ADSR2_ADDRESS: u8 = 0x06;
pub const V0GAIN_ADDRESS: u8 = 0x07;
pub const V0ENVX_ADDRESS: u8 = 0x08;
pub const V0OUTX_ADDRESS: u8 = 0x09;

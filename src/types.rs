/// SPCレジスタ
pub struct SPCRegister {
    /// PC（プログラムカウンタ）
    pub pc: u16,
    /// A（アキュムレータ）
    pub a: u8,
    /// X（インデックスレジスタ）
    pub x: u8,
    /// Y（インデックスレジスタ）
    pub y: u8,
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
    DirectPageToDirectPage { direct_page1: u8, direct_page2: u8 },
    ImmediateToDirectPage { direct_page: u8, immediate: u8 },
    ImmediateToX { immediate: u8 },
    AbsoluteMemoryBit { address_bit: u16 },
    AbsoluteInverseMemoryBit { address_bit: u16 },
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
    DirectPageToAY { direct_page: u8 },
    AYToDirectPage { direct_page: u8 },
    DirectPageYPCRelative { direct_page: u8 },
    DirectPageBit { bit: u8 },
    DirectPageBitPCRelative { bit: u8, pc_relative: i8 },
    IndirectPage,
    Immediate { immediate: u8 },
    AbsoluteXIndirect { address: u16 },
    AbsoluteX { address: u16 },
    AbsoluteY { address: u16 },
    IndirectAutoIncremenToA,
    DirectPageToA { direct_page: u8 },
    AbsoluteToA { address: u16 },
    IndirectToA,
    DirectPageXToA { direct_page: u8 },
    DirectPageXToY { direct_page: u8 },
    Indirect,
    IndirectToIndirect,
    YPCRelative { pc_relative: i8 },
}

/// SPCオペコード
#[derive(Debug)]
pub enum SPCOpcode {
    /// NOP
    NOP,
    /// TCALL (Table Call)
    TCALL { table_index: u8 },
    /// SET1
    SET1 { direct_page: u8, oprand: SPCOprand },
    /// BBS (Branch on Bit Set)
    BBS { direct_page: u8, oprand: SPCOprand },
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
    CLR1 { direct_page: u8, oprand: SPCOprand },
    /// BBC (Branch if Memory Bit is Cleared)
    BBC { direct_page: u8, oprand: SPCOprand },
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
    /// NOTC (NOT Carry Flag)
    NOTC,
    /// SLEEP (Sleep the Processor)
    SLEEP,
    /// BEQ (Branch if Equal)
    BEQ { oprand: SPCOprand },
    /// STOP (Stop the Processor)
    STOP,
}

pub fn make_u16_from_u8(data: &[u8]) -> u16 {
    ((data[0] as u16) << 8) | data[1] as u16
}

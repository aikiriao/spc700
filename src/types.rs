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

pub enum SPCOprand {
    Accumulator,
    XIndexRegister,
    YIndexRegister,
    ProgramStatusWord,
    RelativeAddress { address: i8 },
    DirectPage { direct_page: u8 },
    DirectPageX { direct_page: u8 },
    Absolute { address: u16 },
    ImmediateToA { immediate: u8 },
    IndirectPageToA,
    DirectPageToA { direct_page: u8 },
    DirectPageXToA { direct_page: u8 },
    AbsoluteToA { address: u16 },
    AbsoluteXToA { address: u16 },
    AbsoluteYToA { address: u16 },
    DirectPageXIndirectToA { direct_page: u8 },
    DirectPageIndirectYToA { direct_page: u8 },
    IndirectPageToIndirectPage,
    DirectPageToDirectPage { direct_page1: u8, direct_page2: u8 },
    ImmediateToDirectPage { direct_page: u8, immediate: u8 },
    ImmediateToX { immdiate: u8 },
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
    AToAbosluteX { address: u16 },
    AToAbosluteY { address: u16 },
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
    DirectPageToAccumlatorY { direct_page: u8 },
    AccumlatorYToDirectPage { direct_page: u8 },
    DirectPageYPCRelative { direct_page: u8 },
}

pub enum SPCOpcode {
    /// NOP
    NOP,
    /// TCALL (Table Call)
    TCALL { table_index: u8, oprand: SPCOprand },
    /// SET1
    SET1 { direct_page: u8, oprand: SPCOprand },
    /// BBS (Branch on Bit Set)
    BBS { direct_page: u8 },
    /// OR (Logical OR with Memory)
    OR { oprand: SPCOprand },
    /// OR1 (Logical OR Carry Flag and Memory Bit)
    OR1 { oprand: SPCOprand, },
    /// ASL (Arithmetic Left Shift Memory)
    ASL { oprand: SPCOprand },
    /// PUSH
    PUSH { oprand: SPCOprand },
    /// TSET1 (Test and set memory bits)
    TEST1 { oprand: SPCOprand },
    /// BRK (Trigger a software interrupt)
    BRK,
    /// BPL (Branch if Plus)
    BPL { oprand: SPCOprand },
    /// CLR1
    CLR1 { direct_page: u8, oprand: SPCOprand },
    /// BBC (Branch if Memory Bit is Cleared)
    BBC { direct_page: u8, oprand: SPCOprand },
    /// DECW
    DECW { direct_page: u8 },
    /// DEC (Decrement Memory)
    DEC { oprand: SPCOprand },
    /// CMP (Compare)
    CMP { oprand: SPCOprand },
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
    TCLR { oprand: SPCOprand },
    /// PCALL (Page Call)
    PCALL { oprand: SPCOprand },
    /// BVC (Branch if Overflow Flag is Cleared)
    BVC { oprand: SPCOprand },
    /// CMPW (Compare 16bit Value)
    CMPW { oprand: SPCOprand },
    /// MOV (Byte Move)
    MOV { oprand: SPCOprand },
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
    DAS,
    /// DI (Disable Interrupt)
    DI,
    /// MUL (Multiple Accumulator and Y Register)
    MUL,
    /// BNE (Branch if Not Equal)
    BNE { oprand: SPCOprand },
    /// MOVW (Word Move)
    MOVW { oprand: SPCOprand },
    /// DAA (Decimal Adjust for Addition)
    DAA,
    /// CLRV (Clear Overflow Flag)
    CLRV,
    /// NOTC (NOT Carry Flag)
    NOTC,
    /// SLEEP (Sleep the Processor)
    SLEEP,
    /// BEQ (Branch if Equal)
    BEQ { oprand: SPCOprand },
    /// DBNZ (Decrement and Branch if not Zero)
    DBNZ { oprand: SPCOprand },
    /// STOP (Stop the Processor)
    STOP,
}

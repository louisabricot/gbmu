/// https://gbdev.io/pandocs/CPU_Instruction_Set.html

/// Represents both 8-bit and 16-bit instructions
struct Instruction {
    /// A string that represents the instruction eg LD A (BC). Operands like addresses and values
    /// are left out.
    mnemonic: String,
    /// The portion of the instruction specifying the operation to perform. In the case of 
    opcode: u8,
    /// The number of operands
    size: u8,
    /// 
    clock_cycle: u8,
    /// The flags affected by the instruction
    flags: u8,
}
pub enum AddrMode {
    Imm8,
    Imm16,
    Address,
}
pub enum Operation {
    //8-bit transfer and Input/Output instructions
    

/// 16-bit load instructions
    
    /// Loads 2 bytes of immediate data to Register16
    LoadImm16(Register16),

    /// Loads the content of Register16 in SP
    LoadSP(Register16),

    /// Pushes the content of Register16 onto the memory stack
    Push(Register16),

    /// Pops the content from the memory stack and into Register16
    Pop(Register16),

    /// Stores the sum of SP + 1 byte of immediate data to HL register
    LoadHL(Imm8),

    /// Stores the lower byte of SP at Address specified by 2 bytes of immediate data and the upper
    /// byte of SP at address nn + 1
    LoadSP(Address(Imm16)),

/// 8-bit Arithmetic and Logical Operation Instructions
    
    /// Add the contents of Register8 to Register8::A and stores the result in register A
    Add(Register8),

    /// Add one byte of immediate data to the contents of register A and stores the result in register A
    Add(Imm8),

    /// Add the content of memory specified by the contents of register16 to the contents of
    /// register A and stores the result in register A
    Add(Address(Register16)),

    /// 
    Adc(Imm8),

    ///
    Adc(Register8),

    ///
    Adc(Address(Register16)),
}
pub enum ??? {
    LD,
    PUSH,
    POP,
    LDHL,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
    CP,
    INC,
    DEC,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SRA,
    SRL,
    SWAP,
    BIT,
    SET,
    RES,
    JP,
    JR,
    CALL,
    RET,
    RETI,
    RST,
    DAA,
    CPL,
    NOP,
    HALT,
    STOP,
}

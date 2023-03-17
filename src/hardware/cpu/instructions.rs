use crate::hardware::cpu::{Registers, Register8, Register16};

/// https://gbdev.io/pandocs/CPU_Instruction_Set.html


/// Represents both 8-bit and 16-bit instructions
struct Instruction {
    /// The portion of the instruction specifying the operation to perform. In the case of 
    opcode: Opcode,
    
    /// A string that represents the instruction eg LD A (BC)
    mnemonic: String,
    
    /// The operation to perform
    operation: Operation,
    
    /// The number of clock cycle
    clock_cycle: Clock,
    
    // /The flags affected by the instruction
    //flags: Flags,
}

/*pub enum Flags {
    ZERO,
    SUBTRACT,
    HALFCARRY,
    CARRY,
    NOT_AFFECTED,
    RESET,
}

pub enum Addr {
    
}*/

/// Enumerates the instructions speed in clock cycle
pub enum Clock {
    Four,
    Eight,
    Twelve,
    Sixteen,
    Twenty,
    ThirtyTwo,
}

impl Instruction {
    /// ...
    pub fn new(opcode: Opcode, mnemonic: String, operation: Operation, clock_cycle: u8) -> Self {
        Self {
            opcode: opcode,
            mnemonic: mnemonic,
            operation: operation,
            clock_cycle: clock_cycle,
        }
    }
    
    /// Returns the Instruction matching the same Opcode
    pub fn get(opcode: Opcode) -> Self {
        for instruction in INSTRUCTIONS {
            if instruction.opcode == opcode {
                return instruction;
            }
        }
    }
}

//static CB_INSTRUCTIONS: [Instruction; 1] = []

static INSTRUCTIONS: [Instruction; 7] = [
    Instruction::new(Opcode::LD_A_A, "LD A, A", Operation::Load(Register8::A, Register8::A), Clock::One),
    Instruction::new(Opcode::LD_A_B, "LD A, B", Operation::Load(Register8::A, Register8::B), Clock::One),
    Instruction::new(Opcode::LD_A_C, "LD A, C", Operation::Load(Register8::A, Register8::C), Clock::One),
    Instruction::new(Opcode::LD_A_D, "LD A, D", Operation::Load(Register8::A, Register8::D), Clock::One),
    Instruction::new(Opcode::LD_A_E, "LD A, E", Operation::Load(Register8::A, Register8::E), Clock::One),
    Instruction::new(Opcode::LD_A_H, "LD A, H", Operation::Load(Register8::A, Register8::H), Clock::One),
    Instruction::new(Opcode::LD_A_L, "LD A, L", Operation::Load(Register8::A, Register8::L), Clock::One),
]

pub enum Operation {
    //8-bit transfer and Input/Output instructions
   Load(Register8, Register8), 
/*
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

    /// ????
    Adc(Imm8),

    /// ???? 
    Adc(Register8),

    /// ????
    Adc(Address(Register16)),

    Adc(Imm8),


    /// Substracts the contents of Register8 from the contents of register A and store the results
    /// in register A
    Sub(Register8),

    /// Substracts one byte of immediate data to the contents of register A and stores the result
    /// to register A
    Sub(Imm8),
    
    /// Substracts the content of memory specified by the contents of register16 to the contents of
    /// register A and stores the result to register A
    Sub(Address(Register16)),

    /// Substracts the contents of Register8 and CY from the contents of register A and stores the
    /// result to register A
    Sbc(Register8),

    /// Substracts one byte of immediate data and CY to the contents of register A and stores the
    /// result to register A
    Sbc(Imm8),

    /// Substracts the content of memory specified by the contents of register16 and CY and stores
    /// the result to register A
    Sbc(Address(Register16)),

    /// Takes the logical-AND for each bit of the contents in Register8 and registerA and stores
    /// the result in register A
    And(Register8),

    /// Takes the logical-AND for each bit of one byte of immediate data and registerA and stores
    /// the result in register A
    And(Imm8),

    /// Takes the logical-AND for each bit of data pointed to by the contents of the Register16 and
    /// the register A and stores the result in register A
    And(Address(Register16)),

    /// Takes the logical-OR for each bit of the contents in Register8 and registerA and stores
    /// the result in register A
    Or(Register8),
    
    /// Takes the logical-OR for each bit of one byte of immediate data and registerA and stores
    /// the result in register A
    Or(Imm8),

    /// Takes the logical-OR for each bit of data pointed to by the contents of the Register16 and
    /// the register A and stores the result in register A
    Or(Address(Register16)),

    //TODO: le reste
    */
}

pub enum Opcode {

/// 8-BIT TRANSFER AND INPUT/OUTPUT INSTRUCTIONS
    LD_A_A,
    LD_A_B,
    LD_A_C,
    LD_A_D,
    LD_A_E,
    LD_A_H,
    LD_A_L,
}

/// https://gbdev.io/pandocs/CPU_Instruction_Set.html

/// Represents both 8-bit and 16-bit instructions
pub struct Instruction {
    /// The portion of the instruction specifying the operation to perform. In the case of
    pub opcode: Opcode,

    /// A string that represents the instruction eg LD A (BC)
    pub mnemonic: &'static str,

    // / The operation to perform
    // pub operation: Operation,
    /// The number of clock cycle
    pub clock_cycle: Clock,
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
}*/

pub enum Addr {
    IMM8,
}

/// Enumerates the instructions speed in clock cycle
pub enum Clock {
    Four,
    Eight,
    Twelve,
    Sixteen,
    Twenty,
    ThirtyTwo,
}

/*
impl Instruction {
    /// ...
    pub fn new(opcode: Opcode, mnemonic: &str, operation: Operation, clock_cycle: Clock) -> Self {
        Self {
            opcode: opcode,
            mnemonic: mnemonic,
            operation: operation,
            clock_cycle: clock_cycle,
        }
    }

    pub fn get(opcode: Opcode) -> &'static Instruction {
        for instruction in INSTRUCTIONS {
            if instruction.opcode == opcode {
                return instruction
            }
        }
        return &INSTRUCTIONS[0]
    }
}

static INSTRUCTIONS: &'static [Instruction; 7] = &[
    Instruction::new(
        Opcode::LD_A_A,
        "LD A, A",
        Operation::Load(Register8::A, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_B,
        "LD A, B",
        Operation::Load(Register8::A, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_C,
        "LD A, C",
        Operation::Load(Register8::A, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_D,
        "LD A, D",
        Operation::Load(Register8::A, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_E,
        "LD A, E",
        Operation::Load(Register8::A, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_H,
        "LD A, H",
        Operation::Load(Register8::A, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_L,
        "LD A, L",
        Operation::Load(Register8::A, Register8::L),
        Clock::Four,
    ),
];

pub enum From {
    HL,
    BC,
    DE,
    Imm16,
    C,
}
pub enum Destination {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Address(From),
}

pub enum Source {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Imm8,
    Address(From),
}
Load(Destination, Source)
pub enum Operation {
    //8-bit transfer and Input/Output instructions
    Load(Register8, Register8),
    Load(Register8, Imm8)

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
}
*/
#[allow(non_camel_case_types)]
pub enum Opcode {
    /// Non CB prefixed opcodes
    NOP,
    LD_BC_d16,
    LD_BC_A,
    INC_BC,
    INC_B,
    DEC_B,
    LD_B_d8,
    RLCA,
    LD_a16_SP,
    ADD_HL_BC,
    LD_A_BC,
    DEC_BC,
    INC_C,
    DEC_C,
    LD_C_d8,
    RRCA,

    STOP,
    LD_DE_d16,
    LD_DE_A,
    INC_DE,
    INC_D,
    DEC_D,
    LD_D_d8,
    RLA,
    JR_r8,
    ADD_HL_DE,
    LD_A_DE,
    DEC_DE,
    INC_E,
    DEC_E,
    LD_E_d8,
    RRA,

    JR_NZ_r8,
    LD_HL_d16,
    LD_HL_A,
    INC_HL,
    INC_H,
    DEC_H,
    LD_H_d8,
    DAA,
    JR_Z_d8,
    ADD_HL_HL,
    LD_A_HL,
    DEC_HL,
    INC_L,
    DEC_L,
    LD_L,
    CPL,

    JR_NC_r8,
    LD_SP_d16,
    LD_HL_A,
    INC_SP,
    INC_HL,
    DEC_HL,
    LD_HL_d8,
    SCF,
    JR_C_r8,
    ADD_HL_SP,
    LD_A_HL,
    DEC_SP,
    INC_A,
    DEC_A,
    LD_A_d8,
    CCF,

    LD_B_B,
    LD_B_C,
    LD_B_D,
    LD_B_E,
    LD_B_H,
    LD_B_L,
    LD_B_HL,
    LD_B_A,
    LD_C_B,
    LD_C_C,
    LD_C_D,
    LD_C_E,
    LD_C_H,
    LD_C_L,
    LD_C_HL,
    LD_C_A,

    LD_D_B,
    LD_D_C,
    LD_D_D,
    LD_D_E,
    LD_D_H,
    LD_D_L,
    LD_D_HL,
    LD_D_A,
    LD_E_B,
    LD_E_C,
    LD_E_D,
    LD_E_E,
    LD_E_H,
    LD_E_L,
    LD_E_HL,
    LD_E_A,

    LD_H_B,
    LD_H_C,
    LD_H_D,
    LD_H_E,
    LD_H_H,
    LD_H_L,
    LD_H_HL,
    LD_H_A,
    LD_L_B,
    LD_L_C,
    LD_L_D,
    LD_L_E,
    LD_L_H,
    LD_L_L,
    LD_L_HL,
    LD_L_A,

    LD_HL_B,
    LD_HL_C,
    LD_HL_D,
    LD_HL_E,
    LD_HL_H,
    LD_HL_L,
    HALT,
    LD_HL_A,
    LD_A_B,
    LD_A_C,
    LD_A_D,
    LD_A_E,
    LD_A_H,
    LD_A_L,
    LD_A_HL,
    LD_A_A,

    ADD_A_B,
    ADD_A_C,
    ADD_A_D,
    ADD_A_E,
    ADD_A_H,
    ADD_A_L,
    ADD_A_HL,
    ADD_A_A,
    ADC_A_B,
    ADC_A_C,
    ADC_A_D,
    ADC_A_E,
    ADC_A_H,
    ADC_A_L,
    ADC_A_HL,
    ADC_A_A,

    SUB_B,
    SUB_C,
    SUB_D,
    SUB_E,
    SUB_H,
    SUB_L,
    SUB_HL,
    SUB_A,
    SBC_A_B,
    SBC_A_C,
    SBC_A_D,
    SBC_A_E,
    SBC_A_H,
    SBC_A_L,
    SBC_A_HL,
    SBC_A_A,

    AND_B,
    AND_C,
    AND_D,
    AND_E,
    AND_H,
    AND_L,
    AND_HL,
    AND_A,
    XOR_B,
    XOR_C,
    XOR_D,
    XOR_E,
    XOR_H,
    XOR_L,
    XOR_HL,
    XOR_A,

    OR_B,
    OR_C,
    OR_D,
    OR_E,
    OR_H,
    OR_L,
    OR_HL,
    OR_A,
    CP_B,
    CP_C,
    CP_D,
    CP_D,
    CP_E,
    CP_H,
    CP_L,
    CP_HL,
    CP_A,

    RET_NZ,
    POP,
    JP_NZ_a16,
    JP_a16,
    CALL_NZ_a16,
    PUSH_BC,
    ADD_A_d8,
    RST_00H,
    RET_Z,
    RET,
    JP_Z_a16,
    CALL_Z_a16,
    CALL_a16,
    ADC_A_d8,
    RST_08H,

    RET_NC,
    POP_DE,
    JP_NC_a16,
    CALL_NC_a16,
    PUSH_DE,
    SUB_d8,
    RST_10H,
    RET_C,
    RETI,
    JP_C_a16,
    CALL_C_A16,
    SBC_A_d8,
    RST_18H,

    LDH_a8_A,
    POP_HL,
    LD_aC_A,
    PUSH_HL,
    AND_d8,
    RST_20H,
    ADD_SP_r8,
    JP_HL,
    LD_a16_A,
    XOR_d8,
    RST_28h,

    LDH_A_a8,
    POP_AF,
    LD_A_aC,
    DI,
    PUSH_AF,
    OR_d8,
    RST_30H,
    LD_HL_SPr8
    LD_SP_HL,
    LD_A_a16,
    EI,
    CP_d8,
    RST_38H,

    /// CB prefixed opcodes
    RLC_B,
    RLC_C,
    RLC_D,
    RLC_E,
    RLC_H,
    RLC_L,
    RLC_HL,
    RLC_A,
    RRC_B,
    RRC_C,
    RRC_D,
    RRC_E,
    RRC_H,
    RRC_L,
    RRC_HL,
    RRC_A,
            
    RL_B,
    RL_C,
    RL_D,
    RL_E,
    RL_H,
    RL_L,
    RL_HL,
    RL_A,
    RR_B,
    RR_C,
    RR_D,
    RR_E,
    RR_H,
    RR_L,
    RR_HL,
    RR_A,
            
    SLA_B,
    SLA_C,
    SLA_D,
    SLA_E,
    SLA_H,
    SLA_L,
    SLA_HL,
    SLA_A,
    SRA_B,
    SRA_C,
    SRA_D,
    SRA_E,
    SRA_H,
    SRA_L,
    SRA_HL,
    SRA_A,
            
    SWAP_B,
    SWAP_C,
    SWAP_D,
    SWAP_E,
    SWAP_H,
    SWAP_L,
    SWAP_HL,
    SWAP_A,
    SRL_B,
    SRL_C,
    SRL_D,
    SRL_E,
    SRL_H,
    SRL_L,
    SRL_HL,
    SRL_A,
            
    BIT_0_B,
    BIT_0_C,
    BIT_0_D,
    BIT_0_E,
    BIT_0_H,
    BIT_0_L,
    BIT_0_HL,
    BIT_0_A,
    BIT_1_B,
    BIT_1_C,
    BIT_1_D,
    BIT_1_E,
    BIT_1_H,
    BIT_1_L,
    BIT_1_HL,
    BIT_1_A,

    BIT_2_B,
    BIT_2_C,
    BIT_2_D,
    BIT_2_E,
    BIT_2_H,
    BIT_2_L,
    BIT_2_HL,
    BIT_2_A,
    BIT_3_B,
    BIT_3_C,
    BIT_3_D,
    BIT_3_E,
    BIT_3_H,
    BIT_3_L,
    BIT_3_HL,
    BIT_3_A,
            
    BIT_4_B,
    BIT_4_C,
    BIT_4_D,
    BIT_4_E,
    BIT_4_H,
    BIT_4_L,
    BIT_4_HL,
    BIT_4_A,
    BIT_5_B,
    BIT_5_C,
    BIT_5_D,
    BIT_5_E,
    BIT_5_H,
    BIT_5_L,
    BIT_5_HL,
    BIT_5_A,
            
    BIT_6_B,
    BIT_6_C,
    BIT_6_D,
    BIT_6_E,
    BIT_6_H,
    BIT_6_L,
    BIT_6_HL,
    BIT_6_A,
    BIT_7_B,
    BIT_7_C,
    BIT_7_D,
    BIT_7_E,
    BIT_7_H,
    BIT_7_L,
    BIT_7_HL,
    BIT_7_A,

    RES_B,
    RES_C,
    RES_D,
    RES_E,
    RES_H,
    RES_L,
    RES_HL,
    RES_A,
    RES_1_B,
    RES_1_C,
    RES_1_D,
    RES_1_E,
    RES_1_H,
    RES_1_L,
    RES_1_HL,
    RES_1_A,

    RES_2_B,
    RES_2_C,
    RES_2_D,
    RES_2_E,
    RES_2_H,
    RES_2_L,
    RES_2_HL,
    RES_2_A,
    RES_3_B,
    RES_3_C,
    RES_3_D,
    RES_3_E,
    RES_3_H,
    RES_3_L,
    RES_3_HL,
    RES_3_A,

    RES_4_B,
    RES_4_C,
    RES_4_D,
    RES_4_E,
    RES_4_H,
    RES_4_L,
    RES_4_HL,
    RES_4_A,
    RES_5_B,
    RES_5_C,
    RES_5_D,
    RES_5_E,
    RES_5_H,
    RES_5_L,
    RES_5_HL,
    RES_5_A,

    RES_6_B,
    RES_6_C,
    RES_6_D,
    RES_6_E,
    RES_6_H,
    RES_6_L,
    RES_6_HL,
    RES_6_A,
    RES_7_B,
    RES_7_C,
    RES_7_D,
    RES_7_E,
    RES_7_H,
    RES_7_L,
    RES_7_HL,
    RES_7_A,
            
    SET_0_B,
    SET_0_C,
    SET_0_D,
    SET_0_E,
    SET_0_H,
    SET_0_L,
    SET_0_HL,
    SET_0_A,
    SET_1_B,
    SET_1_C,
    SET_1_D,
    SET_1_E,
    SET_1_H,
    SET_1_L,
    SET_1_HL,
    SET_1_A,

    SET_2_B,
    SET_2_C,
    SET_2_D,
    SET_2_E,
    SET_2_H,
    SET_2_L,
    SET_2_HL,
    SET_2_A,
    SET_3_B,
    SET_3_C,
    SET_3_D,
    SET_3_E,
    SET_3_H,
    SET_3_L,
    SET_3_HL,
    SET_3_A,
            
    SET_4_B,
    SET_4_C,
    SET_4_D,
    SET_4_E,
    SET_4_H,
    SET_4_L,
    SET_4_HL,
    SET_4_A,
    SET_5_B,
    SET_5_C,
    SET_5_D,
    SET_5_E,
    SET_5_H,
    SET_5_L,
    SET_5_HL,
    SET_5_A,

    SET_6_B,
    SET_6_C,
    SET_6_D,
    SET_6_E,
    SET_6_H,
    SET_6_L,
    SET_6_HL,
    SET_6_A,
    SET_7_B,
    SET_7_C,
    SET_7_D,
    SET_7_E,
    SET_7_H,
    SET_7_L,
    SET_7_HL,
}

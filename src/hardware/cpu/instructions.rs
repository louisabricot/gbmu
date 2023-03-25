/// https://gbdev.io/pandocs/CPU_Instruction_Set.html

/// Represents both 8-bit and 16-bit instructions
pub struct Instruction {

    /// The portion of the instruction specifying the operation to perform. In the case of
    pub opcode: Opcode,

    /// A string that represents the instruction eg LD A (BC)
    pub mnemonic: &'static str,

    /// The operation to perform
    pub operation: Operation,
    
    /// The number of clock cycle
    pub clock_cycle: Vec<Clock>, // TODO !!
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

    pub fn getByOpcode(opcode: Opcode) -> Option<&'static Instruction> {
        for instruction in INSTRUCTIONS {
            if instruction.opcode == opcode {
                return &instruction
            }
        }
        None 
    }
}

static INSTRUCTIONS: &'static [Instruction; 7] = &[
    Instruction::new(
        Opcode::NOP,
        "NOP",
        Operation::NOP,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_BC_d16,
        "LD B, d16",
        Operation::Load(Register16::BC, Imm16),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LD_BC_A,
        "LD (BC), A",
        Operation::Load(Address::BC, Register8::A),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_BC,
        "INC B",
        Operation::Inc(Register16::BC),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_B,
        "INC B",
        Operation::Inc(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_B,
        "DEC B",
        Operation::Dec(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_d8,
        "LD B, d8",
        Operation::Load(Register8::B, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::RLCA,
        "RLCA",
        Operation::Rlca,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_a16_SP,
        "LD (a16), SP",
        Operation::Load(Address::Imm16, Register16::SP),
        Clock::Twenty,
    ),
    Instruction::new(
        Opcode::ADD_HL_BC,
        "ADD HL, BC",
        Operation::Add(Register16::HL, Register16::BC),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_A_BC,
        "LD A, (BC)",
        Operation::Load(Register8::A, Address::BC),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DEC_BC,
        "DEC BC",
        Operation::Dec(Register16::BC),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_C,
        "INC C",
        Operation::Inc(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_C,
        "DEC C",
        Operation::Dec(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_d8,
        "LD C, d8",
        Operation::Load(Register8::C, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::RRCA,
        "RRCA",
        Operation::Rrca,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::STOP,
        "STOP",
        Operation::Stop,
        Clock::None, //pas compris 4 ? 8?
    ),
    Instruction::new(
        Opcode::LD_DE_d16,
        "LD DE, d16",
        Operation::Load(Register16::DE, Imm16),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LD_DE_A,
        "LD (DE), A",
        Operation::Load(Address::DE, Register8::A),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_DE,
        "INC DE",
        Operation::Inc(Register16::DE),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_D,
        "INC D",
        Operation::Inc(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_D,
        "DEC D",
        Operation::Dec(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_d8,
        "LD D, d8",
        Operation::Load(Register::D, Imm8)
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::RLA,
        "RLA",
        Operation::Rla,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_r8,
        "JR r8",
        Operation::Jr(Imm8),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::ADD_HL_DE,
        "ADD HL, DE",
        Operation::Add(Register16::HL, Register16::DE),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_A_DE,
        "LD A, (DE)",
        Operation::Load(Register8::A, Address::DE),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DEC_DE,
        "DEC DE",
        Operation::Dec(Register16::DE),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_E,
        "INC E",
        Operation::Inc(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_E,
        "DEC E",
        Operation::Dec(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_d8,
        "LD E, d8",
        Operation::Load(Register8::E, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::RRA,
        "RRA",
        Operation::Rra,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_NZ_r8,
        "JR NZ, r8",
        Operation::Jr(Condition::NZ, Imm8),
        Clock::Condition, //12/8 depending on the condition
    ),
    Instruction::new(
        Opcode::LD_HL_d16,
        "LD HL, d16",
        Operation::Load(Register16::HL, Imm16),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LDI_HL_A,
        "LD (HL+), A",
        Operation::LoadI(Adress::HL, Register8::A),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_HL,
        "INC HL",
        Operation::Inc(Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_H,
        "INC H",
        Operation::Inc(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_H,
        "DEC H",
        Operation::Dec(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_d8,
        "LD H, d8",
        Operation::Load(Register8::H, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DAA,
        "DAA",
        Operation::Daa,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_Z_r8,
        "JR Z, r8",
        Operation::Jr(Condition::Z, Imm8),
        Clock::Condition, // 12/8
    ),
    Instruction::new(
        Opcode::ADD_HL_HL,
        "ADD HL, HL",
        Operation::Add(Register16::HL, Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LDI_A_HL,
        "LD A, (HL+)",
        Operation::LoadI(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DEC_HL,
        "DEC HL",
        Operation::Dec(Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_L,
        "INC L",
        Operation::Inc(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_L,
        "DEC L",
        Operation::Dec(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_d8,
        "LD L, d8",
        Operation::Load(Register8::L. Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::CPL,
        "CPL",
        Operation::Cpl,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_NC_r8,
        "JR NC, r8",
        Operation::Jr(Condition::NC, Imm8),
        Clock::Condition, // 12/8
    ),
    Instruction::new(
        Opcode::LD_SP_d16,
        "LD SP, d16",
        Operation::Load(Register16::SP, Imm16),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LDI_HL_A,
        "LD (HL+), A",
        Operation::LoadI(Address::HL, Register8::A),
        Clock::Eight, 
    ),
    Instruction::new(
        Opcode::INC_HL,
        "INC HL",
        Operation::Inc(Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_H,
        "INC H",
        Operation::Inc(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_H,
        "DEC H",
        Operation::Dec(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_d8,
        "LD H, d8",
        Operation::Load(Register8::H, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DDA,
        "DDA",
        Operation::Dda,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_Z_r8,
        "JR Z, r8",
        Operation::Jr(Condition::Z, Imm8),
        Clock::Condition,
    ),
    Instruction::new(
        Opcode::ADD_HL_HL,
        "ADD HL, HL"
        Operation::Add(Register16::HL, Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LDI_A_HL,
        "LD A, (HL+)",
        Operation::LoadI(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DEC_HL,
        "DEC HL",
        Operation::Dec(Register16::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_L,
        "INC L",
        Operation::Inc(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_L,
        "DEC L",
        Operation::Dec(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_d8,
        "LD L, d8",
        Operation::Load(Register8::L, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::CPL,
        "CPL",
        Operation::Cpl,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_NC_r8,
        "JR NC, r8",
        Operation::Jr(Condition::NC, Imm8),
        Clock::Condition,
    ),
    Instruction::new(
        Opcode::LD_SP_d16,
        "LD SP, d16",
        Operation::Load(Register16:;SP, Imm16),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LDD_HL_A,
        "LD (HL-), A",
        Operation::LoadD(Address::HL, Register8::A),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_SP,
        "INC SP",
        Operation::Inc(Register16::SP),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_aHL,
        "INC (HL)",
        Operation::Inc(Address::HL),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::DEC_aHL,
        "DEC (HL)",
        Operation::Dec(Address::HL),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::LD_aHL_d8,
        "LD (HL), d8",
        Operation::Load(Address::HL, Imm8),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::SCF,
        "SCF",
        Operation::Scf,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::JR_C_r8,
        "JR C, d8",
        Operation::Jr(Condition::C, Imm8),
        Clock::Condition,
    ),
    Instruction::new(
        Opcode::ADD_HL_SP,
        "ADD HL, SP",
        Operation::Add(Register16::HL, Register16::SP),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LDD_A_aHL,
        "LOAD A, (HL-)",
        Operation::Load(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::DEC_SP,
        "DEC SP",
        Operation::Dec(Register16::SP),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::INC_A,
        "INC A",
        Operation::Inc(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::DEC_A,
        "DEC A",
        Operation::Dec(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_A_d8,
        "LD A, d8",
        Operation::Load(Register8::A, Imm8),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::CCF,
        "CCF",
        Operation::Ccf,
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_B,
        "LD B, B",
        Operation::Load(Register8::B, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_C,
        "LD B, C",
        Operation::Load(Register8::B, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_D,
        "LD B, D",
        Operation::Load(Register8::B, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_E,
        "LD B, E",
        Operation::Load(Register8::B, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_H,
        "LD B, H",
        Operation::Load(Register8::B, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_L,
        "LD B, L",
        Operation::Load(Register8::B, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_B_aHL,
        "LD B, (HL)",
        Operation::Load(Register8::B, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_B_A,
        "LD B, A",
        Operation::Load(Register8::B, Register8::A),
        Clock::Four,
    ),

    Instruction::new(
        Opcode::LD_C_B,
        "LD C, B",
        Operation::Load(Register8::C, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_C,
        "LD C, C",
        Operation::Load(Register8::C, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_D,
        "LD C, D",
        Operation::Load(Register8::C, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_E,
        "LD C, E",
        Operation::Load(Register8::C, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_H,
        "LD C, H",
        Operation::Load(Register8::C, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_L,
        "LD C, L",
        Operation::Load(Register8::C, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_C_aHL,
        "LD C, (HL)",
        Operation::Load(Register8::C, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_C_A,
        "LD C, A",
        Operation::Load(Register8::C, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_B,
        "LD D, B",
        Operation::Load(Register8::D, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_C,
        "LD D, C",
        Operation::Load(Register8::D, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_D,
        "LD D, D",
        Operation::Load(Register8::D, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_E,
        "LD D, E",
        Operation::Load(Register8::D, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_H,
        "LD D, H",
        Operation::Load(Register8::D, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_L,
        "LD D, L",
        Operation::Load(Register8::D, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_D_aHL,
        "LD D, (HL)",
        Operation::Load(Register8::D, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_D_A,
        "LD D, A",
        Operation::Load(Register8::D, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_B,
        "LD E, B",
        Operation::Load(Register8::E, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_C,
        "LD E, C",
        Operation::Load(Register8::E, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_D,
        "LD E, D",
        Operation::Load(Register8::E, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_E,
        "LD E, E",
        Operation::Load(Register8::E, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_H,
        "LD E, H",
        Operation::Load(Register8::E, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_L,
        "LD E, L",
        Operation::Load(Register8::E, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_E_aHL,
        "LD E, (HL)",
        Operation::Load(Register8::E, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_E_A,
        "LD E, A",
        Operation::Load(Register8::E, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_B,
        "LD H, B",
        Operation::Load(Register8::H, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_C,
        "LD H, C",
        Operation::Load(Register8::H, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_D,
        "LD H, D",
        Operation::Load(Register8::H, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_E,
        "LD H, E",
        Operation::Load(Register8::H, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_H,
        "LD H, H",
        Operation::Load(Register8::H, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_L,
        "LD H, L",
        Operation::Load(Register8::H, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_H_aHL,
        "LD H, (HL)",
        Operation::Load(Register8::H, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_H_A,
        "LD H, A",
        Operation::Load(Register8::H, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_B,
        "LD L, B",
        Operation::Load(Register8::L, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_C,
        "LD L, C",
        Operation::Load(Register8::L, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_D,
        "LD L, D",
        Operation::Load(Register8::L, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_E,
        "LD L, E",
        Operation::Load(Register8::L, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_H,
        "LD L, H",
        Operation::Load(Register8::L, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_L,
        "LD L, L",
        Operation::Load(Register8::L, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_L_aHL,
        "LD L, (HL)",
        Operation::Load(Register8::L, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_L_A,
        "LD L, A",
        Operation::Load(Register8::L, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_B,
        "LD (HL), B",
        Operation::Load(Address::HL, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_C,
        "LD (HL), C",
        Operation::Load(Address::HL, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_D,
        "LD (HL), D",
        Operation::Load(Address::HL, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_E,
        "LD (HL), E",
        Operation::Load(Address::HL, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_H,
        "LD (HL), H",
        Operation::Load(Address::HL, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::LD_HL_L,
        "LD (HL), L",
        Operation::Load(Address::HL, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::HALT,
        "HALT",
        Operation::Halt,
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_HL_A,
        "LD (HL), A",
        Operation::Load(Address::HL, Register8::A),
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
    Instruction::new(
        Opcode::LD_A_aHL,
        "LD A, (HL)",
        Operation::Load(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::LD_A_A,
        "LD A, A",
        Operation::Load(Register8::A, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_B,
        "ADD A, B",
        Operation::Add(Register8::A, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_C,
        "ADD A, C",
        Operation::Add(Register8::A, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_D,
        "ADD A, D",
        Operation::Add(Register8::A, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_E,
        "ADD A, E",
        Operation::Add(Register8::A, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_H,
        "ADD A, H",
        Operation::Add(Register8::A, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_L,
        "ADD A, L",
        Operation::Add(Register8::A, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADD_A_HL,
        "ADD A, (HL)",
        Operation::Add(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::ADD_A_A,
        "ADD A, A",
        Operation::Add(Register8::A, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_B,
        "ADC A, B",
        Operation::Adc(Register8::A, Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_C,
        "ADC A, C",
        Operation::Adc(Register8::A, Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_D,
        "ADC A, D",
        Operation::Adc(Register8::A, Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_E,
        "ADC A, E",
        Operation::Adc(Register8::A, Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_H,
        "ADC A, H",
        Operation::Adc(Register8::A, Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_L,
        "ADC A, L",
        Operation::Adc(Register8::A, Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::ADC_A_HL,
        "ADC A, (HL)",
        Operation::Adc(Register8::A, Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::ADC_A_A,
        "ADC A, A",
        Operation::Adc(Register8::A, Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_B,
        "SUB B",
        Operation::Sub(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_C,
        "SUB C",
        Operation::Sub(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_D,
        "SUB D",
        Operation::Sub(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_E,
        "SUB E",
        Operation::Sub(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_H,
        "SUB H",
        Operation::Sub(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_L,
        "SUB L",
        Operation::Sub(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SUB_HL,
        "SUB (HL)",
        Operation::Sub(Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::SUB_A,
        "SUB A",
        Operation::Sub(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_A_B,
        "SBC A, B",
        Operation::Sbc(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_A_C,
        "SBC A, C",
        Operation::Sbc(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_A_D,
        "SBC A, D",
        Operation::Sbc(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_E,
        "SBC A, E",
        Operation::Sbc(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_H,
        "SBC A, H",
        Operation::Sbc(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_L,
        "SBC A, L",
        Operation::Sbc(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::SBC_A_HL,
        "SBC A, (HL)",
        Operation::Sbc(Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::SBC_A_A,
        "SBC A, A",
        Operation::Sbc(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_B,
        "AND B",
        Operation::And(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_C,
        "AND C",
        Operation::And(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_D,
        "AND D",
        Operation::And(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_E,
        "AND E",
        Operation::And(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_H,
        "AND H",
        Operation::And(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_L,
        "AND L",
        Operation::And(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::AND_HL,
        "AND (HL)",
        Operation::And(Address::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::AND_A,
        "AND A",
        Operation::And(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_B,
        "XOR B",
        Operation::Xor(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_C,
        "XOR C",
        Operation::Xor(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_D,
        "XOR D",
        Operation::Xor(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_E,
        "XOR E",
        Operation::Xor(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_H,
        "XOR H",
        Operation::Xor(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_L,
        "XOR L",
        Operation::Xor(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::XOR_HL,
        "XOR (HL)",
        Operation::Xor(Adress::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::XOR_A,
        "XOR A",
        Operation::Xor(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_B,
        "OR B",
        Operation::Or(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_C,
        "OR C",
        Operation::Or(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_D,
        "OR D",
        Operation::Or(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_E,
        "OR E",
        Operation::Or(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_H,
        "OR H",
        Operation::Or(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_L,
        "OR L",
        Operation::Or(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::OR_HL,
        "OR (HL)",
        Operation::Or(Adress::HL),
        Clock::Eight,
    ),
    Instruction::new(
        Opcode::OR_A,
        "OR A",
        Operation::Or(Register8::A),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_B,
        "CP B",
        Operation::Cp(Register8::B),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_C,
        "CP C",
        Operation::Cp(Register8::C),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_D,
        "CP D",
        Operation::Cp(Register8::D),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_E,
        "CP E",
        Operation::Cp(Register8::E),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_H,
        "CP H",
        Operation::Cp(Register8::H),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_L,
        "CP L",
        Operation::Cp(Register8::L),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::CP_HL,
        "CP (HL)",
        Operation::Cp(Address::HL),
        Clock::Four,
    ),
    Instruction::new(
        Opcode::RET_NZ,
        "RET NZ",
        Operation::Ret(Condition::NZ),
        Clock::Condition,
    ),
    Instruction::new(
        Opcode::POP_BC,
        "POP BC",
        Operation::Pop(Register16::BC),
        Clock::Twelve,
    ),
    Instruction::new(
        Opcode::JP_NZ_a16,
        "JP NZ, a16",
        Operation::Jp(Condition::NZ, Imm16),
        Clock::Condition,
    ),
    Instruction::new(
        Opcode::JP_a16,
        "JP a16",
        Operation::Jp(Imm16),
        Clock::Sixteen,
    ),
];

pub enum Address {
    HL,
    BC,
    DE,
    Imm16,
    C,
    Imm8,
}

pub enum Operation {

    /// 8-bit load instructions
    
    /// Loads to the left-hand 8-bit register, data from the right-hand 8-bit register 
    Load(Register8, Register8),

    /// Loads to the 8-bit register, the immediate 8-bit data
    Load(Register8, Imm8),

    /// Loads to the 8-bit register, data from the absolute address
    Load(Register8, Address),

    /// Loads to the absolute address, data from the 8-bit register
    Load(Address, Register8),

    /// Loads to the absolute address, the immediate 8-bit data
    Load(Address, Imm8),

    /// Loads to the 8-bit register, data from the address. The full 16-bit absolute address is
    /// obtained by setting the most significant byte to 0xFF and the least significant byte to the
    /// value . So the possible range is 0xFF00- 0xFFFF.
    LoadH(Register8, Address),

    /// Loads to the address specified by the indirect, data from the 8-bit register. The full
    /// 16-bit absolute address is obtained by setting the most significant byte to 0xFF and the
    /// least significant byte to the value of Address, so the possible range is 0xFF00 - 0xFFFF.
    LoadH(Address, Register8),

    /// Loads to the absolute address specified by the 16-bit register, data from the 8-bit register. The value of the register is decremented after the memory write.
    LoadD(Address, Register8),

    
    /// Loads to the 8-bit register, data from the absolute address specified by the 16-bit register. The value of the register is decremented after the memory write.
    LoadD(Register8, Address),

    
    /// Loads to the absolute address specified by the 16-bit register, data from the 8-bit register. The value of the register is incremented after the memory write.
    LoadI(Address, Register8),

    
    /// Loads to the 8-bit register, data from the absolute address specified by the 16-bit register. The value of the register is incremented after the memory write.
    LoadI(Register8, Address),

    /// 16-bit load instructions

    /// Loads to the 16-bit register, the immeidate 16-bit data
    Load(Register16, Imm16),

    /// Loads to the absolute address specified by the 16-bit operand, data from the 16-bit
    /// register
    Load(Address, Register16),

    /// Loads to the left-hand 16-bit register, data from the right-hand 16-bit register
    Load(Register16, Register16),

    /// Pushes to the stack memory, data from the 16-bit register
    Push(Register16),

    /// Pops to the 16-bit register, data from the stack memory
    Pop(Register16),

    /// 8-bit arithmetic and logical instructions
    
    /// Adds to the 8-bit register A, the 8-bit register and stores the  
    Add(Register8),

    /// Adds to the 8-bit register A, data from the absolute address specified by the 16-bit
    /// register, and stores the result bakc into register A
    Add(Address),

    /// Adds to the 8-bit register A, the immediata 8-bit data and stores the result back into
    /// register A
    Add(Imm8),

    /// Adds to the 8-bit register A, the carry flag and the 8-bit register, and stores the result
    /// back into register A
    Adc(Register8),

    /// Adds to the 8-bit register A, the carry flag and data from the absolute address specified
    /// by the 16-bit register and stores the result back into register A
    Adc(Address),

    /// Adds to the 8-bit register A, the carry flag and the immediate data, and stores the result
    /// back into register A
    Adc(Imm8),

    /// Substracts from the 8-bit register A, the 8-bit register, and stores the result back into
    /// register A
    Sub(Register8),

    /// Substracts from the 8-bit register A, data from the absolute address specified by the
    /// 16-bit register and stores the result back into register A
    Sub(Address),

    /// Substracts from the 8-bit register A, the immediate data and stores the result back into
    /// register A
    Sub(Imm8),

    /// Substracts from the 8-bit register A, the carry flag and 8-bit register, and stores the
    /// result back into register A
    Sbc(Register8),

    /// Substracts from the 8-bit register A, the carry flag and data from the absolute address
    /// specified by the 16-bit register and stores the result back into register A
    Sbc(Address),

    /// Substracts from the 8-bit register A, the carry flag and the immediate 8-bit data, and
    /// stores the result back into register A.
    Sbc(Imm8),

    /// Substracts from the 8-bit register A, the 8-bit register and updates flags based on the
    /// result. This instruction is identical to SUB r but does not update register A
    Cp(Register8),

    /// Substracts from the 8-bit registe A, data from the absolute address specified by the 16-bit
    /// register, and updates flags based on the result. This instruction is identifical to SUB
    /// addr but does not udpate register A
    Cp(Address),

    /// Substracts from the 8-bit register A, the immediate data and updates flags based on the
    /// result. This instruction is identifical to SUB n but does not update register A.
    Cp(Imm8),

    /// Increments data in the 8-bit register
    Inc(Register8),

    /// Increments data at the absolute address specified by the 16-bit register
    Inc(Address),

    /// Decrements data in the 8-bit register
    Dec(Register8),

    /// Decrements data at the absolute address specified by the 16-bit register
    Dec(Address),

    /// Performs a bitwise AND operation between the 8-bit register A and the 8-bit register, and
    /// stores the result into register A
    And(Register8),

    
    /// Performs a bitwise AND operation between the 8-bit register A and data from the absolute
    /// address specified by the 16-bit register, and
    /// stores the rsult back into register A
    And(Address),

    /// Performs a bitwise AND operation between the 8-bit register A and the immediate value, and
    /// stores the result back into register A
    And(Imm8),

    /// Performs a bitwise OR operation between 8-bit register A and the 8-bit register, and stores
    /// the result back into register A
    Or(Register8),
    
    /// Performs a bitwise OR operation between the 8-bit register A and the immediate value, and
    /// stores the result back into register A
    Or(Imm8),

    /// Performs a bitwise OR operation between the 8-bit register A and data from the absolute
    /// address specified by the 16-bit register, and
    /// stores the rsult back into register A
    Or(Address),
    
    /// Performs a bitwise XOR operation between 8-bit register A and the 8-bit register, and stores
    /// the result back into register A
    Xor(Register8),
    
    /// Performs a bitwise XOR operation between the 8-bit register A and the immediate value, and
    /// stores the result back into register A
    Xor(Imm8),

    /// Performs a bitwise XOR operation between the 8-bit register A and data from the absolute
    /// address specified by the 16-bit register, and
    /// stores the rsult back into register A
    Xor(Address),

    /// Flips the carry flag, and clears the N and H flags
    Ccf,

    /// Sets the carry flags, and clears the N and H flags
    Scf,

    /// Decimal Adjust Accumulator
    /// TODO: describe operation
    Daa,

    /// Flops all the bits in the 8-bit register A and set s the N and H flags
    Cpl,

    /// TODO: 16-bit arithmetic instructions
    /// TODO: Rotate, shift and bit operation instructions


    /// Control flow instructions
    
    /// Unconditional jump to the absolute address specified by the 16-bit operand
    Jp(Address),

    /// Condition jump to the absolute address specified by the 16-bit operand, depending on the
    /// condition. Note that the operand (absolute address) is read even when condition is false.
    Jp(Condition, Imm16),

    /// Unconditional jump to the relative address specified by the signed 8-bit operand
    Jr(Imm8),

    /// Condition jump to the relative address specified by the signed 8-bit operand, adepending
    /// on the condition
    Jr(Condition, Imm8),

    /// Unconditional function call to the absolute address specified b the 16-bit operand
    Call(Imm16),

    /// Condition function call to the absolute address specified by the 16-bit operand,
    /// depending on the condition
    /// Note that the operand (absolute address) is read even when the condition is false!
    Call(Condition, Imm16),

    /// Unconditional return from a function
    Ret,

    /// Conditonal return from a function, depending on the condition
    Ret(Condition),

    /// Unconditional return from a function. Also enable interrupts by setting IME=1
    Reti,

    /// Unconditional function call to the absolute fixed address defined by the opcode
    Rst(Imm8),

    /// TODO: Halt
    Halt,

    /// TODO: Stop
    Stop,

    /// TODO: di
    Di,

    /// Schedules interrupt handling to be enabled after the next machine cycle
    Ei,

    /// No operation. This instruction adds a delay of one machine cycle and increment PC by one
    Nop,
}

pub enum Condition {

}

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
    LDI_HL_A,
    INC_HL,
    INC_H,
    DEC_H,
    LD_H_d8,
    DAA,
    JR_Z_r8,
    ADD_HL_HL,
    LDI_A_HL,
    DEC_HL,
    INC_L,
    DEC_L,
    LD_L_d8,
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

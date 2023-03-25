/// https://gbdev.io/pandocs/CPU_Instruction_Set.html

/// Represents both 8-bit and 16-bit instructions
pub struct Instruction {

    /// The portion of the instruction specifying the operation to perform. In the case of
    pub opcode: Opcode,

    /// A string that represents the instruction eg LD A (BC)
    pub mnemonic: &'static str,

    pub operand: Option<Imm>,

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
    TwentyFour,
    ThirtyTwo,
}

pub enum Page0 {
    Byte0,
    Byte1,
    Byte2,
    Byte3,
    Byte4,
    Byte5,
    Byte6,
}
pub enum Imm {
    8,
    16,
}

impl Instruction {
    /// ...
    pub fn new(opcode: Opcode, mnemonic: &str, operand: Option<Imm>, operation: Operation, clock_cycle: Clock) -> Self {
        Self {
            opcode: opcode,
            mnemonic: mnemonic,
            operand: operand,
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
        None,
        Operation::NOP,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_BC_d16,
        "LD B, {:#06x}",
        Some(Imm::16),
        Operation::Load(Register16::BC, Imm::16),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::LD_BC_A,
        "LD (BC), A",
        None,
        Operation::Load(Address::BC, Register8::A),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_BC,
        "INC BC",
        None,
        Operation::Inc(Register16::BC),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_B,
        "INC B",
        None,
        Operation::Inc(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_B,
        "DEC B",
        None,
        Operation::Dec(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_d8,
        "LD B, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::B, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RLCA,
        "RLCA",
        None,
        Operation::Rlca,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_a16_SP,
        "LD {:#06x}, SP",
        Some(Imm::16),
        Operation::Load(Address::Imm::16, Register16::SP),
        vec![Clock::Twenty],
    ),
    Instruction::new(
        Opcode::ADD_HL_BC,
        "ADD HL, BC",
        None,
        Operation::Add(Register16::HL, Register16::BC),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_A_BC,
        "LD A, (BC)",
        None,
        Operation::Load(Register8::A, Address::BC),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::DEC_BC,
        "DEC BC",
        None,
        Operation::Dec(Register16::BC),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_C,
        "INC C",
        Operation::Inc(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_C,
        "DEC C",
        Operation::Dec(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_d8,
        "LD C, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::C, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RRCA,
        "RRCA",
        None,
        Operation::Rrca,
        vec![Clock::Four],
    ),

    /// 0x10-0x1F
    Instruction::new(
        Opcode::STOP,
        "STOP",
        None,
        Operation::Stop,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_DE_d16,
        "LD DE, {:#06x}",
        Some(Imm::16),
        Operation::Load(Register16::DE, Imm::16),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::LD_DE_A,
        "LD (DE), A",
        None,
        Operation::Load(Address::DE, Register8::A),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_DE,
        "INC DE",
        None,
        Operation::Inc(Register16::DE),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_D,
        "INC D",
        None,
        Operation::Inc(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_D,
        "DEC D",
        None,
        Operation::Dec(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_d8,
        "LD D, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register::D, Imm::8)
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RLA,
        "RLA",
        None,
        Operation::Rla,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::JR_r8,
        "JR {:#04x}",
        Some(Imm::8),
        Operation::Jr(Imm::8),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::ADD_HL_DE,
        "ADD HL, DE",
        None,
        Operation::Add(Register16::HL, Register16::DE),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_A_DE,
        "LD A, (DE)",
        None,
        Operation::Load(Register8::A, Address::DE),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::DEC_DE,
        "DEC DE",
        None,
        Operation::Dec(Register16::DE),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_E,
        "INC E",
        None,
        Operation::Inc(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_E,
        "DEC E",
        None,
        Operation::Dec(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_d8,
        "LD E, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::E, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RRA,
        "RRA",
        None,
        Operation::Rra,
        vec![Clock::Four],
    ),
    /// 0x20 - 0x2F
    Instruction::new(
        Opcode::JR_NZ_r8,
        "JR NZ, {:#04x}",
        None,
        Operation::Jr(Condition::NZ, Imm::8),
        vec![Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_HL_d16,
        "LD HL, {:#06x}",
        Operation::Load(Register16::HL, Imm::16),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::LDI_HL_A,
        "LD (HL+), A",
        None,
        Operation::LoadI(Adress::HL, Register8::A),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_HL,
        "INC HL",
        None,
        Operation::Inc(Register16::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_H,
        "INC H",
        None,
        Operation::Inc(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_H,
        "DEC H",
        None,
        Operation::Dec(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_d8,
        "LD H, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::H, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::DAA,
        "DAA",
        None,
        Operation::Daa,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::JR_Z_r8,
        "JR Z, {:#04x}",
        Some(Imm::8),
        Operation::Jr(Condition::Z, Imm::8),
        vec![Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::ADD_HL_HL,
        "ADD HL, HL",
        None,
        Operation::Add(Register16::HL, Register16::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LDI_A_HL,
        "LD A, (HL+)",
        None,
        Operation::LoadI(Register8::A, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::DEC_HL,
        "DEC HL",
        None,
        Operation::Dec(Register16::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_L,
        "INC L",
        None,
        Operation::Inc(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_L,
        "DEC L",
        None,
        Operation::Dec(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_d8,
        "LD L, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::L. Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::CPL,
        "CPL",
        None,
        Operation::Cpl,
        vec![Clock::Four],
    ),
    /// 0x30 -0x3F
    Instruction::new(
        Opcode::JR_NC_r8,
        "JR NC, {:#04x}",
        Some(Imm::8),
        Operation::Jr(Condition::NC, Imm::8),
        vec![Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_SP_d16,
        "LD SP, {:#06x}",
        Some(Imm::16),
        Operation::Load(Register16::SP, Imm::16),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::LDD_HL_A,
        "LD (HL-), A",
        None,
        Operation::LoadD(Address::HL, Register8::A),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_SP,
        "INC SP",
        None,
        Operation::Inc(Register16::SP),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_aHL,
        "INC (HL)",
        None,
        Operation::Inc(Address::HL),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::DEC_aHL,
        "DEC (HL)",
        None,
        Operation::Dec(Address::HL),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::LD_aHL_d8,
        "LD (HL), {:#04x}",
        Some(Imm::8),
        Operation::Load(Address::HL, Imm::8),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::SCF,
        "SCF",
        None,
        Operation::Scf,
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::JR_C_r8,
        "JR C, {:#04x}",
        Some(Imm::8),
        Operation::Jr(Condition::C, Imm::8),
        vec![Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::ADD_HL_SP,
        "ADD HL, SP",
        None,
        Operation::Add(Register16::HL, Register16::SP),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LDD_A_aHL,
        "LD A, (HL-)",
        None,
        Operation::Load(Register8::A, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::DEC_SP,
        "DEC SP",
        None,
        Operation::Dec(Register16::SP),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::INC_A,
        "INC A",
        None,
        Operation::Inc(Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::DEC_A,
        "DEC A",
        None,
        Operation::Dec(Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_d8,
        "LD A, {:#04x}",
        Some(Imm::8),
        Operation::Load(Register8::A, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::CCF,
        "CCF",
        None,
        Operation::Ccf,
        vec![Clock::Four],
    ),
    /// 0x40 - 0x4F
    Instruction::new(
        Opcode::LD_B_B,
        "LD B, B",
        None,
        Operation::Load(Register8::B, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_C,
        "LD B, C",
        None,
        Operation::Load(Register8::B, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_D,
        "LD B, D",
        None,
        Operation::Load(Register8::B, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_E,
        "LD B, E",
        None,
        Operation::Load(Register8::B, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_H,
        "LD B, H",
        None,
        Operation::Load(Register8::B, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_L,
        "LD B, L",
        None,
        Operation::Load(Register8::B, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_B_aHL,
        "LD B, (HL)",
        None,
        Operation::Load(Register8::B, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_B_A,
        "LD B, A",
        None,
        Operation::Load(Register8::B, Register8::A),
        vec![Clock::Four],
    ),

    Instruction::new(
        Opcode::LD_C_B,
        "LD C, B",
        None,
        Operation::Load(Register8::C, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_C,
        "LD C, C",
        None,
        Operation::Load(Register8::C, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_D,
        "LD C, D",
        None,
        Operation::Load(Register8::C, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_E,
        "LD C, E",
        None,
        Operation::Load(Register8::C, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_H,
        "LD C, H",
        None,
        Operation::Load(Register8::C, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_L,
        "LD C, L",
        None,
        Operation::Load(Register8::C, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_C_aHL,
        "LD C, (HL)",
        None,
        Operation::Load(Register8::C, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_C_A,
        "LD C, A",
        None,
        Operation::Load(Register8::C, Register8::A),
        vec![Clock::Four],
    ),
    /// 0x50 = 0x5F
    Instruction::new(
        Opcode::LD_D_B,
        "LD D, B",
        None,
        Operation::Load(Register8::D, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_C,
        "LD D, C",
        None,
        Operation::Load(Register8::D, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_D,
        "LD D, D",
        None,
        Operation::Load(Register8::D, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_E,
        "LD D, E",
        None,
        Operation::Load(Register8::D, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_H,
        "LD D, H",
        None,
        Operation::Load(Register8::D, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_L,
        "LD D, L",
        None,
        Operation::Load(Register8::D, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_D_aHL,
        "LD D, (HL)",
        None,
        Operation::Load(Register8::D, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_D_A,
        "LD D, A",
        None,
        Operation::Load(Register8::D, Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_B,
        "LD E, B",
        None,
        Operation::Load(Register8::E, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_C,
        "LD E, C",
        None,
        Operation::Load(Register8::E, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_D,
        "LD E, D",
        None,
        Operation::Load(Register8::E, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_E,
        "LD E, E",
        None,
        Operation::Load(Register8::E, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_H,
        "LD E, H",
        None,
        Operation::Load(Register8::E, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_L,
        "LD E, L",
        None,
        Operation::Load(Register8::E, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_E_aHL,
        "LD E, (HL)",
        None,
        Operation::Load(Register8::E, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_E_A,
        "LD E, A",
        None,
        Operation::Load(Register8::E, Register8::A),
        vec![Clock::Four],
    ),
    /// 0x60 - 0x6F
    Instruction::new(
        Opcode::LD_H_B,
        "LD H, B",
        None,
        Operation::Load(Register8::H, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_C,
        "LD H, C",
        None,
        Operation::Load(Register8::H, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_D,
        "LD H, D",
        None,
        Operation::Load(Register8::H, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_E,
        "LD H, E",
        None,
        Operation::Load(Register8::H, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_H,
        "LD H, H",
        None,
        Operation::Load(Register8::H, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_L,
        "LD H, L",
        None,
        Operation::Load(Register8::H, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_H_aHL,
        "LD H, (HL)",
        None,
        Operation::Load(Register8::H, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_H_A,
        "LD H, A",
        None,
        Operation::Load(Register8::H, Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_B,
        "LD L, B",
        None,
        Operation::Load(Register8::L, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_C,
        "LD L, C",
        None,
        Operation::Load(Register8::L, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_D,
        "LD L, D",
        None,
        Operation::Load(Register8::L, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_E,
        "LD L, E",
        None,
        Operation::Load(Register8::L, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_H,
        "LD L, H",
        None,
        Operation::Load(Register8::L, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_L,
        "LD L, L",
        None,
        Operation::Load(Register8::L, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_L_aHL,
        "LD L, (HL)",
        None,
        Operation::Load(Register8::L, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_L_A,
        "LD L, A",
        None,
        Operation::Load(Register8::L, Register8::A),
        vec![Clock::Four],
    ),
    /// 0x70 - 0x7F
    Instruction::new(
        Opcode::LD_HL_B,
        "LD (HL), B",
        None,
        Operation::Load(Address::HL, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_HL_C,
        "LD (HL), C",
        None,
        Operation::Load(Address::HL, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_HL_D,
        "LD (HL), D",
        None,
        Operation::Load(Address::HL, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_HL_E,
        "LD (HL), E",
        None,
        Operation::Load(Address::HL, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_HL_H,
        "LD (HL), H",
        None,
        Operation::Load(Address::HL, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_HL_L,
        "LD (HL), L",
        None,
        Operation::Load(Address::HL, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::HALT,
        "HALT",
        None,
        Operation::Halt,
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_HL_A,
        "LD (HL), A",
        None,
        Operation::Load(Address::HL, Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_B,
        "LD A, B",
        None,
        Operation::Load(Register8::A, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_C,
        "LD A, C",
        None,
        Operation::Load(Register8::A, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_D,
        "LD A, D",
        None,
        Operation::Load(Register8::A, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_E,
        "LD A, E",
        None,
        Operation::Load(Register8::A, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_H,
        "LD A, H",
        None,
        Operation::Load(Register8::A, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_L,
        "LD A, L",
        None,
        Operation::Load(Register8::A, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::LD_A_aHL,
        "LD A, (HL)",
        None,
        Operation::Load(Register8::A, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::LD_A_A,
        "LD A, A",
        None,
        Operation::Load(Register8::A, Register8::A),
        vec![Clock::Four],
    ),
    /// 0x80 - 0x8F
    Instruction::new(
        Opcode::ADD_A_B,
        "ADD A, B",
        None,
        Operation::Add(Register8::A, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_C,
        "ADD A, C",
        None,
        Operation::Add(Register8::A, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_D,
        "ADD A, D",
        None,
        Operation::Add(Register8::A, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_E,
        "ADD A, E",
        None,
        Operation::Add(Register8::A, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_H,
        "ADD A, H",
        None,
        Operation::Add(Register8::A, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_L,
        "ADD A, L",
        None,
        Operation::Add(Register8::A, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADD_A_HL,
        "ADD A, (HL)",
        None,
        Operation::Add(Register8::A, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::ADD_A_A,
        "ADD A, A",
        None,
        Operation::Add(Register8::A, Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_B,
        "ADC A, B",
        None,
        Operation::Adc(Register8::A, Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_C,
        "ADC A, C",
        None,
        Operation::Adc(Register8::A, Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_D,
        "ADC A, D",
        None,
        Operation::Adc(Register8::A, Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_E,
        "ADC A, E",
        None,
        Operation::Adc(Register8::A, Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_H,
        "ADC A, H",
        None,
        Operation::Adc(Register8::A, Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_L,
        "ADC A, L",
        None,
        Operation::Adc(Register8::A, Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::ADC_A_HL,
        "ADC A, (HL)",
        None,
        Operation::Adc(Register8::A, Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::ADC_A_A,
        "ADC A, A",
        None,
        Operation::Adc(Register8::A, Register8::A),
        vec![Clock::Four],
    ),
    /// 0x90 - 0x9F
    Instruction::new(
        Opcode::SUB_B,
        "SUB B",
        None,
        Operation::Sub(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_C,
        "SUB C",
        None,
        Operation::Sub(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_D,
        "SUB D",
        None,
        Operation::Sub(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_E,
        "SUB E",
        None,
        Operation::Sub(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_H,
        "SUB H",
        None,
        Operation::Sub(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_L,
        "SUB L",
        None,
        Operation::Sub(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SUB_HL,
        "SUB (HL)",
        None,
        Operation::Sub(Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::SUB_A,
        "SUB A",
        None,
        Operation::Sub(Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_A_B,
        "SBC A, B",
        None,
        Operation::Sbc(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_A_C,
        "SBC A, C",
        None,
        Operation::Sbc(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_A_D,
        "SBC A, D",
        None,
        Operation::Sbc(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_E,
        "SBC A, E",
        None,
        Operation::Sbc(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_H,
        "SBC A, H",
        None,
        Operation::Sbc(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_L,
        "SBC A, L",
        None,
        Operation::Sbc(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::SBC_A_HL,
        "SBC A, (HL)",
        None,
        Operation::Sbc(Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::SBC_A_A,
        "SBC A, A",
        None,
        Operation::Sbc(Register8::A),
        vec![Clock::Four],
    ),
    /// 0xA0 - 0xAF
    Instruction::new(
        Opcode::AND_B,
        "AND B",
        None,
        Operation::And(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_C,
        "AND C",
        None,
        Operation::And(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_D,
        "AND D",
        None,
        Operation::And(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_E,
        "AND E",
        None,
        Operation::And(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_H,
        "AND H",
        None,
        Operation::And(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_L,
        "AND L",
        None,
        Operation::And(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::AND_HL,
        "AND (HL)",
        None,
        Operation::And(Address::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::AND_A,
        "AND A",
        None,
        Operation::And(Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_B,
        "XOR B",
        None,
        Operation::Xor(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_C,
        "XOR C",
        None,
        Operation::Xor(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_D,
        "XOR D",
        None,
        Operation::Xor(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_E,
        "XOR E",
        None,
        Operation::Xor(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_H,
        "XOR H",
        None,
        Operation::Xor(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_L,
        "XOR L",
        None,
        Operation::Xor(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::XOR_HL,
        "XOR (HL)",
        None,
        Operation::Xor(Adress::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::XOR_A,
        "XOR A",
        None,
        Operation::Xor(Register8::A),
        vec![Clock::Four],
    ),
    /// 0xB0 - 0xBF
    Instruction::new(
        Opcode::OR_B,
        "OR B",
        None,
        Operation::Or(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_C,
        "OR C",
        None,
        Operation::Or(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_D,
        "OR D",
        None,
        Operation::Or(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_E,
        "OR E",
        None,
        Operation::Or(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_H,
        "OR H",
        None,
        Operation::Or(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_L,
        "OR L",
        None,
        Operation::Or(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::OR_HL,
        "OR (HL)",
        None,
        Operation::Or(Adress::HL),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::OR_A,
        "OR A",
        None,
        Operation::Or(Register8::A),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_B,
        "CP B",
        None,
        Operation::Cp(Register8::B),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_C,
        "CP C",
        None,
        Operation::Cp(Register8::C),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_D,
        "CP D",
        None,
        Operation::Cp(Register8::D),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_E,
        "CP E",
        None,
        Operation::Cp(Register8::E),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_H,
        "CP H",
        None,
        Operation::Cp(Register8::H),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_L,
        "CP L",
        None,
        Operation::Cp(Register8::L),
        vec![Clock::Four],
    ),
    Instruction::new(
        Opcode::CP_HL,
        "CP (HL)",
        None,
        Operation::Cp(Address::HL),
        vec![Clock::Four],
    ),
    /// 0xC0 - 0xCF
    Instruction::new(
        Opcode::RET_NZ,
        "RET NZ",
        None,
        Operation::Ret(Condition::NZ),
        vec![Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::POP_BC,
        "POP BC",
        None,
        Operation::Pop(Register16::BC),
        vec![Clock::Twelve],
    ),
    Instruction::new(
        Opcode::JP_NZ_a16,
        "JP NZ, {:#06x}",
        Some(Imm::16),
        Operation::Jp(Condition::NZ, Imm::16),
        vec![Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::JP_a16,
        "JP {:#06x}",
        Some(Imm::16),
        Operation::Jp(Imm::16),
        vec![Clock::Sixteen],
    ),
    Instruction::new(
        Opcode::CALL_NZ_a16,
        "CALL NZ, {:#06x}",
        Some(Imm::16),
        Operation::Call(Condition::NZ, Imm::16),
        vec![Clock::TwentyFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::PUSH_BC,
        "PUSH BC",
        None,
        Operation::Push(Register16::BC),
        vec![Clock::Sixteen],
    ),
    Instruction::new(
        Opcode::ADD_A_d8,
        "ADD A, {:#04x}",
        Some(Imm::8),
        Operation::Add(Register8::A, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RST_00H,
        "RST 00H",
        None,
        Operation::Rst(Page0::Byte0),
        vec![Clock::Sixteen],
    ),
    Instruction::new(
        Opcode::RET_Z,
        "RET Z",
        None,
        Operation::Ret(Condition::Z),
        vec![Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::RET,
        "RET",
        None,
        Operation::Ret,
        vec![Clock::Sixteen],
    ),
    Instruction::new(
        Opcode::JP_Z_a16,
        "JP Z, {:#06x}",
        Some(Imm::16),
        Operation::Jp(Condition::Z, Imm::16),
        vec![Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::CALL_Z_a16,
        "CALL Z, {:#06x}",
        Some(Imm::16),
        Operation::Call(Condition::Z, Imm::16),
        vec![Clock::TwelveFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::CALL_a16,
        "CALL {:#06x}",
        Some(Imm::16),
        Operation::Call(Imm::16),
        vec![Clock::TwentyFour],
    ),
    Instruction::new(
        Opcode::ADC_A_d8,
        "ADC A, {:#04x}",
        Some(Imm::8).
        Operation::Adc(Register8::A, Imm::8),
        vec![Clock::Eight],
    ),
    Instruction::new(
        Opcode::RST_08H,
        "RST 08H",
        None,
        Operation::Rst(Page0::Byte1),
        vec![Clock::Sixteen],
    ),
    /// 0xD0 -0xDF
    /// TODO!
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
    Rst(Page0),

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

/// Describes information about the instruction set
#[derive(Copy, Clone)]
pub struct Instruction {
    /// An [Opcode] variant representing the instruction.  
    pub opcode: Opcode,

    /// The instruction's mnemonic (e.g "LD A, B").  
    /// In the case of instructions with immediate operands, the string stores "imm" as a
    /// placeholder.  
    /// See [super::Cpu::format_instruction()]  
    pub mnemonic: &'static str,

    /// An optional variable to inform whether the instruction has an operand stored in memory.  
    /// Set to None if the instruction does not require an immediate operand.
    /// See [Imm]. 
    pub operand: Option<Imm>,

    /// An [Operation] variant to represent the instruction's operation.
    pub operation: Operation,

    /// A list of clock cycles associated with the instruction.  
    /// Unconditional instructions have exactly one clock and set the second [Clock] to `Clock::None`.  
    /// Conditional instructions have exactly two clocks and store the clock for true conditions first, and then the clock for false
    /// conditions.
    pub cycles: [Clock; 2],
}

/// Enumerates the instruction speed in clock cycle
#[derive(Copy, Clone)]
pub enum Clock {
    None,
    Four,
    Eight,
    Twelve,
    Sixteen,
    Twenty,
    TwentyFour,
    ThirtyTwo,
}

#[derive(Copy, Clone)]
pub enum Page0 {
    Byte0 = 0x0000,
    Byte1 = 0x0008,
    Byte2 = 0x0010,
    Byte3 = 0x0018,
    Byte4 = 0x0020,
    Byte5 = 0x0028,
    Byte6 = 0x0030,
    Byte7 = 0x0038,
}

/// Represents the two possible sizes of immediate operands: either 8-bit or 16-bit
#[derive(Copy, Clone, PartialEq)]
/// Enumerates the sizes of immediate operands in bits  
pub enum Imm {

    /// 8-bit immediate value
    Eight,

    /// 16-bit immediate value
    Sixteen,
}

impl Instruction {
    /// Creates an Instruction struct with values given as parameter
    const fn new(
        opcode: Opcode,
        mnemonic: &'static str,
        operand: Option<Imm>,
        operation: Operation,
        cycles: [Clock; 2],
    ) -> Self {
        Self {
            opcode,
            mnemonic,
            operand,
            operation,
            cycles,
        }
    }

    /// Returns the instruction with a matching opcode
    pub fn get_by_opcode(opcode: Opcode) -> Option<Instruction> {
        INSTRUCTIONS
            .into_iter()
            .find(|&instruction| instruction.opcode == opcode)
    }
}
/// A static array of all the instructions in the instructions set.
static INSTRUCTIONS: [Instruction; 324] = [
    Instruction::new(
        Opcode::Nop,
        "NOP",
        None,
        Operation::Nop,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_bc_d16,
        "LD BC, imm",
        Some(Imm::Sixteen),
        Operation::Load16(Operand16::BC, Operand16::Imm16),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_bc_a,
        "LD (BC), A",
        None,
        Operation::Load8(Operand8::Addr(At::BC), Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_bc,
        "INC BC",
        None,
        Operation::Inc16(Operand16::BC),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_b,
        "INC B",
        None,
        Operation::Inc8(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_b,
        "DEC B",
        None,
        Operation::Dec8(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_d8,
        "LD B, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::B, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlca,
        "RLCA",
        None,
        Operation::Rlca,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a16_sp,
        "LD [imm], SP",
        Some(Imm::Sixteen),
        Operation::Load16(Operand16::Addr(At::Imm16), Operand16::SP),
        [Clock::Twenty, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_hl_bc,
        "ADD HL, BC",
        None,
        Operation::AddHL_r16(Operand16::BC),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_bc,
        "LD A, (BC)",
        None,
        Operation::Load8(Operand8::A, Operand8::Addr(At::BC)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_bc,
        "DEC BC",
        None,
        Operation::Dec16(Operand16::BC),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_c,
        "INC C",
        None,
        Operation::Inc8(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_c,
        "DEC C",
        None,
        Operation::Dec8(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_d8,
        "LD C, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::C, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrca,
        "RRCA",
        None,
        Operation::Rrca,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Stop,
        "STOP",
        None,
        Operation::Stop,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_de_d16,
        "LD DE, imm",
        Some(Imm::Sixteen),
        Operation::Load16(Operand16::DE, Operand16::Imm16),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_de_a,
        "LD (DE), A",
        None,
        Operation::Load8(Operand8::Addr(At::DE), Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_de,
        "INC DE",
        None,
        Operation::Inc16(Operand16::DE),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_d,
        "INC D",
        None,
        Operation::Inc8(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_d,
        "DEC D",
        None,
        Operation::Dec8(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_d8,
        "LD D, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::D, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rla,
        "RLA",
        None,
        Operation::Rla,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Jr_r8,
        "JR imm",
        Some(Imm::Eight),
        Operation::Jr(Condition::Always),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_hl_de,
        "ADD HL, DE",
        None,
        Operation::AddHL_r16(Operand16::DE),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_de,
        "LD A, (DE)",
        None,
        Operation::Load8(Operand8::A, Operand8::Addr(At::DE)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_de,
        "DEC DE",
        None,
        Operation::Dec16(Operand16::DE),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_e,
        "INC E",
        None,
        Operation::Inc8(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_e,
        "DEC E",
        None,
        Operation::Dec8(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_d8,
        "LD E, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::E, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rra,
        "RRA",
        None,
        Operation::Rr(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Jr_nz_r8,
        "JR NZ, imm",
        Some(Imm::Eight),
        Operation::Jr(Condition::NZ),
        [Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Ld_hl_d16,
        "LD HL, imm",
        Some(Imm::Sixteen),
        Operation::Load16(Operand16::HL, Operand16::Imm16),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldi_hl_a,
        "LD (HL+), A",
        None,
        Operation::Load8Inc(Operand8::Addr(At::HL), Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_hl,
        "INC HL",
        None,
        Operation::Inc16(Operand16::HL),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_h,
        "INC H",
        None,
        Operation::Inc8(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_h,
        "DEC H",
        None,
        Operation::Dec8(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_d8,
        "LD H, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::H, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Daa,
        "DAA",
        None,
        Operation::Daa,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Jr_z_r8,
        "JR Z, imm",
        Some(Imm::Eight),
        Operation::Jr(Condition::Z),
        [Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Add_hl_hl,
        "ADD HL, HL",
        None,
        Operation::AddHL_r16(Operand16::HL),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldi_a_hl,
        "LD A, (HL+)",
        None,
        Operation::Load8Inc(Operand8::A, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_hl,
        "DEC HL",
        None,
        Operation::Dec16(Operand16::HL),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_l,
        "INC L",
        None,
        Operation::Inc8(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_l,
        "DEC L",
        None,
        Operation::Dec8(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_d8,
        "LD L, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::L, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Cpl,
        "CPL",
        None,
        Operation::Cpl,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Jr_nc_r8,
        "JR NC, imm",
        Some(Imm::Eight),
        Operation::Jr(Condition::NC),
        [Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Ld_sp_d16,
        "LD SP, imm",
        Some(Imm::Sixteen),
        Operation::Load16(Operand16::SP, Operand16::Imm16),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldd_hl_a,
        "LD (HL-), A",
        None,
        Operation::Load8Dec(Operand8::Addr(At::HL), Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_sp,
        "INC SP",
        None,
        Operation::Inc16(Operand16::SP),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_ahl,
        "INC (HL)",
        None,
        Operation::Inc8(Operand8::Addr(At::HL)),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_ahl,
        "DEC (HL)",
        None,
        Operation::Dec8(Operand8::Addr(At::HL)),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_ahl_d8,
        "LD (HL), imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::Addr(At::HL), Operand8::Imm8),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Scf,
        "SCF",
        None,
        Operation::Scf,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Jr_c_r8,
        "JR C, imm",
        Some(Imm::Eight),
        Operation::Jr(Condition::C),
        [Clock::Twelve, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Add_hl_sp,
        "ADD HL, SP",
        None,
        Operation::AddHL_r16(Operand16::SP),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldd_a_ahl,
        "LD A, (HL-)",
        None,
        Operation::Load8(Operand8::A, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_sp,
        "DEC SP",
        None,
        Operation::Dec16(Operand16::SP),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Inc_a,
        "INC A",
        None,
        Operation::Inc8(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Dec_a,
        "DEC A",
        None,
        Operation::Dec8(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_d8,
        "LD A, imm",
        Some(Imm::Eight),
        Operation::Load8(Operand8::A, Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ccf,
        "CCF",
        None,
        Operation::Ccf,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_b,
        "LD B, B",
        None,
        Operation::Load8(Operand8::B, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_c,
        "LD B, C",
        None,
        Operation::Load8(Operand8::B, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_d,
        "LD B, D",
        None,
        Operation::Load8(Operand8::B, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_e,
        "LD B, E",
        None,
        Operation::Load8(Operand8::B, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_h,
        "LD B, H",
        None,
        Operation::Load8(Operand8::B, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_l,
        "LD B, L",
        None,
        Operation::Load8(Operand8::B, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_ahl,
        "LD B, (HL)",
        None,
        Operation::Load8(Operand8::B, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_b_a,
        "LD B, A",
        None,
        Operation::Load8(Operand8::B, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_b,
        "LD C, B",
        None,
        Operation::Load8(Operand8::C, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_c,
        "LD C, C",
        None,
        Operation::Load8(Operand8::C, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_d,
        "LD C, D",
        None,
        Operation::Load8(Operand8::C, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_e,
        "LD C, E",
        None,
        Operation::Load8(Operand8::C, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_h,
        "LD C, H",
        None,
        Operation::Load8(Operand8::C, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_l,
        "LD C, L",
        None,
        Operation::Load8(Operand8::C, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_ahl,
        "LD C, (HL)",
        None,
        Operation::Load8(Operand8::C, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_c_a,
        "LD C, A",
        None,
        Operation::Load8(Operand8::C, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_b,
        "LD D, B",
        None,
        Operation::Load8(Operand8::D, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_c,
        "LD D, C",
        None,
        Operation::Load8(Operand8::D, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_d,
        "LD D, D",
        None,
        Operation::Load8(Operand8::D, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_e,
        "LD D, E",
        None,
        Operation::Load8(Operand8::D, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_h,
        "LD D, H",
        None,
        Operation::Load8(Operand8::D, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_l,
        "LD D, L",
        None,
        Operation::Load8(Operand8::D, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_ahl,
        "LD D, (HL)",
        None,
        Operation::Load8(Operand8::D, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_d_a,
        "LD D, A",
        None,
        Operation::Load8(Operand8::D, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_b,
        "LD E, B",
        None,
        Operation::Load8(Operand8::E, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_c,
        "LD E, C",
        None,
        Operation::Load8(Operand8::E, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_d,
        "LD E, D",
        None,
        Operation::Load8(Operand8::E, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_e,
        "LD E, E",
        None,
        Operation::Load8(Operand8::E, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_h,
        "LD E, H",
        None,
        Operation::Load8(Operand8::E, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_l,
        "LD E, L",
        None,
        Operation::Load8(Operand8::E, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_ahl,
        "LD E, (HL)",
        None,
        Operation::Load8(Operand8::E, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_e_a,
        "LD E, A",
        None,
        Operation::Load8(Operand8::E, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_b,
        "LD H, B",
        None,
        Operation::Load8(Operand8::H, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_c,
        "LD H, C",
        None,
        Operation::Load8(Operand8::H, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_d,
        "LD H, D",
        None,
        Operation::Load8(Operand8::H, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_e,
        "LD H, E",
        None,
        Operation::Load8(Operand8::H, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_h,
        "LD H, H",
        None,
        Operation::Load8(Operand8::H, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_l,
        "LD H, L",
        None,
        Operation::Load8(Operand8::H, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_ahl,
        "LD H, (HL)",
        None,
        Operation::Load8(Operand8::H, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_h_a,
        "LD H, A",
        None,
        Operation::Load8(Operand8::H, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_b,
        "LD L, B",
        None,
        Operation::Load8(Operand8::L, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_c,
        "LD L, C",
        None,
        Operation::Load8(Operand8::L, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_d,
        "LD L, D",
        None,
        Operation::Load8(Operand8::L, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_e,
        "LD L, E",
        None,
        Operation::Load8(Operand8::L, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_h,
        "LD L, H",
        None,
        Operation::Load8(Operand8::L, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_l,
        "LD L, L",
        None,
        Operation::Load8(Operand8::L, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_ahl,
        "LD L, (HL)",
        None,
        Operation::Load8(Operand8::L, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_l_a,
        "LD L, A",
        None,
        Operation::Load8(Operand8::L, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_b,
        "LD (HL), B",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_c,
        "LD (HL), C",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_d,
        "LD (HL), D",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_e,
        "LD (HL), E",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_h,
        "LD (HL), H",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_l,
        "LD (HL), L",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Halt,
        "HALT",
        None,
        Operation::Halt,
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_a,
        "LD (HL), A",
        None,
        Operation::Load8(Operand8::Addr(At::HL), Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_b,
        "LD A, B",
        None,
        Operation::Load8(Operand8::A, Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_c,
        "LD A, C",
        None,
        Operation::Load8(Operand8::A, Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_d,
        "LD A, D",
        None,
        Operation::Load8(Operand8::A, Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_e,
        "LD A, E",
        None,
        Operation::Load8(Operand8::A, Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_h,
        "LD A, H",
        None,
        Operation::Load8(Operand8::A, Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_l,
        "LD A, L",
        None,
        Operation::Load8(Operand8::A, Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_ahl,
        "LD A, (HL)",
        None,
        Operation::Load8(Operand8::A, Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_a,
        "LD A, A",
        None,
        Operation::Load8(Operand8::A, Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_b,
        "ADD A, B",
        None,
        Operation::Add8(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_c,
        "ADD A, C",
        None,
        Operation::Add8(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_d,
        "ADD A, D",
        None,
        Operation::Add8(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_e,
        "ADD A, E",
        None,
        Operation::Add8(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_h,
        "ADD A, H",
        None,
        Operation::Add8(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_l,
        "ADD A, L",
        None,
        Operation::Add8(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_hl,
        "ADD A, (HL)",
        None,
        Operation::Add8(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_a,
        "ADD A, A",
        None,
        Operation::Add8(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_b,
        "ADC A, B",
        None,
        Operation::Adc(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_c,
        "ADC A, C",
        None,
        Operation::Adc(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_d,
        "ADC A, D",
        None,
        Operation::Adc(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_e,
        "ADC A, E",
        None,
        Operation::Adc(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_h,
        "ADC A, H",
        None,
        Operation::Adc(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_l,
        "ADC A, L",
        None,
        Operation::Adc(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_hl,
        "ADC A, (HL)",
        None,
        Operation::Adc(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_a,
        "ADC A, A",
        None,
        Operation::Adc(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_b,
        "SUB B",
        None,
        Operation::Sub(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_c,
        "SUB C",
        None,
        Operation::Sub(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_d,
        "SUB D",
        None,
        Operation::Sub(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_e,
        "SUB E",
        None,
        Operation::Sub(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_h,
        "SUB H",
        None,
        Operation::Sub(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_l,
        "SUB L",
        None,
        Operation::Sub(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_hl,
        "SUB (HL)",
        None,
        Operation::Sub(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_a,
        "SUB A",
        None,
        Operation::Sub(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_b,
        "SBC A, B",
        None,
        Operation::Sbc(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_c,
        "SBC A, C",
        None,
        Operation::Sbc(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_d,
        "SBC A, D",
        None,
        Operation::Sbc(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_e,
        "SBC A, E",
        None,
        Operation::Sbc(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_h,
        "SBC A, H",
        None,
        Operation::Sbc(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_l,
        "SBC A, L",
        None,
        Operation::Sbc(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_hl,
        "SBC A, (HL)",
        None,
        Operation::Sbc(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sbc_a_a,
        "SBC A, A",
        None,
        Operation::Sbc(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_b,
        "AND B",
        None,
        Operation::And(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_c,
        "AND C",
        None,
        Operation::And(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_d,
        "AND D",
        None,
        Operation::And(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_e,
        "AND E",
        None,
        Operation::And(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_h,
        "AND H",
        None,
        Operation::And(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_l,
        "AND L",
        None,
        Operation::And(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::And_hl,
        "AND (HL)",
        None,
        Operation::And(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::And_a,
        "AND A",
        None,
        Operation::And(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_b,
        "XOR B",
        None,
        Operation::Xor(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_c,
        "XOR C",
        None,
        Operation::Xor(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_d,
        "XOR D",
        None,
        Operation::Xor(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_e,
        "XOR E",
        None,
        Operation::Xor(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_h,
        "XOR H",
        None,
        Operation::Xor(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_l,
        "XOR L",
        None,
        Operation::Xor(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_hl,
        "XOR (HL)",
        None,
        Operation::Xor(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_a,
        "XOR A",
        None,
        Operation::Xor(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_b,
        "OR B",
        None,
        Operation::Or(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_c,
        "OR C",
        None,
        Operation::Or(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_d,
        "OR D",
        None,
        Operation::Or(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_e,
        "OR E",
        None,
        Operation::Or(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_h,
        "OR H",
        None,
        Operation::Or(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_l,
        "OR L",
        None,
        Operation::Or(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_hl,
        "OR (HL)",
        None,
        Operation::Or(Operand8::Addr(At::HL)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_a,
        "OR A",
        None,
        Operation::Or(Operand8::A),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_b,
        "CP B",
        None,
        Operation::Cp(Operand8::B),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_c,
        "CP C",
        None,
        Operation::Cp(Operand8::C),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_d,
        "CP D",
        None,
        Operation::Cp(Operand8::D),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_e,
        "CP E",
        None,
        Operation::Cp(Operand8::E),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_h,
        "CP H",
        None,
        Operation::Cp(Operand8::H),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_l,
        "CP L",
        None,
        Operation::Cp(Operand8::L),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_hl,
        "CP (HL)",
        None,
        Operation::Cp(Operand8::Addr(At::HL)),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ret_nz,
        "RET NZ",
        None,
        Operation::Ret(Condition::NZ),
        [Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Pop_bc,
        "POP BC",
        None,
        Operation::Pop(Operand16::BC),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Jp_nz_a16,
        "JP NZ, (imm)",
        Some(Imm::Sixteen),
        Operation::Jp(Condition::NZ, Operand16::Imm16),
        [Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Jp_a16,
        "JP (imm)",
        Some(Imm::Sixteen),
        Operation::Jp(Condition::Always, Operand16::Imm16),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Call_nz_a16,
        "CALL NZ, (imm)",
        Some(Imm::Sixteen),
        Operation::Call(Condition::NZ, Operand16::Imm16),
        [Clock::TwentyFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Push_bc,
        "PUSH BC",
        None,
        Operation::Push(Operand16::BC),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_a_d8,
        "ADD A, imm",
        Some(Imm::Eight),
        Operation::Add8(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_00h,
        "RST 00H",
        None,
        Operation::Rst(Page0::Byte0),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ret_z,
        "RET Z",
        None,
        Operation::Ret(Condition::Z),
        [Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Ret,
        "RET",
        None,
        Operation::Ret(Condition::Always),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Jp_z_a16,
        "JP Z, (imm)",
        Some(Imm::Sixteen),
        Operation::Jp(Condition::Z, Operand16::Imm16),
        [Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Call_z_a16,
        "CALL Z, (imm)",
        Some(Imm::Sixteen),
        Operation::Call(Condition::Z, Operand16::Imm16),
        [Clock::TwentyFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Call_a16,
        "CALL (imm)",
        Some(Imm::Sixteen),
        Operation::Call(Condition::Always, Operand16::Imm16),
        [Clock::TwentyFour, Clock::None],
    ),
    Instruction::new(
        Opcode::Adc_a_d8,
        "ADC A, imm",
        Some(Imm::Eight),
        Operation::Adc(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_08h,
        "RST 08H",
        None,
        Operation::Rst(Page0::Byte1),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ret_nc,
        "RET NC",
        None,
        Operation::Ret(Condition::NC),
        [Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Pop_de,
        "POP DE",
        None,
        Operation::Pop(Operand16::DE),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Jp_nc_a16,
        "JP NC, (imm)",
        Some(Imm::Sixteen),
        Operation::Jp(Condition::NC, Operand16::Imm16),
        [Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Call_nc_a16,
        "CALL NC, (imm)",
        Some(Imm::Sixteen),
        Operation::Call(Condition::NC, Operand16::Imm16),
        [Clock::TwentyFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Push_de,
        "PUSH DE",
        None,
        Operation::Push(Operand16::DE),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Sub_d8,
        "SUB imm",
        Some(Imm::Eight),
        Operation::Sub(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_10h,
        "RST 10H",
        None,
        Operation::Rst(Page0::Byte2),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ret_c,
        "RET C",
        None,
        Operation::Ret(Condition::C),
        [Clock::Twenty, Clock::Eight],
    ),
    Instruction::new(
        Opcode::Reti,
        "RETI",
        None,
        Operation::Reti,
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Jp_c_a16,
        "JP C, (imm)",
        Some(Imm::Sixteen),
        Operation::Jp(Condition::C, Operand16::Imm16),
        [Clock::Sixteen, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Call_c_a16,
        "CALL C, (imm)",
        Some(Imm::Sixteen),
        Operation::Call(Condition::C, Operand16::Imm16),
        [Clock::TwentyFour, Clock::Twelve],
    ),
    Instruction::new(
        Opcode::Sbc_a_d8,
        "SBC A, imm",
        Some(Imm::Eight),
        Operation::Sbc(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_18h,
        "RST 18H",
        None,
        Operation::Rst(Page0::Byte3),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldh_a8_a,
        "LDH (imm), A",
        Some(Imm::Eight),
        Operation::Load8(Operand8::Addr(At::Imm8), Operand8::A),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Pop_hl,
        "POP HL",
        None,
        Operation::Pop(Operand16::HL),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_ac_a,
        "LD (C), A",
        None,
        Operation::Load8(Operand8::Addr(At::C), Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Push_hl,
        "PUSH HL",
        None,
        Operation::Push(Operand16::HL),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::And_d8,
        "AND imm",
        Some(Imm::Eight),
        Operation::And(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_20h,
        "RST 20H",
        None,
        Operation::Rst(Page0::Byte4),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Add_sp_r8,
        "ADD SP, imm",
        Some(Imm::Eight),
        Operation::AddSP_dd,
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Jp_ahl,
        "JP (HL)",
        None,
        Operation::Jp(Condition::Always, Operand16::HL),
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a16_a,
        "LD (imm), A",
        Some(Imm::Sixteen),
        Operation::Load8(Operand8::Addr(At::Imm16), Operand8::A),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_d8,
        "XOR imm",
        Some(Imm::Eight),
        Operation::Xor(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_28h,
        "RST 20H",
        None,
        Operation::Rst(Page0::Byte5),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ldh_a_a8,
        "LDH A, (imm)",
        Some(Imm::Eight),
        Operation::Load8(Operand8::A, Operand8::Addr(At::Imm8)),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Pop_af,
        "POP AF",
        None,
        Operation::Pop(Operand16::AF),
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_ac,
        "LD A, (C)",
        None,
        Operation::Load8(Operand8::A, Operand8::Addr(At::C)),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Di,
        "DI",
        None,
        Operation::Di,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Push_af,
        "PUSH AF",
        None,
        Operation::Push(Operand16::AF),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Or_d8,
        "OR imm",
        Some(Imm::Eight),
        Operation::Or(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_30h,
        "RST 30H",
        None,
        Operation::Rst(Page0::Byte6),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_hl_sp_r8,
        "LD HL, SP+imm",
        Some(Imm::Eight),
        Operation::LoadHL,
        [Clock::Twelve, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_sp_hl,
        "LD SP, HL",
        None,
        Operation::Load16(Operand16::SP, Operand16::HL),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Ld_a_a16,
        "LD A, (imm)",
        Some(Imm::Sixteen),
        Operation::Load8(Operand8::A, Operand8::Addr(At::Imm16)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Xor_d8,
        "XOR imm",
        Some(Imm::Eight),
        Operation::Xor(Operand8::Imm8),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Ei,
        "EI",
        None,
        Operation::Ei,
        [Clock::Four, Clock::None],
    ),
    Instruction::new(
        Opcode::Cp_d8,
        "CP imm",
        Some(Imm::Eight),
        Operation::Cp(Operand8::Imm8),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rst_38h,
        "RST 38H",
        None,
        Operation::Rst(Page0::Byte7),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_b,
        "RLC B",
        None,
        Operation::Rlc(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_c,
        "RLC C",
        None,
        Operation::Rlc(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_d,
        "RLC D",
        None,
        Operation::Rlc(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_e,
        "RLC E",
        None,
        Operation::Rlc(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_h,
        "RLC H",
        None,
        Operation::Rlc(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_l,
        "RLC L",
        None,
        Operation::Rlc(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_hl,
        "RLC HL",
        None,
        Operation::Rlc(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Rlc_a,
        "RLC A",
        None,
        Operation::Rlc(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_b,
        "RRC B",
        None,
        Operation::Rrc(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_c,
        "RRC C",
        None,
        Operation::Rrc(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_d,
        "RRC D",
        None,
        Operation::Rrc(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_e,
        "RRC E",
        None,
        Operation::Rrc(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_h,
        "RRC H",
        None,
        Operation::Rrc(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_l,
        "RRC L",
        None,
        Operation::Rrc(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_hl,
        "RRC HL",
        None,
        Operation::Rrc(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Rrc_a,
        "RRC A",
        None,
        Operation::Rrc(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_b,
        "RL B",
        None,
        Operation::Rl(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_c,
        "RL C",
        None,
        Operation::Rl(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_d,
        "RL D",
        None,
        Operation::Rl(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_e,
        "RL E",
        None,
        Operation::Rl(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_h,
        "RL H",
        None,
        Operation::Rl(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_l,
        "RL L",
        None,
        Operation::Rl(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_hl,
        "RL HL",
        None,
        Operation::Rl(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Rl_a,
        "RL A",
        None,
        Operation::Rl(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_b,
        "RR B",
        None,
        Operation::Rr(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_c,
        "RR C",
        None,
        Operation::Rr(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_d,
        "RR D",
        None,
        Operation::Rr(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_e,
        "RR E",
        None,
        Operation::Rr(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_h,
        "RR H",
        None,
        Operation::Rr(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_l,
        "RR L",
        None,
        Operation::Rr(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_hl,
        "RR HL",
        None,
        Operation::Rr(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Rr_a,
        "RR A",
        None,
        Operation::Rr(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_b,
        "SLA B",
        None,
        Operation::Sla(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_c,
        "SLA C",
        None,
        Operation::Sla(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_d,
        "SLA D",
        None,
        Operation::Sla(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_e,
        "SLA E",
        None,
        Operation::Sla(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_h,
        "SLA H",
        None,
        Operation::Sla(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_l,
        "SLA L",
        None,
        Operation::Sla(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_hl,
        "SLA HL",
        None,
        Operation::Sla(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Sla_a,
        "SLA A",
        None,
        Operation::Sla(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_b,
        "SRA B",
        None,
        Operation::Sra(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_c,
        "SRA C",
        None,
        Operation::Sra(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_d,
        "SRA D",
        None,
        Operation::Sra(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_e,
        "SRA E",
        None,
        Operation::Sra(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_h,
        "SRA H",
        None,
        Operation::Sra(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_l,
        "SRA L",
        None,
        Operation::Sra(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_hl,
        "SRA HL",
        None,
        Operation::Sra(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Sra_a,
        "SRA A",
        None,
        Operation::Sra(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_b,
        "SWAP B",
        None,
        Operation::Swap(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_c,
        "SWAP C",
        None,
        Operation::Swap(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_d,
        "SWAP D",
        None,
        Operation::Swap(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_e,
        "SWAP E",
        None,
        Operation::Swap(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_h,
        "SWAP H",
        None,
        Operation::Swap(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_l,
        "SWAP L",
        None,
        Operation::Swap(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_hl,
        "SWAP HL",
        None,
        Operation::Swap(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Swap_a,
        "SWAP A",
        None,
        Operation::Swap(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_b,
        "SRL B",
        None,
        Operation::Srl(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_c,
        "SRL C",
        None,
        Operation::Srl(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_d,
        "SRL D",
        None,
        Operation::Srl(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_e,
        "SRL E",
        None,
        Operation::Srl(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_h,
        "SRL H",
        None,
        Operation::Srl(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_l,
        "SRL L",
        None,
        Operation::Srl(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_hl,
        "SRL HL",
        None,
        Operation::Srl(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Srl_a,
        "SRL A",
        None,
        Operation::Srl(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_b,
        "BIT 0, B",
        None,
        Operation::Sla(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_c,
        "BIT 0, C",
        None,
        Operation::Sla(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_d,
        "BIT 0, D",
        None,
        Operation::Sla(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_e,
        "BIT 0, E",
        None,
        Operation::Sla(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_h,
        "BIT 0, H",
        None,
        Operation::Sla(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_l,
        "BIT 0, L",
        None,
        Operation::Sla(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_hl,
        "BIT 0, HL",
        None,
        Operation::Sla(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_0_a,
        "BIT 0, A",
        None,
        Operation::Sla(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_b,
        "BIT 1,B",
        None,
        Operation::Sra(Operand8::B),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_c,
        "BIT 1,C",
        None,
        Operation::Sra(Operand8::C),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_d,
        "BIT 1,D",
        None,
        Operation::Sra(Operand8::D),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_e,
        "BIT 1,E",
        None,
        Operation::Sra(Operand8::E),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_h,
        "BIT 1,H",
        None,
        Operation::Sra(Operand8::H),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_l,
        "BIT 1,L",
        None,
        Operation::Sra(Operand8::L),
        [Clock::Eight, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_hl,
        "BIT 1,HL",
        None,
        Operation::Sra(Operand8::Addr(At::HL)),
        [Clock::Sixteen, Clock::None],
    ),
    Instruction::new(
        Opcode::Bit_1_a,
        "BIT 1,A",
        None,
        Operation::Sra(Operand8::A),
        [Clock::Eight, Clock::None],
    ),
];

#[derive(PartialEq, Clone, Copy)]
/// Enumerates the possible conditions for conditional instructions. 
/// Conditional instructions are executed if their condition matches the flag status. 
///
/// | Condition | Flag  |
/// |-----------|-------|
/// |    NZ     | Z = 0 |
/// |    NC     | C = 0 |
/// |    Z      | Z = 1 |
/// |    C      | C = 1 |
///
/// # Example
///
/// ```
/// todo!();
/// 
/// ```
/// 
/// # Special case 
///
/// The variant *Always* is not part of the conditions described in the GameBoy documentation:
/// it is always true and allows to re-use the code of conditional instructions for unconditional
/// instructions.
///
/// ```
/// todo!();
///  
/// ```
pub enum Condition {
    
    /// NZ is true when the *Zero* flag is not set  
    NZ,

    /// NC is true when the *Carry* flag is not set  
    NC,

    /// C is true when the *Carry* flag is set  
    C,

    /// Z is true when the *Zero* flag is set  
    Z,

    /// Always is always true  
    Always,
}

#[derive(PartialEq, Clone, Copy)]
/// Enumerates the location where addresses can be stored
///
/// # Examples
///
/// ```
/// let memory = vec![10, 24, 53, 34, 140, 39, 03, 93];
///
/// let mut cpu = Cpu::new(memory);
///
/// cpu.registers.write16(Register16::HL, 0x0003); //we write 0x0003 into HL
///
/// let value = cpu.get_operand8(Operand8::Addr(At::HL)); //gets the 8-bit value at address 0x0003
/// in memory
///
/// assert_eq!(value, 53);
/// ```
///
pub enum At {

    /// The 16-bit register `HL`
    HL,

    /// The 16-bit register `BC`
    BC,

    /// The 16-bit register `DE`
    DE,

    /// The 16-bit immediate value
    Imm16,

    /// The 8-bit register `C`  
    C,

    /// The 8-bit immediate value
    Imm8,
}

#[derive(PartialEq, Clone, Copy)]
/// Enumerates the operands for 8-bit instructions.  
pub enum Operand8 {

    /// The 8-bit register `A`.  
    A,

    /// The 8-bit register `B`. 
    B,

    /// The 8-bit register `C`.  
    C,

    /// The 8-bit register `D`.  
    D,

    /// The 8-bit register `E`.  
    E,

    /// The 8-bit register `H`.  
    H,

    /// The 8-bit register `L`.  
    L,

    /// The 8-bit immediate value.  
    Imm8,

    /// The 8-bit value at the address stored in `At`.
    /// See [At]. 
    Addr(At),
}

#[derive(PartialEq, Clone, Copy)]
/// Enumerates the operands for 16-bit instructions.  
pub enum Operand16 {

    /// The 16-bit register `AF`.  
    AF,
    
    /// The 16-bit register `BC`.  
    BC,

    /// The 16-bit register `DE`.  
    DE,

    /// The 16-bit register `HL`.  
    HL,

    /// The 16-bit `Stack Pointer`.  
    SP,
    
    /// The 16-bit value at the address stored in `At`.  
    /// See [At].  
    Addr(At),

    /// The 16-bit immediate value.  
    Imm16,

    /// The 8-bit immediate value.  
    Imm8,
}

#[derive(Copy, Clone)]
/// Enumerates the bit position in a byte.  
pub enum Bit {

    /// The first bit
    Zero = 0b0000_0001,

    /// The second bit
    One = 0b0000_0010,

    /// The third bit
    Two = 0b0000_0100,

    /// The fourth bit  
    Three = 0b0000_1000,

    /// The fifth bit
    Four = 0b0001_0000,

    /// The sixth bit 
    Five = 0b0010_0000,

    /// The seventh bit
    Six = 0b0100_0000,

    /// The eighth bit
    Seven = 0b1000_0000,
}

#[derive(Copy, Clone)]
/// Enumerates all the operations representend by instructions.  
pub enum Operation {

    /// see [super::Cpu::load8()]
    Load8(Operand8, Operand8),

    /// see [super::Cpu::load8dec()]
    Load8Dec(Operand8, Operand8),

    /// see [super::Cpu::load8inc()]
    Load8Inc(Operand8, Operand8),

    /// see [super::Cpu::load16()]
    Load16(Operand16, Operand16),

    /// see [super::Cpu::push()]
    Push(Operand16),

    /// see [super::Cpu::pop()]
    Pop(Operand16),

    /// see [super::Cpu::add8()]
    Add8(Operand8),

    /// see [super::Cpu::adc()]
    Adc(Operand8),

    /// see [super::Cpu::sub()]
    Sub(Operand8),

    /// see [super::Cpu::sbc()]
    Sbc(Operand8),

    /// see [super::Cpu::and()]
    And(Operand8),
 
    /// see [super::Cpu::xor()]
    Xor(Operand8),

    /// see [super::Cpu::or()]
    Or(Operand8),

    /// see [super::Cpu::cp()]
    Cp(Operand8),

    /// see [super::Cpu::inc8()]
    Inc8(Operand8),

    /// see [super::Cpu::dec8()]
    Dec8(Operand8),

    /// see [super::Cpu::daa()]
    Daa,

    /// see [super::Cpu::cpl()]
    Cpl,

    /// see [super::Cpu::addHL_r16()]
    AddHL_r16(Operand16),

    /// see [super::Cpu::inc16()]
    Inc16(Operand16),

    /// see [super::Cpu::dec16()]
    Dec16(Operand16),

    /// see [super::Cpu::loadHL()]
    LoadHL,

    /// see [super::Cpu::addSP_dd()]
    AddSP_dd,

    /// see [super::Cpu::rlca()]
    Rlca,

    /// see [super::Cpu::rla()]
    Rla,

    /// see [super::Cpu::rrca()]
    Rrca,

    /// see [super::Cpu::rlc()]
    Rlc(Operand8),

    /// see [super::Cpu::rl()]
    Rl(Operand8),

    /// see [super::Cpu::rrc()]
    Rrc(Operand8),

    /// see [super::Cpu::rr()]
    Rr(Operand8),

    /// see [super::Cpu::sla()]
    Sla(Operand8),

    /// see [super::Cpu::swap()]
    Swap(Operand8),

    /// see [super::Cpu::sra()]
    Sra(Operand8),

    /// see [super::Cpu::srl()]
    Srl(Operand8),

    /// see [super::Cpu::bit()]
    Bit(Bit, Operand8),

    /// see [super::Cpu::set()]
    Set(Bit, Operand8),

    /// see [super::Cpu::res()]
    Res(Bit, Operand8),

    /// see [super::Cpu::ccf()]
    Ccf,

    /// see [super::Cpu::scf()]
    Scf,

    /// see [super::Cpu::nop()]
    Nop,

    /// see [super::Cpu::halt()]
    Halt,

    /// see [super::Cpu::stop()]
    Stop,

    /// see [super::Cpu::di()]
    Di,

    /// see [super::Cpu::ei()]
    Ei,

    /// see [super::Cpu::jp()]
    Jp(Condition, Operand16),

    /// see [super::Cpu::jr()]
    Jr(Condition),

    /// see [super::Cpu::call()]
    Call(Condition, Operand16),

    /// see [super::Cpu::ret()]
    Ret(Condition),

    /// see [super::Cpu::reti()]
    Reti,

    /// see [super::Cpu::rst()]
    Rst(Page0),
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
/// Enumerates all the GameBoy opcodes both regular and CB-prefixed
pub enum Opcode {
    Nop,
    Ld_bc_d16,
    Ld_bc_a,
    Inc_bc,
    Inc_b,
    Dec_b,
    Ld_b_d8,
    Rlca,
    Ld_a16_sp,
    Add_hl_bc,
    Ld_a_bc,
    Dec_bc,
    Inc_c,
    Dec_c,
    Ld_c_d8,
    Rrca,
    Stop,
    Ld_de_d16,
    Ld_de_a,
    Inc_de,
    Inc_d,
    Dec_d,
    Ld_d_d8,
    Rla,
    Jr_r8,
    Add_hl_de,
    Ld_a_de,
    Dec_de,
    Inc_e,
    Dec_e,
    Ld_e_d8,
    Rra,
    Jr_nz_r8,
    Ld_hl_d16,
    Ldi_hl_a,
    Inc_hl,
    Inc_h,
    Dec_h,
    Ld_h_d8,
    Daa,
    Jr_z_r8,
    Add_hl_hl,
    Ldi_a_hl,
    Dec_hl,
    Inc_l,
    Dec_l,
    Ld_l_d8,
    Cpl,
    Jr_nc_r8,
    Ld_sp_d16,
    Ldd_hl_a,
    Inc_sp,
    Inc_ahl,
    Dec_ahl,
    Ld_ahl_d8,
    Scf,
    Jr_c_r8,
    Add_hl_sp,
    Ldd_a_ahl,
    Dec_sp,
    Inc_a,
    Dec_a,
    Ld_a_d8,
    Ccf,
    Ld_b_b,
    Ld_b_c,
    Ld_b_d,
    Ld_b_e,
    Ld_b_h,
    Ld_b_l,
    Ld_b_ahl,
    Ld_b_a,
    Ld_c_b,
    Ld_c_c,
    Ld_c_d,
    Ld_c_e,
    Ld_c_h,
    Ld_c_l,
    Ld_c_ahl,
    Ld_c_a,
    Ld_d_b,
    Ld_d_c,
    Ld_d_d,
    Ld_d_e,
    Ld_d_h,
    Ld_d_l,
    Ld_d_ahl,
    Ld_d_a,
    Ld_e_b,
    Ld_e_c,
    Ld_e_d,
    Ld_e_e,
    Ld_e_h,
    Ld_e_l,
    Ld_e_ahl,
    Ld_e_a,
    Ld_h_b,
    Ld_h_c,
    Ld_h_d,
    Ld_h_e,
    Ld_h_h,
    Ld_h_l,
    Ld_h_ahl,
    Ld_h_a,
    Ld_l_b,
    Ld_l_c,
    Ld_l_d,
    Ld_l_e,
    Ld_l_h,
    Ld_l_l,
    Ld_l_ahl,
    Ld_l_a,
    Ld_hl_b,
    Ld_hl_c,
    Ld_hl_d,
    Ld_hl_e,
    Ld_hl_h,
    Ld_hl_l,
    Halt,
    Ld_hl_a,
    Ld_a_b,
    Ld_a_c,
    Ld_a_d,
    Ld_a_e,
    Ld_a_h,
    Ld_a_l,
    Ld_a_ahl,
    Ld_a_a,
    Add_a_b,
    Add_a_c,
    Add_a_d,
    Add_a_e,
    Add_a_h,
    Add_a_l,
    Add_a_hl,
    Add_a_a,
    Adc_a_b,
    Adc_a_c,
    Adc_a_d,
    Adc_a_e,
    Adc_a_h,
    Adc_a_l,
    Adc_a_hl,
    Adc_a_a,
    Sub_b,
    Sub_c,
    Sub_d,
    Sub_e,
    Sub_h,
    Sub_l,
    Sub_hl,
    Sub_a,
    Sbc_a_b,
    Sbc_a_c,
    Sbc_a_d,
    Sbc_a_e,
    Sbc_a_h,
    Sbc_a_l,
    Sbc_a_hl,
    Sbc_a_a,
    And_b,
    And_c,
    And_d,
    And_e,
    And_h,
    And_l,
    And_hl,
    And_a,
    Xor_b,
    Xor_c,
    Xor_d,
    Xor_e,
    Xor_h,
    Xor_l,
    Xor_hl,
    Xor_a,
    Or_b,
    Or_c,
    Or_d,
    Or_e,
    Or_h,
    Or_l,
    Or_hl,
    Or_a,
    Cp_b,
    Cp_c,
    Cp_d,
    Cp_e,
    Cp_h,
    Cp_l,
    Cp_hl,
    Cp_a,
    Ret_nz,
    Pop_bc,
    Jp_nz_a16,
    Jp_a16,
    Call_nz_a16,
    Push_bc,
    Add_a_d8,
    Rst_00h,
    Ret_z,
    Ret,
    Jp_z_a16,
    Call_z_a16,
    Call_a16,
    Adc_a_d8,
    Rst_08h,
    Ret_nc,
    Pop_de,
    Jp_nc_a16,
    Call_nc_a16,
    Push_de,
    Sub_d8,
    Rst_10h,
    Ret_c,
    Reti,
    Jp_c_a16,
    Call_c_a16,
    Sbc_a_d8,
    Rst_18h,
    Ldh_a8_a,
    Pop_hl,
    Ld_ac_a,
    Push_hl,
    And_d8,
    Rst_20h,
    Add_sp_r8,
    Jp_ahl,
    Ld_a16_a,
    Xor_d8,
    Rst_28h,
    Ldh_a_a8,
    Pop_af,
    Ld_a_ac,
    Di,
    Push_af,
    Or_d8,
    Rst_30h,
    Ld_hl_sp_r8,
    Ld_sp_hl,
    Ld_a_a16,
    Ei,
    Cp_d8,
    Rst_38h,
    Rlc_b,
    Rlc_c,
    Rlc_d,
    Rlc_e,
    Rlc_h,
    Rlc_l,
    Rlc_hl,
    Rlc_a,
    Rrc_b,
    Rrc_c,
    Rrc_d,
    Rrc_e,
    Rrc_h,
    Rrc_l,
    Rrc_hl,
    Rrc_a,
    Rl_b,
    Rl_c,
    Rl_d,
    Rl_e,
    Rl_h,
    Rl_l,
    Rl_hl,
    Rl_a,
    Rr_b,
    Rr_c,
    Rr_d,
    Rr_e,
    Rr_h,
    Rr_l,
    Rr_hl,
    Rr_a,
    Sla_b,
    Sla_c,
    Sla_d,
    Sla_e,
    Sla_h,
    Sla_l,
    Sla_hl,
    Sla_a,
    Sra_b,
    Sra_c,
    Sra_d,
    Sra_e,
    Sra_h,
    Sra_l,
    Sra_hl,
    Sra_a,
    Swap_b,
    Swap_c,
    Swap_d,
    Swap_e,
    Swap_h,
    Swap_l,
    Swap_hl,
    Swap_a,
    Srl_b,
    Srl_c,
    Srl_d,
    Srl_e,
    Srl_h,
    Srl_l,
    Srl_hl,
    Srl_a,
    Bit_0_b,
    Bit_0_c,
    Bit_0_d,
    Bit_0_e,
    Bit_0_h,
    Bit_0_l,
    Bit_0_hl,
    Bit_0_a,
    Bit_1_b,
    Bit_1_c,
    Bit_1_d,
    Bit_1_e,
    Bit_1_h,
    Bit_1_l,
    Bit_1_hl,
    Bit_1_a,
    Bit_2_b,
    Bit_2_c,
    Bit_2_d,
    Bit_2_e,
    Bit_2_h,
    Bit_2_l,
    Bit_2_hl,
    Bit_2_a,
    Bit_3_b,
    Bit_3_c,
    Bit_3_d,
    Bit_3_e,
    Bit_3_h,
    Bit_3_l,
    Bit_3_hl,
    Bit_3_a,

    Bit_4_b,
    Bit_4_c,
    Bit_4_d,
    Bit_4_e,
    Bit_4_h,
    Bit_4_l,
    Bit_4_hl,
    Bit_4_a,
    Bit_5_b,
    Bit_5_c,
    Bit_5_d,
    Bit_5_e,
    Bit_5_h,
    Bit_5_l,
    Bit_5_hl,
    Bit_5_a,

    Bit_6_b,
    Bit_6_c,
    Bit_6_d,
    Bit_6_e,
    Bit_6_h,
    Bit_6_l,
    Bit_6_hl,
    Bit_6_a,
    Bit_7_b,
    Bit_7_c,
    Bit_7_d,
    Bit_7_e,
    Bit_7_h,
    Bit_7_l,
    Bit_7_hl,
    Bit_7_a,

    Res_b,
    Res_c,
    Res_d,
    Res_e,
    Res_h,
    Res_l,
    Res_hl,
    Res_a,
    Res_1_b,
    Res_1_c,
    Res_1_d,
    Res_1_e,
    Res_1_h,
    Res_1_l,
    Res_1_hl,
    Res_1_a,

    Res_2_b,
    Res_2_c,
    Res_2_d,
    Res_2_e,
    Res_2_h,
    Res_2_l,
    Res_2_hl,
    Res_2_a,
    Res_3_b,
    Res_3_c,
    Res_3_d,
    Res_3_e,
    Res_3_h,
    Res_3_l,
    Res_3_hl,
    Res_3_a,

    Res_4_b,
    Res_4_c,
    Res_4_d,
    Res_4_e,
    Res_4_h,
    Res_4_l,
    Res_4_hl,
    Res_4_a,
    Res_5_b,
    Res_5_c,
    Res_5_d,
    Res_5_e,
    Res_5_h,
    Res_5_l,
    Res_5_hl,
    Res_5_a,

    Res_6_b,
    Res_6_c,
    Res_6_d,
    Res_6_e,
    Res_6_h,
    Res_6_l,
    Res_6_hl,
    Res_6_a,
    Res_7_b,
    Res_7_c,
    Res_7_d,
    Res_7_e,
    Res_7_h,
    Res_7_l,
    Res_7_hl,
    Res_7_a,

    Set_0_b,
    Set_0_c,
    Set_0_d,
    Set_0_e,
    Set_0_h,
    Set_0_l,
    Set_0_hl,
    Set_0_a,
    Set_1_b,
    Set_1_c,
    Set_1_d,
    Set_1_e,
    Set_1_h,
    Set_1_l,
    Set_1_hl,
    Set_1_a,

    Set_2_b,
    Set_2_c,
    Set_2_d,
    Set_2_e,
    Set_2_h,
    Set_2_l,
    Set_2_hl,
    Set_2_a,
    Set_3_b,
    Set_3_c,
    Set_3_d,
    Set_3_e,
    Set_3_h,
    Set_3_l,
    Set_3_hl,
    Set_3_a,

    Set_4_b,
    Set_4_c,
    Set_4_d,
    Set_4_e,
    Set_4_h,
    Set_4_l,
    Set_4_hl,
    Set_4_a,
    Set_5_b,
    Set_5_c,
    Set_5_d,
    Set_5_e,
    Set_5_h,
    Set_5_l,
    Set_5_hl,
    Set_5_a,

    Set_6_b,
    Set_6_c,
    Set_6_d,
    Set_6_e,
    Set_6_h,
    Set_6_l,
    Set_6_hl,
    Set_6_a,
    Set_7_b,
    Set_7_c,
    Set_7_d,
    Set_7_e,
    Set_7_h,
    Set_7_l,
    Set_7_hl,
    Set_7_a,
}

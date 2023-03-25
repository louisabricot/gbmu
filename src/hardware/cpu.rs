use self::instructions::{Opcode};
use self::registers::{Register8, Registers};
use super::memory::Memory;

#[allow(dead_code)]

pub mod instructions;
pub mod registers;

pub struct Cpu {
    registers: Registers,
    pc: u8,
    sp: u16,
    state: State,
    memory: Memory,
}

#[derive(Debug)]
pub enum State {
    RUNNING,
    HALT,
    INTERRUPT,
}

impl Cpu {
    /// Initializes registers
    /// TODO: Set Stack Pointer and Program Counter
    /// TODO: Maybe add a INIT state?
    pub fn new(memory: Memory) -> Self {

        Self {
            registers: Registers::new(),
            pc: 0,
            sp: 0,
            state: State::RUNNING,
            memory: memory,
        }
    }

    /// Read a byte from memory
    /// Tries to match the opcode with valid opcode listed in Opcode enum
    /// If opcode is 0xCB, reads the next byte and tries to match the opcode with CB prefixed
    /// opcodes
    ///
    fn fetch_decode(&mut self) -> Opcode {
        let opcode = self.read_imm8();

        if opcode == 0xCB {
            return self.fetch_cb_opcode();
        }

        match opcode {
            0x00 => Opcode::NOP,
            0x01 => Opcode::LD_BC_d16,
            0x02 => Opcode::LD_BC_A,
            0x03 => Opcode::INC_BC,
            0x04 => Opcode::INC_B,
            0x05 => Opcode::DEC_B,
            0x06 => Opcode::LD_B_d8,
            0x07 => Opcode::RLCA,
            0x08 => Opcode::LD_a16_SP,
            0x09 => Opcode::ADD_HL_BC,
            0x0A => Opcode::LD_A_BC,
            0x0B => Opcode::DEC_BC,
            0x0C => Opcode::INC_C,
            0x0D => Opcode::DEC_C,
            0x0E => Opcode::LD_C_d8,
            0x0F => Opcode::RRCA,

            0x10 => Opcode::STOP,
            0x11 => Opcode::LD_DE_d16,
            0x12 => Opcode::LD_DE_A,
            0x13 => Opcode::INC_DE,
            0x14 => Opcode::INC_D,
            0x15 => Opcode::DEC_D,
            0x16 => Opcode::LD_D_d8,
            0x17 => Opcode::RLA,
            0x18 => Opcode::JR_r8,
            0x19 => Opcode::ADD_HL_DE,
            0x1A => Opcode::LD_A_DE,
            0x1B => Opcode::DEC_DE,
            0x1C => Opcode::INC_E,
            0x1D => Opcode::DEC_E,
            0x1E => Opcode::LD_E_d8,
            0x1F => Opcode::RRA,

            0x20 => Opcode::JR_NZ_r8,
            0x21 => Opcode::LD_HL_d16,
            0x22 => Opcode::LD_HL_A,
            0x23 => Opcode::INC_HL,
            0x24 => Opcode::INC_H,
            0x25 => Opcode::DEC_H,
            0x26 => Opcode::LD_H_d8,
            0x27 => Opcode::DAA,
            0x28 => Opcode::JR_Z_d8,
            0x29 => Opcode::ADD_HL_HL,
            0x2A => Opcode::LDI_A_HL,
            0x2B => Opcode::DEC_HL,
            0x2C => Opcode::INC_L,
            0x2D => Opcode::DEC_L,
            0x2E => Opcode::LD_L_d8,
            0x2F => Opcode::CPL,

            0x30 => Opcode::JR_NC_r8,
            0x31 => Opcode::LD_SP_d16,
            0x32 => Opcode::LDD_HL_A,
            0x33 => Opcode::INC_SP,
            0x34 => Opcode::INC_aHL,
            0x35 => Opcode::DEC_aHL,
            0x36 => Opcode::LD_aHL_d8,
            0x37 => Opcode::SCF,
            0x38 => Opcode::JR_C_r8,
            0x39 => Opcode::ADD_HL_SP,
            0x3A => Opcode::LDD_A_aHL,
            0x3B => Opcode::DEC_SP,
            0x3C => Opcode::INC_A,
            0x3D => Opcode::DEC_A,
            0x3E => Opcode::LD_A_d8,
            0x3F => Opcode::CCF,

            0x40 => Opcode::LD_B_B,
            0x41 => Opcode::LD_B_C,
            0x42 => Opcode::LD_B_D,
            0x43 => Opcode::LD_B_E,
            0x44 => Opcode::LD_B_H,
            0x45 => Opcode::LD_B_L,
            0x46 => Opcode::LD_B_HL,
            0x47 => Opcode::LD_B_A,
            0x48 => Opcode::LD_C_B,
            0x49 => Opcode::LD_C_C,
            0x4A => Opcode::LD_C_D,
            0x4B => Opcode::LD_C_E,
            0x4C => Opcode::LD_C_H,
            0x4D => Opcode::LD_C_L,
            0x4E => Opcode::LD_C_HL,
            0x4F => Opcode::LD_C_A,

            0x50 => Opcode::LD_D_B,
            0x51 => Opcode::LD_D_C,
            0x52 => Opcode::LD_D_D,
            0x53 => Opcode::LD_D_E,
            0x54 => Opcode::LD_D_H,
            0x55 => Opcode::LD_D_L,
            0x56 => Opcode::LD_D_HL,
            0x57 => Opcode::LD_D_A,
            0x58 => Opcode::LD_E_B,
            0x59 => Opcode::LD_E_C,
            0x5A => Opcode::LD_E_D,
            0x5B => Opcode::LD_E_E,
            0x5C => Opcode::LD_E_H,
            0x5D => Opcode::LD_E_L,
            0x5E => Opcode::LD_E_HL,
            0x5F => Opcode::LD_E_A,

            0x60 => Opcode::LD_H_B,
            0x61 => Opcode::LD_H_C,
            0x62 => Opcode::LD_H_D,
            0x63 => Opcode::LD_H_E,
            0x64 => Opcode::LD_H_H,
            0x65 => Opcode::LD_H_L,
            0x66 => Opcode::LD_H_HL,
            0x67 => Opcode::LD_H_A,
            0x68 => Opcode::LD_L_B,
            0x69 => Opcode::LD_L_C,
            0x6A => Opcode::LD_L_D,
            0x6B => Opcode::LD_L_E,
            0x6C => Opcode::LD_L_H,
            0x6D => Opcode::LD_L_L,
            0x6E => Opcode::LD_L_HL,
            0x6F => Opcode::LD_L_A,

            0x70 => Opcode::LD_HL_B,
            0x71 => Opcode::LD_HL_C,
            0x72 => Opcode::LD_HL_D,
            0x73 => Opcode::LD_HL_E,
            0x74 => Opcode::LD_HL_H,
            0x75 => Opcode::LD_HL_L,
            0x76 => Opcode::HALT,
            0x77 => Opcode::LD_HL_A,
            0x78 => Opcode::LD_A_B,
            0x79 => Opcode::LD_A_C,
            0x7A => Opcode::LD_A_D,
            0x7B => Opcode::LD_A_E,
            0x7C => Opcode::LD_A_H,
            0x7D => Opcode::LD_A_L,
            0x7E => Opcode::LD_A_HL,
            0x7F => Opcode::LD_A_A,

            0x80 => Opcode::ADD_A_B,
            0x81 => Opcode::ADD_A_C,
            0x82 => Opcode::ADD_A_D,
            0x83 => Opcode::ADD_A_E,
            0x84 => Opcode::ADD_A_H,
            0x85 => Opcode::ADD_A_L,
            0x86 => Opcode::ADD_A_HL,
            0x87 => Opcode::ADD_A_A,
            0x88 => Opcode::ADC_A_B,
            0x89 => Opcode::ADC_A_C,
            0x8A => Opcode::ADC_A_D,
            0x8B => Opcode::ADC_A_E,
            0x8C => Opcode::ADC_A_H,
            0x8D => Opcode::ADC_A_L,
            0x8E => Opcode::ADC_A_HL,
            0x8F => Opcode::ADC_A_A,

            0x90 => Opcode::SUB_B,
            0x91 => Opcode::SUB_C,
            0x92 => Opcode::SUB_D,
            0x93 => Opcode::SUB_E,
            0x94 => Opcode::SUB_H,
            0x95 => Opcode::SUB_L,
            0x96 => Opcode::SUB_HL,
            0x97 => Opcode::SUB_A,
            0x98 => Opcode::SBC_A_B,
            0x99 => Opcode::SBC_A_C,
            0x9A => Opcode::SBC_A_D,
            0x9B => Opcode::SBC_A_E,
            0x9C => Opcode::SBC_A_H,
            0x9D => Opcode::SBC_A_L,
            0x9E => Opcode::SBC_A_HL,
            0x9F => Opcode::SBC_A_A,

            0xA0 => Opcode::AND_B,
            0xA1 => Opcode::AND_C,
            0xA2 => Opcode::AND_D,
            0xA3 => Opcode::AND_E,
            0xA4 => Opcode::AND_H,
            0xA5 => Opcode::AND_L,
            0xA6 => Opcode::AND_HL,
            0xA7 => Opcode::AND_A,
            0xA8 => Opcode::XOR_B,
            0xA9 => Opcode::XOR_C,
            0xAA => Opcode::XOR_D,
            0xAB => Opcode::XOR_E,
            0xAC => Opcode::XOR_H,
            0xAD => Opcode::XOR_L,
            0xAE => Opcode::XOR_HL,
            0xAF => Opcode::XOR_A,

            0xB0 => Opcode::OR_B,
            0xB1 => Opcode::OR_C,
            0xB2 => Opcode::OR_D,
            0xB3 => Opcode::OR_E,
            0xB4 => Opcode::OR_H,
            0xB5 => Opcode::OR_L,
            0xB6 => Opcode::OR_HL,
            0xB7 => Opcode::OR_A,
            0xB8 => Opcode::CP_B,
            0xB9 => Opcode::CP_C,
            0xBA => Opcode::CP_D,
            0xBA => Opcode::CP_D,
            0xBB => Opcode::CP_E,
            0xBC => Opcode::CP_H,
            0xBD => Opcode::CP_L,
            0xBE => Opcode::CP_HL,
            0xBF => Opcode::CP_A,

            0xC0 => Opcode::RET_NZ,
            0xC1 => Opcode::POP,
            0xC2 => Opcode::JP_NZ_a16,
            0xC3 => Opcode::JP_a16,
            0xC4 => Opcode::CALL_NZ_a16,
            0xC5 => Opcode::PUSH_BC,
            0xC6 => Opcode::ADD_A_d8,
            0xC7 => Opcode::RST_00H,
            0xC8 => Opcode::RET_Z,
            0xC9 => Opcode::RET,
            0xCA => Opcode::JP_Z_a16,
            0xCC => Opcode::CALL_Z_a16,
            0xCD => Opcode::CALL_a16,
            0xCE => Opcode::ADC_A_d8,
            0xCF => Opcode::RST_08H,

            0xD0 => Opcode::RET_NC,
            0xD1 => Opcode::POP_DE,
            0xD2 => Opcode::JP_NC_a16,
            0xD4 => Opcode::CALL_NC_a16,
            0xD5 => Opcode::PUSH_DE,
            0xD6 => Opcode::SUB_d8,
            0xD7 => Opcode::RST_10H,
            0xD8 => Opcode::RET_C,
            0xD9 => Opcode::RETI,
            0xDA => Opcode::JP_C_a16,
            0xDC => Opcode::CALL_C_A16,
            0xDE => Opcode::SBC_A_d8,
            0xDF => Opcode::RST_18H,

            0xE0 => Opcode::LDH_a8_A,
            0xE1 => Opcode::POP_HL,
            0xE2 => Opcode::LD_aC_A,
            0xE5 => Opcode::PUSH_HL,
            0xE6 => Opcode::AND_d8,
            0xE7 => Opcode::RST_20H,
            0xE8 => Opcode::ADD_SP_r8,
            0xE9 => Opcode::JP_HL,
            0xEA => Opcode::LD_a16_A,
            0xEE => Opcode::XOR_d8,
            0xEF => Opcode::RST_28h,

            0xF0 => Opcode::LDH_A_a8,
            0xF1 => Opcode::POP_AF,
            0xF2 => Opcode::LD_A_aC,
            0xF3 => Opcode::DI,
            0xF5 => Opcode::PUSH_AF,
            0xF6 => Opcode::OR_d8,
            0xF7 => Opcode::RST_30H,
            0xF8 => Opcode::LD_HL_SPr8
            0xF9 => Opcode::LD_SP_HL,
            0xFA => Opcode::LD_A_a16,
            0xFB => Opcode::EI,
            0xFE => Opcode::CP_d8,
            0xFF => Opcode::RST_38H
            _ => panic!("value not part of the instruction set: {:?}", opcode),
        }
    }

    fn fetch_cb_opcode(&mut self) -> Opcode {
        let opcode = self.read_imm8();
        match opcode {
            0x00 => Opcode::RLC_B,
            0x01 => Opcode::RLC_C,
            0x02 => Opcode::RLC_D,
            0x03 => Opcode::RLC_E,
            0x04 => Opcode::RLC_H,
            0x05 => Opcode::RLC_L,
            0x06 => Opcode::RLC_HL,
            0x07 => Opcode::RLC_A,
            0x08 => Opcode::RRC_B,
            0x09 => Opcode::RRC_C,
            0x0A => Opcode::RRC_D,
            0x0B => Opcode::RRC_E,
            0x0C => Opcode::RRC_H,
            0x0D => Opcode::RRC_L,
            0x0E => Opcode::RRC_HL,
            0x0F => Opcode::RRC_A,
            
            0x10 => Opcode::RL_B,
            0x11 => Opcode::RL_C,
            0x12 => Opcode::RL_D,
            0x13 => Opcode::RL_E,
            0x14 => Opcode::RL_H,
            0x15 => Opcode::RL_L,
            0x16 => Opcode::RL_HL,
            0x17 => Opcode::RL_A,
            0x18 => Opcode::RR_B,
            0x1F => Opcode::RR_C,
            0x1A => Opcode::RR_D,
            0x1B => Opcode::RR_E,
            0x1C => Opcode::RR_H,
            0x1D => Opcode::RR_L,
            0x1E => Opcode::RR_HL,
            0x1F => Opcode::RR_A,
            
            0x20 => Opcode::SLA_B,
            0x21 => Opcode::SLA_C,
            0x22 => Opcode::SLA_D,
            0x23 => Opcode::SLA_E,
            0x24 => Opcode::SLA_H,
            0x25 => Opcode::SLA_L,
            0x26 => Opcode::SLA_HL,
            0x27 => Opcode::SLA_A,
            0x28 => Opcode::SRA_B,
            0x2F => Opcode::SRA_C,
            0x2A => Opcode::SRA_D,
            0x2B => Opcode::SRA_E,
            0x2C => Opcode::SRA_H,
            0x2D => Opcode::SRA_L,
            0x2E => Opcode::SRA_HL,
            0x2F => Opcode::SRA_A,
            
            0x30 => Opcode::SWAP_B,
            0x31 => Opcode::SWAP_C,
            0x32 => Opcode::SWAP_D,
            0x33 => Opcode::SWAP_E,
            0x34 => Opcode::SWAP_H,
            0x35 => Opcode::SWAP_L,
            0x36 => Opcode::SWAP_HL,
            0x37 => Opcode::SWAP_A,
            0x38 => Opcode::SRL_B,
            0x39 => Opcode::SRL_C,
            0x3A => Opcode::SRL_D,
            0x3B => Opcode::SRL_E,
            0x3C => Opcode::SRL_H,
            0x3D => Opcode::SRL_L,
            0x3E => Opcode::SRL_HL,
            0x3F => Opcode::SRL_A,
            
            0x40 => Opcode::BIT_0_B,
            0x41 => Opcode::BIT_0_C,
            0x42 => Opcode::BIT_0_D,
            0x43 => Opcode::BIT_0_E,
            0x44 => Opcode::BIT_0_H,
            0x45 => Opcode::BIT_0_L,
            0x46 => Opcode::BIT_0_HL,
            0x47 => Opcode::BIT_0_A,
            0x48 => Opcode::BIT_1_B,
            0x49 => Opcode::BIT_1_C,
            0x4A => Opcode::BIT_1_D,
            0x4B => Opcode::BIT_1_E,
            0x4C => Opcode::BIT_1_H,
            0x4D => Opcode::BIT_1_L,
            0x4E => Opcode::BIT_1_HL,
            0x4F => Opcode::BIT_1_A,

            0x50 => Opcode::BIT_2_B,
            0x51 => Opcode::BIT_2_C,
            0x52 => Opcode::BIT_2_D,
            0x53 => Opcode::BIT_2_E,
            0x54 => Opcode::BIT_2_H,
            0x55 => Opcode::BIT_2_L,
            0x56 => Opcode::BIT_2_HL,
            0x57 => Opcode::BIT_2_A,
            0x58 => Opcode::BIT_3_B,
            0x59 => Opcode::BIT_3_C,
            0x5A => Opcode::BIT_3_D,
            0x5B => Opcode::BIT_3_E,
            0x5C => Opcode::BIT_3_H,
            0x5D => Opcode::BIT_3_L,
            0x5E => Opcode::BIT_3_HL,
            0x5F => Opcode::BIT_3_A,
            
            0x60 => Opcode::BIT_4_B,
            0x61 => Opcode::BIT_4_C,
            0x62 => Opcode::BIT_4_D,
            0x63 => Opcode::BIT_4_E,
            0x64 => Opcode::BIT_4_H,
            0x65 => Opcode::BIT_4_L,
            0x66 => Opcode::BIT_4_HL,
            0x67 => Opcode::BIT_4_A,
            0x68 => Opcode::BIT_5_B,
            0x69 => Opcode::BIT_5_C,
            0x6A => Opcode::BIT_5_D,
            0x6B => Opcode::BIT_5_E,
            0x6C => Opcode::BIT_5_H,
            0x6D => Opcode::BIT_5_L,
            0x6E => Opcode::BIT_5_HL,
            0x6F => Opcode::BIT_5_A,
            
            0x70 => Opcode::BIT_6_B,
            0x71 => Opcode::BIT_6_C,
            0x72 => Opcode::BIT_6_D,
            0x73 => Opcode::BIT_6_E,
            0x74 => Opcode::BIT_6_H,
            0x75 => Opcode::BIT_6_L,
            0x76 => Opcode::BIT_6_HL,
            0x77 => Opcode::BIT_6_A,
            0x78 => Opcode::BIT_7_B,
            0x79 => Opcode::BIT_7_C,
            0x7A => Opcode::BIT_7_D,
            0x7B => Opcode::BIT_7_E,
            0x7C => Opcode::BIT_7_H,
            0x7D => Opcode::BIT_7_L,
            0x7E => Opcode::BIT_7_HL,
            0x7F => Opcode::BIT_7_A,

            0x80 => Opcode::RES_B,
            0x81 => Opcode::RES_C,
            0x82 => Opcode::RES_D,
            0x83 => Opcode::RES_E,
            0x84 => Opcode::RES_H,
            0x85 => Opcode::RES_L,
            0x86 => Opcode::RES_HL,
            0x87 => Opcode::RES_A,
            0x88 => Opcode::RES_1_B,
            0x89 => Opcode::RES_1_C,
            0x8A => Opcode::RES_1_D,
            0x8B => Opcode::RES_1_E,
            0x8C => Opcode::RES_1_H,
            0x8D => Opcode::RES_1_L,
            0x8E => Opcode::RES_1_HL,
            0x8F => Opcode::RES_1_A,

            0x90 => Opcode::RES_2_B,
            0x91 => Opcode::RES_2_C,
            0x92 => Opcode::RES_2_D,
            0x93 => Opcode::RES_2_E,
            0x94 => Opcode::RES_2_H,
            0x95 => Opcode::RES_2_L,
            0x96 => Opcode::RES_2_HL,
            0x97 => Opcode::RES_2_A,
            0x98 => Opcode::RES_3_B,
            0x99 => Opcode::RES_3_C,
            0x9A => Opcode::RES_3_D,
            0x9B => Opcode::RES_3_E,
            0x9C => Opcode::RES_3_H,
            0x9D => Opcode::RES_3_L,
            0x9E => Opcode::RES_3_HL,
            0x9F => Opcode::RES_3_A,

            0xA0 => Opcode::RES_4_B,
            0xA1 => Opcode::RES_4_C,
            0xA2 => Opcode::RES_4_D,
            0xA3 => Opcode::RES_4_E,
            0xA4 => Opcode::RES_4_H,
            0xA5 => Opcode::RES_4_L,
            0xA6 => Opcode::RES_4_HL,
            0xA7 => Opcode::RES_4_A,
            0xA8 => Opcode::RES_5_B,
            0xA9 => Opcode::RES_5_C,
            0xAA => Opcode::RES_5_D,
            0xAB => Opcode::RES_5_E,
            0xAC => Opcode::RES_5_H,
            0xAD => Opcode::RES_5_L,
            0xAE => Opcode::RES_5_HL,
            0xAF => Opcode::RES_5_A,

            0xB0 => Opcode::RES_6_B,
            0xB1 => Opcode::RES_6_C,
            0xB2 => Opcode::RES_6_D,
            0xB3 => Opcode::RES_6_E,
            0xB4 => Opcode::RES_6_H,
            0xB5 => Opcode::RES_6_L,
            0xB6 => Opcode::RES_6_HL,
            0xB7 => Opcode::RES_6_A,
            0xB8 => Opcode::RES_7_B,
            0xB9 => Opcode::RES_7_C,
            0xBA => Opcode::RES_7_D,
            0xBB => Opcode::RES_7_E,
            0xBC => Opcode::RES_7_H,
            0xBD => Opcode::RES_7_L,
            0xBE => Opcode::RES_7_HL,
            0xBF => Opcode::RES_7_A,
            
            0xC0 => Opcode::SET_0_B,
            0xC1 => Opcode::SET_0_C,
            0xC2 => Opcode::SET_0_D,
            0xC3 => Opcode::SET_0_E,
            0xC4 => Opcode::SET_0_H,
            0xC5 => Opcode::SET_0_L,
            0xC6 => Opcode::SET_0_HL,
            0xC7 => Opcode::SET_0_A,
            0xC8 => Opcode::SET_1_B,
            0xC9 => Opcode::SET_1_C,
            0xCA => Opcode::SET_1_D,
            0xCB => Opcode::SET_1_E,
            0xCC => Opcode::SET_1_H,
            0xCD => Opcode::SET_1_L,
            0xCE => Opcode::SET_1_HL,
            0xCF => Opcode::SET_1_A,

            0xD0 => Opcode::SET_2_B,
            0xD1 => Opcode::SET_2_C,
            0xD2 => Opcode::SET_2_D,
            0xD3 => Opcode::SET_2_E,
            0xD4 => Opcode::SET_2_H,
            0xD5 => Opcode::SET_2_L,
            0xD6 => Opcode::SET_2_HL,
            0xD7 => Opcode::SET_2_A,
            0xD8 => Opcode::SET_3_B,
            0xD9 => Opcode::SET_3_C,
            0xDA => Opcode::SET_3_D,
            0xDB => Opcode::SET_3_E,
            0xDC => Opcode::SET_3_H,
            0xDD => Opcode::SET_3_L,
            0xDE => Opcode::SET_3_HL,
            0xDF => Opcode::SET_3_A,
            
            0xE0 => Opcode::SET_4_B,
            0xE1 => Opcode::SET_4_C,
            0xE2 => Opcode::SET_4_D,
            0xE3 => Opcode::SET_4_E,
            0xE4 => Opcode::SET_4_H,
            0xE5 => Opcode::SET_4_L,
            0xE6 => Opcode::SET_4_HL,
            0xE7 => Opcode::SET_4_A,
            0xE8 => Opcode::SET_5_B,
            0xE9 => Opcode::SET_5_C,
            0xEA => Opcode::SET_5_D,
            0xEB => Opcode::SET_5_E,
            0xEC => Opcode::SET_5_H,
            0xED => Opcode::SET_5_L,
            0xEE => Opcode::SET_5_HL,
            0xEF => Opcode::SET_5_A,

            0xF0 => Opcode::SET_6_B,
            0xF1 => Opcode::SET_6_C,
            0xF2 => Opcode::SET_6_D,
            0xF3 => Opcode::SET_6_E,
            0xF4 => Opcode::SET_6_H,
            0xF5 => Opcode::SET_6_L,
            0xF6 => Opcode::SET_6_HL,
            0xF7 => Opcode::SET_6_A,
            0xF8 => Opcode::SET_7_B,
            0xF9 => Opcode::SET_7_C,
            0xFA => Opcode::SET_7_D,
            0xFB => Opcode::SET_7_E,
            0xFC => Opcode::SET_7_H,
            0xFD => Opcode::SET_7_L,
            0xFE => Opcode::SET_7_HL,
            0xFF => Opcode::SET_7_A,
            _ => panic!("value not part of the instruction set: 0xCB {:x}", opcode),
        }
    }

    /// Read the next byte from memory
    /// Update PC
    fn read_imm8(&mut self) -> u8 {
        let addr = self.pc;
        self.pc.wrapping_add(1);
        return self.memory.read8(addr);
    }

    /// Read the next 16 bits from memory
    /// Convert the value from little endian to big endian
    /// Update PC
    /// Read imm16 takes two cycles
    fn read_imm16(&mut self) -> u16 {
        let least = self.read_imm8();
        let most = self.read_imm8();
        return u16::from_le_bytes([least, most]);
    }

    /// Execute an instruction
    /// TODO: returns the CPU state
    fn execute(&mut self, instruction: Instruction) {
        match instruction.operation {
            Operation::Load(dst, src)  => { 
                load(dst, src);
            },
            Operation::Jp(condition) => {
                match conditon {
                    condition => Condition::
                }
                jump(condition);
            },
            _ => todo!(),
        }
    }

    /// Jump to the absolute address speicified by the 16-bit operand, depending on the condition
    /// Reads the 16-bit operand from immediate memory
    /// Update the value of PC with the operand
    /// Note that the operand is read even if the condition is false
    /// Unconditional jumps are also handled by this function, their condition is of type
    /// Condition::Always
    fn jump(condition: Condition) {
        
        let address = self.read_imm16();

        if FlagsRegister.check_condition(condition) {
            self.pc = address;
        }
    }
    
    /// Jump to the relative address specified by the signed 8-bit operand, depending
    /// on condition
    /// Reads the 8-bit operand from immediate memory
    /// Adds operand to PC if the condition is checked
    /// Note that the operand is read even when the condition is false
    /// Unconditional relative jumps are also handled by this fonction, their condition is of type
    /// Condition::Always
    fn jump_relative(condition: Condition) {
        
        let operand = self.read_imm8() as u16;

        if FlagsRegister.check_condition(condition) {
            self.pc += operand;
        }
    }

    fn load8(destination: LoadParam, source: LoadParam) {
        match source {
            LoadParam::Imm8 => self.read_imm8(self.pc),
            LoadParam::Address => todo!(),
            LoadParam::
        }
    }

    fn load16(destination: Load16Target, 
    //TODO:
    // Faire une fonction qui prend en parametre le nombre de ligne 
    // d'instruction a renvoyer a partir de l'addresse actuelle du CPU
    //
    // Les instructions doivent contenir la longueur de l'operande
    // pour que l'on puisse parser les instructions sans les executer.
    //
    // Une fonction doit pouvoir retourner une string contenant
    // l'instruction et sont operand exacte, exemple:
    // mov eax, 0x8 # operand numerique
    // mov eax, [0x800] # addresse a laquelle recuperer
    // La fonction doit pouvoir recuperer cette information
    // a partir de n'importe quelle addresse
    //
    pub fn step(&mut self) {
        match self.state {
            State::RUNNING => {
                let opcode = self.fetch_decode();
                self.execute(opcode);
            }
            State::HALT => {}
            State::INTERRUPT => {}
        }
    }
}

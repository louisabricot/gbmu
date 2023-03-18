use self::instructions::{Addr, Opcode};
use self::registers::{Register8, Registers};
use super::memory::Memory;

pub mod instructions;
pub mod registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
    state: State,
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
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            pc: 0,
            sp: 0,
            state: State::RUNNING,
        }
    }

    /// Read a byte from memory
    /// Tries to match the opcode with valid opcode listed in Opcode enum
    /// If opcode is 0xCB, reads the next byte and tries to match the opcode with CB prefixed
    /// opcodes
    ///
    fn fetch_decode(&mut self, memory: &Memory) -> Opcode {
        let opcode = self.read_imm8(memory);

        if opcode == 0xCB {
            return self.fetch_cb_opcode(memory);
        }

        match opcode {
            0x7f => Opcode::LD_A_A,
            0x78 => Opcode::LD_A_B,
            0x79 => Opcode::LD_A_C,
            0x7A => Opcode::LD_A_D,
            0x7B => Opcode::LD_A_E,
            0x7C => Opcode::LD_A_H,
            0x7D => Opcode::LD_A_L,
            _ => panic!("Opcode not implemented yet! {:?}", opcode),
        }
    }

    fn fetch_cb_opcode(&mut self, memory: &Memory) -> Opcode {
        let opcode = self.read_imm8(memory);
        match opcode {
            _ => todo!(),
        }
    }

    /// Read the next byte from memory
    /// Update PC
    fn read_imm8(&mut self, memory: &Memory) -> u8 {
        let addr = self.pc;
        self.pc.wrapping_add(1);
        return Memory::read8(memory, addr);
    }

    /// Read the next 16 bits from memory
    /// Convert the value from little endian to big endian
    /// Update PC
    fn read_imm16(&mut self, memory: &Memory) -> u16 {
        let least = self.read_imm8(memory);
        let most = self.read_imm8(memory);
        return u16::from_le_bytes([least, most]);
    }

    /// Execute an instruction
    /// TODO: returns the CPU state
    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::LD_A_A => self.load(Register8::A, Register8::A),
            Opcode::LD_A_B => self.load(Register8::A, Register8::B),
            Opcode::LD_A_C => self.load(Register8::A, Register8::C),
            Opcode::LD_A_D => self.load(Register8::A, Register8::D),
            Opcode::LD_A_E => self.load(Register8::A, Register8::E),
            Opcode::LD_A_H => self.load(Register8::A, Register8::H),
            Opcode::LD_A_L => self.load(Register8::A, Register8::L),
        }
    }

    pub fn step(&mut self, memory: &Memory) {
        match self.state {
            State::RUNNING => {
                let opcode = self.fetch_decode(memory);
                self.execute(opcode);
            }
            State::HALT => {}
            State::INTERRUPT => {}
        }
    }

    /// Put the value r2 into r1
    /// TODO: returns the flags that are affected ?
    fn load(&mut self, r1: Register8, r2: Register8) {
        println!("LD {:?} {:?}", r1, r2);
        let value = Registers::read8(&self.registers, r2);
        Registers::write8(&mut self.registers, r1, value);
    }
}

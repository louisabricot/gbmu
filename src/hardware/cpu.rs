use self::registers::{Registers, Register8, Register16};
use self::instructions::{Instruction, Opcode, Operation};
use super::memory::Memory;

pub mod registers;
pub mod instructions;

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
    fn fetch(&mut self, memory: Memory) -> Opcode {
        let opcode = self.read_imm8(memory);
        //if opcode == 0xCB {
        //    return fetch_cb_opcode(memory);
        //}
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

    //fn fetch_cb_opcode(&mut self, memory: Memory) -> Opcode {
    //    let opcode = self.read_imm8(memory);
    //    match opcode {
            //match opcode with Opcode
    //    }
   // }

    /// Read the next byte from memory
    /// Update PC
    fn read_imm8(&mut self, memory: Memory) -> u8 {
        let addr = self.pc;
        self.pc.wrapping_add(1);
        return memory::read8(self.pc);
    }

    /// Read the next 16 bits from memory
    /// Convert the value from little endian to big endian
    /// Update PC
    fn read_imm16(&mut self, memory: Memory) -> u16 {
        let least = self.read_imm8(memory);
        let most = self.read_imm8(memory);
        return u16::from_le_bytes([least, most]);
    }

    /// Decode the opcode and fetch its 
    /// Cette fonction ne sert a rien
    fn decode(&mut self, opcode: Opcode) -> Instruction {
            return Instruction::get(opcode);
    }

    /// Execute an instruction and returns the CPU state
    fn execute(&mut self, operation: Operation) {
        match operation {
            Operation::Load(r1, r2) => { 
                self.load(r1, r2);
            }
        }
    }

    pub fn step(&mut self, memory: Memory) {
        match self.state {
            State::RUNNING => {
                let opcode = self.fetch(memory);
                let instruction = self.decode(opcode, memory);
                self.state = self.execute(instruction.operation);
            },
            State::HALT => {},
            State::INTERRUPT => {},
            _ => panic!("State not implemented! {:?}", self.state),
        }
    }

    /// Put the value r2 into r1
    /// TODO: returns the flags that are affected ?
    fn load(&mut self, r1: Register8, r2: Register8) {
        let value = Registers::read8(&self.registers, r2);
        Registers::write8(&mut self.registers, r1, value);
    }
}


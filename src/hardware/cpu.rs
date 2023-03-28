use self::instructions::*;
use self::registers::{Register16, Registers};
use super::memory::Memory;
pub mod fetch;
#[allow(dead_code)]
pub mod instructions;
pub mod registers;
pub struct Cpu {
    registers: Registers,
    pc: u16,
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

    fn decode(opcode: Opcode) -> &'static Instruction {
        let instruction = Instruction::getByOpcode(opcode);
        match instruction {
            None => panic!("No Instruction found for opcode, this should never happend"),
            _ => instruction.unwrap(),
        }
    }

    /// Execute an instruction
    /// TODO: returns the CPU state
    fn execute(&mut self, instruction: &Instruction) {
        match &instruction.operation {
            Operation::Load8(dst, src) => {
                self.load8(dst, src);
            }
            Operation::Jp(condition, source) => {
                self.absolute_jump(condition, source);
            }
            Operation::Jr(condition) => {
                self.relative_jump(condition);
            }
            _ => todo!(),
        }
    }

    /// Jump to the absolute address speicified by the 16-bit operand, depending on the condition
    /// Reads the 16-bit operand from immediate memory
    /// Update the value of PC with the operand
    /// Note that the operand is read even if the condition is false
    /// Unconditional jumps are also handled by this function, their condition is of type
    /// Condition::Always
    fn absolute_jump(&mut self, condition: &Condition, source: &Source16) {
        let address: u16;

        match source {
            Source16::Imm16 => address = self.read_imm16(),
            Source16::HL => address = self.registers.read16(Register16::HL),
            _ => panic!("Not a valid Source16 for absolute jumps"),
        }

        /* if source == Source16::Imm16 {
            address = self.read_imm16();
        } else {
            address = self.registers.read16(Register16::HL);
        }*/

        if self.registers.f.check_condition(condition) {
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
    fn relative_jump(&mut self, condition: &Condition) {
        let operand = self.read_imm8() as u16;

        if self.registers.f.check_condition(condition) {
            self.pc += operand;
        }
    }

    fn load8(&self, destination: &Target8, source: &Source8) {
        todo!();
    }

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
                let opcode = self.fetch();
                let instruction = Cpu::decode(opcode);
                self.execute(instruction);
            }
            State::HALT => {
                todo!();
            }
            State::INTERRUPT => {
                todo!();
            }
        }
    }
}

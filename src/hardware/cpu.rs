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
            // 8-bit load instructions
            Operation::Load8(dst, src) => {
                self.load8(dst, src);
            }

            // TODO: clean code because super redundant
            Operation::Load8Dec(dst, src) => {
                self.load8(dst, src);
                let hl = self.registers.read16(Register16::HL);
                self.registers.write16(Register16::HL, hl - 1);
            }
            Operation::Load8Inc(dst, src) => {
                self.load8(dst, src);
                let hl = self.registers.read16(Register16::HL);
                self.registers.write16(Register16::HL, hl + 1);
            }

            // 16-bit load instructions
            Operation::Load16(dst, src) => {
                self.load16(dst, src);
            }
            Operation::Push(target) => {
                self.push(target);
            }
            Operation::Pop(target) => {
                self.pop(target);
            }

            // 8-bit arithmetic and logical instructions
            Operation::Add8(source) => {
                self.add8(source);
            }
            //TODO: adc, sub, sbc, and, xor, or, cp, inc8, dec8, daa, cpl

            // 16-bit arithmetic/logic instructions

            //TODO: add16, inc16, dec16, loadHL

            // Rotate, shift and bit operations
            //TODO: Rlca, Rla, Rrca, Rra, Rlc, Rl, Rrc, Rr, Sla, Swap, Sra, Srl
            //TODO: bit, set, res
            // Control Flow instruction
            //TODO: Ccf, Scf, Nop, Halt, Stop, Di, Ei, Jp, Jr, Call, Ret, Reti, Rst
            Operation::Jp(condition, source) => {
                self.absolute_jump(condition, source);
            }
            Operation::Jr(condition) => {
                self.relative_jump(condition);
            }
            _ => todo!(),
        }
    }

    fn add8(source: Source8) {
        let value = self.get_source8(source);

        //TODO: half carry

        let (result, carry) = self.registers.a.carrying_add(value);

        /// Z: Set if the result is 0, otherwise reset
        self.registers.f.set(FlagsRegister::ZERO, result == 0);

        /// H: Set if there is a carry from bit3; otherwise reset
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);

        /// N: Reset
        self.registers.f.set(FlagsRegister::SUBTRACT, false);

        /// CY: Set if there is a carry from bit7; otherwise reset
        self.registers.f.set(FlagsRegister::CARRY, carry);

        self.registers.write8(Register8::A, result);
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

    /// 8-bit load instructions
    fn get_source8(&mut self, source: &Source8) -> u8 {
        match source {
            Source8::A => self.registers.a,
            Source8::B => self.registers.b,
            Source8::C => self.registers.c,
            Source8::D => self.registers.d,
            Source8::E => self.registers.e,
            Source8::H => self.registers.h,
            Source8::L => self.registers.l,
            Source8::Imm8 => self.read_imm8(),
            Source8::Addr(at) => {
                let address = self.get_address(at);
                self.memory.read8(address)
            }
        }
    }

    fn get_address(&mut self, addr: &At) -> u16 {
        match addr {
            At::HL => self.registers.read16(Register16::HL),
            At::BC => self.registers.read16(Register16::BC),
            At::DE => self.registers.read16(Register16::DE),
            At::Imm16 => self.read_imm16(),
            At::Imm8 => 0xFF00 | self.read_imm8() as u16,
            At::C => 0xFF00 | self.registers.c as u16,
        }
    }

    fn load8(&mut self, destination: &Target8, source: &Source8) {
        let value = self.get_source8(source);
        match destination {
            Target8::A => self.registers.a = value,
            Target8::B => self.registers.b = value,
            Target8::C => self.registers.c = value,
            Target8::D => self.registers.d = value,
            Target8::E => self.registers.e = value,
            Target8::H => self.registers.h = value,
            Target8::L => self.registers.l = value,
            Target8::Addr(at) => {
                let address = self.get_address(at);
                self.memory.write8(address, value);
            }
        }
    }

    /// 16-bit load instructions
    fn get_source16(&mut self, source: &Source16) -> u16 {
        match source {
            Source16::BC => self.registers.read16(Register16::BC),
            Source16::DE => self.registers.read16(Register16::DE),
            Source16::HL => self.registers.read16(Register16::HL),
            Source16::SP => self.sp,
            Source16::Imm16 => self.read_imm16(),
        }
    }
    fn load16(&mut self, destination: &Target16, source: &Source16) {
        let value = self.get_source16(source);
        match destination {
            Target16::BC => self.registers.write16(Register16::BC, value),
            Target16::DE => self.registers.write16(Register16::DE, value),
            Target16::HL => self.registers.write16(Register16::HL, value),
            Target16::SP => self.sp = value,
            Target16::Addr => {
                let address = self.read_imm16();
                self.memory.write16(address, value);
            }
        }
    }

    fn push(&mut self, target: &Register16) {
        let value = self.registers.read16(target);
        let [lo, hi] = u16::to_le_bytes(value);

        self.sp.wrapping_sub(1);
        self.memory.write8(self.sp, hi);
        self.sp.wrapping_sub(1);
        self.memory.write8(self.sp, lo);
    }

    fn pop(&mut self, target: &Register16) {
        self.sp.wrapping_add(1);
        let lo = self.memory.read8(self.sp);
        self.sp.wrapping_add(1);
        let hi = self.memory.read8(self.sp);
        self.sp.wrapping_add(1);

        self.registers.write16(target, u16::from_le_bytes([lo, hi]));
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

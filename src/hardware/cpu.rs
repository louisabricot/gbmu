use self::registers::flags::Flags;
use self::registers::{Register16, Registers};
use super::memory::Memory;
use crate::hardware::cpu::instructions::{
    At, Condition, Instruction, Opcode, Operation, Source16, Source8, Target16, Target8,
};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};
pub mod fetch;
#[allow(dead_code)]
pub mod instructions;
pub mod registers;
pub struct Cpu {
    registers: Registers,
    state: State,
    memory: Memory,
}

pub enum State {
    Running,
    Halt,
    Interrupt,
}

impl Cpu {
    /// Initializes registers
    /// TODO: Set Stack Pointer and Program Counter
    /// TODO: Maybe add a INIT state?
    pub fn new(memory: Memory) -> Self {
        Self {
            registers: Registers::new(),
            state: State::Running,
            memory,
        }
    }

    /// Read the next byte from memory
    /// Update PC
    fn read_imm8(&mut self) -> u8 {
        let addr = self.registers.pc;
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.memory.read8(addr)
    }

    /// Read the next 16 bits from memory
    /// Convert the value from little endian to big endian
    /// Update PC
    /// Read imm16 takes two cycles
    fn read_imm16(&mut self) -> u16 {
        let least = self.read_imm8();
        let most = self.read_imm8();
        u16::from_le_bytes([least, most])
    }

    fn decode(opcode: Opcode) -> Instruction {
        let instruction = Instruction::get_by_opcode(opcode);
        match instruction {
            None => panic!("No Instruction found for opcode, this should never happend"),
            _ => instruction.unwrap(),
        }
    }

    /// Execute an instruction
    /// TODO: returns the CPU state
    fn execute(&mut self, instruction: Instruction) {
        match instruction.operation {
            
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
            Operation::Load16(dst, src) => self.load16(dst, src),
            Operation::Push(target) => self.push(target),
            Operation::Pop(target) => self.pop(target),

            // 8-bit arithmetic and logical instructions
            Operation::Add8(source) => self.add8(source),
            Operation::Adc(source) => self.adc8(source),
            Operation::Sub(source) => self.sub(source),
            Operation::Sbc(source) => self.sbc(source),
            Operation::And(source) => self.and(source),
            Operation::Xor(source) => self.xor(source),
            Operation::Or(source) => self.or(source),
            Operation::Cp(source) => self.cp(source),
            Operation::Inc8(target) => self.inc8(target),
            Operation::Dec8(target) => self.dec8(target),
            Operation::Daa => self.daa(),
            Operation::Cpl => self.cpl(),

            // 16-bit arithmetic/logic instructions
            Operation::AddHL_r16(source) => self.addHL_r16(source),
            Operation::Inc16(target) => self.inc16(target),
            Operation::Dec16(target) => self.dec16(target),
            Operation::AddSP_dd => self.addSP_dd(),
            Operation::LoadHL => self.loadHL(),

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
    /// The 8-bit operand is added to SP and the result is stored in HL.
    /// Flags affected:
    /// Z: Reset
    /// H: Set if there is a carry from bit 11, otherwise reset
    /// N: Reset
    /// C: Set if there is a carry from bit 15, otherwise reset
    fn loadHL(&mut self) {
        let value = self.read_imm8() as u16;

        let (result, carry) = self.registers.sp.overflowing_add(value);
        let half_carry = (self.registers.sp & 0x0FFF)
            .checked_add(value | 0xF000)
            .is_none();

        self.registers.write16(Register16::HL, result);
        self.registers.f.set(Flags::Z, false);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::C, carry);
    }

    /// Add to Stack Pointer the 8-bit immediate value
    /// Flags affected:
    /// Z: Reset
    /// N: Reset
    /// H: Set if there is a carry on bit7, otherwise reset
    /// C: Set if there is a carry on bit15, otherwise reset
    fn addSP_dd(&mut self) {
        let value = self.read_imm8() as u16;

        let (result, carry) = self.registers.pc.overflowing_add(value);

        let half_carry = (self.registers.sp & 0x0FFF)
            .checked_add(value | 0xF000)
            .is_none();
        self.registers.pc = result;
        self.registers.f.set(Flags::Z, false);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::C, carry);
    }

    /// Increments the contents of register pair by 1
    /// Flags are not affected
    fn inc16(&mut self, target: Register16) {
        let mut value = self.registers.read16(target);
        value = value.wrapping_add(1);
        self.registers.write16(target, value);
    }

    /// Decrements the contents of register pair by 1
    /// Flags are not affected
    fn dec16(&mut self, target: Register16) {
        let mut value = self.registers.read16(target);
        value = value.wrapping_sub(1);
        self.registers.write16(target, value);
    }

    /// Add to register HL, the content of the source register
    /// The Flags are affected as follows:
    /// Z: Not affected
    /// H: Set if there is a carry from bit11, otherwise reset
    /// N: Reset
    /// C: Set if there is a carry from bit5, otherwise reset
    fn addHL_r16(&mut self, source: Register16) {
        let value = self.registers.read16(source);
        let target = self.registers.read16(Register16::HL);

        let (result, carry) = u16::overflowing_add(target, value);
        let half_carry = u8::checked_add((target & 0x00FF) as u8, (value & 0x00FF) as u8).is_none();

        self.registers.write16(Register16::HL, result);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::C, carry);
    }
    /// Flips all the bits in the 8-bit register A
    /// The Flag Register are affected as follows:
    /// N: Set
    /// H: Set
    /// C: Not affected
    /// Z: Not affected
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.set(Flags::N, true);
        self.registers.f.set(Flags::H, true);
    }
    /// The DAA instruction adjusts the result of a binary addition or substraction to obtain the
    /// Binary Coded Decimal representation.
    /// The Flag Register is updated as follows:
    /// C: Set if there is carry from bit7, otherwise reset
    /// N: Not affected
    /// H: Reset
    /// Z: Set if the result is 0, otherwise reset
    ///
    fn daa(&mut self) {
        if self.registers.f.contains(Flags::N) {
            //SUB or SBC

            if self.registers.f.contains(Flags::C) {
                if self.registers.f.contains(Flags::H) {
                    self.registers.a = self.registers.a.wrapping_add(0x9A);
                } else {
                    self.registers.a = self.registers.a.wrapping_add(0xA0);
                }
            } else if self.registers.f.contains(Flags::H) {
                self.registers.a = self.registers.a.wrapping_add(0xFA);
            }
        } else {
            //ADD or ADC
            if self.registers.f.contains(Flags::H) || self.registers.a & 0x0F > 0x09 {
                self.registers.a = self.registers.a.wrapping_add(0x06);
            }

            if self.registers.f.contains(Flags::C) || self.registers.a & 0xF0 > 0x90 {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                self.registers.f.set(Flags::C, true);
            }
        }
        self.registers.f.set(Flags::Z, self.registers.a == 0);
        self.registers.f.set(Flags::H, false);
    }

    fn get_target8(&mut self, target: Target8) -> u8 {
        todo!();
    }

    /// Decrements data represented by target
    /// The Flag Register is updated as follows:
    /// Z: Set if the result is 0, otherwise reset
    /// N: Set
    /// H: Set if there is a carry from bit3, otherwise reset
    /// C: Not affected
    fn dec8(&mut self, target: Target8) {
        let value = self.get_target8(target);

        let result = value.wrapping_sub(1);

        let half_carry = (value & 0x0F).checked_sub(1 & 0xF0).is_none();

        self.registers.f.set(Flags::Z, result == 0);

        self.registers.f.set(Flags::N, true);

        self.registers.f.set(Flags::H, half_carry);

        self.load_u8(target, result);
    }

    /// Increments data represented by target
    /// The Flag Register is updated as follows:
    /// Z: Set if the result is 0, otherwise reset
    /// N: Reset
    /// H: Set if there is a carry from bit3, otherwise reset
    /// C: Not affected
    fn inc8(&mut self, target: Target8) {
        let mut value: u8 = self.get_target8(target);

        value = value.wrapping_add(1);

        let half_carry = (value & 0x0F).checked_add(1 & 0xF0).is_none();

        self.registers.f.set(Flags::Z, value == 0);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, half_carry);

        self.load_u8(target, value);
    }
    /// Subtracts from register A, the value represented by source, and updates flags based on the
    /// result. This instruction does not update the content of register A.
    ///
    fn cp(&mut self, source: Source8) {
        let value = self.get_source8(source);

        self.sub_u8(value);
    }
    /// Performs a bitwise operation OR between register A and the value represented by source, and
    /// stores the result back into register A.
    /// The Flag Register is affected as follows:
    /// Z: Set if the result is 0, otherwise reset
    /// N: Reset
    /// H: Reset
    /// C: Reset
    fn or(&mut self, source: Source8) {
        let value = self.get_source8(source);

        self.registers.a.bitor_assign(&value);

        self.registers.f.set(Flags::Z, self.registers.a == 0);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::C, false);
    }
    /// Performs a bitwise operation XOR between register A and the value represented by source, and
    /// stores the result back into register A.
    /// The Flag Register is affected as follows:
    /// Z: set if the result is 0, otherwise reset
    /// N: Reset
    /// H: Reset
    /// C: Reset
    fn xor(&mut self, source: Source8) {
        let value = self.get_source8(source);

        self.registers.a.bitxor_assign(&value);

        self.registers.f.set(Flags::Z, self.registers.a == 0);

        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::C, false);
    }

    /// Performs a bitwise AND operation between register A and the value represented by source, and
    /// stores the result back into register A
    /// The Flag Register is affected as follows:
    /// Z: set if the result is 0, otherwise reset
    /// N: Reset
    /// H: Set
    /// C: Reset
    fn and(&mut self, source: Source8) {
        let value = self.get_source8(source);

        self.registers.a.bitand_assign(&value);

        // Z: Set if the result is 0, otherwise reset
        self.registers.f.set(Flags::Z, self.registers.a == 0);

        // H: Set
        self.registers.f.set(Flags::H, true);

        // N: Reset
        self.registers.f.set(Flags::N, false);

        // C: Reset
        self.registers.f.set(Flags::C, false);
    }
    /// Adds to the 8-bit register the 8-bit value and updates FlagRegister as follows:.
    /// Z: Set if the result of the addition is 0, otherwise reset
    /// H: Set if there is a carry from bit3, otherwise reset
    /// N: Reset
    /// CY: Set if there is a carry from bit7, otherwise reset
    /// TODO: Explain how the half carry is computed
    fn add_u8_to_A(&mut self, value: u8) {
        let (result, overflow) = self.registers.a.overflowing_add(value);

        let half_carry = (self.registers.a & 0x0F)
            .checked_add(value | 0xF0)
            .is_none();

        self.registers.a = result;

        // Z: Set if the result is 0, otherwise reset
        self.registers.f.set(Flags::Z, result == 0);

        // H: Set if there is a carry from bit3; otherwise reset
        self.registers.f.set(Flags::H, half_carry);

        // N: Reset
        self.registers.f.set(Flags::N, false);

        // CY: Set if there is a carry from bit7; otherwise reset
        self.registers.f.set(Flags::C, overflow);
    }

    /// Adds to the 8-bit register A, the 8-bit of data represented by source, and stores the result
    /// back into register A.
    /// Source can be an 8-bit register, an address to an 8-bit data, or an immediate 8-bit data.
    /// The Flag register is affected as follows:
    /// Z: Set if the result of the addition is 0, otherwise reset
    /// H: Set if there is a carry from bit3, otherwise reset
    /// N: Reset
    /// CY: Set if there is a carry from bit7, otherwise reset
    fn add8(&mut self, source: Source8) {
        let value = self.get_source8(source);
        self.add_u8_to_A(value);
    }

    /// Adds to the 8-bit register A, the carry flag and the 8-bit of data represented by source, and stores the
    /// result back into register A.
    /// Source can be an 8-bit register, an address to an 8-bit data, or an immediate 8-bit data.
    /// The Flag register is affected as follows:
    /// Z: Set if the result of the addition is 0, otherwise reset
    /// H: Set if there is a carry from bit3, otherwise reset
    /// N: Reset
    /// CY: Set if there is a carry from bit7, otherwise reset
    fn adc8(&mut self, source: Source8) {
        let carry = 1; //TODO
        let value = self.get_source8(source);
        self.add_u8_to_A(carry + value);
    }

    /// Substracts u8 value from the 8-bit register A and returns the result
    /// The Flag register is affected as follows:
    /// Z: Set if the result is 0, otherwise reset
    /// H: Set if there is a borrow from bit4, otherwise reset
    /// N: Set
    /// CY: Set if there is a borrow, otherwise reset
    /// TODO: Explain how the half carry is computed
    fn sub_u8(&mut self, value: u8) -> u8 {
        let (result, overflow) = self.registers.a.overflowing_sub(value);

        let half_carry = (self.registers.a & 0x0F)
            .checked_sub(value | 0xF0)
            .is_none();

        // Z: Set if the result is 0, otherwise reset
        self.registers.f.set(Flags::Z, result == 0);

        // H: Set if there is a carry fromt bit3, otherwise reset
        self.registers.f.set(Flags::H, half_carry);

        // N: Set
        self.registers.f.set(Flags::N, true);

        // CY: Set if there is carry from bit7, otherwise reset
        self.registers.f.set(Flags::C, overflow);

        result
    }
    /// Substracts from the 8-bit register A, the 8-bit value represented by source and stores the
    /// result back into register A
    /// Source can be an 8-bit register, an address to an 8-bit data, or an immediate 8-bit data.
    /// The Flag register is affected as follows:
    /// Z: Set if the result is 0, otherwise reset
    /// H: Set if there is a borrow from bit4, otherwise reset
    /// N: Set
    /// CY: Set if there is a borrow, otherwise reset
    fn sub(&mut self, source: Source8) {
        let value = self.get_source8(source);
        self.registers.a = self.sub_u8(value);
    }

    fn sbc(&mut self, source: Source8) {
        let carry = self.registers.f.contains(Flags::C) as u8;

        let value = self.get_source8(source);

        let (result, overflow) = self.registers.a.overflowing_sub(value + carry);

        let half_carry = false; //TODO

        self.registers.a = result;

        // Z: Set if the result is 0, otherwise reset
        self.registers.f.set(Flags::Z, result == 0);

        // H: Set if there is a carry fromt bit3, otherwise reset
        self.registers.f.set(Flags::H, half_carry);

        // N: Set
        self.registers.f.set(Flags::N, true);

        // CY: Set if there is carry from bit7, otherwise reset
        self.registers.f.set(Flags::C, overflow);
    }

    /// Jump to the absolute address speicified by the 16-bit operand, depending on the condition
    /// Reads the 16-bit operand from immediate memory
    /// Update the value of PC with the operand
    /// Note that the operand is read even if the condition is false
    /// Unconditional jumps are also handled by this function, their condition is of type
    /// Condition::Always
    fn absolute_jump(&mut self, condition: Condition, source: Source16) {
        let address: u16 = match source {
            Source16::Imm16 => self.read_imm16(),
            Source16::HL => self.registers.read16(Register16::HL),
            _ => panic!("Not a valid Source16 for absolute jumps"),
        };

        /* if source == Source16::Imm16 {
            address = self.read_imm16();
        } else {
            address = self.registers.read16(Register16::HL);
        }*/

        if self.registers.f.check_condition(condition) {
            self.registers.pc = address;
        }
    }

    /// Jump to the relative address specified by the signed 8-bit operand, depending
    /// on condition
    /// Reads the 8-bit operand from immediate memory
    /// Adds operand to PC if the condition is checked
    /// Note that the operand is read even when the condition is false
    /// Unconditional relative jumps are also handled by this fonction, their condition is of type
    /// Condition::Always
    fn relative_jump(&mut self, condition: Condition) {
        let operand = self.read_imm8() as u16;

        if self.registers.f.check_condition(condition) {
            self.registers.pc += operand;
        }
    }

    /// 8-bit load instructions
    fn get_source8(&mut self, source: Source8) -> u8 {
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

    fn get_address(&mut self, addr: At) -> u16 {
        match addr {
            At::HL => self.registers.read16(Register16::HL),
            At::BC => self.registers.read16(Register16::BC),
            At::DE => self.registers.read16(Register16::DE),
            At::Imm16 => self.read_imm16(),
            At::Imm8 => 0xFF00 | self.read_imm8() as u16,
            At::C => 0xFF00 | self.registers.c as u16,
        }
    }

    fn load8(&mut self, destination: Target8, source: Source8) {
        let value = self.get_source8(source);
        self.load_u8(destination, value);
    }

    fn load_u8(&mut self, destination: Target8, value: u8) {
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
    fn get_source16(&mut self, source: Source16) -> u16 {
        match source {
            Source16::BC => self.registers.read16(Register16::BC),
            Source16::DE => self.registers.read16(Register16::DE),
            Source16::HL => self.registers.read16(Register16::HL),
            Source16::SP => self.registers.sp,
            Source16::Imm16 => self.read_imm16(),
            Source16::Imm8 => todo!(),
        }
    }
    fn load16(&mut self, destination: Target16, source: Source16) {
        let value = self.get_source16(source);
        self.load_u16(destination, value);
    }

    fn load_u16(&mut self, destination: Target16, value: u16) {
        match destination {
            Target16::BC => self.registers.write16(Register16::BC, value),
            Target16::DE => self.registers.write16(Register16::DE, value),
            Target16::HL => self.registers.write16(Register16::HL, value),
            Target16::SP => self.registers.sp = value,
            Target16::Addr => {
                let address = self.read_imm16();
                self.memory.write16(address, value);
            }
        }
    }

    fn push(&mut self, target: Register16) {
        let value = self.registers.read16(target);
        let [lo, hi] = u16::to_le_bytes(value);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write8(self.registers.sp, hi);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write8(self.registers.sp, lo);
    }

    fn pop(&mut self, target: Register16) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let lo = self.memory.read8(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let hi = self.memory.read8(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        self.registers
            .write16(target, u16::from_le_bytes([lo, hi]));
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
            State::Running => {
                let opcode = self.fetch();
                let instruction = Cpu::decode(opcode);
                self.execute(instruction);
            }
            State::Halt => {
                todo!();
            }
            State::Interrupt => {
                todo!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_get_value_from_source() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: 5,
                f: Flags::empty(),
                h: 6,
                l: 7,
                sp: 0,
                pc: 0,
            },
            state: State::Running,
            memory: Memory::new(vec![0; 10]),
        };
        assert_eq!(cpu.get_source8(Source8::A), 1);
        assert_eq!(cpu.get_source8(Source8::B), 2);
        assert_eq!(cpu.get_source8(Source8::C), 3);
        assert_eq!(cpu.get_source8(Source8::D), 4);
        assert_eq!(cpu.get_source8(Source8::E), 5);
        assert_eq!(cpu.get_source8(Source8::H), 6);
        assert_eq!(cpu.get_source8(Source8::L), 7);
        //TODO: check Imm8
    }
    #[test]
    fn should_read_addr_source() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: 5,
                f: Flags::empty(),
                h: 6,
                l: 7,
                sp: 0,
                pc: 0,
            },
            state: State::Running,
            memory: Memory::new(vec![8; 10]),
        };
        assert_eq!(cpu.get_address(At::BC), 0x0203);
        assert_eq!(cpu.get_address(At::HL), 0x0607);
        assert_eq!(cpu.get_address(At::DE), 0x0405);
        assert_eq!(cpu.get_address(At::C), 0xFF03);
        assert_eq!(cpu.get_address(At::Imm8), 0xFF08);
    }
}

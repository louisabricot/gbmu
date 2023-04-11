//! CPU routine and instructions.
//!
//! Implementation of the GameBoy's CPU, its registers and instructions.   

use self::registers::flags::Flags;
use self::registers::{Register16, Registers};
use super::memory::MemoryMap;
use crate::hardware::cpu::instructions::{
    At, Bit, Condition, Imm, Instruction, Opcode, Operand16, Operand8, Operation, Page0,
};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign};
pub mod fetch;

#[allow(dead_code)]
pub mod instructions;
pub mod registers;

pub struct Cpu {
    /// General-purpose registers, Program Counter and Stack Pointer
    pub registers: Registers,

    /// CPU state
    pub state: State,

    /// TODO
    pub memory: MemoryMap,
}

/// CPU states
pub enum State {
    /// Describes the default state during which the boot ROM is mapped to the
    /// `0x0000 - 0x00FF` area, over the cartridge. When the system initializes
    /// or resets, the CPU starts execution from `0x0000`, it executes the boot
    /// ROM.  
    /// In this state, all reads from `0x0000-0x00FF` are handled by the boot ROM
    /// and all writes to this area are ignored.  
    Booting,

    /// TODO
    Running,

    /// TODO
    Halt,

    /// TODO
    Stop,

    /// TODO
    Interrupt,
}

impl Cpu {
    /// Initializes CPU with default values
    pub fn new(memory: MemoryMap) -> Self {
        Self {
            registers: Registers::new(),
            state: State::Booting,
            memory,
        }
    }

    /// Sets the `Program Counter` to *pc*.  
    pub fn set_program_counter(&mut self, pc: u16) {
        self.registers.pc = pc
    }
    /// Reads from the 8-bit immediate value from `Program Counter`.  
    /// Increments the `Program Counter` by 1.  
    fn read_imm8(&mut self) -> u8 {
        let byte = self.memory.read8(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    /// Reads from the 16-bit immediate value from `Program Counter`.  
    /// Increments the `Program Counter` by 2.  
    fn read_imm16(&mut self) -> u16 {
        let word = self.memory.read16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        word
    }

    /// Returns the `Instruction` matching *opcode*.  
    /// If the opcode does not match any instructions, the function panics.  
    fn decode(opcode: Opcode) -> Instruction {
        let instruction = Instruction::get_by_opcode(opcode);
        match instruction {
            None => panic!("No Instruction found for opcode, this should never happend"),
            _ => instruction.unwrap(),
        }
    }

    /// TODO
    pub fn step(&mut self) {
        match self.state {
            State::Running => {
                let (opcode, size) = match self.fetch(self.registers.pc) {
                    Ok(t) => t,
                    Err(msg) => panic!("{}", msg),
                };
                self.registers.pc += size;
                let instruction = Cpu::decode(opcode);
                self.execute(instruction);
            }
            State::Halt => {
                todo!();
            }
            State::Interrupt => {
                todo!();
            }
            State::Stop => {
                todo!();
            }
            State::Booting => {
                todo!();
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.operation {
            Operation::Load8(dst, src) => self.load8(dst, src),
            Operation::Load8Dec(dst, src) => self.load8dec(dst, src),
            Operation::Load8Inc(dst, src) => self.load8inc(dst, src),

            Operation::Load16(dst, src) => self.load16(dst, src),
            Operation::Push(target) => self.push(target),
            Operation::Pop(target) => self.pop(target),

            Operation::Add8(source) => self.add8(source),
            Operation::Adc(source) => self.adc(source),
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

            Operation::AddHL_r16(source) => self.add_hl_r16(source),
            Operation::Inc16(target) => self.inc16(target),
            Operation::Dec16(target) => self.dec16(target),
            Operation::AddSP_dd => self.add_sp_dd(),
            Operation::LoadHL => self.load_hl(),

            Operation::Rlca => self.rlca(),
            Operation::Rla => self.rla(),
            Operation::Rrca => self.rrca(),
            Operation::Rlc(target) => self.rlc(target),
            Operation::Rl(target) => self.rl(target),
            Operation::Rrc(target) => self.rrc(target),
            Operation::Rr(target) => self.rr(target),
            Operation::Sla(target) => self.sla(target),
            Operation::Sra(target) => self.sra(target),
            Operation::Srl(target) => self.srl(target),
            Operation::Swap(target) => self.swap(target),

            Operation::Bit(bit, target) => self.bit(bit, target),
            Operation::Set(bit, target) => self.set(bit, target),
            Operation::Res(bit, target) => self.res(bit, target),

            Operation::Ccf => self.ccf(),
            Operation::Scf => self.scf(),
            Operation::Nop => self.nop(),
            Operation::Halt => self.halt(),
            Operation::Stop => self.stop(),
            Operation::Di => self.di(),
            Operation::Ei => self.ei(),
            Operation::Jp(condition, source) => self.jp(condition, source),
            Operation::Jr(condition) => self.jr(condition),
            Operation::Call(condition, source) => self.call(condition, source),
            Operation::Ret(condition) => self.ret(condition),
            Operation::Reti => self.reti(),
            Operation::Rst(page) => self.rst(page),
        }
    }

    // 8-bit load instructions

    /// Loads the value represented by *source* into *destination*.  
    /// This function calls `get_operand8` to read the value from source and `load_u8` to load it
    /// into *destination*.  
    /// `Flag Register` is not affected.  
    fn load8(&mut self, destination: Operand8, source: Operand8) {
        let value = self.get_operand8(source);
        self.load_u8(destination, value);
    }

    /// Loads the 8-bit *data* into *destination*.  
    /// If *destination* is either the 8-bit register `A`, `B`, `C`, `D`, `E`, `H`, `L`
    /// or an address (represented by `Addr(at)`), data is loaded.
    /// Otherwise, the function panics.
    fn load_u8(&mut self, destination: Operand8, data: u8) {
        match destination {
            Operand8::A => self.registers.a = data,
            Operand8::B => self.registers.b = data,
            Operand8::C => self.registers.c = data,
            Operand8::D => self.registers.d = data,
            Operand8::E => self.registers.e = data,
            Operand8::H => self.registers.h = data,
            Operand8::L => self.registers.l = data,
            Operand8::Addr(at) => {
                let address = self.get_address(at);
                self.memory.write8(address, data);
            }
            _ => panic!("Not a valid Operand8 for load_u8()"),
        }
    }

    /// Loads the value of *source* into *destination* and decrements the value of the 16-bit
    /// register `HL`.  
    fn load8dec(&mut self, destination: Operand8, source: Operand8) {
        self.load8(destination, source);
        let new_value = self.registers.read16(Register16::HL).wrapping_sub(1);
        self.registers.write16(Register16::HL, new_value);
    }

    /// Loads the value of *source* into *destination* and increments the value of the 16-bit
    /// register `HL`.  
    fn load8inc(&mut self, destination: Operand8, source: Operand8) {
        self.load8(destination, source);
        let new_value = self.registers.read16(Register16::HL).wrapping_add(1);
        self.registers.write16(Register16::HL, new_value);
    }

    // 16-bit load instructions

    /// Loads the 16-bit *source* into *destination*.  
    /// If *destination* is either the 16-bit register `BC`, `DE`, `HL`, `SP`
    /// or an address (represented by `Addr(at)`), data is loaded.
    /// Otherwise, the function panics.
    fn load16(&mut self, destination: Operand16, source: Operand16) {
        let data = self.get_operand16(source);
        match destination {
            Operand16::BC => self.registers.write16(Register16::BC, data),
            Operand16::DE => self.registers.write16(Register16::DE, data),
            Operand16::HL => self.registers.write16(Register16::HL, data),
            Operand16::SP => self.registers.sp = data,
            Operand16::Addr(at) => {
                let address = self.get_address(at);
                self.memory.write16(address, data);
            }
            _ => panic!("Not a valid Operand16 for load16()"),
        }
    }

    /// Pushes to the stack memory, the 16-bit *source*.  
    /// Before and after writing to memory, the stack pointer `sp` is decremented.  
    /// If *source* is not one of the 16-bit registers `AF`, `BC`, `DE` or `HL`, the function
    /// `Registers::get_register16()` panics.
    fn push(&mut self, source: Operand16) {
        let value = self.registers.read16(Registers::get_register16(source));

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write16(self.registers.sp, value);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    /// Pops to the 16-bit register *target*, 16-bit of data pointed to by stack pointer `sp`.  
    /// After reading the stack memory, `sp` is incremented by 2.  
    /// If `Operand16` is not a 16-bit register (`AF`, `BC`, `DE`, or `HL`), the function
    /// `Registers::get_register16()` panics.
    fn pop(&mut self, target: Operand16) {
        let value = self.memory.read16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);

        self.registers
            .write16(Registers::get_register16(target), value);
    }

    // 8-bit arithmetic and logic instructions

    /// Flips all the bits in the 8-bit register `A`.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Not affected  
    /// `N`: Set  
    /// `H`: Set  
    /// `C`: Not affected  
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.set(Flags::N, true);
        self.registers.f.set(Flags::H, true);
    }

    /// The DAA instruction adjusts the result of a binary addition or substraction to obtain the
    /// Binary Coded Decimal representation.
    /// `Flag Register` is updated as follows:
    /// `Z`: Set if the result is 0, otherwise reset
    /// `N`: Not affected
    /// `H`: Reset
    /// `C`: Set if there is carry from bit7, otherwise reset
    fn daa(&mut self) {
        if self.registers.f.contains(Flags::N) {
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

    /// Decrements the content of `target` by 1.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Set  
    /// `H`: Set if there is a carry from bit3, otherwise reset  
    /// `C`: Not affected
    fn dec8(&mut self, target: Operand8) {
        let value = self.get_operand8(target);

        let half_carry = (value & 0x0F).checked_sub(1).is_none();

        let result = value.wrapping_sub(1);

        self.registers.f.set(Flags::Z, result == 0);
        self.registers.f.set(Flags::N, true);
        self.registers.f.set(Flags::H, half_carry);

        self.load_u8(target, result);
    }

    /// Increments data represented by `target` by 1.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Set if there is a carry from bit3, otherwise reset  
    /// `C`: Not affected  
    fn inc8(&mut self, target: Operand8) {
        let value: u8 = self.get_operand8(target);

        let result = value.wrapping_add(1);

        let half_carry = (value & 0x0F).checked_add(1 | 0xF0).is_none();

        self.registers.f.set(Flags::Z, result == 0);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, half_carry);

        self.load_u8(target, result);
    }

    /// Subtracts *source* from the 8-bit register `A` without updating the content of `A`.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Set   
    /// `H`: Set if there is a carry from bit3, otherwise reset  
    /// `C`: Set if there is a carry from bit7, otherwise reset  
    fn cp(&mut self, source: Operand8) {
        let value = self.get_operand8(source);

        self.sub_u8(value);
    }

    /// Performs a bitwise operation OR between 8-bit register `A` and *source*, and
    /// stores the result back into `A`.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Reset  
    fn or(&mut self, source: Operand8) {
        let value = self.get_operand8(source);

        self.registers.a.bitor_assign(&value);

        self.registers.f.set(Flags::Z, self.registers.a == 0);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::C, false);
    }

    /// Performs a bitwise operation XOR between 8-bit register `A` and *source*, and
    /// stores the result back into `A`.
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Reset  
    fn xor(&mut self, source: Operand8) {
        let value = self.get_operand8(source);

        self.registers.a.bitxor_assign(&value);

        self.registers.f.set(Flags::Z, self.registers.a == 0);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::C, false);
    }

    /// Performs a bitwise AND operation between the 8-bit register `A` and *source*, and
    /// stores the result back into `A`.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Set  
    /// `C`: Reset  
    fn and(&mut self, source: Operand8) {
        let value = self.get_operand8(source);

        self.registers.a.bitand_assign(&value);
        self.registers.f.set(Flags::Z, self.registers.a == 0);
        self.registers.f.set(Flags::H, true);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::C, false);
    }

    /// Adds *data* to the 8-bit register `A` and stores the result back into `A`.  
    /// `FlagRegister` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `H`: Set if there is a carry from bit3, otherwise reset  
    /// `N`: Reset  
    /// `C`: Set if there is a carry from bit7, otherwise reset  
    fn add_u8_to_a(&mut self, data: u8) {
        let (result, overflow) = self.registers.a.overflowing_add(data);
        let half_carry = (self.registers.a & 0x0F).checked_add(data | 0xF0).is_none();

        self.registers.a = result;
        self.registers.f.set(Flags::Z, result == 0);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::C, overflow);
    }

    /// Adds *source* to the 8-bit register `A`, and stores the result
    /// back into `A`.  
    /// Calls `add_u8_to_a` with the value returned by `get_operand8`.  
    fn add8(&mut self, source: Operand8) {
        let value = self.get_operand8(source);
        self.add_u8_to_a(value);
    }

    /// Adds *source* and the `carry flag` to the 8-bit register `A`, and stores the result
    /// back into `A`.  
    /// Calls `add_u8_to_a` with the value returned by `get_operand8` and the `carry` flag.  
    fn adc(&mut self, source: Operand8) {
        let carry = self.registers.f.contains(Flags::C) as u8;
        let value = self.get_operand8(source);
        self.add_u8_to_a(carry + value);
    }

    /// Substracts the 8-bit *data* from the 8-bit register `A` and returns the result.  
    /// `FlagRegister` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Set  
    /// `H`: Set if there is a carry from bit3, otherwise reset  
    /// `C`: Set if there is a carry from bit7, otherwise reset  
    fn sub_u8(&mut self, data: u8) -> u8 {
        let (result, overflow) = self.registers.a.overflowing_sub(data);

        let half_carry = (self.registers.a & 0x0F).checked_sub(data & 0x0F).is_none();

        self.registers.f.set(Flags::Z, result == 0);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::N, true);
        self.registers.f.set(Flags::C, overflow);
        result
    }

    /// Substracts *source* from the 8-bit register `A` and stores the
    /// result back into `A`.  
    /// Calls `sub_u8` with the value returned by `get_operand8`.  
    fn sub(&mut self, source: Operand8) {
        let value = self.get_operand8(source);
        self.registers.a = self.sub_u8(value);
    }

    /// Substracts *source* and the `carry` flag from the 8-bit register `A` and stores the
    /// result back into `A`.  
    /// Calls `sub_u8` with the value returned by `get_operand8` and the `carry` flag.  
    fn sbc(&mut self, source: Operand8) {
        let carry = self.registers.f.contains(Flags::C) as u8;
        let value = self.get_operand8(source);
        self.registers.a = self.sub_u8(value + carry);
    }

    // 16-bit arithmetic and logic instructions

    /// Loads the sum of `SP` and the 8-bit immediate value to the 16-bit register `HL`.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Reset  
    /// `N`: Reset  
    /// `H`: Set if there is a carry from bit 11, otherwise reset  
    /// `C`: Set if there is a carry from bit 15, otherwise reset  
    fn load_hl(&mut self) {
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

    /// Add to `Stack Pointer` the 8-bit immediate value.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Reset  
    /// `N`: Reset  
    /// `H`: Set if there is a carry on bit7, otherwise reset  
    /// `C`: Set if there is a carry on bit15, otherwise reset  
    fn add_sp_dd(&mut self) {
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

    /// Increments data represented by `target` by 1.  
    /// `Flag Register` is not updated.  
    fn inc16(&mut self, target: Operand16) {
        let mut value = match target {
            Operand16::SP => self.registers.sp,
            _ => self.registers.read16(Registers::get_register16(target)),
        };
        value = value.wrapping_add(1);
        self.registers
            .write16(Registers::get_register16(target), value);
    }

    /// Decrements data represented by `target` by 1.  
    /// `Flag Register` is not updated.  
    fn dec16(&mut self, target: Operand16) {
        let mut value = match target {
            Operand16::SP => self.registers.sp,
            _ => self.registers.read16(Registers::get_register16(target)),
        };
        value = value.wrapping_sub(1);
        self.registers
            .write16(Registers::get_register16(target), value);
    }

    /// Adds *source* to 16-bit register `HL`.  
    /// `Flag Register` are updated as follows:  
    /// `Z`: Not affected  
    /// `H`: Set if there is a carry from bit11, otherwise reset  
    /// `N`: Reset  
    /// `C`: Set if there is a carry from bit5, otherwise reset  
    fn add_hl_r16(&mut self, source: Operand16) {
        let value = self.registers.read16(Registers::get_register16(source));
        let target = self.registers.read16(Register16::HL);

        let (result, carry) = u16::overflowing_add(target, value);
        let half_carry = u8::checked_add((target & 0x00FF) as u8, (value & 0x00FF) as u8).is_none();

        self.registers.write16(Register16::HL, result);
        self.registers.f.set(Flags::N, false);
        self.registers.f.set(Flags::H, half_carry);
        self.registers.f.set(Flags::C, carry);
    }

    // Rotate and shift instructions

    /// Swap the lower and higher nibbles of the value represented by *target*.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Reset  
    fn swap(&mut self, target: Operand8) {
        let value = self.get_operand8(target);

        let swapped = value >> 4 | value << 4;

        self.load_u8(target, swapped);

        self.registers.f.set(Flags::C, false);
        self.registers.f.set(Flags::Z, swapped == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the right.  
    /// Copies bit0 to the `carry` flag and resets bit7.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit0 is 1 before the rotation, otherwise reset  
    fn srl(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit0 = value & 1;
        let new_value = value >> 1;

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit0 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the right.  
    /// Copies bit0 to the `carry` flag and preserves bit7.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit0 is 1 before the rotation, otherwise reset  
    fn sra(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit0 = value & 1;
        let bit7 = value >> 7;
        let new_value = value >> 1 | bit7 << 7;

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit0 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the left.  
    /// Copies bit7 into the `carry` flag and resets bit0.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit7 is 1 before the rotation, otherwise reset  
    fn sla(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit7 = value >> 7;
        let new_value = value << 1;

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit7 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the right.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit0 is 1 before the rotation, otherwise reset  
    fn rr(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit0 = value & 1;
        let new_value = value >> 1;

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit0 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the right.  
    /// Wraps the truncated bit to the end of the resulting integer.   
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit0 is 1 before the rotation, otherwise reset  
    fn rrc(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit0 = value & 1;
        let new_value = value.rotate_right(1);

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit0 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the left.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit7 is 1 before the rotation, otherwise reset  
    fn rl(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit7 = value >> 7;
        let new_value = value << 1;

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit7 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of *target* to the left.  
    /// Wraps the truncated bit to the begining of the resulting integer.   
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the result is 0, otherwise reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit7 is 1 before the rotation, otherwise reset  
    fn rlc(&mut self, target: Operand8) {
        let value = self.get_operand8(target);
        let bit7 = value >> 7;
        let new_value = value.rotate_left(1);

        self.load_u8(target, new_value);

        self.registers.f.set(Flags::C, bit7 == 1);
        self.registers.f.set(Flags::Z, new_value == 0);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of the 8-bit register `A` to the right.   
    /// Places the content of bit0 both in the `carry` flag and bit 7.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit0 is 1 before the rotation, otherwise reset  
    fn rrca(&mut self) {
        let bit0 = self.registers.a & 1;

        self.registers.a = self.registers.a.rotate_right(1);
        self.registers.f.set(Flags::C, bit0 == 1);
        self.registers.f.set(Flags::Z, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of the 8-bit register `A` to the left.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit7 is 1 before the rotation, otherwise reset  
    fn rla(&mut self) {
        let bit7 = self.registers.a >> 7;

        self.registers.a = self.registers.a.rotate_left(1);

        self.registers.f.set(Flags::C, bit7 == 1);
        self.registers.f.set(Flags::Z, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Rotates the content of the 8-bit register `A` to the left.  
    /// Places the content of bit7 both in the `carry` flag and bit 0.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Reset  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Set if bit7 is 1 before the rotation  
    fn rlca(&mut self) {
        let bit7 = self.registers.a >> 7;

        self.registers.a = self.registers.a.rotate_left(1);

        self.registers.f.set(Flags::C, bit7 == 1);
        self.registers.f.set(Flags::Z, false);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    // Single-bit operation instructions

    /// Copies the bit specified by *bit* in *target* to the `zero` flag.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Set if the specified bit is zero, otherwise reset  
    /// `N`: Reset  
    /// `H`: Set  
    /// `C`: Not affected  
    fn bit(&mut self, bit: Bit, target: Operand8) {
        let value = self.get_operand8(target);
        let bit = value.bitand(bit as u8);
        self.registers.f.set(Flags::Z, bit == 0);
        self.registers.f.set(Flags::H, true);
        self.registers.f.set(Flags::N, false);
    }

    /// Sets the bit specified by *bit* in *target* to 1.  
    /// `Flag Register` is not updated.  
    fn set(&mut self, bit: Bit, target: Operand8) {
        let value = self.get_operand8(target);
        let new_value = value | (bit as u8);
        self.load_u8(target, new_value);
    }

    /// Resets the bit specified by *bit* in *target* to 0.  
    /// `Flag Register` is not updated.  
    fn res(&mut self, bit: Bit, target: Operand8) {
        let value = self.get_operand8(target);
        let new_value = value ^ (bit as u8);
        self.load_u8(target, new_value);
    }

    // CPU Control instructions

    /// Toggles the `carry` flag.  
    /// If `carry` flag is set, then reset it.  
    /// If `carry` flag is reset, then set it.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Not affected  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Toggled  
    fn ccf(&mut self) {
        self.registers.f.toggle(Flags::C);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Sets the `carry` flag.  
    /// `Flag Register` is updated as follows:  
    /// `Z`: Not affected  
    /// `N`: Reset  
    /// `H`: Reset  
    /// `C`: Toggled  
    fn scf(&mut self) {
        self.registers.f.set(Flags::C, true);
        self.registers.f.set(Flags::H, false);
        self.registers.f.set(Flags::N, false);
    }

    /// Stops the system clock and enters HALT mode.  
    /// HALT mode is canceled by an interrupt or a reset signal.  
    /// Although the system clock is stopped in this state, the oscillator circuit and LCD
    /// controller continue to operate.  
    fn halt(&mut self) {
        self.state = State::Halt;
    }

    /// Does nothing.  
    fn nop(&mut self) {}
    /// Stops both the system clock and the oscillator circuit.  
    /// Stop mode stops the LCD controller.  
    /// Stop mode is canceled by a reset signal.
    fn stop(&mut self) {
        self.state = State::Stop;
    }

    /// Disables interrupts
    fn di(&mut self) {}

    /// Enables interrupts
    fn ei(&mut self) {}

    // Jump instructions

    /// Loads the value of *source* to the `Program Counter` if *condition* is true.  
    /// `Flag Register` is not updated.  
    fn jp(&mut self, condition: Condition, source: Operand16) {
        let address = self.get_operand16(source);
        if self.registers.f.check_condition(condition) {
            self.registers.pc = address;
        }
    }

    /// Adds the 8-bit immediate value to the `Program Counter` if *condition* is true.  
    /// `Flag Register` is not updated.  
    fn jr(&mut self, condition: Condition) {
        let steps = self.read_imm8() as u16;
        if self.registers.f.check_condition(condition) {
            self.registers.pc += steps;
        }
    }

    /// If *condition* is true, pushes `Program Counter` of the next instruction on the stack and loads *source* onto `Program Counter`.  
    /// `Flag Register` is not updated.  
    fn call(&mut self, condition: Condition, source: Operand16) {
        let opcode_len = if condition == Condition::Always { 3 } else { 5 };
        let address = self.get_operand16(source);
        if self.registers.f.check_condition(condition) {
            self.registers.sp = self.registers.sp.wrapping_sub(2);
            self.memory
                .write16(self.registers.sp, self.registers.pc + opcode_len);
            self.registers.pc = address;
        }
    }

    /// If *condition* is true, pops from the memory stack the 16-bit value and loads that value
    /// onto `Program Counter`.  
    /// `Flag Register` is not updated.  
    fn ret(&mut self, condition: Condition) {
        let address = self.memory.read16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        if self.registers.f.check_condition(condition) {
            self.registers.pc = address;
        }
    }

    /// Pops the 16-bit value on the top of memory stack and loads it onto `Program Counter`.  
    /// Enables the Master Interrupt flag.  
    fn reti(&mut self) {
        let address = self.memory.read16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.registers.pc = address;
        //ime = 1;
    }

    /// Loads the `Program Counter` into the memory stack and loads the page0 memory address onto
    /// `Program Counter`.
    fn rst(&mut self, address: Page0) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.memory.write16(self.registers.sp, self.registers.pc);
        self.registers.pc = address as u16;
    }

    /// Returns the 16-bit data stored at address represented by *addr*.  
    /// `At` can be the 16-bit register `HL`, `BC`, or `DE`,  
    /// a 16-bit immediate data (`Imm16`) or  
    /// an 8-bit data (`Imm8` or 8-bit register `C`).  
    /// When *addr* represents an 8-bit data, the returned value is obtained by
    /// setting the most significant byte to `0xFF` and the least significant to the 8-bit data to
    /// form an address in the range `0xFF00-0xFFFF`.
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

    /// Returns the 8-bit data represented by *operand*.  
    /// `Operand8` is either a 8-bit register (`A`, `B`, `C`, `D`, `E`, `H`, `L`),  
    /// an 8-bit immediate data (`Imm8`) or  
    /// an 8-bit data stored at location (`Addr(at)` where `at` represents the location).
    fn get_operand8(&mut self, operand: Operand8) -> u8 {
        match operand {
            Operand8::A => self.registers.a,
            Operand8::B => self.registers.b,
            Operand8::C => self.registers.c,
            Operand8::D => self.registers.d,
            Operand8::E => self.registers.e,
            Operand8::H => self.registers.h,
            Operand8::L => self.registers.l,
            Operand8::Imm8 => self.read_imm8(),
            Operand8::Addr(at) => {
                let address = self.get_address(at);
                self.memory.read8(address)
            }
        }
    }

    /// Returns the 16-bit data represented by *operand*.  
    /// `Operand16` is either a 16-bit register (`AF`, `BC`, `DE`, `HL`, `SP`),
    /// a 16-bit or 8-bit immediate data (`Imm16` or `Imm8`) or,
    /// the 16-bit data stored at location (`Addr(at)` where `at` represents the location).
    /// When *operand* represents an 8-bit data, the returned value is obtained by setting the most
    /// significant byte to `0x00` and the least significant byte to the 8-bit data to form a
    /// 16-bit value in the range `0x0000-0x00FF`.
    fn get_operand16(&mut self, operand: Operand16) -> u16 {
        match operand {
            Operand16::AF => self.registers.read16(Register16::AF),
            Operand16::BC => self.registers.read16(Register16::BC),
            Operand16::DE => self.registers.read16(Register16::DE),
            Operand16::HL => self.registers.read16(Register16::HL),
            Operand16::SP => self.registers.sp,
            Operand16::Imm16 => self.read_imm16(),
            Operand16::Imm8 => self.read_imm8() as u16,
            Operand16::Addr(at) => {
                let address = self.get_address(at);
                self.memory.read16(address)
            }
        }
    }

    /// TODO
    fn format_instruction(&self, imm: Imm, mnemonic: &str, address: u16) -> (String, u16) {
        let (value, size) = match imm {
            Imm::Eight => (format!("{:#04x}", self.memory.read8(address)), 1),
            Imm::Sixteen => (format!("{:#08x}", self.memory.read16(address)), 2),
        };

        (mnemonic.replace("imm", &value), size)
    }

    /// TODO
    pub fn disassemble(&self, lines: u16, mut address: u16) -> Vec<String> {
        let mut mnemonics = Vec::new();

        for i in 0..lines {
            let mnemonic: String = match self.fetch(address) {
                Ok((opcode, size)) => {
                    let instruction = Cpu::decode(opcode);
                    address += size;
                    let (mnemonic, size) = match instruction.operand {
                        Some(imm) => self.format_instruction(imm, instruction.mnemonic, address),
                        None => (instruction.mnemonic.to_string(), 0),
                    };
                    address += size;
                    mnemonic
                }
                Err(..) => {
                    address += 1;
                    "BAADD".to_string()
                }
            };
            mnemonics.push(mnemonic);
        }
        mnemonics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_operand8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 17,
                b: 62,
                c: 53,
                d: 43,
                e: 145,
                f: Flags::empty(),
                h: 0,
                l: 7,
                sp: 1,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        assert_eq!(cpu.get_operand8(Operand8::A), cpu.registers.a);
        assert_eq!(cpu.get_operand8(Operand8::E), cpu.registers.e);
        assert_eq!(cpu.get_operand8(Operand8::Imm8), cpu.memory.read8(0));
        assert_eq!(
            cpu.get_operand8(Operand8::Addr(At::HL)),
            cpu.memory.read8(cpu.registers.l as u16)
        );
    }

    #[test]
    fn test_get_operand16() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 17,
                b: 62,
                c: 53,
                d: 43,
                e: 145,
                f: Flags::empty(),
                h: 0,
                l: 7,
                sp: 1,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        assert_eq!(
            cpu.get_operand16(Operand16::AF),
            cpu.registers.read16(Register16::AF)
        );
        assert_eq!(
            cpu.get_operand16(Operand16::BC),
            cpu.registers.read16(Register16::BC)
        );
        assert_eq!(cpu.get_operand16(Operand16::Imm8), 10);
        assert_eq!(
            cpu.get_operand16(Operand16::Imm16),
            cpu.memory.read16(cpu.registers.pc - 2)
        );
        assert_eq!(
            cpu.get_operand16(Operand16::Addr(At::HL)),
            cpu.memory.read16(cpu.registers.l as u16)
        );
    }

    #[test]
    fn test_get_address() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 17,
                b: 62,
                c: 53,
                d: 43,
                e: 145,
                f: Flags::empty(),
                h: 0,
                l: 7,
                sp: 1,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        assert_eq!(
            cpu.get_address(At::BC),
            (cpu.registers.b as u16) << u8::BITS | cpu.registers.c as u16
        );
        assert_eq!(cpu.get_address(At::C), cpu.registers.c as u16 | 0xFF00);
        assert_eq!(
            cpu.get_address(At::Imm8),
            0xFF00 | cpu.memory.read8(0) as u16
        );
        assert_eq!(cpu.get_address(At::Imm16), cpu.memory.read16(1));
    }

    #[test]
    fn test_pop() {
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
            memory: MemoryMap::new(vec![0, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        cpu.pop(Operand16::BC);
        assert_eq!(cpu.registers.read16(Register16::BC), cpu.memory.read16(0));
        cpu.pop(Operand16::HL);
        assert_eq!(cpu.registers.read16(Register16::HL), cpu.memory.read16(2));
        cpu.pop(Operand16::DE);
        assert_eq!(cpu.registers.read16(Register16::DE), cpu.memory.read16(4));
        cpu.pop(Operand16::AF);
        assert_eq!(
            cpu.registers.read16(Register16::AF),
            cpu.memory.read16(6) & 0xFFF0,
        );
    }

    #[test]
    #[should_panic(expected = "Not a pair of 8-bit registers")]
    fn test_pop_with_invalid_operand16() {
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
            memory: MemoryMap::new(vec![0, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        cpu.pop(Operand16::SP);
    }

    #[test]
    fn test_push() {
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
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![0, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.push(Operand16::BC);
        assert_eq!(cpu.registers.read16(Register16::BC), cpu.memory.read16(10));
        cpu.push(Operand16::HL);
        assert_eq!(cpu.registers.read16(Register16::HL), cpu.memory.read16(8));
        cpu.push(Operand16::DE);
        assert_eq!(cpu.registers.read16(Register16::DE), cpu.memory.read16(6));
        cpu.push(Operand16::AF);
        assert_eq!(cpu.registers.read16(Register16::AF), cpu.memory.read16(4));
    }
    #[test]
    #[should_panic(expected = "Not a pair of 8-bit registers")]
    fn test_push_with_invalid_operand16() {
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
            memory: MemoryMap::new(vec![0, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43]),
        };
        cpu.push(Operand16::SP);
    }

    #[test]
    fn test_load16() {
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
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.load16(Operand16::BC, Operand16::SP);
        assert_eq!(cpu.registers.read16(Register16::BC), cpu.registers.sp);

        cpu.load16(Operand16::HL, Operand16::Imm16);
        assert_eq!(cpu.registers.read16(Register16::HL), cpu.memory.read16(0));

        cpu.load16(Operand16::DE, Operand16::Imm8);
        assert_eq!(
            cpu.registers.read16(Register16::DE),
            cpu.memory.read8(2) as u16
        );
    }

    #[test]
    #[should_panic(expected = "Not a valid Operand16 for load16()")]
    fn test_load16_with_invalid_operand() {
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
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.load16(Operand16::AF, Operand16::AF);
    }

    #[test]
    fn test_load_u8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.load_u8(Operand8::L, cpu.memory.read8(0));
        assert_eq!(cpu.registers.l, 10);

        cpu.load_u8(Operand8::A, cpu.memory.read8(1));
        assert_eq!(cpu.registers.a, 255);

        cpu.load_u8(Operand8::E, cpu.registers.l);
        assert_eq!(cpu.registers.e, cpu.registers.l);

        cpu.load_u8(Operand8::Addr(At::HL), cpu.memory.read8(4));
        assert_eq!(cpu.memory.read8(3), 239);
    }

    #[test]
    #[should_panic(expected = "Not a valid Operand8 for load_u8()")]
    fn test_load_u8_with_invalid_operand() {
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
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.load_u8(Operand8::Imm8, cpu.memory.read8(0));
    }

    #[test]
    fn test_dec8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.dec8(Operand8::A);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.dec8(Operand8::D);
        assert_eq!(cpu.registers.d, u8::MAX);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.dec8(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read8(3), 238);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.dec8(Operand8::E);
        assert_eq!(cpu.registers.e, 15);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_inc8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 15,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.inc8(Operand8::A);
        assert_eq!(cpu.registers.a, 2);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.inc8(Operand8::D);
        assert_eq!(cpu.registers.d, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.inc8(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read8(3), 240);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.inc8(Operand8::E);
        assert_eq!(cpu.registers.e, 16);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_cp() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.cp(Operand8::A);
        assert_eq!(cpu.registers.a, 10);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.cp(Operand8::D);
        assert_eq!(cpu.registers.a, 10);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.cp(Operand8::Addr(At::HL));
        assert_eq!(cpu.registers.a, 10);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.cp(Operand8::E);
        assert_eq!(cpu.registers.a, 10);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_or() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 10,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.or(Operand8::A);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.or(Operand8::E);
        assert_eq!(cpu.registers.a, 5);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.or(Operand8::H);
        assert_eq!(cpu.registers.a, 15);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }
    #[test]
    fn test_xor() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 5,
                b: 1,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 7,
                l: 2,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.xor(Operand8::B);
        assert_eq!(cpu.registers.a, 4);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.xor(Operand8::C);
        assert_eq!(cpu.registers.a, 7);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.xor(Operand8::H);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_and() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0xff,
                b: 1,
                c: 0,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 7,
                l: 2,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.and(Operand8::H);
        assert_eq!(cpu.registers.a, 7);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.and(Operand8::E);
        assert_eq!(cpu.registers.a, 5);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.and(Operand8::C);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_sub_u8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        assert_eq!(cpu.sub_u8(0xff), 11);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        assert_eq!(cpu.sub_u8(3), 7);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        assert_eq!(cpu.sub_u8(239), 27);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        assert_eq!(cpu.sub_u8(5), 5);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }
    #[test]
    fn test_sbc() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.sbc(Operand8::D);
        assert_eq!(cpu.registers.a, 11);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.sbc(Operand8::L);
        assert_eq!(cpu.registers.a, 7);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.sbc(Operand8::Addr(At::HL));
        assert_eq!(cpu.registers.a, 24);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.sbc(Operand8::E);
        assert_eq!(cpu.registers.a, 18);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }
    #[test]
    fn test_add_u8_to_a() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.add_u8_to_a(2);
        assert_eq!(cpu.registers.a, 12);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.add_u8_to_a(0xff);
        assert_eq!(cpu.registers.a, 11);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.registers.a = 0;
        cpu.add_u8_to_a(0);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_add8() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.add8(Operand8::B);
        assert_eq!(cpu.registers.a, 12);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.add8(Operand8::D);
        assert_eq!(cpu.registers.a, 11);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.add8(Operand8::Addr(At::HL));
        assert_eq!(cpu.registers.a, 250);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_adc() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.adc(Operand8::B);
        assert_eq!(cpu.registers.a, 12);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.adc(Operand8::D);
        assert_eq!(cpu.registers.a, 11);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.adc(Operand8::Addr(At::HL));
        assert_eq!(cpu.registers.a, 251);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_cpl() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 10,
                b: 2,
                c: 3,
                d: 0xFF,
                e: 5,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.cpl();
        assert_eq!(cpu.registers.a, 0b11110101);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.registers.a = 0;
        cpu.cpl();
        assert_eq!(cpu.registers.a, 0xff);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.cpl();
        assert_eq!(cpu.registers.a, 0x0);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_dec16() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
        cpu.dec16(Operand16::AF);
        assert_eq!(cpu.registers.read16(Register16::AF), 240);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.dec16(Operand16::DE);
        assert_eq!(cpu.registers.read16(Register16::DE), 15);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_inc16() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 11,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![10, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
        cpu.inc16(Operand16::AF);
        assert_eq!(cpu.registers.read16(Register16::AF), 256);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.inc16(Operand16::DE);
        assert_eq!(cpu.registers.read16(Register16::DE), 17);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_load_hl() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 1,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.load_hl();
        assert_eq!(cpu.registers.read16(Register16::HL), 0xFFFA);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rlca() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x85,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rlca();
        assert_eq!(cpu.registers.a, 0x0B);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rla() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x95,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rla();
        assert_eq!(cpu.registers.a, 0x2B);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rrca() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x3B,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rrca();
        assert_eq!(cpu.registers.a, 0x9D);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rra() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x81,
                b: 2,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 239, 94, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rr(Operand8::A);
        assert_eq!(cpu.registers.a, 0x40);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rlc() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x81,
                b: 0x85,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rlc(Operand8::B);
        assert_eq!(cpu.registers.b, 0x0B);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.rlc(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rl() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x81,
                b: 0x80,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 17, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rl(Operand8::B);
        assert_eq!(cpu.registers.b, 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.rl(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x22);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rrc() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x81,
                b: 0x1,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rrc(Operand8::B);
        assert_eq!(cpu.registers.b, 0x80);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.rrc(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_rr() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x1,
                b: 0x80,
                c: 3,
                d: 0,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 138, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rr(Operand8::A);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.rr(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x45);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_sla() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x81,
                b: 0x85,
                c: 3,
                d: 0x80,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 255, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.sla(Operand8::D);
        assert_eq!(cpu.registers.d, 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.sla(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0xFE);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_sra() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x8A,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 1, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.sra(Operand8::D);
        assert_eq!(cpu.registers.d, 0xC5);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.sra(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_srl() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x01,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 255, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.srl(Operand8::A);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.srl(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x7F);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }
    #[test]
    fn test_swap() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x00,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 3,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.swap(Operand8::A);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.swap(Operand8::Addr(At::HL));
        assert_eq!(cpu.memory.read16(3), 0x0F);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_bit() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0xEF,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.bit(Bit::Seven, Operand8::A);
        assert_eq!(cpu.registers.a, 0x80);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.bit(Bit::Four, Operand8::L);
        assert_eq!(cpu.registers.l, 0xEF);
        assert!(cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_set() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.set(Bit::Two, Operand8::A);
        assert_eq!(cpu.registers.a, 0x84);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.set(Bit::Seven, Operand8::L);
        assert_eq!(cpu.registers.l, 0xBB);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_res() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.res(Bit::Seven, Operand8::A);
        assert_eq!(cpu.registers.a, 0x00);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.res(Bit::One, Operand8::L);
        assert_eq!(cpu.registers.l, 0x39);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_ccf() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.ccf();
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.ccf();
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_scf() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0xFFF8,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.scf();
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));

        cpu.scf();
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_jp() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0x00,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 255, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.jp(Condition::Always, Operand16::HL);
        assert_eq!(cpu.registers.pc, 0x3b);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.jp(Condition::Always, Operand16::BC);
        assert_eq!(cpu.registers.pc, 0x8503);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));
    }

    #[test]
    fn test_jr() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0x8A,
                e: 16,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0x00,
                pc: 0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 55, 147, 0xF0, 2, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.jr(Condition::Always);
        assert_eq!(cpu.registers.pc, 3);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
        assert!(!cpu.registers.f.contains(Flags::C));

        cpu.jr(Condition::Always);
        assert_eq!(cpu.registers.pc, 0xF4);
        assert!(!cpu.registers.f.contains(Flags::Z));
        assert!(!cpu.registers.f.contains(Flags::N));
        assert!(!cpu.registers.f.contains(Flags::H));
    }

    #[test]
    fn test_call() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0,
                e: 0x4,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0x3,
                pc: 0x800,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![2, 55, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.call(Condition::Always, Operand16::DE);
        assert_eq!(cpu.registers.pc, 0x4);
        assert_eq!(cpu.registers.sp, 0x1);
        assert_eq!(cpu.memory.read16(cpu.registers.sp), 0x803);
    }

    #[test]
    fn test_ret() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0,
                e: 0x04,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0x3,
                pc: 0x0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![5, 0, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };
        cpu.ret(Condition::Always);
        //assert_eq!(cpu.registers.pc, 0x03);
        assert_eq!(cpu.registers.sp, 0x5);
    }

    #[test]
    fn test_rst() {
        let mut cpu = Cpu {
            registers: Registers {
                a: 0x80,
                b: 0x85,
                c: 3,
                d: 0,
                e: 0x04,
                f: Flags::empty(),
                h: 0,
                l: 0x3B,
                sp: 0x3,
                pc: 0x0,
            },
            state: State::Running,
            memory: MemoryMap::new(vec![5, 0, 147, 0xF0, 0, 38, 23, 3, 34, 213, 99, 43, 13]),
        };

        cpu.rst(Page0::Byte1);
        assert_eq!(cpu.registers.pc, 0x0008);
    }
}

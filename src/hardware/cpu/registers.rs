use crate::hardware::cpu::registers::flags::FlagsRegister;

pub mod flags;

/// Accumulator and auxiliary registers
/// Accumulator register (A) is an 8-bit register for storing data and the result of arithmetic and
/// logical operations
/// Auxiliary registers (B, C, D, E, F, H and L) serve as auxiliary registers to the acculmulator

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: FlagsRegister,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

#[derive(Debug)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    /// Initializes registers with value 0
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::empty(),
            h: 0,
            l: 0,
        }
    }

    /// Reads the content of a pair of 8-bit register
    ///
    pub fn read16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::AF => (self.a as u16) << 8 | self.f.bits() as u16,
            Register16::BC => (self.b as u16) << 8 | self.c as u16,
            Register16::DE => (self.d as u16) << 8 | self.e as u16,
            Register16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    /// Writes the u16 value into the pair of registers represented by Register16
    ///
    pub fn write16(&mut self, r16: Register16, value: u16) {
        match r16 {
            Register16::AF => {
                self.a = (value >> 8) as u8;
                self.f = FlagsRegister::from_bits_truncate(value as u8);
            }
            Register16::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            }
            Register16::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            }
            Register16::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            }
        }
    }

    /// Reads the content in the register represented by r8
    pub fn read8(&self, r8: Register8) -> u8 {
        match r8 {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f.bits() as u8,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    /// Writes the u8 value into the register represented by r8
    pub fn write8(&mut self, r8: Register8, value: u8) {
        match r8 {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::F => self.f = FlagsRegister::from_bits_truncate(value),
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }
}

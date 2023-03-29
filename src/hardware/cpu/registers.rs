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

#[derive(Debug, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    /// Returns an empty Registers struct
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
    /// The first register of the pair stores the most significant byte and the second register
    /// stores the least significant byte
    pub fn read16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::AF => (self.a as u16) << u8::BITS | self.f.bits() as u16,
            Register16::BC => (self.b as u16) << u8::BITS | self.c as u16,
            Register16::DE => (self.d as u16) << u8::BITS | self.e as u16,
            Register16::HL => (self.h as u16) << u8::BITS | self.l as u16,
        }
    }

    pub fn msb(&self, r16: Register16) -> u8 {
        match r16 {
            Register16::AF => self.a,
            Register16::BC => self.b,
            Register16::DE => self.d,
            Register16::HL => self.h,
        }
    }

    pub fn lsb(&self, r16: Register16) -> u8 {
        match r16 {
            Register16::AF => self.f.bits(),
            Register16::BC => self.c,
            Register16::DE => self.e,
            Register16::HL => self.l,
        }
    }

    /// Writes the u16 num into the pair of registers represented by Register16
    /// The first register of the pair stores the most significant byte of the num, and the
    /// second register stores the least significant byte
    pub fn write16(&mut self, r16: Register16, num: u16) {
        match r16 {
            Register16::AF => {
                self.a = (num >> u8::BITS) as u8;
                self.f = FlagsRegister::from_bits_truncate(num as u8);
            }
            Register16::BC => {
                self.b = (num >> u8::BITS) as u8;
                self.c = num as u8;
            }
            Register16::DE => {
                self.d = (num >> u8::BITS) as u8;
                self.e = num as u8;
            }
            Register16::HL => {
                self.h = (num >> u8::BITS) as u8;
                self.l = num as u8;
            }
        }
    }

    /// Returns the content stored in the register represented by r8
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

    /// Writes to the register represented by r8, the u8 num
    pub fn write8(&mut self, r8: Register8, num: u8) {
        match r8 {
            Register8::A => self.a = num,
            Register8::B => self.b = num,
            Register8::C => self.c = num,
            Register8::D => self.d = num,
            Register8::E => self.e = num,
            Register8::F => self.f = FlagsRegister::from_bits_truncate(num),
            Register8::H => self.h = num,
            Register8::L => self.l = num,
        }
    }
}

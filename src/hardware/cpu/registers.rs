use crate::hardware::cpu::registers::flags::Flags;

pub mod flags;

/// Accumulator and auxiliary registers
/// Accumulator register (A) is an 8-bit register for storing data and the result of arithmetic and
/// logical operations
/// Auxiliary registers (B, C, D, E, F, H and L) serve as auxiliary registers to the acculmulator
#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
}

#[derive(Debug, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
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
            f: Flags::empty(),
            h: 0,
            l: 0,
            sp: 0,
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
            Register16::SP => self.sp,
        }
    }

    /// Writes the u16 num into the pair of registers represented by Register16
    /// The first register of the pair stores the most significant byte of the num, and the
    /// second register stores the least significant byte
    pub fn write16(&mut self, r16: Register16, num: u16) {
        match r16 {
            Register16::AF => {
                self.a = (num >> u8::BITS) as u8;
                self.f = Flags::from_bits_truncate(num as u8);
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
            Register16::SP => self.sp = num,
        }
    }
}

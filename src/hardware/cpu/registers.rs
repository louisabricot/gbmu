//! General-Purpose registers
//!
//! Implements the eight 8-bit registers (`A`, `B`, `C`, `D`, `E`, `F`, `H`, `L`),  
//! the `Program Counter` register and the `Stack Pointer` register.  
//! Also implements the four 16-bit registers `AF`, `BC`, `DE` and `HL`.  
//!

use crate::hardware::cpu::instructions::Operand16;
use crate::hardware::cpu::registers::flags::Flags;

pub mod flags;

#[derive(Debug)]
/// The CPU registers.  
pub struct Registers {
    /// The 8-bit register `A` (Accumulator) stores data and the results of arithmetic and logical
    /// operations.  
    pub a: u8,

    /// The `Flag Register` stores four flags that are set and reset according to the result of
    /// instruction execution. See []  
    pub f: Flags,

    /// The 8-bit auxiliary register `B`.  
    pub b: u8,

    /// The 8-bit auxiliary register `C`.  
    pub c: u8,

    /// The 8-bit auxiliary register `D`.  
    pub d: u8,

    /// The 8-bit auxiliary register `E`.  
    pub e: u8,

    /// The 8-bit auxiliary register `H`.  
    pub h: u8,

    /// The 8-bit auxiliary register `L`.  
    pub l: u8,

    /// The 16-bit register `Stack Pointer` holds the starting address of the stack area of memory.  
    pub sp: u16,

    /// The 16-bit register `Program Counter` holds the address data of the program to be executed.  
    pub pc: u16,
}

#[derive(Debug, Copy, Clone)]
/// Enumerates the 4 possible pairing of 16-bit registers
pub enum Register16 {
    /// The 16-bit register `AF` pairs the 8-bit register `A` as it most significant byte and the
    /// 8-bit register `F` as it least significant byte.  
    ///
    /// Since the lowest nibble of the register `F` is always set to 0, `AF`
    /// actually stores 12-bit of data. See [Flag register].  
    AF,

    /// The 16-bit register `BC` pairs the 8-bit register `B` as it most significant byte and the
    /// 8-bit register `C` as it least significant byte.  
    BC,

    /// The 16-bit register `DE` pairs the 8-bit register `D` as it most significant byte and the
    /// 8-bit register `E` as it least significant byte.  
    DE,

    /// The 16-bit register `HL` pairs the 8-bit register `H` as it most significant byte and the
    /// 8-bit register `L` as it least significant byte.  
    HL,
}

impl Registers {
    /// Constructs empty registers.  
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
            pc: 0,
        }
    }

    /// Reads the 16-bit value of *r16* from the values stored in the 8-bit register pair.
    /// The first 8-bit register of the pair becomes the most significant byte of the 16-bit
    /// returned value.  
    /// The second 8-bit register becomes the least significant byte.  
    ///
    /// # Example
    ///
    /// ```
    /// let registers = Registers::new();
    ///
    /// registers.b = 0b0011_0010;
    /// registers.c = 0b0010_0011;
    ///
    /// let bc = registers.read16(Register16::BC);
    ///
    /// assert_eq!(bc, 0b0011_0010_0010_0011);
    /// ```
    pub fn read16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::AF => (self.a as u16) << u8::BITS | self.f.bits() as u16,
            Register16::BC => (self.b as u16) << u8::BITS | self.c as u16,
            Register16::DE => (self.d as u16) << u8::BITS | self.e as u16,
            Register16::HL => (self.h as u16) << u8::BITS | self.l as u16,
        }
    }

    /// Writes the 16-bit value *data* into *r16*.
    /// The first 8-bit register of the pair stores the most significant byte of *data*.  
    /// The second 8-bit register stores the least significant byte.  
    ///
    /// # Example
    ///
    /// ```
    /// let registers = Registers::new();
    ///
    /// let data = 0b10101111_01101010;
    ///
    /// registers.write16(Register16::DE, data);
    /// registers.write16(Register16::AF, data);
    ///
    /// let de = registers.read16(Register16::DE);
    /// let af = registers.read16(Register16::AF);
    ///
    /// assert_eq!(de, data);
    /// assert_eq!(af, data & 0xFFF0); //the lowest nibble of register `F` is always set to 0
    /// ```
    ///
    pub fn write16(&mut self, r16: Register16, data: u16) {
        match r16 {
            Register16::AF => {
                self.a = (data >> u8::BITS) as u8;
                self.f = Flags::from_bits_truncate(data as u8);
            }
            Register16::BC => {
                self.b = (data >> u8::BITS) as u8;
                self.c = data as u8;
            }
            Register16::DE => {
                self.d = (data >> u8::BITS) as u8;
                self.e = data as u8;
            }
            Register16::HL => {
                self.h = (data >> u8::BITS) as u8;
                self.l = data as u8;
            }
        }
    }

    /// Converts the *operand* variant into its `Register16` equivalent,   
    /// If *operand* is not a pair register, the function panics!  
    /// See [Operand16] for more...
    pub fn get_register16(operand: Operand16) -> Register16 {
        match operand {
            Operand16::AF => Register16::AF,
            Operand16::BC => Register16::BC,
            Operand16::DE => Register16::DE,
            Operand16::HL => Register16::HL,
            _ => panic!("Not a pair of 8-bit registers"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_register16() {
        let mut registers = Registers::new();

        registers.d = 0xC3;
        registers.e = 0xF0;

        let de = Registers::get_register16(Operand16::DE);

        assert_eq!(registers.read16(de), 0xC3F0);
    }

    #[test]
    #[should_panic(expected = "Not a pair of 8-bit registers")]
    pub fn test_invalid_get_register16() {
        let mut registers = Registers::new();

        registers.d = 0xC3;
        registers.e = 0xF0;

        Registers::get_register16(Operand16::Imm16);
    }

    #[test]
    pub fn test_read16() {
        let mut registers = Registers::new();

        let data = 0xC3F5;

        registers.write16(Register16::DE, data);
        assert_eq!(registers.d, 0xC3);
        assert_eq!(registers.e, 0xF5);

        registers.write16(Register16::AF, data);
        assert_eq!(registers.a, 0xC3);
        assert_eq!(registers.f.bits(), 0xF0);
    }

    #[test]
    pub fn test_write16() {
        let mut registers = Registers::new();

        let data = 0xC3F0;

        registers.write16(Register16::DE, data);
        registers.write16(Register16::AF, data);

        let de = registers.read16(Register16::DE);
        let af = registers.read16(Register16::AF);

        assert_eq!(de, data);
        assert_eq!(af, data & 0xFFF0);
    }
}

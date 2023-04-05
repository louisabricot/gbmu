//! `Flag Register` F.  
//!
//! The `Flag Register` is a 8-bit register consisting in 4 flags (`Z`, `N`, `H`, and `C`) that are set or reset according to the result of instruction execution.  
//!
//! | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//! |---|---|---|---|---|---|---|---|
//! | Z | N | H | C | 0 | 0 | 0 | 0 |
//!
//! - **Z** : The *Zero* flag.  
//! Set to 1 when the result of an operation is 0; otherwise reset.  
//!
//! 
//! - **N** : The *Subtract* flag.  
//! Set to 1 after a subtraction instruction.  
//! 
//!
//! - **H** : The *Half-Carry* flag.  
//! Set to 1 when an operation results in carrying or borrowing
//! to bit3.  
//! 
//!
//! - **C** : The *Carry* flag.  
//! Set to 1 when an operation results in carrying or borrowing to
//! bit7.  
//! 
//!
//!
//! The lowest nibble (bit0-3) is always set to 0, even after a load to the 16-bit register `AF`.  
//!

use crate::hardware::cpu::Condition;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

#[derive(Debug)]
/// 
pub struct Flags {
    flags: u8,
}

impl Flags {
    
    /// The *Zero* flag.  
    /// Set to 1 when the result of an operation is 0; otherwise reset.
    pub const Z: u8 = 0b1000_0000;

    /// The *Subtraction* flag.  
    /// Set to 1 after subtraction instruction.
    pub const N: u8 = 0b0100_0000;

    /// The *Half-Carry* flag.  
    /// Set to 1 when an operation results in carrying from or borrowing to bit 3.
    pub const H: u8 = 0b0010_0000;

    /// The *Carry* flag.  
    /// Set to 1 when an operation results in carrying from or borrowing to bit 7.
    pub const C: u8 = 0b0001_0000;

    /// Returns true if *flag* is set in the `Flag Register`.
    pub fn contains(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }

    /// If *set* is true, sets *flag* into the `Flag Register`, otherwise resets it.
    pub fn set(&mut self, flag: u8, set: bool) {
        match set {
            true => self.flags.bitor_assign(flag),
            false => self.flags.bitand_assign(!flag),
        }
    }

    /// Constructs an empty `Flag Register`.  
    pub fn empty() -> Self {
        Self { flags: 0 }
    }

    /// Returns `Flag Register` as u8.  
    pub fn bits(&self) -> u8 {
        self.flags
    }

    /// Creates a `Flag Register` from *flags*, truncating the lowest nibble.  
    pub fn from_bits_truncate(flags: u8) -> Self {
        Self {
            flags: flags & 0b1111_0000,
        }
    }

    /// Toggles the value of *flag* in the `Flag Register`.  
    pub fn toggle(&mut self, flag: u8) {
        self.flags.bitxor_assign(flag)
    }

    /// Returns true when the flag status matches the condition, otherwise returns false.  
    /// Special case: Condition::Always always returns true.  
    /// The relation between conditions and flags are as follows: 
    ///
    /// | Condition | Flag  |  
    /// |-----------|-------|  
    /// |     NC    | C = 0 |  
    /// |     NZ    | Z = 0 |  
    /// |     C     | C = 1 |  
    /// |     Z     | Z = 1 |  
    ///
    /// 
    pub fn check_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::NC => !self.contains(Flags::C),
            Condition::NZ => !self.contains(Flags::Z),
            Condition::Z => self.contains(Flags::Z),
            Condition::C => self.contains(Flags::C),
            Condition::Always => true,
        }
    }
}

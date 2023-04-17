//! Interrupts implements the Interrupt Registers.  
//!
//! Interrupts are controlled by the `IE` flag.
//! The `IF` flag can be used to determine which interrupt was requested.  
//! When multiple interrupts occur simultaneously, the `IE` flag of each is
//! set, but only that with the highest priority is started.
//! Those with lowest priority are suspended.  
//! When using an interrupt, set the `IF` register to 0 before setting the `IE`
//! register.  
//! Thhe interrupt process is as follows:
//! 1. When an interrupt is processed, the corresponding IF flag is set.
//! 2. Interrupted enabled
//! If the IME flag and the corresponding IE flag are set, the interrupt is
//! performed by the following steps:
//!   2.1 The IME flag is reset, and all interrupts are prohibited.
//!   2.2 The contents of the PC are pushed onto the stack RAM
//!   2.3 Control jumps to the interrupt starting address of the interrupt.
//!   TODO: 2.2 and 2.3 => call interrupt_address
//!   TODO: 2.1 implement that interrupts are prohibited when IME is false

use std::ops::{BitAndAssign, BitOrAssign};
pub struct Interrupts {
    /// Interrupt Flag (IF)
    flags: u8,

    /// Interrupt Enable (IE)
    requests: u8,

    /// Interrupt Master Enable (IME) flag
    master: bool,
}

impl Interrupts {
    /// Starting address of Vertical Blanking Interrupt
    pub const VBLANK_ADDRESS: u16 = 0x0040;

    /// Starting address of LCDC Status Interrupt
    pub const LCDC_ADDRESS: u16 = 0x0048;

    /// Starting address of Timer Overflow Interrupt
    pub const TIMER_ADDRESS: u16 = 0x0050;

    /// Starting address of Serial Transfer completion
    pub const SERIAL_ADDRESS: u16 = 0x0058;

    /// Starting address of P10-P13 Input Signal Goes Low Interrupt
    pub const JOYPAD_ADDRESS: u16 = 0x0060;

    /// The position of the *Vertical Blanking* flag in both the `IF` and `IE`
    /// registers.
    pub const VBLANK: u8 = 0b0000_0001;

    /// The position of the *LDC (STAT referenced) flag in both the `IF` and
    /// `IE` registers.  
    pub const LCDC: u8 = 0b0000_0010;

    /// The position of the *Timer overflow* flag in both the `IF` and `IE`
    /// registers.  
    pub const TIMER: u8 = 0b0000_0100;

    /// The position of the *Serial I/O transfer completion* flag in both the
    /// `IF` and `IE` registers.  
    pub const SERIAL: u8 = 0b0000_1000;

    /// The position of the *P10-P13 terminal negative edge* (Joypad) flag in
    /// both `IF` and `IE` registers.  
    pub const JOYPAD: u8 = 0b0001_0000;

    /// Creates an empty Interrupt Register.  
    pub fn empty() -> Self {
        Self {
            flags: 0,
            requests: 0,
            master: false,
        }
    }

    pub fn set_request(&mut self, flag: u8, set: bool) {
        match set {
            true => self.flags.bitor_assign(flag),
            false => self.flags.bitand_assign(!flag),
        }
    }

    pub fn set_flag(&mut self, flag: u8, set: bool) {
        match set {
            true => self.requests.bitor_assign(flag),
            false => self.requests.bitand_assign(!flag),
        }
    }

    /// Returns true if *flag* is set in the `Interrupt Enable` register.   
    fn requested(&self, flag: u8) -> bool {
        (self.requests & flag) == flag
    }

    /// Returns true if *flag* is set in the `Interrupt Flag` register.  
    fn flagged(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }

    /// Returns true if *flag* is set in both the `Interrupt Flag` and
    /// the `Interrupt Enable` registers.  
    pub fn is_triggered(&self, flag: u8) -> bool {
        self.requested(flag) && self.flagged(flag)
    }

    /// Loads the value of *set* into IME flag, enabling interrupts.    
    pub fn set_ime(&mut self, set: bool) {
        self.master = set;
    }

    /// Returns the triggered interrupt with the highest priority.  
    ///
    /// | Cause of Interrupt | Priority |
    /// | ------------------ | -------- |
    /// | Vertical Blanking  |     1    |
    /// | LCDC Status Inter. |     2    |
    /// | Timer Overflow     |     3    |
    /// | Serial Transfer    |     4    |
    /// | Joypad goes low    |     5    |

    /// Returns the triggered interrupt with the highest priority.  
    pub fn get_highest_priority(&self) -> Option<u8> {
        // List of interrupts sorted by order of priority  
        const INTERRUPTS_BY_PRIORITY: [u8; 5] = [Interrupts::JOYPAD, Interrupts::SERIAL,
        Interrupts::TIMER, Interrupts::LCDC, Interrupts::VBLANK];
        
        INTERRUPTS_BY_PRIORITY.into_iter().find(|&interrupt|
        self.is_triggered(interrupt))
    }

    pub fn remove(&mut self, interrupt: u8) {
        self.set_request(interrupt, false);
        self.set_flag(interrupt, false);
    }

    /// Returns the *interrupt* address.  
    pub fn get_address(&self, interrupt: u8) -> u16 {
      match interrupt {
        Interrupts::JOYPAD => Interrupts::JOYPAD_ADDRESS,
        Interrupts::SERIAL => Interrupts::SERIAL_ADDRESS,
        Interrupts::TIMER => Interrupts::TIMER_ADDRESS,
        Interrupts::LCDC => Interrupts::LCDC_ADDRESS,
        Interrupts::VBLANK => Interrupts::VBLANK_ADDRESS,
        _ => panic!("dont know what to do")
      }
    }
}

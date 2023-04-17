//! Timer Registers

//! Timer Controller
//!
//! | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//! |---|---|---|---|---|---|---|---|
//! | X | X | X | X | X |TS | ICS   |

 use std::ops::BitAnd;

const DIVIDER_REGISTER: u16 = 0xFF04;
const TIMER_COUNTER:    u16 = 0xFF05;
const TIMER_MODULO:     u16 = 0xFF06;
const TIMER_CONTROLLER: u16 = 0xFF07;

pub struct Timer {

  pub divider: u8,

  /// Main timer unit, generates an interrupt when it overflows. 
  pub counter: u8,

  /// Modulo register of `counter`.  When `counter` overlofws, `modulo` data is
  /// loaded into `counter`.  
  pub modulo: u8,

  /// Specifies the clock frequency  
  pub controller: u8,
}

impl Timer {

  /// Increments the timer *counter* and returns true if an overflow occured.  
  pub fn count(&mut self) -> bool {
      let (new_count, overflow) = self.counter.overflowing_add(1);
      self.counter = new_count;
      overflow
  }

  pub fn new() -> Self {
    Self {
      divider: 0,
      counter: 0,
      modulo: 0,
      controller: 0,
    }
  }
  // TODO: somewhere, call count() at the clock frequency specified by the
  // controller and trigger an interrupt if an overflow occured...
  
  
  pub fn get_register(&self, address: u16) -> u8 {
      match address {
          DIVIDER_REGISTER => self.divider,
          TIMER_COUNTER => self.counter,
          TIMER_MODULO => self.modulo,
          TIMER_CONTROLLER => self.controller,
          _ => todo!(),
      }
  }

  pub fn set_register(&mut self, address: u16, value: u8) {
      match address {
          DIVIDER_REGISTER => self.divider = value,
          TIMER_COUNTER => self.counter = value,
          TIMER_MODULO => self.modulo = value,
          TIMER_CONTROLLER => self.controller = value.bitand(0b00000111),
          _ => todo!(),
      }
  }
}

//! Timer Registers

//! Timer Controller
//!
//! | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//! |---|---|---|---|---|---|---|---|
//! | X | X | X | X | X |TS | ICS   |

pub struct Timer {
  /// Main timer unit, generates an interrupt when it overflows. 
  counter: u8;

  /// Modulo register of `counter`.  When `counter` overlofws, `modulo` data is
  /// loaded into `counter`.  
  modulo: u8;

  /// Specifies the clock frequency  
  controller: u8;
}

impl Timer {

  /// Increments the timer *counter* and returns true if an overflow occured.  
  pub fn count(&mut self) -> bool {
      let (new_count, overflow) = self.counter.wrapping_add(1);
      self.counter = new_count;
      overflow
  }

  // TODO: somewhere, call count() at the clock frequency specified by the
  // controller and trigger an interrupt if an overflow occured...
}

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

  /// Selects the timer input clock.  
  controller: u8;
}

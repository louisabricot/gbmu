pub struct Interrupts {
  /// Interrupt enable
  IE: u8,

  /// Interrupt flag
  IF: u8,
  IME: bool,
}


impl Interrupts {
    pub fn new() -> Self {
      Self {
        IE: 0,
        IF: 0,

        /// The IME flag is used to disable all interrupts, overriding any
        /// enabled bits in the IE register. 
        IME: 0,
      },
    }

    /// Sets the IME flag (that is, IME=true), enabling interrupts.    
    pub fn set_IME(&mut self) {
        self.IME = true;
    } 

    /// Unsets the IME flag (that is, IME=false), disabling interrupts.  
    pub fn unset_IME(&mut self) {
        self.IME = false;
    }
}

//! 
//!
//!

/// Enumerates the types of cartridge implemented.  
enum Type {

    /// 
    NoMCB,

    /// Supports 
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

pub trait Cartridge {

    /// Associated function signature; `Self` refers to the implementor type.  
    fn new(cartridge: Vec<u8>) -> Self;

    /// Method signature; returns the `u8` value at *address*.  
    fn read8(&self, address: u16) -> u8;

    /// Method signature; writes the u8 *value* at *address*.  
    fn write8(&mut self, address: u16, value: u8);
}


impl Cartridge for NoMCB {

    /// Creates a NoMCB type of cartridge.  
    pub fn new(cartridge: Vec<u8>) -> Self {
        Self { data: cartridge }
    }

    /// Returns the `u8` value at *address*.  
    pub fn read8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Writes the u8 *value* at *address*. 
    pub fn write8(&self, address: u16, value: u8) {
        self.data[address as usize] = value
    }
}

impl Cartridge for MCB1 {

}

//! 
//!
//!

pub mod mbc1;

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
    fn new(&self, cartridge: Vec<u8>) -> Self where Self: Sized;

    /// Method signature; returns the `u8` value at *address*.  
    fn read8(&self, address: u16) -> u8;

    /// Method signature; writes the u8 *value* at *address*.  
    fn write8(&mut self, address: u16, value: u8);


    fn get_type(&self) -> Type {
        todo!()
    } 
}



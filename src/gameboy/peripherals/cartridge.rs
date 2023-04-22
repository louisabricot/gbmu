pub mod mbc1;
pub mod nombc;

use crate::gameboy::peripherals::cartridge::mbc1::MBC1;
use crate::gameboy::peripherals::cartridge::nombc::NoMBC;

const CARTRIDGE_TYPE: u16 = 0x147;
/// Enumerates the types of cartridge implemented.  

pub trait Cartridge {

    /// Method signature; returns the `u8` value at *address*.  
    fn read8(&self, address: u16) -> u8;

    /// Method signature; writes the u8 *value* at *address*.  
    fn write8(&mut self, address: u16, value: u8);
}

pub fn make_cartridge(content: Vec<u8>) -> Box< dyn Cartridge> {
    match content[CARTRIDGE_TYPE as usize] {
        0x00 => Box::new( NoMBC::new(content)),
        0x01 => Box::new( MBC1::new(content)),
        _ => todo!(),
    }
}

use crate::gameboy::Cartridge;

pub struct MBC1 {
    
    /// RAM gate register enables access to the cartridge SRAM (if there is one). 
    /// RAM access is disabled by default but can be enabled by writing to the `0x0000-0x1FFF`
    /// address range, the value 0b1010 in the lower nibble.  
    /// When RAM access is disabled, all writes to the external RAM area `0xA000-0xBFFF` are
    /// ignored and reads return undefined values.
    gate: u8,

    /// Selects the bank to access to in the `0x4000-0x7FFF` memory area. 
    ///
    bank1: u8,
}


impl MBC1 {
    pub fn new(content: Vec<u8>) -> Self {
        todo!()
    }
}

impl Cartridge for MBC1 {

    fn read8(&self, address: u16) -> u8 {
        todo!()
    }

    fn write8(&mut self, address: u16, value: u8) {
        todo!();
    }
}



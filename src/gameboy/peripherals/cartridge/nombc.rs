use crate::gameboy::Cartridge;

pub struct NoMBC {
    data: Vec<u8>,
}

impl NoMBC {

    /// Creates a NoMCB type of cartridge.  
    pub fn new(content: Vec<u8>) -> Self {
        Self { data: content }
    }
}

impl Cartridge for NoMBC {

    /// Returns the `u8` value at *address*.  
    fn read8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Writes the u8 *value* at *address*. 
    fn write8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value
    }
}

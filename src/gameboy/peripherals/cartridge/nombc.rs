
impl Cartridge for NoMCB {

    /// Creates a NoMCB type of cartridge.  
    fn new(cartridge: Vec<u8>) -> Self {
        Self { data: cartridge }
    }

    /// Returns the `u8` value at *address*.  
    fn read8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Writes the u8 *value* at *address*. 
    fn write8(&self, address: u16, value: u8) {
        self.data[address as usize] = value
    }
}

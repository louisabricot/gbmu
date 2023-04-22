pub mod timer;
pub mod interrupts;
pub mod dmg;

/// Defines the shared behaviour between DMG and CGB memory.  
pub trait Memory {
    
    /// Method signature to read a `u8` value at *address* from memory.
    fn read8(&self, address: u16) -> u8;

    /// Method signature to write the u8 *value* at *address* in memory.
    fn write8(&mut self, address: u16, value: u8);
    
    /// Reads at *address* the `u16` value, converting it from little endian.
    fn read16(&self, address: u16) -> u16 {
        let hi = self.read8(address);
        let lo = self.read8(address + 1);
        u16::from_le_bytes([hi, lo])
    }

    /// Write at *address* the u16 *value* converting it into little endian.
    fn write16(&mut self, address: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.write8(address, bytes[0]);
        self.write8(address + 1, bytes[1]);
    }

    fn set_ime(&mut self, set: bool);

    fn get_interrupt(&mut self) -> Option<u8>;

    fn get_interrupt_address(&self, interrupt: u8) -> u16;

    fn remove_interrupt(&mut self, interrupt: u8); 

}

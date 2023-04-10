pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(data: Vec<u8>) -> Self {
        Self { memory: data }
    }

    /// Reads the 8-bit value at address pc
    pub fn read8(&self, pc: u16) -> u8 {
        self.memory[pc as usize]
    }

    /// Reads the 16-bit value at address pc
    /// Returns a native endian value
    pub fn read16(&self, pc: u16) -> u16 {
        let hi = self.read8(pc);
        let lo = self.read8(pc + 1);
        u16::from_le_bytes([hi, lo])
    }

    /// Writes at address pc the u8 value given as parameter
    pub fn write8(&mut self, pc: u16, value: u8) {
        self.memory[pc as usize] = value;
    }

    /// Write at address pc the u16 value converted into little endian
    pub fn write16(&mut self, pc: u16, value: u16) {
        let bytes = value.to_le_bytes();

        self.memory[pc as usize] = bytes[0];
        self.memory[pc as usize + 1] = bytes[1];
    }
}

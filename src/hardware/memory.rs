pub struct Memory {
    memory: Vec<u8>,
    //0x0000 - 0x00FF : Boot ROM then interrupt table
    //0x0000 - 0x3FFF : Bank0, always the same memory, cannot be switched out later
    //0x4000 - 0x7FFF : Bank N, for bank switching
    //0x8000 - 0x9FFF : Tile RAM
    //0x9800 - 0x9FFF : Background Map
    //0xA000 - 0xBFFF : Cartridge RAM
    //0xC000 - 0xDFFF : Working RAM
    //0xE000 - 0xFDFF : Echo RAM
    //0xFE00 - 0xFE9F : Object Attribute Memory
    //0xFEA0 - 0xFEFF : Unused
    //0xFF00 - 0xFF7F : I/O registers
    //0xFF80 - 0xFFFE : High RAM Area
    //0xFFFF : Interrupt Enabled Register
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

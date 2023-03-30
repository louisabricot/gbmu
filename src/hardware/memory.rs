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

    pub fn read8(&self, pc: u16) -> u8 {
        self.memory[pc as usize]
    }

    pub fn write8(&self, pc: u16, value: u8) {
        todo!()
    }

    pub fn write16(&self, address: u16, value: u16) {
        todo!()
    }
}

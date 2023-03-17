pub struct Memory {
    lol: u8
    //https://gbdev.io/pandocs/Memory_Map.html
}
impl Memory {
    pub fn read8(pc: u16) -> u8 {
        pc as u8
    }

    pub fn read16(pc: u16) -> u16 {
        pc
    }
}

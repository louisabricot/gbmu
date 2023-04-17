const CARTRIDGE_HEADER_START: u16 = 0x100;
const CARTRIDGE_HEADER_END: u16 = 0x14F;
const CARTRIDGE_GAME_TITLE_START: u16 = 0x134;
const CARTRIDGE_GAME_TITLE_SIZE: u16 = 0x10;
const CARTRIDGE_NINTENDO_LOGO_START: u16 = 0x104;
const CARTRIDGE_NINTENDO_LOGO_END: u16 = 0x133;

pub struct Cartridge {
    cartridge: Vec<u8>,
}

enum Type {
    ROMONLY,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

enum RomSize {}

impl Cartridge {
    pub fn new(cartridge: Vec<u8>) -> Self {
        Self { cartridge }
    }
}

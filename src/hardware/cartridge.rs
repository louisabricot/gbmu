pub struct Cartridge {
    //header
    ctype: Type,
}

enum Type {
    ROMONLY,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

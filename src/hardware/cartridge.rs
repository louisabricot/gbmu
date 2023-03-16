pub struct Cartridge {
    //header
    type: Type,

}

enum Type {
    ROMONLY,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

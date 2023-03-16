
// Bank register for CGB only
// SVBK at addr FF70 8 bits

const ZERO_F: u8 = 0b1000_0000;
const SUBSTRACT_F: u8 = 0b0100_0000;
const HALF_CARRY_F: u8 = 0b0010_0000;
const CARRY_F : u8 = 0b0001_0000;

//TODO: A specific flag register struct
//TODO: A specific SP and PC register struct

pub struct FlagRegister {
    flags: u8,
}
pub struct Registers {
    a: u8,              //Accumulator
    f: FlagRegister,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

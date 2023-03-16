pub struct FlagRegister {
    const ZERO_FLAG : u8 = 0b1000_0000;
    const SUB_FLAG : u8 = 0b0100_0000;
    const HALF_CARRY_FLAG : u8 = 0b0010_0000;
    const CARRY_FLAG : u8 = 0b0001_0000;
    
    flags: u8,
}

impl FlagRegister {
    fn new() -> Self {
        //TODO: set to 0
    }


}

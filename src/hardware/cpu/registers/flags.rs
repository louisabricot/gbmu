use bitflags::bitflags;

bitflags! {
    /// Consists of 4 flags that are set and reset according to the results of instruction
    /// execution
    pub struct FlagsRegister: u8 {

        /// Set to 1 when the result of an operation is 0; otherwise reset
        const ZERO = 0b1000_0000;

        /// Set to 1 following execution of the subtraction instruction, regardless of the result
        const SUBTRACT = 0b0100_0000;

        /// Set to 1 when an operation results in carrying from or borrowing to bit 3
        const HALF_CARRY = 0b0010_0000;

        /// Set to 1 when an operation results in carrying from or borrowing to bit 7
        const CARRY = 0b0001_0000;
    }
}

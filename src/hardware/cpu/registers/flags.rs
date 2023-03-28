use bitflags::bitflags;
use crate::hardware::cpu::Condition;

bitflags! {
    /// Consists of 4 flags that are set and reset according to the results of instruction
    /// execution
    #[derive(Debug)]
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

impl FlagsRegister {

    /// Returns true when the flag status matches the condition, otherwise returns false
    /// Special case: Condition::Always always returns true.
    /// The relation between conditions and flags are as follows:
    /// | Condition | Flag  |
    /// |-----------|-------|
    /// |     NC    | C = 0 |
    /// |     NZ    | Z = 0 |
    /// |     C     | C = 1 |
    /// |     Z     | Z = 1 |
    ///
    pub fn check_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::NC => { !self.contains(FlagsRegister::CARRY) },
            Condition::NZ => { !self.contains(FlagsRegister::ZERO) },
            Condition::Z => { self.contains(FlagsRegister::ZERO) },
            Condition::C => { self.contains(FlagsRegister::CARRY) },
            Condition::Always => { true },
        }
    }
}

use crate::hardware::cpu::Condition;
use bitflags::bitflags;

bitflags! {
    /// Consists of 4 flags that are set and reset according to the results of instruction
    /// execution
    #[derive(Debug)]
    pub struct Flags: u8 {

        /// Set to 1 when the result of an operation is 0; otherwise reset
        const Z = 0b1000_0000;

        /// Set to 1 following execution of the subtraction instruction, regardless of the result
        const N = 0b0100_0000;

        /// Set to 1 when an operation results in carrying from or borrowing to bit 3
        const H = 0b0010_0000;

        /// Set to 1 when an operation results in carrying from or borrowing to bit 7
        const C = 0b0001_0000;
    }
}

impl Flags {
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
            Condition::NC => !self.contains(Flags::C),
            Condition::NZ => !self.contains(Flags::Z),
            Condition::Z => self.contains(Flags::Z),
            Condition::C => self.contains(Flags::C),
            Condition::Always => true,
        }
    }
}

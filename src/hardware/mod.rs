//! Hardware module includes CPU, memory and peripheral elements for the
//! GameBoy
//!
//! # Example
//!
//! ``
//!
//! let mut gameboy: GameBoy = GameBoy::new();
//! gameboy.start();
//! ``
pub mod cpu;
pub mod gameboy;
pub mod memory;

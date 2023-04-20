//!

use super::memory::interrupts::Interrupts;
use super::memory::timer::Timer;

pub mod interrupts;
pub mod timer;


const KIB_IN_BYTE: u64 = 1024;

pub struct CGB {
    pub rom: Vec<u8>,
    pub eram: Vec<u8>,
    pub wram: Vec<u8>,
    pub interrupts: Interrupts,
    pub timer: Timer,
}

impl Memory for CGB {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            rom: data,
            eram: vec![0; 8 * KIB_IN_BYTE as usize],
            wram: vec![0; 8 * KIB_IN_BYTE as usize],
            interrupts: Interrupts::empty(),
            timer: Timer::new(),
        }
    }


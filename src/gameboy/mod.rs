use std::rc::Rc;
use self::cpu::Cpu;
use self::memory::Memory;

pub mod memory;
pub mod cpu;

const PROGRAM_START_ADDRESS: u16 = 0x100;
const CARTRIDGE_TITLE: u16 = 0x134;
const CARTRIDGE_HEADER_CHECKSUM: u16 = 0x14D;

const CARTRIDGE_NINTENDO_LOGO_START: u16 = 0x104;
const CARTRIDGE_NINTENDO_LOGO_END: u16 = 0x133;
const NINTENDO_LOGO_SIZE: u16 = 48;

const NINTENDO_LOGO: [u8; NINTENDO_LOGO_SIZE as usize] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// 
pub struct GameBoy {
    cpu: Rc<Cpu>,
    speed: SpeedMode,
    cpu: Cpu,
}

/// Enumerates the GameBoy's speed mode.  
enum SpeedMode {
    Double,
    Normal,
}

/// Enumerates the GameBoy's model.  
pub enum Model {
    DMG,
    CGB,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: Rc::new(Cpu::new()),
            memory: None,
        }
    }
    
    fn set_memory(&mut self, cartridge: Cartridge) {
        self.cpu.memory = match cartridge.get_model() {
            Model::CGB => Some(Box::new( CGB { cartridge })),
            _ => Some(Box::new( DMG{ cartridge })),
        };
    }

    fn run(&mut self) {
        match self.cpu.memory {
            Some(memory) => loop { self.cpu.step() },
            None => println!("Missing cartridge"),
        }
    }
}

use super::interrupts::Interrupts;
use super::timer::Timer;
use crate::gameboy::Memory;
use crate::gameboy::Cartridge;


const CARTRIDGE_TYPE: u16 = 0x147;

//Read-Only Memory from cartridge
const ROM0_START: u16 = 0x0000;
const ROM0_END: u16 = 0x7FFF;

// Character data
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;

// External Expansion Working RAM
const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;

// Unit Working RAM 8KB
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;

// Use of area 0xE000 - 0xFDFF is prohibited
const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

// Object Attribute Memory 
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;

// Use of area 0xFEA0 - 0xFF00 is prohibited
const NOT_USABLE_START: u16 = 0xFEA0;
const NOT_USABLE_END: u16 = 0xFEFF;

// Input/Output registers (Port, Mode, Control, Sound ...)
const IO_REGISTERS_START: u16 = 0xFF00;
const IO_REGISTERS_END: u16 = 0xFF7F;

// Working & Stack RAM (127 bytes)
const WSRAM_START: u16 = 0xFF80;
const WSRAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

const KIB_IN_BYTE: u64 = 1024;

pub struct DMG {
    pub cartridge: Box<dyn Cartridge>,
    pub eram: Vec<u8>,
    pub wram: Vec<u8>,
    pub oam: OAM,
    pub interrupts: Interrupts,
    pub timer: Timer,
}

impl DMG {
    pub fn new(content: Vec<u8>) -> Self {
        Self {
            cartridge: make_cartridge(content),
            eram: vec![0; 8 * KIB_IN_BYTE as usize],
            wram: vec![0; 8 * KIB_IN_BYTE as usize],
            interrupts: Interrupts::empty(),
            timer: Timer::new(),
        }
    }

}
impl Memory for DMG {

    fn set_ime(&mut self, set: bool) {
        self.interrupts.set_ime(set)
    }

    fn get_interrupt(&mut self) -> Option<u8> {
        self.interrupts.get_interrupt()
    }

    fn get_interrupt_address(&self, interrupt: u8) -> u16 {
        self.interrupts.get_address(interrupt)
    }

    fn remove_interrupt(&mut self, interrupt: u8) {
        self.interrupts.remove(interrupt);
    }

    /// Maps the address to the correct memory area
    fn read8(&self, address: u16) -> u8 {
        match address {
            ROM0_START..=ROM0_END => self.cartridge.read8(address),
            VRAM_START..=VRAM_END => {
                //0x9800 - 0x9BFF -> background tile map
                //0x9C00 - 0x9FFF -> window tile map
                // Each tile map contains the 1-byte indexes of the tiles to be
                // displayed. 
                // Tiles are obtained from the Tile Data Table using either of
                // the two addressing modes, which can be selected via the LCDC
                // register 
                todo!()
            },
            ERAM_START..=ERAM_END => self.eram[(address - ERAM_START) as usize],
            WRAM_START..=WRAM_END => self.wram[(address - WRAM_START) as usize],
            ECHO_RAM_START..=ECHO_RAM_END => self.read8(address - 0x2000),
            OAM_START..=OAM_END => {
              todo!()
            },
            IO_REGISTERS_START..=IO_REGISTERS_END => todo!(),
            WSRAM_START..=WSRAM_END => todo!(),
            INTERRUPT_ENABLE_REGISTER => todo!(),
            _ => panic!("Forbidden to read at address:{:#06x}",address),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            BANK0_START..=BANK0_END => todo!(),
            ERAM_START..=ERAM_END => self.eram[(address -
            ERAM_START) as usize] = value,
            WRAM_START..=WRAM_END => self.wram[(address -
            WRAM_START) as usize] = value,
            ECHO_RAM_START..=ECHO_RAM_END => {
              // writes to work RAM instead
              self.write8(address - 0x2000, value)
            }
            OAM_START..=OAM_END => {
                todo!();
            }
            IO_REGISTERS_START..=IO_REGISTERS_END => todo!(),
            WSRAM_START..=WSRAM_END => todo!(),
            INTERRUPT_ENABLE_REGISTER => todo!(),
              _ => panic!("Forbidden to write at address:
              {:#06x}",address)
        }
    }
}

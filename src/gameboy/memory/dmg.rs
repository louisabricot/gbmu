//!

use super::memory::interrupts::Interrupts;
use super::memory::timer::Timer;

pub mod interrupts;
pub mod timer;


// From cartridge, usually a fixed bank
const ROM0_START: u16 = 0x0000;
const INTERRUPT: u16 = 0x0000;
const ROM_DATA_AREA: u16 = 0x100;

const ROM0_END: u16 = 0x3FFF;

// From cartridge, switchable bank via mapper
const ROM1_START: u16 = 0x4000;
const ROM1_END: u16 = 0x7FFF;

// In CGB mode, switchable bank 0
const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;
const DMG_VIDEO_RAM_SIZE: u64 = 8 * KIB_IN_BYTE;
const CGB_VIDEO_RAM_SIZE: u64 = 16 * KIB_IN_BYTE;

// From cartridge, switchable bank if any
const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END: u16 = 0xBFFF;

const WORK_RAM_00_START: u16 = 0xC000;
//const WORK_RAM_00_END: u16 = 0xCFFF;

// In CGB mode, switchable bank 1~7
//const WORK_RAM_01_START: u16 = 0xD000;
const WORK_RAM_01_END: u16 = 0xDFFF;

// Mirror of C000~DDFF, Nintendo says use of this area is prohibited
const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

// Display RAM
const SPRITE_ATTRIBUTE_TABLE_START: u16 = 0xFE00;
const SPRITE_ATTRIBUTE_TABLE_END: u16 = 0xFE9F;

// Nintendo says use of this area is prohibited
const NOT_USABLE_START: u16 = 0xFEA0;
const NOT_USABLE_END: u16 = 0xFEFF;

const INPUT_OUTPUT_REGISTERS_START: u16 = 0xFF00;
const INPUT_OUTPUT_REGISTERS_END: u16 = 0xFF7F;

const JOYPAD_INPUT: u16 = 0xFF00;

const SERIAL_TRANSFER_START: u16 = 0xFF01;
const SERIAL_TRANSFER_END: u16 = 0xFF02;

const TIMER_DIVIDER_START: u16 = 0xFF04;
const TIMER_DIVIDER_END: u16 = 0xFF07;

const DIVIDER_REGISTER: u16 = 0xFF04;
const TIMER_COUNTER:    u16 = 0xFF05;
const TIMER_MODULO:     u16 = 0xFF06;
const TIMER_CONTROLLER: u16 = 0xFF07;

const AUDIO_START: u16 = 0xFF10;
const AUDIO_END: u16 = 0xFF26;

const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;

// LCD Control, status, position, scrolling, and palettes
const LCD_CONTROL_START: u16 = 0xFF40;
const DMA: u16 = 0xFF46;
const BGP: u16 = 0xFF47;
const LCD_CONTROL_END: u16 = 0xFF4B;
//const KEY1: u16 = 0xFF4D; //CGB see 2.6.2
//const VRAM_BANK_SELECT: u16 = 0xFF4F;
const BOOT_ROM_LOCK: u16 = 0xFF50;

const VRAM_DMA_START: u16 = 0xFF51;
const VRAM_DMA_END: u16 = 0xFF55;

//const BACKGROUND_OBJ_PALETTES_START: u16 = 0xFF68;
//const BACKGROUND_OBJ_PALETTES_END: u16 = 0xFF69;

//const WRAM_BANK_SELECT: u16 = 0xFF70;
//TODO: CGB WRAM BANK SELECT is a register to switch between banks 1-7 see
//2.6.1
const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

const KIB_IN_BYTE: u64 = 1024;

pub struct DMG {
    pub rom: Vec<u8>,
    pub eram: Vec<u8>,
    pub wram: Vec<u8>,
    pub interrupts: Interrupts,
    pub timer: Timer,
}

impl Memory for DMG {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            rom: data,
            eram: vec![0; 8 * KIB_IN_BYTE as usize],
            wram: vec![0; 8 * KIB_IN_BYTE as usize],
            interrupts: Interrupts::empty(),
            timer: Timer::new(),
        }
    }

    fn get_eram(data: &Vec<u8>) -> Vec<u8> {
        //TODO: load the eram from rom here
        data.clone()
    }

    /// Maps the address to the correct memory area
    // To become read8
    pub fn read8(&self, address: u16) -> u8 {
        match address {
            ROM0_START..=ROM0_END => self.rom[address as usize],
            ROM1_START..=ROM1_END => {
              //TODO: Implement bank switching
              self.rom[address as usize]
            },
            VIDEO_RAM_START..=VIDEO_RAM_END => {

              //0x8000 - 0x87FF: Block 0 -> 
              //0x8800 - 0x8FFF: Block 1 -> 
              //0X9000 - 0x9FFF: Block 2 -> 
                // 0x9800 - 0x9BFF: 32x32 tile map 1
                // 0x9C00 - 0x9FFF: 32x32 tile map 2 
                // Any of these maps can be used to display the Background or
                // the Window
                // Each tile map contains the 1-byte indexes of the tiles to be
                // displayed
            },
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
              //TODO: if cartridge has extra RAM, maps this extra RAM here
              self.eram[(address - EXTERNAL_RAM_START) as usize]
            },
            WORK_RAM_00_START..=WORK_RAM_01_END => {
              //TODO: free ram for the game to use
              self.wram[(address - WORK_RAM_00_START) as usize]
            },
            ECHO_RAM_START..=ECHO_RAM_END => {
              //Reads to work ram instead
              return self.read8(address - 0x2000);
            },
            SPRITE_ATTRIBUTE_TABLE_START..=SPRITE_ATTRIBUTE_TABLE_END => {
              todo!()
            },
            INPUT_OUTPUT_REGISTERS_START..=INPUT_OUTPUT_REGISTERS_END => match address {
                JOYPAD_INPUT => todo!(),
                TIMER_DIVIDER_START..=TIMER_DIVIDER_END => self.timer.get_register(address),
                AUDIO_START..=AUDIO_END => todo!(),
                WAVE_PATTERN_START..=WAVE_PATTERN_END => todo!(),
                LCD_CONTROL_START..=LCD_CONTROL_END => {
                    match address {
                      //0xFF40-0XFF4B;
                      LCDC => todo!(), //0xFF40
                      STAT => todo!(), //0xFF41
                      SCY => todo!(), //0xFF42
                      SCX => todo!(), //0xFF43
                      LY => todo!(), //0xFF44
                      LYC => todo!(), //0xFF45
                      DMA => todo!(), //0xFF46

                      BCPS => todo!(), //0xFF68
                      BCPD => todo!(), //0xFF69
                      OCPS => todo!(), //0xFF6A
                      OCPD => todo!(), //0xFF6B

                      BGP => 
                    },
                },
                BOOT_ROM_LOCK => todo!(),
                _ => panic!("Forbidden to read at address: {:#06x}", address),
            },
            HIGH_RAM_START..=HIGH_RAM_END => todo!(),
            INTERRUPT_ENABLE_REGISTER => todo!(),
            _ => panic!("Forbidden to read at address:{:#06x}",address),
        }
    }

    // To become write8
    pub fn write8(&mut self, address: u16, value: u8) {
        match address {
            ROM0_START..=ROM1_END => self.rom[address as usize] = value,
            VIDEO_RAM_START..=VIDEO_RAM_END => todo!(),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => self.eram[(address -
            EXTERNAL_RAM_START) as usize] = value,
            WORK_RAM_00_START..=WORK_RAM_01_END => self.wram[(address -
            WORK_RAM_00_START) as usize] = value,
            ECHO_RAM_START..=ECHO_RAM_END => {
              // writes to work RAM instead
              self.write8(address - 0x2000, value)
            }
            SPRITE_ATTRIBUTE_TABLE_START..=SPRITE_ATTRIBUTE_TABLE_END => {
                todo!();
            }
            INPUT_OUTPUT_REGISTERS_START..=INPUT_OUTPUT_REGISTERS_END => match address {
                JOYPAD_INPUT => todo!(),
                TIMER_DIVIDER_START..=TIMER_DIVIDER_END =>
                self.timer.set_register(address, value),
                LCD_CONTROL_START..=LCD_CONTROL_START => todo!(),
                BOOT_ROM_LOCK => todo!(),
              _ => panic!("Forbidden to write at address:
              {:#06x}",address)
            },
            HIGH_RAM_START..=HIGH_RAM_END => todo!(),
            INTERRUPT_ENABLE_REGISTER => todo!(),
              _ => panic!("Forbidden to write at address:
              {:#06x}",address)
        }
    }

}

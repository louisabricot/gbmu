//!

use super::interrupts::Interrupts;

// From cartridge, usually a fixed bank
const ROM_BANK_00_START: u16 = 0x0000;
const INTERRUPT_ADDRESS: u16 = 0x0000;
const ROM_DATA_AREA: u16 = 0x100;

const ROM_BANK_00_END: u16 = 0x3FFF;

// From cartridge, switchable bank via mapper
const ROM_BANK_01_START: u16 = 0x4000;
const ROM_BANK_01_END: u16 = 0x7FFF;

// In CGB mode, switchable bank 0/1
const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

// From cartridge, switchable bank if any
const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END: u16 = 0xBFFF;

const WORK_RAM_00_START: u16 = 0xC000;
const WORK_RAM_00_END: u16 = 0xCFFF;

// In CGB mode, switchable bank 1~7
const WORK_RAM_01_START: u16 = 0xD000;
const WORK_RAM_01_END: u16 = 0xDFFF;

// Mirror of C000~DDFF, Nintendo says use of this area is prohibited
const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

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

const AUDIO_START: u16 = 0xFF10;
const AUDIO_END: u16 = 0xFF26;

const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;

// LCD Control, status, position, scrolling, and palettes
const LCD_CONTROL_START: u16 = 0xFF40;
const LCD_CONTROL_END: u16 = 0xFF4B;

//const VRAM_BANK_SELECT: u16 = 0xFF4F;
const BOOT_ROM_LOCK: u16 = 0xFF50;

const VRAM_DMA_START: u16 = 0xFF51;
const VRAM_DMA_END: u16 = 0xFF55;

//const BACKGROUND_OBJ_PALETTES_START: u16 = 0xFF68;
//const BACKGROUND_OBJ_PALETTES_END: u16 = 0xFF69;

//const WRAM_BANK_SELECT: u16 = 0xFF70;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

pub struct MemoryMap {
    pub memory: Vec<u8>,
    pub interrupts: Interrupts,
}

impl MemoryMap {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            memory: data,
            interrupts: Interrupts::empty(),
        }
    }

    /// Maps the address to the correct memory area
    // To become read8
    pub fn map(&mut self, address: u16) -> Result<u8, String> {
        match address {
            ROM_BANK_00_START..=ROM_BANK_00_END => Ok(self.memory[address as
            usize]),
            ROM_BANK_01_START..=ROM_BANK_01_END => {
              //TODO: Implement bank switching
              Ok(self.memory[address as usize],
            },
            VIDEO_RAM_START..=VIDEO_RAM_END => println!("video ram"),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
              //TODO: if cartridge has extra RAM, maps this extra RAM here
              Ok(self.eram[address - EXTERNAL_RAM_START as usize]),
            },
            WORK_RAM_00_START..=WORK_RAM_00_END => {
              //TODO: free ram for the game to use
              Ok(self.wram0[address - WORK_RAM_00_START] as usize),
            },
            WORK_RAM_01_START..=WORK_RAM_01_END => {
              //TODO: free ram for the game to use
              Ok(self.wram1[address - WORK_RAM_01_START] as usize),
            },
            ECHO_RAM_START..=ECHO_RAM_END => {
              //Reads to work ram instead
              self.map(address - 0x2000);
            }
            SPRITE_ATTRIBUTE_TABLE_START..=SPRITE_ATTRIBUTE_TABLE_END => {
                println!("sprite attribute table")
            }
            INPUT_OUTPUT_REGISTERS_START..=INPUT_OUTPUT_REGISTERS_END => match address {
                JOYPAD_INPUT => todo!(),
                SERIAL_TRANSFER_START..=SERIAL_TRANSFER_END => {
                    //y aura pas de communication entre gameboys
                }
                TIMER_DIVIDER_START..=TIMER_DIVIDER_END => todo!(),
                AUDIO_START..=AUDIO_END => todo!(),
                WAVE_PATTERN_START..=WAVE_PATTERN_END => todo!(),
                LCD_CONTROL_START..=LCD_CONTROL_START => todo!(),
                BOOT_ROM_LOCK => todo!(),
                _ => Err("Reading this area is forbidden"),
            },
            HIGH_RAM_START..=HIGH_RAM_END => println!("high ram"),
            INTERRUPT_ENABLE_REGISTER => println!("interrupt enable register"),
            _ => Err("Reading this area is forbidden"),
        }
    }

    // To become write8
    pub fn map8(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            VIDEO_RAM_START..=VIDEO_RAM_END => println!("video ram"),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => println!("video ram"),
            WORK_RAM_00_START..=WORK_RAM_00_END => {
              self.memory[pc as usize] = value;
              Ok()
            }
            WORK_RAM_01_START..=WORK_RAM_01_END => println!("video ram"),
            ECHO_RAM_START..=ECHO_RAM_END => {
              // writes to work RAM instead
              self.map8(address - 0x2000, value);
            }
            SPRITE_ATTRIBUTE_TABLE_START..=SPRITE_ATTRIBUTE_TABLE_END => {
                todo!();
            }
            INPUT_OUTPUT_REGISTERS_START..=INPUT_OUTPUT_REGISTERS_END => match address {
                JOYPAD_INPUT => todo!(),
                SERIAL_TRANSFER_START..=SERIAL_TRANSFER_END => {
                    //y aura pas de communication entre gameboys
                }
                TIMER_DIVIDER_START..=TIMER_DIVIDER_END => {
                    todo!()
                }
                AUDIO_START..=AUDIO_END => {
                    //y aura pas d'audio yet
                }
                WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                    //y aura pas d'audio yet
                }
                LCD_CONTROL_START..=LCD_CONTROL_START => todo!(),
                BOOT_ROM_LOCK => todo!(),
                _ => Err("Writing to this area is forbidden"),
            },
            HIGH_RAM_START..=HIGH_RAM_END => println!("high ram"),
            INTERRUPT_ENABLE_REGISTER => println!("interrupt enable register"),
        }
    }
    /// Reads the 8-bit value at address pc
    pub fn read8(&self, pc: u16) -> u8 {
        self.memory[pc as usize]
    }

    /// Reads the 16-bit value at address pc
    /// Returns a native endian value
    pub fn read16(&self, pc: u16) -> u16 {
        let hi = self.read8(pc);
        let lo = self.read8(pc + 1);
        u16::from_le_bytes([hi, lo])
    }

    /// Writes at address pc the u8 value given as parameter
    pub fn write8(&mut self, pc: u16, value: u8) {
        self.memory[pc as usize] = value;
    }

    /// Write at address pc the u16 value converted into little endian
    pub fn write16(&mut self, pc: u16, value: u16) {
        let bytes = value.to_le_bytes();

        self.memory[pc as usize] = bytes[0];
        self.memory[pc as usize + 1] = bytes[1];
    }
}

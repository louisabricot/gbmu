//!

use super::interrupts::Interrupts;

// From cartridge, usually a fixed bank
const ROM0_START_ADDRESS: u16 = 0x0000;
const INTERRUPT_ADDRESS: u16 = 0x0000;
const ROM_DATA_AREA: u16 = 0x100;

const ROM0_END_ADDRESS: u16 = 0x3FFF;

// From cartridge, switchable bank via mapper
const ROM1_START_ADDRESS: u16 = 0x4000;
const ROM1_END_ADDRESS: u16 = 0x7FFF;

// In CGB mode, switchable bank 0/1
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

const DIVIDER_REGISTER_ADDRESS: u16 = 0xFF04;
const TIMER_COUNTER_ADDRESS: u16 = 0xFF05;
const TIMER_MODULO_ADDRESS: u16 = 0xFF06;
const TIMER_CONTROL_ADDRESS: u16 = 0xFF07;

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

const KIB_IN_BYTE: u64 = 1024;
const DMG_WORK_RAM_SIZE: u64 = 8 * KIB_IN_BYTE;

pub struct MemoryMap {
    pub memory: Vec<u8>,
    pub eram: Vec<u8>,

    //TODO: check if really need two work ram 
    pub wram: Vec<u8>,
    pub interrupts: Interrupts,
    pub timer: Timer,
}

impl MemoryMap {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            memory: data,
            eram: self.get_eram(data),
            wram: Vec::with_capacity(DMG_WORK_RAM_SIZE),
            interrupts: Interrupts::empty(),
        }
    }

    fn get_eram(data: Vec<u8>) -> Vec<u8> {
        todo!()
    }

    /// Maps the address to the correct memory area
    // To become read8
    pub fn map(&mut self, address: u16) -> Result<u8, String> {
        match address {
            ROM0_START_ADDRESS..=ROM0_END_ADDRESS => Ok(self.memory[address as
            usize]),
            ROM1_START_ADDRESS..=ROM1_END_ADDRESS => {
              //TODO: Implement bank switching
              Ok(self.memory[address as usize]),
            }
            VIDEO_RAM_START..=VIDEO_RAM_END => println!("video ram"),
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
              //TODO: if cartridge has extra RAM, maps this extra RAM here
              Ok(self.eram[address - EXTERNAL_RAM_START as usize]),
            }
            WORK_RAM_00_START..=WORK_RAM_01_END => {
              //TODO: free ram for the game to use
              Ok(self.wram[address - WORK_RAM_00_START] as usize),
            }
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
                TIMER_DIVIDER_START..=TIMER_DIVIDER_END => {
                  match address {
                    DIVIDER_REGISTER => Ok(self.timer.divider),
                    TIMER_COUNTER => Ok(self.timer.counter),
                    TIMER_MODULO => Ok(self.timer.modulo),
                    TIMER_CONTROLLER => Ok(self.timer.controller),
                  }
                },
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
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => self.eram[address -
            EXTERNAL_RAM_START],
            WORK_RAM_00_START..=WORK_RAM_01_END => self.wram[address - WORK_RAM_00_START as usize] = value,
            ECHO_RAM_START..=ECHO_RAM_END => {
              // writes to work RAM instead
              Ok(self.map8(address - 0x2000, value))
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
                  match address {
                    // Writing to the divider register clears its bits to 0.  
                    DIVIDER_REGISTER_ADDRESS => self.timer.divider = 0,

                    TIMER_COUNTER_ADDRESS => self.timer.counter = value,
                    TIMER_MODULO_ADDRESS => self.timer.modulo = value,
                    
                    // Bit 3-7 of timer controller are unused
                    TIMER_CONTROLLER_ADDRESS => self.timer.controller =
                    value.bitand_assign(0b00000111),
                  }
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

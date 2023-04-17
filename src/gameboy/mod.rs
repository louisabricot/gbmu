use self::cpu::Cpu;
use self::memory::MemoryMap;
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

pub struct GameBoy {
    model: Model,
    speed: SpeedMode,
    cpu: Cpu,
}

enum SpeedMode {
    Double,
    Normal,
}

pub enum Model {
    DMG,
    CGB,
}

impl GameBoy {
    pub fn new(cartridge: MemoryMap) -> Self {
        Self {
            model: Model::DMG,
            speed: SpeedMode::Normal,
            cpu: Cpu::new(cartridge),
        }
    }

    ///
    pub fn boot(&mut self) {
        //unpack the logo from the header
        let logo = self.get_logo();

        self.scroll(logo);

        //ba-ding!

        //check logo header checksums
        if !self.check_logo() || !self.check_header_checksum() {
            println!("invalid logo or checksum");
        }
        println!("passes control to the cartridge");

        // passes control to the cartridge
        self.cpu.set_program_counter(PROGRAM_START_ADDRESS);
    }
    
    pub fn scroll(&self, logo: Vec<u8>) {
    }

    pub fn get_logo(&self) -> Vec<u8> {
        let mut logo = Vec::new();

        for i in CARTRIDGE_NINTENDO_LOGO_START..=CARTRIDGE_NINTENDO_LOGO_END {
            logo.push(self.cpu.memory.read8(i));
        }
        logo
    }

    // cgb checks half of logo
    pub fn check_logo(&self) -> bool {
        for i in 0..NINTENDO_LOGO_SIZE {
            if self.cpu.memory.read8(CARTRIDGE_NINTENDO_LOGO_START + i) != NINTENDO_LOGO[i as usize]
            {
                return false;
            }
        }
        true
    }

    pub fn check_header_checksum(&self) -> bool {
        let mut checksum: u8 = 0;

        for i in CARTRIDGE_TITLE..CARTRIDGE_HEADER_CHECKSUM {
            checksum = checksum.wrapping_sub(self.cpu.memory.read8(i));
            checksum = checksum.wrapping_sub(1);
        }
        checksum == self.cpu.memory.read8(CARTRIDGE_HEADER_CHECKSUM)
    }

    pub fn get_program_counter(&self) -> u16 {
        self.cpu.registers.pc
    }

    pub fn step(&mut self) {
        self.cpu.step()
    }

    pub fn get_registers(&self) -> Vec<String> {
        let mut contents = Vec::new();

        contents.push(format!("{}: {:#04x}", "A", self.cpu.registers.a));
        contents.push(format!("{}: {:#04x}", "B", self.cpu.registers.b));
        contents.push(format!("{}: {:#04x}", "C", self.cpu.registers.c));
        contents.push(format!("{}: {:#04x}", "D", self.cpu.registers.d));
        contents.push(format!("{}: {:#04x}", "E", self.cpu.registers.e));
        contents.push(format!("{}: {:#04x}", "H", self.cpu.registers.h));
        contents.push(format!("{}: {:#04x}", "L", self.cpu.registers.l));
        contents
    }

    pub fn get_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        flags.push(format!("{}: {:08b}", "F", self.cpu.registers.f.bits()));
        flags
    }

    pub fn disassemble(&self, lines: u16, mut address: u16) -> Vec<String> {
        self.cpu.disassemble(lines, address)
    }
}

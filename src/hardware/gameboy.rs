use crate::hardware::cpu::Cpu;
use crate::hardware::memory::Memory;

const NINTENDO_LOGO: [u8; 48] = [
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
    pub fn new(cartridge: Memory) -> Self {
        Self {
            model: GameBoy::check_cbg_flag(&cartridge),
            speed: SpeedMode::Normal,
            cpu: Cpu::new(cartridge),
        }
    }

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
    }
    pub fn scroll(&self, logo: Vec<u8>) {
        println!("Scrolling logo");
    }

    pub fn get_logo(&self) -> Vec<u8> {
        let mut logo = Vec::new();

        for i in 0x0104..0x0134 {
            logo.push(self.cpu.memory.read8(i));
        }
        logo
    }

    pub fn check_cbg_flag(cartridge: &Memory) -> Model {
        match cartridge.read8(0x143) {
            0xC0 => Model::CGB,
            _ => Model::DMG,
        }
    }

    // cgb checks half of logo
    pub fn check_logo(&self) -> bool {
        let address: u16 = 0x104;
        for i in 0..47 {
            if self.cpu.memory.read8(address + i) != NINTENDO_LOGO[i as usize] {
                println!(
                    "{} vs {}",
                    self.cpu.memory.read8(address + i),
                    NINTENDO_LOGO[i as usize]
                );
                return false;
            }
        }
        return true;
    }

    pub fn check_header_checksum(&self) -> bool {
        let mut checksum: u8 = 0;

        for i in 0x0134..0x014D {
            checksum = checksum.wrapping_sub(self.cpu.memory.read8(i));
            checksum = checksum.wrapping_sub(1);
        }

        if checksum == self.cpu.memory.read8(0x014D) {
            return true;
        } else {
            return false;
        }
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

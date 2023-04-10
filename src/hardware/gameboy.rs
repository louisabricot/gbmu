use crate::hardware::cpu::Cpu;
use crate::hardware::memory::Memory;

pub struct GameBoy {
    model: Model,
    speed: SpeedMode,
    cpu: Cpu,
}

enum SpeedMode {
    Double,
    Normal,
}

enum Model {
    DMG,
    CGB,
}

impl GameBoy {
    pub fn new(cartridge: Memory) -> Self {
        Self {
            model: Model::DMG,
            speed: SpeedMode::Normal,
            cpu: Cpu::new(cartridge),
        }
    }

    pub fn check_logo(cartridge: Vec<u8>) -> bool {
        todo!()
    }

    pub fn check_header_checksum(cartridge: &Memory) -> bool {
        let mut checksum: u8 = 0;

        for i in 0x0134..0x014D {
            checksum = checksum.wrapping_sub(cartridge.read8(i));
            checksum = checksum.wrapping_sub(1);
            println!("checksum {} {}", checksum, cartridge.read8(i));
        }

        if checksum == cartridge.read8(0x014D) {
            println!(
                "{} vs {}: Cartridge verified",
                checksum,
                cartridge.read8(0x014D)
            );
            return true;
        } else {
            println!(
                "{} vs {}: Cartridge unverified",
                checksum,
                cartridge.read8(0x014D)
            );
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

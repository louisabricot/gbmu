//! A basic debugger interface to the CPU.  
//!
//! Provides an interactive disassembler, to display instructions and CPU registers.   

/// `Debugger` takes a reference to the CPU.  

impl Debugger for Cpu {

    /// Triggers a fetch-decode-execute cycle. 
    /// See [CPU::step()]
    pub fn step(&mut self) {
        self.cpu.step()
    }

    /// Returns a vector of Strings with the content of the 8-bit CPU registers in hexadecimal format.  
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

    /// Returns a vector of Strings with the content of the CPU's flag register in binary format.  
    pub fn get_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        flags.push(format!("{}: {:08b}", "F", self.cpu.registers.f.bits()));
        flags
    }

    /// Fetches and decodes *nb* instructions from *addresss*.  
    /// See [Cpu::disassemble_instruction()]
    pub fn disassemble_instruction(&self, n: u16, mut address: u16) -> Vec<String> {
        self.cpu.disassemble_instruction(n, address)
    }
}

pub struct Cpu {
    pc: u16,
}

impl Cpu {
    
    pub fn new(pc: u16 ) -> Self {
        Self { pc }
    }

    pub fn run(&mut self, rom: Vec<u8>, rom_size: u16) -> u16 {

        while self.pc < rom_size {
            let opcode = self.fetch(&rom);
            print!("{:x?} ", opcode);
            self.pc += 1;
        }
        return self.pc
    }

    fn fetch(&mut self, rom: &Vec<u8>) -> u8 {
        return self.next_byte(&rom)
        //parse u8 and return the Opcode associated
    }

    fn next_byte(&mut self, rom: &Vec<u8>) -> Byte  {
        let i = self.pc as usize;
        let b = rom[i];
        self.pc += 1;
        return b
    }

    fn decode(&mut self, rom: &Vec<u8>, opcode: u8) -> Instruction {
        match opcode {
            
        }
    }

    fn execute(&mut self, ins: Instruction) {
        
    }
}

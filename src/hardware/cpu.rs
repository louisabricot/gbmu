pub struct Cpu {
    pc: u16,
}

pub enum State {
    RUNNING,
    HALT,
    INTERRUPT,
}

impl Cpu {
    
    pub fn new(pc: u16 ) -> Self {
        Self { pc }
    }

    /// Read the next byte from memory
    fn fetch(&mut self, memory: Memory) -> u8 {
        let opcode = self.read_imm8(memory);
        //if opcode = CB
        // read 1 byte more
        // CB prefixed instruction set
        //match opcode to OPCODE enum
        //return the opcode ?
    }

    fn fetch_imm16(&mut self, memory: Memory) -> u16 {
        //??? 
    }

    /// Read the next byte from memory
    /// Update PC
    fn read_imm8(&mut self, memory: Memory) -> u8 {
        let addr = self.pc;
        self.pc.wrapping_add(1);
        return memory.read_imm8(pc);
    }

    /// Read the next 16 bits from memory
    /// Convert the value from little endian to big endian
    /// Update PC
    fn read_imm16(&mut self, memory: Memory) -> u16 {
        let least = self.read_imm8(memory);
        let most = self.read_imm8(memory);
        return u16::from_le_bytes([least, most]);
    }

    /// Decode the opcode and fetch its 
    fn decode(&mut self, rom: &Vec<u8>, opcode: Opcode) -> Instruction {
        match opcode {
            //match opcode with Instruction
        }
    }

    /// Execute an instruction and returns the CPU state
    fn execute(&mut self, instruction: Instruction) -> Result<State, Error> {
        match instruction {
            Instruction::NOP => {
                //do nop
                Ok(State::Running),
            }
            _ => panic!("Instruction not implemented {:?}", instruction),
        }
    }

    pub fn step(&mut self, memory: Memory) -> Result<(), Error> {
        match self.state {
            State::RUNNING => {
                let opcode = self.fetch(memory);
                let instruction = self.decode(opcode, memory);
                self.state = self.execute(instruction);
            },
            State::HALT => {},
            State::INTERRUPT => {},
            _ => panic!("State not implemented! {:?}", self.state),
        }
    }
}


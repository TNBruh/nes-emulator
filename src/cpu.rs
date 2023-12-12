use crate::{OPCODES_MAP, opcode::{OpCode, OpCodeName}};


pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            register_a: 0, 
            register_x: 0,
            register_y: 0,
            status: 0, 
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => self.mem_read(self.program_counter).wrapping_add(self.register_x) as u16,
            AddressingMode::ZeroPage_Y => self.mem_read(self.program_counter).wrapping_add(self.register_y) as u16,
            AddressingMode::Absolute_X => self.mem_read_u16(self.program_counter).wrapping_add(self.register_x as u16),
            AddressingMode::Absolute_Y => self.mem_read_u16(self.program_counter).wrapping_add(self.register_y as u16),
            AddressingMode::Indirect_X => {
                let ptr: u8 = self.mem_read(self.program_counter).wrapping_add(self.register_x);
                (
                    self.mem_read(
                        ptr.wrapping_add(1) as u16
                    ) as u16
                ) << 8 | 
                (
                    self.mem_read(ptr as u16)
                ) as u16
            },
            AddressingMode::Indirect_Y => {
                let ptr: u8 = self.mem_read(self.program_counter).wrapping_add(self.register_y);
                (
                    self.mem_read(
                        ptr.wrapping_add(1) as u16
                    ) as u16
                ) << 8 | 
                (
                    self.mem_read(ptr as u16)
                ) as u16
            },
            AddressingMode::NonAddressing => {
                panic!("mode {:?} is not supported", mode);
            }

        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16::from_be_bytes([
            self.mem_read(addr+1),
            self.mem_read(addr)
        ])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let data_bytes = data.to_be_bytes();
        self.mem_write(addr, data_bytes[1]);
        self.mem_write(addr+1, data_bytes[0])
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            //saving the state for some reason?
            let temp_program_counter = self.program_counter;

            //coerced (i think that's the word): &&OpCode -> &OpCode
            let entry: &OpCode = OPCODES_MAP.get(&opscode).expect("WHERE IS MY SUPER SUIT?"); 

            match entry.name {
                OpCodeName::LDA => self.lda(entry),
                OpCodeName::LDX => self.ldx(entry),
                OpCodeName::STA => self.sta(entry),
                OpCodeName::TAX => self.tax(),
                OpCodeName::INX => self.inx(),
                OpCodeName::BRK => break,
                #[allow(unreachable_patterns)]
                _ => todo!("AMOGUS")
            }

            // idk why we do this. maybe it will be explained later
            if self.program_counter == temp_program_counter {
                self.program_counter += (entry.len - 1) as u16;
            }
        }
    }

    fn lda(&mut self, op: &OpCode) {
        let data = self.mem_read(
            self.get_operand_address(&op.mode)
        );
        self.register_a = data;

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, op: &OpCode) {
        let data = self.mem_read(
            self.get_operand_address(&op.mode)
        );
        self.register_x = data;

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, op: &OpCode) {
        let addr = self.get_operand_address(&op.mode);

        self.mem_write(addr, self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        (self.register_x, _) = self.register_x.overflowing_add(1);

        self.update_zero_and_negative_flags(self.register_x);
    }

    /* RM: C 3 P 2
    // fn lda(&mut self, mode: &AddressingMode) {
    //     self.register_a = self.mem_read(
    //         self.get_operand_address(mode)
    //     );

    //     self.update_zero_and_negative_flags(self.register_a)
    // }

    // fn ldx(&mut self, mode: &AddressingMode) {
    //     self.register_x = self.mem_read(
    //         self.get_operand_address(mode)
    //     );

    //     self.update_zero_and_negative_flags(self.register_x)
    // }

    // fn sta(&mut self, mode: &AddressingMode) {
    //     let addr = self.get_operand_address(mode);
    //     self.mem_write(addr, self.register_a)
    // }

    // fn tax(&mut self) {
    //     self.register_x = self.register_a;
    //     self.update_zero_and_negative_flags(self.register_x);
    // }

    // fn inx(&mut self) {
    //     (self.register_x, _) = self.register_x.overflowing_add(1);
    //     self.update_zero_and_negative_flags(self.register_x);
    // }
    */

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 { // if zero
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b0000_0010;
        }

        if result & 0b1000_0000 != 0 { //if negative
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0000_0010;
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NonAddressing
}
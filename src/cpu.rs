use crate::{OPCODES_MAP, opcode::{OpCode, OpCodeName}};
use bitflags::bitflags;

const STACK_ORIGIN: u16 = 0x01FF; // stack grows down and ends at 0x100. overflow will cause it to wrap back

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CPUStatus,
    pub stack_pointer: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            register_a: 0, 
            register_x: 0,
            register_y: 0,
            status: CPUStatus::empty(), 
            stack_pointer: 0,
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
                panic!("Caught you tweaking with {:?}.", mode);
            }

        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data
    }

    fn mem_read_u16(&self, addr: u16) -> u16 { // returns little endian
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
        self.status = CPUStatus::empty();
        self.stack_pointer = 0;

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

            // debug print
            println!("RUNNING OPCODE: {:#02x}", opscode);

            //saving the state for some reason?
            // answer's below
            let temp_program_counter = self.program_counter;

            //coerced (i think that's the word): &&OpCode -> &OpCode
            let entry: &OpCode = OPCODES_MAP.get(&opscode).expect("WHERE IS MY SUPER SUIT?"); 

            match entry.name {
                /*
                remember to:
                1. update OpCodeName
                2. update the OPCODES hashmap
                3. double check the corresponding byte
                 */
                OpCodeName::LDA => self.lda(entry),
                OpCodeName::LDX => self.ldx(entry),
                OpCodeName::STA => self.sta(entry),
                OpCodeName::TAX => self.tax(),
                OpCodeName::INX => self.inx(),
                OpCodeName::PHA => self.pha(),
                OpCodeName::PHP => self.php(),
                OpCodeName::JSR => self.jsr(entry),
                OpCodeName::RTS => self.rts(),
                OpCodeName::PLA => self.pla(),
                OpCodeName::PLP => self.plp(),
                OpCodeName::RTI => self.rti(),
                OpCodeName::ADC => self.adc(entry),
                OpCodeName::AND => self.and(entry),
                OpCodeName::ASL => self.asl(entry),
                OpCodeName::BCC => self.bcc(entry),
                OpCodeName::BCS => self.bcs(entry),
                OpCodeName::BEQ => self.beq(entry),
                OpCodeName::BIT => self.bit(entry),
                OpCodeName::BMI => self.bmi(entry),
                OpCodeName::BNE => self.bne(entry),
                OpCodeName::BRK => {
                    // might implement this last
                    self.brk();
                    break;
                } // todo: update this
                // if there's no warning about unreachable pattern, then you know why
                _ => todo!("ඞ AMOGUS JUMPSCARE ඞ")
            }

            // idk why we do this. maybe it will be explained later
            // ANSWER: it's not explained. but JSR is one of the reason
            if self.program_counter == temp_program_counter {
                self.program_counter += (entry.len - 1) as u16;
            }
        }
    }

    fn lda(&mut self, op: &OpCode) {
        self.register_a = self.mem_read(
            self.get_operand_address(&op.mode)
        );

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

    fn pha(&mut self) {
        self.push(self.register_a);
    }

    fn php(&mut self) { // i'm supposed to do something about the break flag? yet the wiki only says to push the status reg. oh well
        self.push(self.status.bits());
    }

    fn jsr(&mut self, op: &OpCode) {
        let jmp_addr = self.get_operand_address(&op.mode);
        let rtn_addr = (self.program_counter + 1).to_be_bytes();
        self.push(rtn_addr[0]);
        self.push(rtn_addr[1]);

        self.program_counter = jmp_addr;
    }

    fn rts(&mut self) {
        self.program_counter = u16::from_le_bytes([
            self.pop(),
            self.pop()
        ]).wrapping_add(1);
    }

    fn pla(&mut self) {
        self.register_a = self.pop();

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        self.status = CPUStatus::from_bits(self.pop()).expect("FAILED TO COERCE STATUS IN PLP");
    }

    fn rti(&mut self) {
        self.status = CPUStatus::from_bits(self.pop()).expect("FAILED TO COERCE STATUS IN RTI");
        self.program_counter = u16::from_le_bytes([
            self.pop(),
            self.pop()
        ]);
    }

    // read :)
    // https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    fn adc(&mut self, op: &OpCode) {
        let (m, n) = (self.register_a, self.mem_read(self.get_operand_address(&op.mode)));
        let data: u16 = m as u16 + 
                        n as u16 + 
                        if self.status.contains(CPUStatus::Carry) { 1 } else { 0 };
        
        self.register_a = data as u8;
        self.update_zero_and_negative_flags(self.register_a);
        self.status.set(CPUStatus::Carry, data > 0xFF);
        self.status.set(CPUStatus::Overflow, 
            (m ^ (data as u8)) & (n ^ (data as u8)) & 0x80 != 0x0
        );
    }

    fn and(&mut self, op: &OpCode) {
        self.register_a &= self.mem_read(self.get_operand_address(&op.mode));

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, op: &OpCode) {
        match &op.mode {
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X |
            AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let addr = self.get_operand_address(&op.mode);
                let data = (
                    self.mem_read(addr) 
                    as u16
                ) << 1;

                self.mem_write(addr, data as u8);

                self.status.set(CPUStatus::Carry, data > 0xFF);
                self.update_zero_and_negative_flags(data as u8);
            },
            AddressingMode::NonAddressing => { //this will catch the ASL with accumulator
                let data = (self.register_a as u16) << 1;
        
                self.register_a = data as u8;

                self.status.set(CPUStatus::Carry, data > 0xFF);
                self.update_zero_and_negative_flags(data as u8);
            },
            _ => self.unknown_opcode_crash(op),
        }
        
    }

    fn bcc(&mut self, op: &OpCode) {
        if !self.status.contains(CPUStatus::Carry) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn bcs(&mut self, op: &OpCode) {
        if self.status.contains(CPUStatus::Carry) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn beq(&mut self, op: &OpCode) {
        if self.status.contains(CPUStatus::Zero) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn bit(&mut self, op: &OpCode) {
        let m = self.mem_read(
            self.get_operand_address(&op.mode)
        );
        let data = self.register_a & m;

        if data == 0 {
            self.status.insert(CPUStatus::Zero);   
        }

        if m & 0b1000_0000 > 0 {
            self.status.insert(CPUStatus::Negative);
        } else {
            self.status.remove(CPUStatus::Negative);
        }

        if m & 0b0100_0000 > 0 {
            self.status.insert(CPUStatus::Overflow);
        } else {
            self.status.remove(CPUStatus::Overflow);
        }
    }

    fn bmi(&mut self, op: &OpCode) {
        if self.status.contains(CPUStatus::Negative) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn bne(&mut self, op: &OpCode) {
        if !self.status.contains(CPUStatus::Zero) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn bpl(&mut self, op: &OpCode) {
        if !self.status.contains(CPUStatus::Negative) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    // read :) again
    // https://web.archive.org/web/20200129081101/http://users.telenet.be:80/kim1-6502/6502/proman.html#911
    fn brk(&mut self) {

    }

    fn bvc(&mut self, op: &OpCode) {
        if !self.status.contains(CPUStatus::Overflow) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn bvs(&mut self, op: &OpCode) {
        if self.status.contains(CPUStatus::Overflow) {
            self.branch(op, self.mem_read(self.program_counter) as i8)
        }
    }

    fn clc(&mut self) {
        self.status.remove(CPUStatus::Carry)
    }
    
    fn cld(&mut self) {
        self.status.remove(CPUStatus::Decimal)
    }

    fn cli(&mut self) {
        self.status.remove(CPUStatus::InterruptDisable)
    }

    fn clv(&mut self) {
        self.status.remove(CPUStatus::Overflow)
    }

    fn cmp(&mut self, op: &OpCode) {
        let res = (self.register_a as i16).wrapping_sub(
            self.mem_read(self.get_operand_address(&op.mode)) as i16
        );
        if res == 0 {
            self.status.set(CPUStatus::Zero, true);
        }
        if res & 0b0000_0000_1000_0000 != 0 {
            self.status.set(CPUStatus::Negative, true);
        } else {
            self.status.set(CPUStatus::Carry, true);
        }


    }

    fn cpx(&mut self, op: &OpCode) {
        let res = (self.register_x as i16).wrapping_sub(
            self.mem_read(self.get_operand_address(&op.mode)) as i16
        );
        if res == 0 {
            self.status.set(CPUStatus::Zero, true);
        }
        if res & 0b0000_0000_1000_0000 != 0 {
            self.status.set(CPUStatus::Negative, true);
        } else {
            self.status.set(CPUStatus::Carry, true);
        }
    }

    // stack
    fn push(&mut self, data: u8) {
        self.mem_write(self.get_stack_pointer(), data);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    fn pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mem_read(self.get_stack_pointer())
    }
    
    // branch
    fn branch(&mut self, op: &OpCode, dist: i8) {
        self.program_counter = self.program_counter.wrapping_add((op.len - 1) as u16).wrapping_add_signed(dist as i16);
    }

    pub fn get_stack_pointer(&self) -> u16 {
        STACK_ORIGIN - self.stack_pointer as u16
    }

    // flag updates
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 { // if zero
            self.status.insert(CPUStatus::Zero);
        } else {
            self.status.remove(CPUStatus::Zero);
        }

        if result & 0b1000_0000 != 0 { //if negative
            self.status.insert(CPUStatus::Negative);
        } else {
            self.status.remove(CPUStatus::Negative);
        }
    }

    // jarvis, uninstall his life
    fn unknown_opcode_crash(&self, op: &OpCode) -> ! {
        todo!("{:#?}: {:02x}", op.mode, op.byte)
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

/*
7  bit  0
---- ----
NV1B DIZC
|||| ||||
|||| |||+- Carry
|||| ||+-- Zero
|||| |+--- Interrupt Disable
|||| +---- Decimal
|||+------ (No CPU effect; see: the B flag)
||+------- (No CPU effect; always pushed as 1)
|+-------- Overflow
+--------- Negative
*/
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CPUStatus: u8 {
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const Decimal = 0b0000_1000;
        const Break = 0b0001_0000;
        const Unused = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}
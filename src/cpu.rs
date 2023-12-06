pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            register_a: 0, 
            register_x: 0,
            status: 0, 
            program_counter: 0 
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param)
                },
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!("AMOGUS")
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        (self.register_x, _) = self.register_x.overflowing_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

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
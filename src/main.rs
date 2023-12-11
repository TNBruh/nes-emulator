use std::collections::HashMap;

use lazy_static::lazy_static;
use opcode::OpCode;
use cpu::AddressingMode;

pub mod cpu;
pub mod opcode;

// pub static OPCODES: &'static Vec<OpCode> = &vec![
//     OpCode{
//         byte: 0xA9,
//         name: "LDA",
//         len: 2,
//         cycles: 2,
//         mode: cpu::AddressingMode::Immediate,
//     },
    
// ];

lazy_static!{
    pub static ref OPCODES: Vec<OpCode> = vec![
        // LDA
        OpCode {
            byte: 0xA9,
            name: "LDA",
            len: 2,
            cycles: 2,
            mode: AddressingMode::Immediate,
        },
        OpCode {
            byte: 0xA5,
            name: "LDA",
            len: 2,
            cycles: 3,
            mode: AddressingMode::ZeroPage,
        },
        OpCode {
            byte: 0xB5,
            name: "LDA",
            len: 2,
            cycles: 4,
            mode: AddressingMode::ZeroPage_X,
        },
        OpCode {
            byte: 0xAD,
            name: "LDA",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute,
        },
        OpCode {
            byte: 0xBD,
            name: "LDA",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute_X,
        },
        OpCode {
            byte: 0xB9,
            name: "LDA",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute_Y,
        },
        OpCode {
            byte: 0xA1,
            name: "LDA",
            len: 2,
            cycles: 6,
            mode: AddressingMode::Indirect_X,
        },
        OpCode {
            byte: 0xB1,
            name: "LDA",
            len: 2,
            cycles: 5,
            mode: AddressingMode::Indirect_Y,
        },
        // LDA END
        // LDX
        OpCode {
            byte: 0xA2,
            name: "LDX",
            len: 2,
            cycles: 2,
            mode: AddressingMode::Immediate,
        },
        OpCode {
            byte: 0xA6,
            name: "LDX",
            len: 2,
            cycles: 3,
            mode: AddressingMode::ZeroPage,
        },
        OpCode {
            byte: 0xB6,
            name: "LDX",
            len: 2,
            cycles: 4,
            mode: AddressingMode::ZeroPage_Y,
        },
        OpCode {
            byte: 0xAE,
            name: "LDX",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute,
        },
        OpCode {
            byte: 0xBE,
            name: "LDX",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute_Y,
        },
        // LDX END
        // STA
        OpCode {
            byte: 0x85,
            name: "STA",
            len: 2,
            cycles: 3,
            mode: AddressingMode::ZeroPage,
        },
        OpCode {
            byte: 0x95,
            name: "STA",
            len: 2,
            cycles: 4,
            mode: AddressingMode::ZeroPage_X,
        },
        OpCode {
            byte: 0x8D,
            name: "STA",
            len: 3,
            cycles: 4,
            mode: AddressingMode::Absolute,
        },
        OpCode {
            byte: 0x9D,
            name: "STA",
            len: 3,
            cycles: 5,
            mode: AddressingMode::Absolute_X,
        },
        OpCode {
            byte: 0x99,
            name: "STA",
            len: 3,
            cycles: 5,
            mode: AddressingMode::Absolute_Y,
        },
        OpCode {
            byte: 0x81,
            name: "STA",
            len: 2,
            cycles: 6,
            mode: AddressingMode::Indirect_X,
        },
        OpCode {
            byte: 0x91,
            name: "STA",
            len: 2,
            cycles: 6,
            mode: AddressingMode::Indirect_Y,
        },
        // STA END
        // TAX
        OpCode {
            byte: 0xAA,
            name: "TAX",
            len: 1,
            cycles: 2,
            mode: AddressingMode::NonAddressing,
        },
        // TAX END
        // INX
        OpCode {
            byte: 0xE8,
            name: "INX",
            len: 1,
            cycles: 2,
            mode: AddressingMode::NonAddressing,
        },
        // INX END
        // BRK
        OpCode {
            byte: 0x00,
            name: "BRK",
            len: 1,
            cycles: 7,
            mode: AddressingMode::NonAddressing,
        },
        // BRK END
        
        
        
        
        
    ];
    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map: HashMap<u8, &OpCode> = HashMap::new();
        OPCODES.iter().for_each(|x| {
            map.insert(x.byte, x);
        });
        map
    };
}

fn main() {
    let num: u16 = 0x1234;
    let le_num = num.to_le_bytes();

    println!("{:X} {:X}", le_num[0], le_num[1]);
    println!("{}", u16::from_be_bytes([0x01, 0x00]));
}

#[cfg(test)]
mod test {
    use crate::cpu::CPU;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() { // set reg a value
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() { // set reg x val
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xff, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }    
}

// #[cfg(test)]
// mod test {
//     use crate::cpu::CPU;

//     #[test]
//     fn test_0xa9_lda_immediate_load_data() {
//         let mut cpu = CPU::new();
//         cpu.interpret(vec![0xa9, 0x05, 0x00]);
//         assert_eq!(cpu.register_a, 0x05);
//         assert!(cpu.status & 0b0000_0010 == 0b00);
//         assert!(cpu.status & 0b1000_0000 == 0);
//     }

//     #[test]
//     fn test_0xa9_lda_zero_flag() {
//         let mut cpu = CPU::new();
//         cpu.interpret(vec![0xa9, 0x00, 0x00]);
//         assert!(cpu.status & 0b0000_0010 == 0b10);
//     }

//     #[test]
//     fn test_0xaa_tax_move_a_to_x() {
//         let mut cpu = CPU::new();
//         cpu.register_a = 10;
//         cpu.interpret(vec![0xaa, 0x00]);

//         assert_eq!(cpu.register_x, 10)
//     }

//     #[test]
//     fn test_5_ops_working_together() {
//         let mut cpu = CPU::new();
//         cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

//         assert_eq!(cpu.register_x, 0xc1)
//     }

//     #[test]
//     fn test_inx_overflow() {
//         let mut cpu = CPU::new();
//         cpu.register_x = 0xff;
//         cpu.interpret(vec![0xe8, 0xe8, 0x00]);

//         assert_eq!(cpu.register_x, 1)
//     }
// }
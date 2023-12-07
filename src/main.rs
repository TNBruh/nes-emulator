
pub mod cpu;

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
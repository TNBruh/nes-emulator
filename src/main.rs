use std::collections::HashMap;

use lazy_static::lazy_static;
use opcode::OpCode;
use cpu::AddressingMode;

use crate::opcode::OpCodeName;

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
        OpCode { byte:0xA9, name:OpCodeName::LDA, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0xA5, name:OpCodeName::LDA, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0xB5, name:OpCodeName::LDA, len:2, cycles:4, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0xAD, name:OpCodeName::LDA, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0xBD, name:OpCodeName::LDA, len:3, cycles:4, mode:AddressingMode::Absolute_X },
        OpCode { byte:0xB9, name:OpCodeName::LDA, len:3, cycles:4, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0xA1, name:OpCodeName::LDA, len:2, cycles:6, mode:AddressingMode::Indirect_X },
        OpCode { byte:0xB1, name:OpCodeName::LDA, len:2, cycles:5, mode:AddressingMode::Indirect_Y },
        OpCode { byte:0xA2, name:OpCodeName::LDX, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0xA6, name:OpCodeName::LDX, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0xB6, name:OpCodeName::LDX, len:2, cycles:4, mode:AddressingMode::ZeroPage_Y },
        OpCode { byte:0xAE, name:OpCodeName::LDX, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0xBE, name:OpCodeName::LDX, len:3, cycles:4, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0x85, name:OpCodeName::STA, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0x95, name:OpCodeName::STA, len:2, cycles:4, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0x8D, name:OpCodeName::STA, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0x9D, name:OpCodeName::STA, len:3, cycles:5, mode:AddressingMode::Absolute_X },
        OpCode { byte:0x99, name:OpCodeName::STA, len:3, cycles:5, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0x81, name:OpCodeName::STA, len:2, cycles:6, mode:AddressingMode::Indirect_X },
        OpCode { byte:0x91, name:OpCodeName::STA, len:2, cycles:6, mode:AddressingMode::Indirect_Y },
        OpCode { byte:0xAA, name:OpCodeName::TAX, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xE8, name:OpCodeName::INX, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x00, name:OpCodeName::BRK, len:1, cycles:7, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x48, name:OpCodeName::PHA, len:1, cycles:3, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x08, name:OpCodeName::PHP, len:1, cycles:3, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x20, name:OpCodeName::JSR, len:3, cycles:6, mode:AddressingMode::Absolute },
        OpCode { byte:0x60, name:OpCodeName::RTS, len:1, cycles:6, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x68, name:OpCodeName::PLA, len:1, cycles:4, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x28, name:OpCodeName::PLP, len:1, cycles:4, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x40, name:OpCodeName::RTI, len:1, cycles:6, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x69, name:OpCodeName::ADC, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0x65, name:OpCodeName::ADC, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0x75, name:OpCodeName::ADC, len:2, cycles:4, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0x6D, name:OpCodeName::ADC, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0x7D, name:OpCodeName::ADC, len:3, cycles:4, mode:AddressingMode::Absolute_X },
        OpCode { byte:0x79, name:OpCodeName::ADC, len:3, cycles:4, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0x61, name:OpCodeName::ADC, len:2, cycles:6, mode:AddressingMode::Indirect_X },
        OpCode { byte:0x71, name:OpCodeName::ADC, len:2, cycles:5, mode:AddressingMode::Indirect_Y },
        OpCode { byte:0x29, name:OpCodeName::AND, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0x25, name:OpCodeName::AND, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0x35, name:OpCodeName::AND, len:2, cycles:4, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0x2D, name:OpCodeName::AND, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0x3D, name:OpCodeName::AND, len:3, cycles:4, mode:AddressingMode::Absolute_X },
        OpCode { byte:0x39, name:OpCodeName::AND, len:3, cycles:4, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0x21, name:OpCodeName::AND, len:2, cycles:6, mode:AddressingMode::Indirect_X },
        OpCode { byte:0x31, name:OpCodeName::AND, len:2, cycles:5, mode:AddressingMode::Indirect_Y },
        OpCode { byte:0x0A, name:OpCodeName::ASL, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x06, name:OpCodeName::ASL, len:2, cycles:5, mode:AddressingMode::ZeroPage },
        OpCode { byte:0x16, name:OpCodeName::ASL, len:2, cycles:6, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0x0E, name:OpCodeName::ASL, len:3, cycles:6, mode:AddressingMode::Absolute },
        OpCode { byte:0x1E, name:OpCodeName::ASL, len:3, cycles:7, mode:AddressingMode::Absolute_X },
        OpCode { byte:0x90, name:OpCodeName::BCC, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xB0, name:OpCodeName::BCS, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xF0, name:OpCodeName::BEQ, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x2C, name:OpCodeName::BIT, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0x89, name:OpCodeName::BIT, len:2, cycles:3, mode:AddressingMode::Immediate },
        OpCode { byte:0x24, name:OpCodeName::BIT, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0x30, name:OpCodeName::BMI, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xD0, name:OpCodeName::BNE, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x10, name:OpCodeName::BPL, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x50, name:OpCodeName::BVC, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x70, name:OpCodeName::BVS, len:2, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x18, name:OpCodeName::CLC, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xD8, name:OpCodeName::CLD, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0x58, name:OpCodeName::CLI, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xB8, name:OpCodeName::CLV, len:1, cycles:2, mode:AddressingMode::NonAddressing },
        OpCode { byte:0xC9, name:OpCodeName::CMP, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0xC5, name:OpCodeName::CMP, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0xD5, name:OpCodeName::CMP, len:2, cycles:4, mode:AddressingMode::ZeroPage_X },
        OpCode { byte:0xCD, name:OpCodeName::CMP, len:3, cycles:4, mode:AddressingMode::Absolute },
        OpCode { byte:0xDD, name:OpCodeName::CMP, len:3, cycles:4, mode:AddressingMode::Absolute_X },
        OpCode { byte:0xD9, name:OpCodeName::CMP, len:3, cycles:4, mode:AddressingMode::Absolute_Y },
        OpCode { byte:0xC1, name:OpCodeName::CMP, len:2, cycles:6, mode:AddressingMode::Indirect_X },
        OpCode { byte:0xD1, name:OpCodeName::CMP, len:2, cycles:5, mode:AddressingMode::Indirect_Y },
        OpCode { byte:0xE0, name:OpCodeName::CPX, len:2, cycles:2, mode:AddressingMode::Immediate },
        OpCode { byte:0xE4, name:OpCodeName::CPX, len:2, cycles:3, mode:AddressingMode::ZeroPage },
        OpCode { byte:0xEC, name:OpCodeName::CPX, len:3, cycles:4, mode:AddressingMode::Absolute }
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
    // let num: u16 = 0x1234;
    // let le_num = num.to_le_bytes();

    // println!("{:X} {:X}", le_num[0], le_num[1]);
    // println!("{}", u16::from_be_bytes([0x01, 0x00]));
    
    let num: u8 = 0b1000_0000;
    // let new_num: u16 = (num as u16) + 1;
    println!("{:X}: {:#018b}", num as i16, num as i16);
    // println!("{:X} {:#}", new_num, new_num);
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::cpu::{CPU, CPUStatus};

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(CPUStatus::Zero));
        assert!(!cpu.status.contains(CPUStatus::Negative));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.contains(CPUStatus::Zero));
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

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_sta_zeropage() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x05, //loads 0x05 into reg a
            0x85, 0x10, // loads reg a into address 0x10
        ]);

        let data = cpu.mem_read(0x10);
        assert_eq!(data, 0x05);
    }

    //test pha
    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x05, // set 0x05 in reg a
            0x48, // push a to stack
            0x00, // break
        ]);

        // real stack pointer should be at 0x01FF - 0x001
        assert_eq!(cpu.get_stack_pointer(), 0x1FE);

        // there should be 0x05 at 0x01FF
        assert_eq!(cpu.mem_read(0x1FF), 0x05);

        // stack pointer should have the value 0x001
        assert_eq!(cpu.stack_pointer, 0x01);
    }

    //test php
    #[test]
    fn test_php() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x00, // set 0x00 in reg a, this should set the status reg
            0x08, // push status to stack
            0x00, // break
        ]);

        // real stack pointer should be at 0x01FF - 0x001
        assert_eq!(cpu.get_stack_pointer(), 0x1FE);

        // there should be status at 0x01FF
        assert_eq!(cpu.mem_read(0x1FF) & 0b0000_0010, 0b0000_0010);

        // stack pointer should have the value 0x001
        assert_eq!(cpu.stack_pointer, 0x01);
    }

    //test jsr
    #[test]
    fn test_jsr() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0x20, 0x05, 0x80, // jump pc to 0x8050, push 0x8002 to stack as [0x1FF: 0x80, 0x1FE: 0x05]
            0xa9, 0x05, // set 0x05 on reg a, this shouldn't run
            0xa2, 0x04, //set 0x04 on reg x, this should run
            0x00, // break
        ]);

        // check reg x
        assert_eq!(cpu.register_x, 0x04);

        // check reg a
        assert_eq!(cpu.register_a, 0x00);

        // check stack
        assert_eq!(cpu.mem_read(0x01FF), 0x80);
        assert_eq!(cpu.mem_read(0x01FE), 0x02);
        
        // check pc
        assert_eq!(cpu.program_counter, 0x8008);

    }

    // deprecated test rts
    // this test isn't deleted because it may trigger my future self's neuron
    // yes, the error is on purpose
    #[test]
    fn deprecated_test_rts() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0x20, 0x05, 0x80, // jump pc to 0x8050, push 0x8002 to stack as [0x1FF: 0x80, 0x1FE: 0x05]
            0xa9, 0x05, // set 0x05 on reg a, this should run after RTS
            0xa2, 0x04, //set 0x04 on reg x, this should run
            0x60, // jumps to 0x8003
            0x00, // break
        ]);

        
        // check reg x
        assert_eq!(cpu.register_x, 0x04);

        // check reg a
        assert_eq!(cpu.register_a, 0x05);

        // check stack
        assert_eq!(cpu.mem_read(0x01FF), 0x80); // if you check our pop func, we don't zeroize it
        assert_eq!(cpu.mem_read(0x01FE), 0x02); // why? as of now, i don't know why we should do so
        
        // check pc
        assert_eq!(cpu.program_counter, 0x8009);
    }

    // test rts
    #[test]
    fn test_rts() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0x20, 0x06, 0x80, // jump pc to 0x8050, push 0x8002 to stack as [0x1FF: 0x80, 0x1FE: 0x05]
            0x00, // break
            0xa9, 0x05, // set 0x05 on reg a, this shouldn't run
            0xa2, 0x04, //set 0x04 on reg x, this should run
            0x60, // jumps to 0x8003
            0xa9, 0x03, // set reg a = 0x03, shouldn't run
            0xa2, 0x02, // set reg x = 0x02, shouldn't run
            0x00, // break
        ]);

        
        // check reg x
        assert_eq!(cpu.register_x, 0x04);

        // check reg a
        assert_eq!(cpu.register_a, 0x00);

        // check stack
        assert_eq!(cpu.mem_read(0x01FF), 0x80); // if you check our pop func, we don't zeroize it
        assert_eq!(cpu.mem_read(0x01FE), 0x02); // why? as of now, i don't know why we should do so
        
        // check pc
        assert_eq!(cpu.program_counter, 0x8004);
    }

    // test pla
    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x0, //set reg a = 0x0
            0x48, //push reg a
            0xa9, 0x5, //set reg a = 0x5
            0x68, //pop stack, set reg a
            0x0, //break
        ]);

        // check stack, should be 0x01FF
        assert_eq!(cpu.get_stack_pointer(), 0x01FF);

        // check flag, flag Z should be up
        assert!(cpu.status.contains(CPUStatus::Zero));

        // check reg a, should be 0x0
        assert_eq!(cpu.register_a, 0x0);
    }

    // test plp
    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x0, //set reg a = 0x0
            0x08, //push status
            0xa9, 0x5, //set reg a = 0x5
            0x28, // pull into status
            0x0, // break
        ]);

        //check stack
        assert_eq!(cpu.get_stack_pointer(), 0x1FF);

        //check flag
        assert!(cpu.status.contains(CPUStatus::Zero));
    }

    // test rti
    #[test]
    fn test_rti() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x80, // reg a = 0x80
            0x48, // push a
            0xa9, 0xD, // reg a = 0x22
            0x48, // push a; [0x01FF: 0x80, 0x01FE: 0x0D]
            0xa9, 0x48, // reg a = 0x48 [0x01FD: 0x48]
            0x48, // push a; this will be status
            0x40, // rti; status = 0x48 and pc = 0x800D // this is 0x8009
            0x0, // 0x800A
            0x0, // 0x800B
            0x0, // 0x800C
            0x0, // 0x800D
        ]);

        // check stack
        assert_eq!(cpu.get_stack_pointer(), 0x1FF);

        // check flag
        assert_eq!(cpu.status.bits(), 0x48);

        // check pc
        assert_eq!(cpu.program_counter, 0x800E);
    }

    // test adc
    #[test]
    fn test_adc() {
        // test normal addition
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xA9, 0x01, // reg a = 0x01
            0x69, 0x01, // reg a += 0x01
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0x02);


        // test overflow addition
        // no overflow
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0xFF,
            0x69, 0xFF,
        ]); 
        assert!(!cpu.status.contains(CPUStatus::Overflow));
        assert!(cpu.status.contains(CPUStatus::Carry));
        assert_eq!(cpu.register_a, 0b1111_1110);
        // overflow
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0b0111_0000,
            0x69, 0b0100_0000,
            0x00,
        ]);
        assert!(cpu.status.contains(CPUStatus::Overflow));
        assert!(!cpu.status.contains(CPUStatus::Carry));
        assert_eq!(cpu.register_a, 0b1011_0000);

        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0b1100_0000,
            0x69, 0b1100_0000,
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(!cpu.status.contains(CPUStatus::Overflow));
        assert!(cpu.status.contains(CPUStatus::Carry));
        assert!(cpu.status.contains(CPUStatus::Negative));
    }

    // test and
    #[test]
    fn test_and() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0b1000_0000,
            0x29, 0x00,
            0x00
        ]);
        assert!(cpu.status.contains(CPUStatus::Zero));
        assert!(!cpu.status.contains(CPUStatus::Negative));
        
        cpu.load_and_run(vec![
            0xa9, 0b1000_0000,
            0x29, 0b1000_0000,
            0x00,
        ]);
        assert!(!cpu.status.contains(CPUStatus::Zero));
        assert!(cpu.status.contains(CPUStatus::Negative));

        cpu.load_and_run(vec![
            0xa9, 0b0011_0000,
            0x29, 0b0011_1100,
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0b0011_0000);
    }

    // test asl
    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0b1100_0000, //reg a = 0b1100_0000
            0x48, // push reg a
            0x0e, 0xff, 0x01, // shift 0x01ff by 1
            0x68, //pop into reg a 
            0x00, 
        ]);

        assert!(cpu.status.contains(CPUStatus::Carry));
        assert!(cpu.status.contains(CPUStatus::Negative));
        assert_eq!(cpu.register_a, 0b1000_0000);
    }

    // test asl_a
    #[test]
    fn test_asl_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0b1100_0000,
            0x0a,
            0x00,
        ]);

        assert!(cpu.status.contains(CPUStatus::Carry));
        assert!(cpu.status.contains(CPUStatus::Negative));
        assert_eq!(cpu.register_a, 0b1000_0000);
    }

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0xff, //reg a = 255
            0x0a, //reg a shift left
            0x90, 0x03, //if carry, branch
            0xa9, 0x02, // if carry is set, go here
            0x0,
            0xa9, 0x03, // if carry is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x02);
        assert!(cpu.status.contains(CPUStatus::Carry));

        cpu.load_and_run(vec![
            0xa9, 0x01, //reg a = 1
            0x90, 0x03, //if carry, branch
            0xa9, 0x02, // if carry is set, go here
            0x0,
            0xa9, 0x03, // if carry is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x03);
        assert!(!cpu.status.contains(CPUStatus::Carry));
        assert!(!cpu.status.contains(CPUStatus::Negative));

    }

    #[test]
    fn test_bcs() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0xff, //reg a = 255
            0x0a, //reg a shift left
            0xB0, 0x03, //if carry, branch
            0xa9, 0x02, // if carry is set, go here
            0x0,
            0xa9, 0x03, // if carry is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x03);
        assert!(cpu.status.contains(CPUStatus::Carry));

        cpu.load_and_run(vec![
            0xa9, 0x01, //reg a = 1
            0xB0, 0x03, //if carry, branch
            0xa9, 0x02, // if carry is set, go here
            0x0,
            0xa9, 0x03, // if carry is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x02);
        assert!(!cpu.status.contains(CPUStatus::Carry));
        assert!(!cpu.status.contains(CPUStatus::Negative));
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0,
            0xF0, 0x03, //if zero, branch
            0xa9, 0x02, // if zero is set, go here
            0x0,
            0xa9, 0x03, // if zero is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x03);

        
        cpu.load_and_run(vec![
            0xa9, 1,
            0xF0, 0x03, //if zero, branch
            0xa9, 0x02, // if zero is set, go here
            0x0,
            0xa9, 0x03, // if zero is set, reg a = 3
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x02);
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0,
            0x89, 0b1100_0000,
            0x0,
        ]);

        assert!(cpu.status.contains(CPUStatus::Zero));
        assert!(cpu.status.contains(CPUStatus::Negative));
        assert!(cpu.status.contains(CPUStatus::Overflow));
        
        cpu.load_and_run(vec![
            0xa9, 0b1000_0000,
            0x89, 0b1100_0000,
            0x0,
        ]);

        assert!(!cpu.status.contains(CPUStatus::Zero));
        assert!(cpu.status.contains(CPUStatus::Negative));
        assert!(cpu.status.contains(CPUStatus::Overflow));
        
        cpu.load_and_run(vec![
            0xa9, 0b1000_0000,
            0x89, 0b1100_0000,
            0x0,
        ]);

        assert!(!cpu.status.contains(CPUStatus::Zero));
        assert!(cpu.status.contains(CPUStatus::Negative));
        assert!(cpu.status.contains(CPUStatus::Overflow));
        
        cpu.load_and_run(vec![
            0xa9, 0b1000_0000,
            0x89, 0b0010_0000,
            0x0,
        ]);

        assert!(cpu.status.contains(CPUStatus::Zero));
        assert!(!cpu.status.contains(CPUStatus::Negative));
        assert!(!cpu.status.contains(CPUStatus::Overflow));
    }

    #[test]
    fn test_bmi() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![ // branch
            0xa9, 0b1000_0000,
            0x30, 0x03,
            0xa9, 0x02,
            0x0,
            0xa9, 0x03,
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x03);

        
        cpu.load_and_run(vec![ // no branch
            0xa9, 1,
            0x30, 0x03,
            0xa9, 0x02,
            0x0,
            0xa9, 0x03,
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x02);

    }

    #[test]
    fn test_bne() {
        
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![ // branch
            0xa9, 0b1000_0000,
            0xD0, 0x03,
            0xa9, 0x02,
            0x0,
            0xa9, 0x03,
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x03);

        
        cpu.load_and_run(vec![ // no branch
            0xa9, 0,
            0xD0, 0x03,
            0xa9, 0x02,
            0x0,
            0xa9, 0x03,
            0x0,
        ]);

        assert_eq!(cpu.register_a, 0x02);
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
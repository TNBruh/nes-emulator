use crate::cpu::AddressingMode;

#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub byte: u8, // mnemonic in byte form (i think it's called "mnemonic")
    pub name: OpCodeName, // the literal name
    pub len: usize, // bytes taken
    pub cycles: usize,
    pub mode: AddressingMode,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum OpCodeName {
    LDA, LDX, STA, TAX, INX,
    BRK, PHA, PHP, JSR, RTS,
    PLA, PLP, RTI, ADC, AND,
    ASL, BCC, BCS, BEQ, BIT,
    BMI, BNE, BPL, BVC, BVS,
}


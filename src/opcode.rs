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
    LDA(u8), LDX(u8), STA(u8), TAX(u8), INX(u8),
    BRK(u8), PHA(u8), PHP(u8), JSR(u8), RTS(u8),
    PLA(u8), PLP(u8), RTI(u8), ADC(u8), AND(u8),
    ASL(u8),
}


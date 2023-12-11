use crate::cpu::AddressingMode;

pub struct OpCode {
    pub byte: u8, // mnemonic in byte form (i think it's called "mnemonic")
    pub name: &'static str, // the literal name
    pub len: usize, // bytes taken
    pub cycles: usize,
    pub mode: AddressingMode,
}


use crate::cpu::{
    cb_instructions::CB_OPCODE_TABLE, instructions::OPCODE_TABLE, registers::Registers,
};

pub mod cb_instructions;
pub mod instructions;
mod macros;
pub mod registers;

pub struct CPU {
    registers: Registers,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self) -> u8 {
        let opcode = 0x2D as u8;
        let cycles = self;
        OPCODE_TABLE[opcode as usize](cycles)
    }
}

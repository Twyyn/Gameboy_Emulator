use crate::cpu::{instructions::DISPATCH, registers::Registers};

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
        let opcode = 0x01 as u8;
        let cycles = DISPATCH[opcode as usize](self);
        cycles
    }
}

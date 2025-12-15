pub mod instructions;
pub mod opcodes;
pub mod registers;

use crate::cpu::registers::Register;

#[allow(dead_code)]
pub struct CPU {
    pub registers: Register,
}

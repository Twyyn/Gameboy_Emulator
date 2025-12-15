pub mod ram;
pub mod rom;

use crate::memory::{ram::RAM, rom::ROM};

#[allow(non_snake_case, dead_code)]
pub struct Memory {
    pub RAM: RAM,
    pub ROM: ROM,
}
#[allow(dead_code)]
impl Memory {
    pub fn new() -> Self {
        Self {
            RAM: RAM::new(),
            ROM: ROM::new(),
        }
    }
}

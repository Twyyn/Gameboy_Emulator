#[allow(non_snake_case, dead_code)]
pub struct MBC1 {
    ROM: Vec<u8>,
    RAM: Vec<u8>,

    /* ---- MBC1 registers ---- */
    /* RAM */
    RAM_bank: u8,
    RAM_enabled: bool,
    /* ROM */
    ROM_bank_low5: u8,
    ROM_bank_high2: u8,
    banking_mode: u8, // 0 = ROM banking, 1 = RAM banking
}
#[allow(non_snake_case, dead_code)]
impl MBC1 {
    pub fn new(ROM: Vec<u8>, RAM_size: usize) -> Self {
        Self {
            ROM,
            RAM: vec![0; RAM_size],
            RAM_bank: 0,
            RAM_enabled: false,
            ROM_bank_low5: 1,
            ROM_bank_high2: 0,
            banking_mode: 0,
        }
    }
}

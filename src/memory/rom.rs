#[allow(non_snake_case, dead_code)]
pub struct ROM {
    ROM: Vec<u8>,
    ROM_bank: usize,
}
#[allow(dead_code)]
impl ROM {
    pub fn new() -> Self {
        Self {
            ROM: Vec::new(),
            ROM_bank: 0,
        }
    }
    /* ----------------Read & Write(MBC Chip Register)---------------- */
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // Bank 00
            0x0000..=0x3FFF => self.ROM[addr as usize],
            // Bank 01-NN:
            0x4000..=0x7FFF => {
                let offset = (self.ROM_bank * 0x4000) + ((addr - 0x4000) as usize);
                self.ROM[offset]
            }
            _ => 0xFF,
        }
    }
    fn mbc_write(&mut self, addr: u16, value: u8) {
        match addr {
            // Bank 00 selection not allowed
            0x2000..=0x3FFF => {
                self.ROM_bank = (value & 0x1F) as usize;
                if self.ROM_bank == 0 {
                    self.ROM_bank = 1;
                }
            }
            _ => {}
        }
    }
}

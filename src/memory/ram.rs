const VRAM_SIZE: usize = 0x2000;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;
const OAM_SIZE: usize = 0xA0;

#[allow(non_snake_case, dead_code)]
pub struct RAM {
    /* Cartridge RAM */
    cartRAM: Vec<u8>, // External/Cartridge RAM
    cartRAM_bank: usize,
    cartRAM_enabled: bool,
    /* Internal RAM */
    WRAM: [u8; WRAM_SIZE], // Work RAM
    HRAM: [u8; HRAM_SIZE], // High RAM
    /* Video RAM */
    VRAM: [u8; VRAM_SIZE], // Video RAM
    OAM: [u8; OAM_SIZE],
}
impl RAM {
    pub fn new() -> Self {
        Self {
            cartRAM: Vec::new(),
            cartRAM_bank: 0,
            cartRAM_enabled: false,
            WRAM: [0; WRAM_SIZE],
            HRAM: [0; HRAM_SIZE],
            VRAM: [0; VRAM_SIZE],
            OAM: [0; OAM_SIZE],
        }
    }

    /* ----------------Read & Write RAM---------------- */
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // VRAM: 0x8000-0x9FFF
            0x8000..=0x9FFF => self.VRAM[(addr - 0x8000) as usize],

            // Cart RAM: 0xA000-0xBFFF
            0xA000..=0xBFFF => match self.cartRAM_enabled {
                true => {
                    let offset = (self.cartRAM_bank * 0x2000) + ((addr - 0xA000) as usize);
                    self.cartRAM[offset]
                }
                _ => 0xFF,
            },

            // WRAM: 0xC000-0xDFFF
            0xC000..=0xDFFF => self.WRAM[(addr - 0xC000) as usize],

            // OAM: 0xFE00-0xFE9F (160 bytes)
            0xFE00..=0xFE9F => self.OAM[(addr - 0xFE00) as usize],

            // HRAM: 0xFF80-0xFFFE (127 bytes)
            0xFF80..=0xFFFE => self.HRAM[(addr - 0xFF80) as usize],

            _ => 0xFF,
        }
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // VRAM: 0x8000-0x9FFF
            0x8000..=0x9FFF => self.VRAM[(addr - 0x8000) as usize] = value,

            // Cart RAM: 0xA000-0xBFFF
            0xA000..=0xBFFF => {
                if self.cartRAM_enabled {
                    let offset = (self.cartRAM_bank * 0x2000) + ((addr - 0xA000) as usize);
                    self.cartRAM[offset] = value
                }
            }

            // WRAM: 0xC000-0xDFFF
            0xC000..=0xDFFF => self.WRAM[(addr - 0xC000) as usize] = value,

            // OAM: 0xFE00-0xFE9F (160 bytes)
            0xFE00..=0xFE9F => self.OAM[(addr - 0xFE00) as usize] = value,

            // Not Usable: 0xFEA0-0xFEFF
            0xFEA0..=0xFEFF => {}

            // HRAM: 0xFF80-0xFFFE (127 bytes)
            0xFF80..=0xFFFE => self.HRAM[(addr - 0xFF80) as usize] = value,

            _ => {}
        }
    }
    /* ----------------Cartridge RAM---------------- */
    pub fn init_cart_ram(&mut self, ram_size: usize) {
        // Initialize Cartridge RAM
        self.cartRAM = vec![0; ram_size];
    }
    pub fn set_cart_status(&mut self, enabled: bool) {
        // Enable/Disable Cartridge RAM
        self.cartRAM_enabled = enabled;
    }
    pub fn set_cart_bank(&mut self, bank: usize) {
        // Switch Cartridge RAM bank
        self.cartRAM_bank = bank;
    }
}

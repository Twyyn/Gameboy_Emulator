const VRAM_SIZE: usize = 0x2000; // 8 KB
const WRAM_SIZE: usize = 0x2000; // 8 KB
const OAM_SIZE: usize = 0x00A0; // 160 bytes
const HRAM_SIZE: usize = 0x007F; // 127 bytes

#[allow(non_snake_case, dead_code)]
pub struct Memory {
    VRAM: [u8; VRAM_SIZE],
    WRAM: [u8; WRAM_SIZE],
    OAM: [u8; OAM_SIZE],
    HRAM: [u8; HRAM_SIZE],
}

#[allow(non_snake_case, dead_code)]
impl Memory {
    pub fn new() -> Self {
        Self {
            VRAM: [0; VRAM_SIZE],
            WRAM: [0; WRAM_SIZE],
            OAM: [0; OAM_SIZE],
            HRAM: [0; HRAM_SIZE],
        }
    }

    /* -------- Read -------- */
    pub fn read_WRAM(&self, addr: u16) -> u8 {
        self.WRAM[(addr - 0xC000) as usize]
    }
    pub fn read_HRAM(&self, addr: u16) -> u8 {
        self.HRAM[(addr - 0xFF80) as usize]
    }
    pub fn read_VRAM(&self, addr: u16, ppu_can_access: bool) -> u8 {
        if ppu_can_access {
            self.VRAM[(addr - 0x8000) as usize]
        } else {
            0xFF
        }
    }
    pub fn read_OAM(&self, addr: u16, ppu_can_access: bool) -> u8 {
        if ppu_can_access {
            self.OAM[(addr - 0xFE00) as usize]
        } else {
            0xFF
        }
    }
    /* -------- Write -------- */
    fn write_WRAM(&mut self, addr: u16, value: u8) {
        self.WRAM[(addr - 0xC000) as usize] = value;
    }
    pub fn write_HRAM(&mut self, addr: u16, value: u8) {
        self.HRAM[(addr - 0xFF80) as usize] = value
    }
    pub fn write_VRAM(&mut self, addr: u16, value: u8, ppu_can_access: bool) {
        if ppu_can_access {
            self.VRAM[(addr - 0x8000) as usize] = value;
        }
    }
    pub fn read_echo(&self, addr: u16) -> u8 {
        self.WRAM[(addr - 0xE000) as usize]
    }
}

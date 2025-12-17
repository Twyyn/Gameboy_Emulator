pub mod cartridge;

const VRAM_SIZE: usize = 8192;
const WRAM_SIZE: usize = 8192;
const OAM_SIZE: usize = 160;
const HRAM_SIZE: usize = 127;
const IO_SIZE: usize = 8192;

pub struct Memory {
    VRAM: [u8; VRAM_SIZE],
    WRAM: [u8; WRAM_SIZE],
    OAM: [u8; OAM_SIZE],
    HRAM: [u8; HRAM_SIZE],
    IO: [u8; IO_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            VRAM: [0; VRAM_SIZE],
            WRAM: [0; WRAM_SIZE],
            OAM: [0; OAM_SIZE],
            HRAM: [0; HRAM_SIZE],
            IO: [0; IO_SIZE],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            /* Video RAM */
            0x8000..0x9FFF => self.VRAM[addr as usize],
            /* Work RAM */
            0xC000..=0xDFFF => self.WRAM[addr as usize],
            /* Echo RAM */
            0xE000..=0xFDFF => self.WRAM[(addr - 0x2000) as usize],
            /* OAM RAM */
            0xFE00..0xFE9F => self.OAM[addr as usize],
            /* IO Registers */
            0xFF00..=0xFF7F => self.IO[addr as usize],
            /* High RAM */
            0xFF80..0xFFFE => self.HRAM[addr as usize],
            /* IE Register */
            _ => 0xFF,
        }
    }
}

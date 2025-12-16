use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b10000000; /* Zero Flag - Bit 7 */
        const N = 0b01000000; /* Add/Sub Flag (BCD) - Bit 6 */
        const H = 0b00100000; /* Half Carry Flag (BCD) - Bit 5 */
        const C = 0b00010000; /* Carry Flag - Bit 4 */
    }
}
#[derive(Debug)]
pub struct Registers {
    pub SP: u16, // 16-Bit Stack Pointer
    pub PC: u16, // 16-Bit Program Counter
    /* ---------------- 8-Bit Registers ---------------- */
    pub A: u8, // Accumulator Register
    F: u8,     // Flags Register
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
}
impl Default for Registers {
    fn default() -> Self {
        Self {
            SP: 0xFFFE,
            PC: 0x0100,
            A: 0x01,
            F: 0xB0,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            H: 0x01,
            L: 0x4D,
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    /* ---------------- 16-Bit Registers ---------------- */
    /* Getters */
    #[inline]
    pub fn AF(&self) -> u16 {
        u16::from_be_bytes([self.A, self.F])
    }
    #[inline]
    pub fn BC(&self) -> u16 {
        u16::from_be_bytes([self.B, self.C])
    }
    #[inline]
    pub fn DE(&self) -> u16 {
        u16::from_be_bytes([self.D, self.E])
    }
    #[inline]
    pub fn HL(&self) -> u16 {
        u16::from_be_bytes([self.H, self.L])
    }
    /* Setters */
    pub fn set_AF(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.A = bytes[0];
        self.F = bytes[1];
    }
    pub fn set_BC(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.B = bytes[0];
        self.C = bytes[1];
    }
    pub fn set_DE(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.D = bytes[0];
        self.E = bytes[1];
    }
    pub fn set_HL(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.H = bytes[0];
        self.L = bytes[1];
    }
    /* ---------------- F(Flags) Register ---------------- */
    /* Getters */
    #[inline]
    pub fn Flags(&self) -> u8 {
        self.F & Flags::all().bits()
    }
    #[inline]
    pub fn Z_Flag(&self) -> bool {
        self.F & Flags::Z.bits() != 0
    }
    #[inline]
    pub fn N_Flag(&self) -> bool {
        self.F & Flags::N.bits() != 0
    }
    #[inline]
    pub fn H_Flag(&self) -> bool {
        self.F & Flags::H.bits() != 0
    }
    #[inline]
    pub fn C_Flag(&self) -> bool {
        self.F & Flags::C.bits() != 0
    }
    /* Setters */
    pub fn set_Flag(&mut self, flag: Flags, value: bool) {
        self.F = if value {
            self.F | flag.bits()
        } else {
            self.F & !flag.bits()
        }
    }
}

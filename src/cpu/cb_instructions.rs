use crate::gb_cb_instructions;

gb_cb_instructions! {
    RLC_B(0x00, 2) => |CPU| {

    },

    RLC_C(0x01, 2) => |CPU| {

    },

    RLC_D(0x02, 2) => |CPU| {

    },

    RLC_E(0x03, 2) => |CPU| {

    },

    RLC_H(0x04, 2) => |CPU| {

    },

    RLC_L(0x05, 2) => |CPU| {

    },

    RLC__HL_(0x06, 4) => |CPU| {

    },

    RLC_A(0x07, 2) => |CPU| {

    },

    RRC_B(0x08, 2) => |CPU| {

    },

    RRC_C(0x09, 2) => |CPU| {

    },

    RRC_D(0x0A, 2) => |CPU| {

    },

    RRC_E(0x0B, 2) => |CPU| {

    },

    RRC_H(0x0C, 2) => |CPU| {

    },

    RRC_L(0x0D, 2) => |CPU| {

    },

    RRC__HL_(0x0E, 4) => |CPU| {

    },

    RRC_A(0x0F, 2) => |CPU| {

    },

    RL_B(0x10, 2) => |CPU| {

    },

    RL_C(0x11, 2) => |CPU| {

    },

    RL_D(0x12, 2) => |CPU| {

    },

    RL_E(0x13, 2) => |CPU| {

    },

    RL_H(0x14, 2) => |CPU| {

    },

    RL_L(0x15, 2) => |CPU| {

    },

    RL__HL_(0x16, 4) => |CPU| {

    },

    RL_A(0x17, 2) => |CPU| {

    },

    RR_B(0x18, 2) => |CPU| {

    },

    RR_C(0x19, 2) => |CPU| {

    },

    RR_D(0x1A, 2) => |CPU| {

    },

    RR_E(0x1B, 2) => |CPU| {

    },

    RR_H(0x1C, 2) => |CPU| {

    },

    RR_L(0x1D, 2) => |CPU| {

    },

    RR__HL_(0x1E, 4) => |CPU| {

    },

    RR_A(0x1F, 2) => |CPU| {

    },

    SLA_B(0x20, 2) => |CPU| {

    },

    SLA_C(0x21, 2) => |CPU| {

    },

    SLA_D(0x22, 2) => |CPU| {

    },

    SLA_E(0x23, 2) => |CPU| {

    },

    SLA_H(0x24, 2) => |CPU| {

    },

    SLA_L(0x25, 2) => |CPU| {

    },

    SLA__HL_(0x26, 4) => |CPU| {

    },

    SLA_A(0x27, 2) => |CPU| {

    },

    SRA_B(0x28, 2) => |CPU| {

    },

    SRA_C(0x29, 2) => |CPU| {

    },

    SRA_D(0x2A, 2) => |CPU| {

    },

    SRA_E(0x2B, 2) => |CPU| {

    },

    SRA_H(0x2C, 2) => |CPU| {

    },

    SRA_L(0x2D, 2) => |CPU| {

    },

    SRA__HL_(0x2E, 4) => |CPU| {

    },

    SRA_A(0x2F, 2) => |CPU| {

    },

    SWAP_B(0x30, 2) => |CPU| {

    },

    SWAP_C(0x31, 2) => |CPU| {

    },

    SWAP_D(0x32, 2) => |CPU| {

    },

    SWAP_E(0x33, 2) => |CPU| {

    },

    SWAP_H(0x34, 2) => |CPU| {

    },

    SWAP_L(0x35, 2) => |CPU| {

    },

    SWAP__HL_(0x36, 4) => |CPU| {

    },

    SWAP_A(0x37, 2) => |CPU| {

    },

    SRL_B(0x38, 2) => |CPU| {

    },

    SRL_C(0x39, 2) => |CPU| {

    },

    SRL_D(0x3A, 2) => |CPU| {

    },

    SRL_E(0x3B, 2) => |CPU| {

    },

    SRL_H(0x3C, 2) => |CPU| {

    },

    SRL_L(0x3D, 2) => |CPU| {

    },

    SRL__HL_(0x3E, 4) => |CPU| {

    },

    SRL_A(0x3F, 2) => |CPU| {

    },

    // -------- BIT 0 --------
    BIT_0_B(0x40, 2) => |CPU| {

    },

    BIT_0_C(0x41, 2) => |CPU| {

    },

    BIT_0_D(0x42, 2) => |CPU| {

    },

    BIT_0_E(0x43, 2) => |CPU| {

    },

    BIT_0_H(0x44, 2) => |CPU| {

    },

    BIT_0_L(0x45, 2) => |CPU| {

    },

    BIT_0__HL_(0x46, 3) => |CPU| {

    },

    BIT_0_A(0x47, 2) => |CPU| {

    },

    // -------- BIT 1 --------
    BIT_1_B(0x48, 2) => |CPU| {

    },

    BIT_1_C(0x49, 2) => |CPU| {

    },

    BIT_1_D(0x4A, 2) => |CPU| {

    },

    BIT_1_E(0x4B, 2) => |CPU| {

    },

    BIT_1_H(0x4C, 2) => |CPU| {

    },

    BIT_1_L(0x4D, 2) => |CPU| {

    },

    BIT_1__HL_(0x4E, 3) => |CPU| {

    },

    BIT_1_A(0x4F, 2) => |CPU| {

    },

    // -------- BIT 2 --------
    BIT_2_B(0x50, 2) => |CPU| {

    },

    BIT_2_C(0x51, 2) => |CPU| {

    },

    BIT_2_D(0x52, 2) => |CPU| {

    },

    BIT_2_E(0x53, 2) => |CPU| {

    },

    BIT_2_H(0x54, 2) => |CPU| {

    },

    BIT_2_L(0x55, 2) => |CPU| {

    },

    BIT_2__HL_(0x56, 3) => |CPU| {

    },

    BIT_2_A(0x57, 2) => |CPU| {

    },

    // -------- BIT 3 --------
    BIT_3_B(0x58, 2) => |CPU| {

    },

    BIT_3_C(0x59, 2) => |CPU| {

    },

    BIT_3_D(0x5A, 2) => |CPU| {

    },

    BIT_3_E(0x5B, 2) => |CPU| {

    },

    BIT_3_H(0x5C, 2) => |CPU| {

    },

    BIT_3_L(0x5D, 2) => |CPU| {

    },

    BIT_3__HL_(0x5E, 3) => |CPU| {

    },

    BIT_3_A(0x5F, 2) => |CPU| {

    },

    // -------- BIT 4 --------
    BIT_4_B(0x60, 2) => |CPU| {

    },

    BIT_4_C(0x61, 2) => |CPU| {

    },

    BIT_4_D(0x62, 2) => |CPU| {

    },

    BIT_4_E(0x63, 2) => |CPU| {

    },

    BIT_4_H(0x64, 2) => |CPU| {

    },

    BIT_4_L(0x65, 2) => |CPU| {

    },

    BIT_4__HL_(0x66, 3) => |CPU| {

    },

    BIT_4_A(0x67, 2) => |CPU| {

    },

    // -------- BIT 5 --------
    BIT_5_B(0x68, 2) => |CPU| {

    },

    BIT_5_C(0x69, 2) => |CPU| {

    },

    BIT_5_D(0x6A, 2) => |CPU| {

    },

    BIT_5_E(0x6B, 2) => |CPU| {

    },

    BIT_5_H(0x6C, 2) => |CPU| {

    },

    BIT_5_L(0x6D, 2) => |CPU| {

    },

    BIT_5__HL_(0x6E, 3) => |CPU| {

    },

    BIT_5_A(0x6F, 2) => |CPU| {

    },

    // -------- BIT 6 --------
    BIT_6_B(0x70, 2) => |CPU| {

    },

    BIT_6_C(0x71, 2) => |CPU| {

    },

    BIT_6_D(0x72, 2) => |CPU| {

    },

    BIT_6_E(0x73, 2) => |CPU| {

    },

    BIT_6_H(0x74, 2) => |CPU| {

    },

    BIT_6_L(0x75, 2) => |CPU| {

    },

    BIT_6__HL_(0x76, 3) => |CPU| {

    },

    BIT_6_A(0x77, 2) => |CPU| {

    },

    // -------- BIT 7 --------
    BIT_7_B(0x78, 2) => |CPU| {

    },

    BIT_7_C(0x79, 2) => |CPU| {

    },

    BIT_7_D(0x7A, 2) => |CPU| {

    },

    BIT_7_E(0x7B, 2) => |CPU| {

    },

    BIT_7_H(0x7C, 2) => |CPU| {

    },

    BIT_7_L(0x7D, 2) => |CPU| {

    },

    BIT_7__HL_(0x7E, 3) => |CPU| {

    },

    BIT_7_A(0x7F, 2) => |CPU| {

    },

    RES_0_B(0x80, 2) => |CPU| {

    },

    RES_0_C(0x81, 2) => |CPU| {

    },

    RES_0_D(0x82, 2) => |CPU| {

    },

    RES_0_E(0x83, 2) => |CPU| {

    },

    RES_0_H(0x84, 2) => |CPU| {

    },

    RES_0_L(0x85, 2) => |CPU| {

    },

    RES_0__HL_(0x86, 4) => |CPU| {

    },

    RES_0_A(0x87, 2) => |CPU| {

    },

    RES_1_B(0x88, 2) => |CPU| {

    },

    RES_1_C(0x89, 2) => |CPU| {

    },

    RES_1_D(0x8A, 2) => |CPU| {

    },

    RES_1_E(0x8B, 2) => |CPU| {

    },

    RES_1_H(0x8C, 2) => |CPU| {

    },

    RES_1_L(0x8D, 2) => |CPU| {

    },

    RES_1__HL_(0x8E, 4) => |CPU| {

    },

    RES_1_A(0x8F, 2) => |CPU| {

    },

    RES_2_B(0x90, 2) => |CPU| {

    },

    RES_2_C(0x91, 2) => |CPU| {

    },

    RES_2_D(0x92, 2) => |CPU| {

    },

    RES_2_E(0x93, 2) => |CPU| {

    },

    RES_2_H(0x94, 2) => |CPU| {

    },

    RES_2_L(0x95, 2) => |CPU| {

    },

    RES_2__HL_(0x96, 4) => |CPU| {

    },

    RES_2_A(0x97, 2) => |CPU| {

    },

    RES_3_B(0x98, 2) => |CPU| {

    },

    RES_3_C(0x99, 2) => |CPU| {

    },

    RES_3_D(0x9A, 2) => |CPU| {

    },

    RES_3_E(0x9B, 2) => |CPU| {

    },

    RES_3_H(0x9C, 2) => |CPU| {

    },

    RES_3_L(0x9D, 2) => |CPU| {

    },

    RES_3__HL_(0x9E, 4) => |CPU| {

    },

    RES_3_A(0x9F, 2) => |CPU| {

    },

    RES_4_B(0xA0, 2) => |CPU| {

    },

    RES_4_C(0xA1, 2) => |CPU| {

    },

    RES_4_D(0xA2, 2) => |CPU| {

    },

    RES_4_E(0xA3, 2) => |CPU| {

    },

    RES_4_H(0xA4, 2) => |CPU| {

    },

    RES_4_L(0xA5, 2) => |CPU| {

    },

    RES_4__HL_(0xA6, 4) => |CPU| {

    },

    RES_4_A(0xA7, 2) => |CPU| {

    },

    RES_5_B(0xA8, 2) => |CPU| {

    },

    RES_5_C(0xA9, 2) => |CPU| {

    },

    RES_5_D(0xAA, 2) => |CPU| {

    },

    RES_5_E(0xAB, 2) => |CPU| {

    },

    RES_5_H(0xAC, 2) => |CPU| {

    },

    RES_5_L(0xAD, 2) => |CPU| {

    },

    RES_5__HL_(0xAE, 4) => |CPU| {

    },

    RES_5_A(0xAF, 2) => |CPU| {

    },

    RES_6_B(0xB0, 2) => |CPU| {

    },

    RES_6_C(0xB1, 2) => |CPU| {

    },

    RES_6_D(0xB2, 2) => |CPU| {

    },

    RES_6_E(0xB3, 2) => |CPU| {

    },

    RES_6_H(0xB4, 2) => |CPU| {

    },

    RES_6_L(0xB5, 2) => |CPU| {

    },

    RES_6__HL_(0xB6, 4) => |CPU| {

    },

    RES_6_A(0xB7, 2) => |CPU| {

    },

    RES_7_B(0xB8, 2) => |CPU| {

    },

    RES_7_C(0xB9, 2) => |CPU| {

    },

    RES_7_D(0xBA, 2) => |CPU| {

    },

    RES_7_E(0xBB, 2) => |CPU| {

    },

    RES_7_H(0xBC, 2) => |CPU| {

    },

    RES_7_L(0xBD, 2) => |CPU| {

    },

    RES_7__HL_(0xBE, 4) => |CPU| {

    },

    RES_7_A(0xBF, 2) => |CPU| {

    },

    SET_0_B(0xC0, 2) => |CPU| {

    },

    SET_0_C(0xC1, 2) => |CPU| {

    },

    SET_0_D(0xC2, 2) => |CPU| {

    },

    SET_0_E(0xC3, 2) => |CPU| {

    },

    SET_0_H(0xC4, 2) => |CPU| {

    },

    SET_0_L(0xC5, 2) => |CPU| {

    },

    SET_0__HL_(0xC6, 4) => |CPU| {

    },

    SET_0_A(0xC7, 2) => |CPU| {

    },

    SET_1_B(0xC8, 2) => |CPU| {

    },

    SET_1_C(0xC9, 2) => |CPU| {

    },

    SET_1_D(0xCA, 2) => |CPU| {

    },

    SET_1_E(0xCB, 2) => |CPU| {

    },

    SET_1_H(0xCC, 2) => |CPU| {

    },

    SET_1_L(0xCD, 2) => |CPU| {

    },

    SET_1__HL_(0xCE, 4) => |CPU| {

    },

    SET_1_A(0xCF, 2) => |CPU| {

    },

    SET_2_B(0xD0, 2) => |CPU| {

    },

    SET_2_C(0xD1, 2) => |CPU| {

    },

    SET_2_D(0xD2, 2) => |CPU| {

    },

    SET_2_E(0xD3, 2) => |CPU| {

    },

    SET_2_H(0xD4, 2) => |CPU| {

    },

    SET_2_L(0xD5, 2) => |CPU| {

    },

    SET_2__HL_(0xD6, 4) => |CPU| {

    },

    SET_2_A(0xD7, 2) => |CPU| {

    },

    SET_3_B(0xD8, 2) => |CPU| {

    },

    SET_3_C(0xD9, 2) => |CPU| {

    },

    SET_3_D(0xDA, 2) => |CPU| {

    },

    SET_3_E(0xDB, 2) => |CPU| {

    },

    SET_3_H(0xDC, 2) => |CPU| {

    },

    SET_3_L(0xDD, 2) => |CPU| {

    },

    SET_3__HL_(0xDE, 4) => |CPU| {

    },

    SET_3_A(0xDF, 2) => |CPU| {

    },

    SET_4_B(0xE0, 2) => |CPU| {

    },

    SET_4_C(0xE1, 2) => |CPU| {

    },

    SET_4_D(0xE2, 2) => |CPU| {

    },

    SET_4_E(0xE3, 2) => |CPU| {

    },

    SET_4_H(0xE4, 2) => |CPU| {

    },

    SET_4_L(0xE5, 2) => |CPU| {

    },

    SET_4__HL_(0xE6, 4) => |CPU| {

    },

    SET_4_A(0xE7, 2) => |CPU| {

    },

    SET_5_B(0xE8, 2) => |CPU| {

    },

    SET_5_C(0xE9, 2) => |CPU| {

    },

    SET_5_D(0xEA, 2) => |CPU| {

    },

    SET_5_E(0xEB, 2) => |CPU| {

    },

    SET_5_H(0xEC, 2) => |CPU| {

    },

    SET_5_L(0xED, 2) => |CPU| {

    },

    SET_5__HL_(0xEE, 4) => |CPU| {

    },

    SET_5_A(0xEF, 2) => |CPU| {

    },

    SET_6_B(0xF0, 2) => |CPU| {

    },

    SET_6_C(0xF1, 2) => |CPU| {

    },

    SET_6_D(0xF2, 2) => |CPU| {

    },

    SET_6_E(0xF3, 2) => |CPU| {

    },

    SET_6_H(0xF4, 2) => |CPU| {

    },

    SET_6_L(0xF5, 2) => |CPU| {

    },

    SET_6__HL_(0xF6, 4) => |CPU| {

    },

    SET_6_A(0xF7, 2) => |CPU| {

    },

    SET_7_B(0xF8, 2) => |CPU| {

    },

    SET_7_C(0xF9, 2) => |CPU| {

    },

    SET_7_D(0xFA, 2) => |CPU| {

    },

    SET_7_E(0xFB, 2) => |CPU| {

    },

    SET_7_H(0xFC, 2) => |CPU| {

    },

    SET_7_L(0xFD, 2) => |CPU| {

    },

    SET_7__HL_(0xFE, 4) => |CPU| {

    },

    SET_7_A(0xFF, 2) => |CPU| {

    },
}

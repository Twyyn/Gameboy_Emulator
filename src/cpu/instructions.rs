use crate::gb_instructions;

gb_instructions!(
    NOP(0x00, 4) => |_CPU| {},

    LD_BC_n16(0x01, 3) => |CPU| {

    },

    LD__BC__A(0x02, 2) => |CPU| {

    },

    INC_BC(0x03, 2) => |CPU| {

    },

    INC_B(0x04, 1) => |CPU| {

    },

    DEC_B(0x05, 1) => |CPU| {

    },

    LD_B_n8(0x06, 2) => |CPU| {

    },

    RLCA(0x07, 1) => |CPU| {

    },

    LD__a16__SP(0x08, 5) => |CPU| {

    },

    ADD_HL_BC(0x09, 2) => |CPU| {

    },

    LD_A__BC__(0x0A, 2) => |CPU| {

    },

    DEC_BC(0x0B, 2) => |CPU| {

    },

    INC_C(0x0C, 1) => |CPU| {

    },

    DEC_C(0x0D, 1) => |CPU| {

    },

    LD_C_n8(0x0E, 2) => |CPU| {

    },

    RRCA(0x0F, 1) => |CPU| {

    },

    STOP_n8(0x10, 1) => |CPU| {

    },

    LD_DE_n16(0x11, 3) => |CPU| {

    },

    LD__DE__A(0x12, 2) => |CPU| {

    },

    INC_DE(0x13, 2) => |CPU| {

    },

    INC_D(0x14, 1) => |CPU| {

    },

    DEC_D(0x15, 1) => |CPU| {

    },

    LD_D_n8(0x16, 2) => |CPU| {

    },

    RLA(0x17, 1) => |CPU| {

    },

    JR_e8(0x18, 3) => |CPU| {

    },

    ADD_HL_DE(0x19, 2) => |CPU| {

    },

    LD_A__DE__(0x1A, 2) => |CPU| {

    },

    DEC_DE(0x1B, 2) => |CPU| {

    },

    INC_E(0x1C, 1) => |CPU| {

    },

    DEC_E(0x1D, 1) => |CPU| {

    },

    LD_E_n8(0x1E, 2) => |CPU| {

    },

    RRA(0x1F, 1) => |CPU| {

    },

    JR_NZ_e8(0x20, 3/2) => |CPU| {

    },

    LD_HL_n16(0x21, 3) => |CPU| {

    },

    LD__HLI__A(0x22, 2) => |CPU| {

    },

    INC_HL(0x23, 2) => |CPU| {

    },

    INC_H(0x24, 1) => |CPU| {

    },

    DEC_H(0x25, 1) => |CPU| {

    },

    LD_H_e8(0x26, 2) => |CPU| {

    },

    DAA(0x27, 1) => |CPU| {

    },

    JR_Z_n8(0x28, 3/2) => |CPU| {

    },

    ADD_HL_HL(0x29, 2) => |CPU| {

    },

    LD_A__HLI__(0x2A, 2) => |CPU| {

    },

    DEC_HL(0x2B, 2) => |CPU| {

    },

    INC_L(0x2C, 1) => |CPU| {

    },

    DEC_L(0x2D, 1) => |CPU| {

    },

    LD_L_n8(0x2E, 2) => |CPU| {

    },

    CPL(0x2F, 1) => |CPU| {

    },

    JR_NC_e8(0x30, 3/2) => |CPU| {

    },

    LD_SP_n16(0x31, 3) => |CPU| {

    },

    LD__HLD__A(0x32, 2) => |CPU| {

    },

    INC_SP(0x33, 2) => |CPU| {

    },

    INC__HL__(0x34, 3) => |CPU| {

    },

    DEC__HL__(0x35, 3) => |CPU| {

    },

    LD__HL__n8(0x36, 3) => |CPU| {

    },

    SCF(0x37, 1) => |CPU| {

    },

    JR_C_e8(0x38, 3/2) => |CPU| {

    },

    ADD_HL_SP(0x39, 2) => |CPU| {

    },

    LD_A__HLD__(0x3A, 2) => |CPU| {

    },

    DEC_SP(0x3B, 2) => |CPU| {

    },

    INC_A(0x3C, 1) => |CPU| {

    },

    DEC_A(0x3D, 1) => |CPU| {

    },

    LD_A_n8(0x3E, 2) => |CPU| {

    },

    CCF(0x3F, 1) => |CPU| {

    },

    LD_B_B(0x40, 1) => |CPU| {

    },

    LD_B_C(0x41, 1) => |CPU| {

    },

    LD_B_D(0x42, 1) => |CPU| {

    },

    LD_B_E(0x43, 1) => |CPU| {

    },

    LD_B_H(0x44, 1) => |CPU| {

    },

    LD_B_L(0x45, 1) => |CPU| {

    },

    LD_B__HL__(0x46, 2) => |CPU| {

    },

    LD_B_A(0x47, 1) => |CPU| {

    },

    LD_C_B(0x48, 1) => |CPU| {

    },

    LD_C_C(0x49, 1) => |CPU| {

    },

    LD_C_D(0x4A, 1) => |CPU| {

    },

    LD_C_E(0x4B, 1) => |CPU| {

    },

    LD_C_H(0x4C, 1) => |CPU| {

    },

    LD_C_L(0x4D, 1) => |CPU| {

    },

    LD_C__HL__(0x4E, 2) => |CPU| {

    },

    LD_C_A(0x4F, 1) => |CPU| {

    },

    LD_D_B(0x50, 1) => |CPU| {

    },

    LD_D_C(0x51, 1) => |CPU| {

    },

    LD_D_D(0x52, 1) => |CPU| {

    },

    LD_D_E(0x53, 1) => |CPU| {

    },

    LD_D_H(0x54, 1) => |CPU| {

    },

    LD_D_L(0x55, 1) => |CPU| {

    },

    LD_D__HL__(0x56, 2) => |CPU| {

    },

    LD_D_A(0x57, 1) => |CPU| {

    },

    LD_E_B(0x58, 1) => |CPU| {

    },

    LD_E_C(0x59, 1) => |CPU| {

    },

    LD_E_D(0x5A, 1) => |CPU| {

    },

    LD_E_E(0x5B, 1) => |CPU| {

    },

    LD_E_H(0x5C, 1) => |CPU| {

    },

    LD_E_L(0x5D, 1) => |CPU| {

    },

    LD_E__HL__(0x5E, 2) => |CPU| {

    },

    LD_E_A(0x5F, 1) => |CPU| {

    },

    LD_H_B(0x60, 1) => |CPU| {

    },

    LD_H_C(0x61, 1) => |CPU| {

    },

    LD_H_D(0x62, 1) => |CPU| {

    },

    LD_H_E(0x63, 1) => |CPU| {

    },

    LD_H_H(0x64, 1) => |CPU| {

    },

    LD_H_L(0x65, 1) => |CPU| {

    },

    LD_H__HL__(0x66, 2) => |CPU| {

    },

    LD_H_A(0x67, 1) => |CPU| {

    },

    LD_L_B(0x68, 1) => |CPU| {

    },

    LD_L_C(0x69, 1) => |CPU| {

    },

    LD_L_D(0x6A, 1) => |CPU| {

    },

    LD_L_E(0x6B, 1) => |CPU| {

    },

    LD_L_H(0x6C, 1) => |CPU| {

    },

    LD_L_L(0x6D, 1) => |CPU| {

    },

    LD_L__HL__(0x6E, 2) => |CPU| {

    },

    LD_L_A(0x6F, 1) => |CPU| {

    },

    LD__HL__B(0x70, 2) => |CPU| {

    },

    LD__HL__C(0x71, 2) => |CPU| {

    },

    LD__HL__D(0x72, 2) => |CPU| {

    },

    LD__HL__E(0x73, 2) => |CPU| {

    },

    LD__HL__H(0x74, 2) => |CPU| {

    },

    LD__HL__L(0x75, 2) => |CPU| {

    },

    HALT(0x76, 1) => |CPU| {

    },

    LD__HL__A(0x77, 2) => |CPU| {

    },

    LD_A_B(0x78, 1) => |CPU| {

    },

    LD_A_C(0x79, 1) => |CPU| {

    },

    LD_A_D(0x7A, 1) => |CPU| {

    },

    LD_A_E(0x7B, 1) => |CPU| {

    },

    LD_A_H(0x7C, 1) => |CPU| {

    },

    LD_A_L(0x7D, 1) => |CPU| {

    },

    LD_A__HL_(0x7E, 2) => |CPU| {

    },

    LD_A_A(0x7F, 1) => |CPU| {

    },

    ADD_A_B(0x80, 1) => |CPU| {

    },

    ADD_A_C(0x81, 1) => |CPU| {

    },

    ADD_A_D(0x82, 1) => |CPU| {

    },

    ADD_A_E(0x83, 1) => |CPU| {

    },

    ADD_A_H(0x84, 1) => |CPU| {

    },

    ADD_A_L(0x85, 1) => |CPU| {

    },

    ADD_A__HL_(0x86, 2) => |CPU| {

    },

    ADD_A_A(0x87, 1) => |CPU| {

    },

    ADC_A_B(0x88, 1) => |CPU| {

    },

    ADC_A_C(0x89, 1) => |CPU| {

    },

    ADC_A_D(0x8A, 1) => |CPU| {

    },

    ADC_A_E(0x8B, 1) => |CPU| {

    },

    ADC_A_H(0x8C, 1) => |CPU| {

    },

    ADC_A_L(0x8D, 1) => |CPU| {

    },

    ADC_A__HL_(0x8E, 2) => |CPU| {

    },

    ADC_A_A(0x8F, 1) => |CPU| {

    },

    SUB_A_B(0x90, 1) => |CPU| {

    },

    SUB_A_C(0x91, 1) => |CPU| {

    },

    SUB_A_D(0x92, 1) => |CPU| {

    },

    SUB_A_E(0x93, 1) => |CPU| {

    },

    SUB_A_H(0x94, 1) => |CPU| {

    },

    SUB_A_L(0x95, 1) => |CPU| {

    },

    SUB_A__HL_(0x96, 2) => |CPU| {

    },

    SUB_A_A(0x97, 1) => |CPU| {

    },

    SBC_A_B(0x98, 1) => |CPU| {

    },

    SBC_A_C(0x99, 1) => |CPU| {

    },

    SBC_A_D(0x9A, 1) => |CPU| {

    },

    SBC_A_E(0x9B, 1) => |CPU| {

    },

    SBC_A_H(0x9C, 1) => |CPU| {

    },

    SBC_A_L(0x9D, 1) => |CPU| {

    },

    SBC_A__HL_(0x9E, 2) => |CPU| {

    },

    SBC_A_A(0x9F, 1) => |CPU| {

    },

    AND_A_B(0xA0, 1) => |CPU| {

    },

    AND_A_C(0xA1, 1) => |CPU| {

    },

    AND_A_D(0xA2, 1) => |CPU| {

    },

    AND_A_E(0xA3, 1) => |CPU| {

    },

    AND_A_H(0xA4, 1) => |CPU| {

    },

    AND_A_L(0xA5, 1) => |CPU| {

    },

    AND_A__HL_(0xA6, 2) => |CPU| {

    },

    AND_A_A(0xA7, 1) => |CPU| {

    },

    XOR_A_B(0xA8, 1) => |CPU| {

    },

    XOR_A_C(0xA9, 1) => |CPU| {

    },

    XOR_A_D(0xAA, 1) => |CPU| {

    },

    XOR_A_E(0xAB, 1) => |CPU| {

    },

    XOR_A_H(0xAC, 1) => |CPU| {

    },

    XOR_A_L(0xAD, 1) => |CPU| {

    },

    XOR_A__HL_(0xAE, 2) => |CPU| {

    },

    XOR_A_A(0xAF, 1) => |CPU| {

    },

    OR_A_B(0xB0, 1) => |CPU| {

    },

    OR_A_C(0xB1, 1) => |CPU| {

    },

    OR_A_D(0xB2, 1) => |CPU| {

    },

    OR_A_E(0xB3, 1) => |CPU| {

    },

    OR_A_H(0xB4, 1) => |CPU| {

    },

    OR_A_L(0xB5, 1) => |CPU| {

    },

    OR_A__HL_(0xB6, 2) => |CPU| {

    },

    OR_A_A(0xB7, 1) => |CPU| {

    },

    CP_A_B(0xB8, 1) => |CPU| {

    },

    CP_A_C(0xB9, 1) => |CPU| {

    },

    CP_A_D(0xBA, 1) => |CPU| {

    },

    CP_A_E(0xBB, 1) => |CPU| {

    },

    CP_A_H(0xBC, 1) => |CPU| {

    },

    CP_A_L(0xBD, 1) => |CPU| {

    },

    CP_A__HL_(0xBE, 1) => |CPU| {

    },

    CP_A_A(0xBF, 1) => |CPU| {

    },

    RET_NZ(0xC0, 5/2) => |CPU| {

    },

    POP_BC(0xC1, 3) => |CPU| {

    },

    JP_NZ_a16(0xC2, 4/3) => |CPU| {

    },

    JP_a16(0xC3, 4) => |CPU| {

    },

    CALL_NZ_a16(0xC4, 6/3) => |CPU| {

    },

    PUSH_BC(0xC5, 4) => |CPU| {

    },

    ADD_A_n8(0xC6, 2) => |CPU| {

    },

    RST_00(0xC7, 4) => |CPU| {

    },

    RET_Z(0xC8, 5/2) => |CPU| {

    },

    RET(0xC9, 4) => |CPU| {

    },

    JP_Z_a16(0xCA, 4/3) => |CPU| {

    },

    PREFIX(0xCB, 1) => |CPU| {

    },

    CALL_Z_a16(0xCC, 6/3) => |CPU| {

    },

    CALL_a16(0xCD, 6) => |CPU| {

    },

    ADC_A_n8(0xCE, 2) => |CPU| {

    },

    RST_08(0xCF, 4) => |CPU| {

    },

    RET_NC(0xD0, 5/2) => |CPU| {

    },

    POP_DE(0xD1, 3) => |CPU| {

    },

    JP_NC_a16(0xD2, 4/3) => |CPU| {

    },

    ILLEGAL_D3(0xD3, 1) => |CPU| {

    },

    CALL_NC_a16(0xD4, 6/3) => |CPU| {

    },

    PUSH_DE(0xD5, 4) => |CPU| {

    },

    SUB_A_n8(0xD6, 2) => |CPU| {

    },

    RST_10(0xD7, 4) => |CPU| {

    },

    RET_C(0xD8, 5/2) => |CPU| {

    },

    RETI(0xD9, 4) => |CPU| {

    },

    JP_C_a16(0xDA, 4/3) => |CPU| {

    },

    ILLEGAL_D8(0xDB, 1) => |CPU| {

    },

    CALL_C_a16(0xDC, 6/3) => |CPU| {

    },

    SBC_A_n8(0xDE, 2) => |CPU| {

    },

    RST_18(0xDF, 4) => |CPU| {

    },

    LDH__a8__A(0xE0, 3) => |CPU| {

    },

    POP_HL(0xE1, 3) => |CPU| {

    },

    LDH__C__A(0xE2, 2) => |CPU| {

    },

    ILLEGAL_E3(0xE3, 1) => |CPU| {

    },

    ILLEGAL_E4(0xE4, 1) => |CPU| {

    },

    PUSH_HL(0xE5, 4) => |CPU| {

    },

    AND_A_n8(0xE6, 2) => |CPU| {

    },

    RST_20(0xE7, 4) => |CPU| {

    },

    ADD_SP_e8(0xE8, 4) => |CPU| {

    },

    JP_HL(0xE9, 1) => |CPU| {

    },

    LD__a16__A(0xEA, 4) => |CPU| {

    },

    ILLEGAL_EB(0xEB, 1) => |CPU| {

    },

    ILLEGAL_EC(0xEC, 1) => |CPU| {

    },

    ILLEGAL_ED(0xED, 1) => |CPU| {

    },

    XOR_A_n8(0xEE, 2) => |CPU| {

    },

    RST_28(0xEF, 4) => |CPU| {

    },

    LDH_A__a8_(0xF0, 3) => |CPU| {

    },

    POP_AF(0xF1, 3) => |CPU| {

    },

    LDH_A__C_(0xF2, 2) => |CPU| {

    },

    DI(0xF3, 1) => |CPU| {

    },

    ILLEGAL_F4(0xF4, 1) => |CPU| {

    },

    PUSH_AF(0xF5, 4) => |CPU| {

    },

    OR_A_n8(0xF6, 2) => |CPU| {

    },

    RST_30(0xF7, 4) => |CPU| {

    },

    LD_HL_SPe8(0xF8, 3) => |CPU| {

    },

    LD_SP_HL(0xF9, 2) => |CPU| {

    },

    LD_A__a16_(0xFA, 4) => |CPU| {

    },

    EI(0xFB, 1) => |CPU| {

    },

    ILLEGAL_FC(0xFC, 1) => |CPU| {

    },

    ILLEGAL_FD(0xFD, 1) => |CPU| {

    },

    CP_A_n8(0xFE, 2) => |CPU| {

    },

    RST_38(0xFF, 4) => |CPU| {

    },
);

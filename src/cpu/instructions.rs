use crate::cpu::opcodes::OPCODE_TABLE;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
    AF,
}
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
pub enum Instruction {
    // 8-bit Load instructions
    LD_r8_r8(Register8, Register8), // LD r8, r8
    LD_r8_n8(Register8, u8),        // LD r8, n8
    LD_r8_HLINDIRECT(Register8),    // LD r8, (HL)
    LD_HLINDIRECT_r8(Register8),    // LD (HL), r8
    LD_HLINDIRECT_n8(u8),           // LD (HL), n8
    LD_A_BC(Register16),            // LD A, (BC)
    LD_A_DE(Register16),            // LD A, (DE)
    LD_BC_A(Register16),            // LD (BC), A
    LD_DE_A(Register16),            // LD (DE), A
    LD_A_n16(u16),                  // LD A, (n16)
    LD_n16_A(u16),                  // LD (n16), A
    LDH_A_C,                        // LD A, (0xFF00+C)
    LDH_C_A,                        // LD (0xFF00+C), A
    LDH_A_n8(u8),                   // LD A, (0xFF00+n8)
    LDH_n8_A(u8),                   // LD (0xFF00+n8), A
    LD_A_HLI,                       // LD A, (HL+)
    LD_A_HLD,                       // LD A, (HL-)
    LD_HLI_A,                       // LD (HL+), A
    LD_HLD_A,                       // LD (HL-), A
    /* 16-Bit Load */
    LD_r16_n16(Register16, u16), // LD r16, n16
    LD_n16_SP(u16),              // LD (n16), SP
    LD_SP_HL,                    // LD SP, HL
    PUSH_r16(Register16),        // PUSH r16
    POP_r16(Register16),         // POP r16
    /* 8-Bit Arithmetic/Logic */
    ADD_A_r8(Register8), // ADD A, r8
    ADD_A_n8(u8),        // ADD A, n8
    ADD_A_HL,            // ADD A, (HL)
    ADC_A_r8(Register8), // ADC A, r8
    ADC_A_n8(u8),        // ADC A, n8
    ADC_A_HL,            // ADC A, (HL)
    SUB_A_r8(Register8), // SUB A, r8
    SUB_A_n8(u8),        // SUB A, n8
    SUB_A_HL,            // SUB A, (HL)
    SBC_A_r8(Register8), // SBC A, r8
    SBC_A_n8(u8),        // SBC A, n8
    SBC_A_HL,            // SBC A, (HL)
    AND_A_r8(Register8), // AND A, r8
    AND_A_n8(u8),        // AND A, n8
    AND_A_HL,            // AND A, (HL)
    XOR_A_r8(Register8), // XOR A, r8
    XOR_A_n8(u8),        // XOR A, n8
    XOR_A_HL,            // XOR A, (HL)
    OR_A_r8(Register8),  // OR A, r8
    OR_A_n8(u8),         // OR A, n8
    OR_A_HL,             // OR A, (HL)
    CP_A_r8(Register8),  // CP A, r8
    CP_A_n8(u8),         // CP A, n8
    CP_A_HL,             // CP A, (HL)
    INC_r8(Register8),   // INC r8
    INC_HL,              // INC (HL)
    DEC_r8(Register8),   // DEC r8
    DEC_HL,              // DEC (HL)
    DAA,                 // DAA
    CPL,                 // CPL (complement A)
    SCF,                 // SCF (set carry flag)
    CCF,                 // CCF (complement carry flag)
    /* 16-Bit Arithmetic */
    ADD_HL_r16(Register16), // ADD HL, r16
    INC_r16(Register16),    // INC r16
    DEC_r16(Register16),    // DEC r16
    ADD_SP_e8(i8),          // ADD SP, e8
    LD_HL_SP_e8(i8),        // LD HL, SP+e8
    /* Rotate/Shift */
    RLCA,               // RLCA
    RLA,                // RLA
    RRCA,               // RRCA
    RRA,                // RRA
    RLC_r8(Register8),  // RLC r8 (CB prefix)
    RLC_HL,             // RLC (HL) (CB prefix)
    RL_r8(Register8),   // RL r8 (CB prefix)
    RL_HL,              // RL (HL) (CB prefix)
    RRC_r8(Register8),  // RRC r8 (CB prefix)
    RRC_HL,             // RRC (HL) (CB prefix)
    RR_r8(Register8),   // RR r8 (CB prefix)
    RR_HL,              // RR (HL) (CB prefix)
    SLA_r8(Register8),  // SLA r8 (CB prefix)
    SLA_HL,             // SLA (HL) (CB prefix)
    SRA_r8(Register8),  // SRA r8 (CB prefix)
    SRA_HL,             // SRA (HL) (CB prefix)
    SWAP_r8(Register8), // SWAP r8 (CB prefix)
    SWAP_HL,            // SWAP (HL) (CB prefix)
    SRL_r8(Register8),  // SRL r8 (CB prefix)
    SRL_HL,             // SRL (HL) (CB prefix)
    /* Bit operations (CB prefix) */
    BIT_b_r8(u8, Register8), // BIT b, r8
    BIT_b_HL(u8),            // BIT b, (HL)
    SET_b_r8(u8, Register8), // SET b, r8
    SET_b_HL(u8),            // SET b, (HL)
    RES_b_r8(u8, Register8), // RES b, r8
    RES_b_HL(u8),            // RES b, (HL)
    /* Control Flow */
    JP_n16(u16),                 // JP n16
    JP_HL,                       // JP HL
    JP_cc_n16(Condition, u16),   // JP cc,n16
    JR_e8(i8),                   // JR e8
    JR_cc_e8(Condition, i8),     // JR cc, e8
    CALL_n16(u16),               // CALL n16
    CALL_cc_n16(Condition, u16), // CALL cc, n16
    RET,                         // RET
    RET_cc(Condition),           // RET cc
    RETI,                        // RETI
    RST_vec(u8),                 // RST vec (0x00, 0x08, ..., 0x38)
    /* Miscellaneous instructions */
    NOP,  // NOP
    HALT, // HALT
    STOP, // STOP
    DI,   // DI (disable interrupts)
    EI,   // EI (enable interrupts)

    // Illegal/Undefined
    ILLEGAL,
}


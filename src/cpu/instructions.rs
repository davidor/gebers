use crate::cpu::registers::*;

pub const PREFIX_INSTR_CODE: u8 = 0xCB;

#[derive(Debug)]
pub enum Instruction {
    // ** 8-bit arithmetic and logic ** //

    // Flags: Z 0 H C
    ADC(Register8bits),

    // Flags: Z 0 H C
    ADCHL,

    // Flags: Z 0 H C
    ADCD8,

    // Flags: Z 0 H C
    ADD(Register8bits),

    // Flags: Z 0 H C
    ADDHL,

    // Flags: Z 0 H C
    ADDD8,

    // Flags: Z 0 1 0
    AND(Register8bits),

    // Flags: Z 0 1 0
    ANDD8,

    // Flags: Z 0 1 0
    ANDHL,

    // Flags: Z 1 H C
    CP(Register8bits),

    // Flags: Z 1 H C
    CPHL,

    // Flags: Z 1 H C
    CPD8,

    // Flags: Z 1 H -
    DEC(Register8bits),

    // Flags: Z 1 H -
    DECHL,

    // Flags: Z 0 H -
    INC(Register8bits),

    // Flags: Z 0 H -
    INCHL,

    // Flags: Z 0 0 0
    OR(Register8bits),

    // Flags: Z 0 0 0
    ORHL,

    // Flags: Z 0 0 0
    ORD8,

    // Flags: Z 1 H C
    SBC(Register8bits),

    // Flags: Z 1 H C
    SBCHL,

    // Flags: Z 1 H C
    SBCD8,

    // Flags: Z 1 H C
    SUB(Register8bits),

    // Flags: Z 1 H C
    SUBHL,

    // Flags: Z 1 H C
    SUBD8,

    // Flags: Z 0 0 0
    XOR(Register8bits),

    // Flags: Z 0 0 0
    XORHL,

    // Flags: Z 0 0 0
    XORD8,

    // Flags: - 1 1 -
    CPL,

    DAA,

    // ** 16-bit arithmetic and logic ** //

    // Flags: - 0 H C
    ADD16(Register16bits),

    // Flags: - - - -
    DEC16(Register16bits),

    // Flags: - - - -
    INC16(Register16bits),

    // ** Bit operations ** //

    // Flags: Z 0 1 -
    BIT(u8, Register8bits),

    // Flags: Z 0 1 -
    BITHL(u8),

    // Flags: - - - -
    RES(u8, Register8bits),

    // Flags: - - - -
    RESHL(u8),

    // Flags: - - - -
    SET(u8, Register8bits),

    // Flags: - - - -
    SETHL(u8),

    // ** Rotates and shifts ** //

    // Flags: Z 0 0 C
    RL(Register8bits),

    // Flags: 0 0 0 C
    RLA,

    // Flags: Z 0 0 C
    RLHL,

    // Flags: Z 0 0 C
    RLC(Register8bits),

    // Flags: 0 0 0 C
    RLCA,

    // Flags: Z 0 0 C
    RLCHL,

    // Flags: Z 0 0 C
    RR(Register8bits),

    // Flags: 0 0 0 C
    RRA,

    // Flags: Z 0 0 C
    RRHL,

    // Flags: Z 0 0 C
    RRC(Register8bits),

    // Flags: 0 0 0 C
    RRCA,

    // Flags: Z 0 0 C
    RRCHL,

    // Flags: Z 0 0 C
    SLA(Register8bits),

    // Flags: Z 0 0 C
    SLAHL,

    // Flags: Z 0 0 0
    SRA(Register8bits),

    // Flags: Z 0 0 0
    SRAHL,

    // Flags: Z 0 0 C
    SRL(Register8bits),

    // Flags: Z 0 0 C
    SRLHL,

    // Flags: Z - - -
    SWAP(Register8bits),

    // Flags: Z - - -
    SWAPHL,

    // ** 8-bit load ** //

    // Flags: - - - -
    LDR8R8(Register8bits, Register8bits),

    // Flags: - - - -
    LDR8D8(Register8bits),

    // Flags - - - -
    LDHLD8,

    // Flags: - - - -
    LDR8ADDR(Register8bits, Register8bits),

    // Flags: - - - -
    LDADDRR8(Register8bits, Register8bits),

    // Flags: - - - -
    LDAHLI,

    // Flags: - - - -
    LDAHLD,

    // Flags: - - - -
    LDHLIA,

    // Flags: - - - -
    LDHLDA,

    // Flags: - - - -
    LDHLR8(Register8bits),

    // Flags: - - - -
    LDR8HL(Register8bits),

    // Flags: - - - -
    LDR16R8(Register16bits, Register8bits),

    // Flags: - - - -
    LDR8R16(Register8bits, Register16bits),

    // Flags: - - - -
    LDA8A,

    // Flags: - - - -
    LDAA8,

    // Flags: - - - -
    LDA16A,

    // Flags: - - - -
    LDAA16,

    // ** 16-bit load ** //

    // Flags: - - - -
    LDR16D16(Register16bits),

    // Flags: - - - -
    LDSPD16,

    // Flags: - - - -
    LDA16SP,

    // Flags: - - - -
    INCSP,

    // Flags: - - - -
    DECSP,

    // Flags: - 0 H C
    ADDHLSP,

    // Flags: - 0 H C
    LDSPHL,

    // Flags: - - - -
    POP(Register16bits),

    // Flags: - - - -
    PUSH(Register16bits),

    // Flags: 0 0 H C
    ADDSPr8,

    // Flags: 0 0 H C
    LDHLSPr8,

    // ** Jumps ** //

    // Flags: - - - -
    JP(JumpCondition),

    // Flags: - - - -
    JPHL,

    // Flags: - - - -
    JR(JumpCondition),

    // Flags: - - - -
    RST(u8),

    // Flags: - - - -
    RET(JumpCondition),

    // Flags: - - - -
    RETI,

    // Flags: - - - -
    CALL(JumpCondition),

    // ** CPU control ** //

    // Flags: - 0 0 C
    CCF,

    // Flags: - - - -
    DI,

    // Flags: - - - -
    EI,

    // Flags: - - - -
    HALT,

    // Flags: - - - -
    NOP,

    // Flags: - 0 0 1
    SCF,

    // Flags: - - - -
    STOP,

    // Flags: - - - -
    PREFIX,

    UNUSED,
}

#[derive(Debug)]
pub enum JumpCondition {
    Z,
    NZ,
    C,
    NC,
    Always,
}

impl Instruction {
    pub fn decode(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,
            0x01 => Instruction::LDR16D16(Register16bits::BC),
            0x02 => Instruction::LDR16R8(Register16bits::BC, Register8bits::A),
            0x03 => Instruction::INC16(Register16bits::BC),
            0x04 => Instruction::INC(Register8bits::B),
            0x05 => Instruction::DEC(Register8bits::B),
            0x06 => Instruction::LDR8D8(Register8bits::B),
            0x07 => Instruction::RLCA,
            0x08 => Instruction::LDA16SP,
            0x09 => Instruction::ADD16(Register16bits::BC),
            0x0a => Instruction::LDR8R16(Register8bits::A, Register16bits::BC),
            0x0b => Instruction::DEC16(Register16bits::BC),
            0x0c => Instruction::INC(Register8bits::C),
            0x0d => Instruction::DEC(Register8bits::C),
            0x0e => Instruction::LDR8D8(Register8bits::C),
            0x0f => Instruction::RRCA,
            0x10 => Instruction::STOP,
            0x11 => Instruction::LDR16D16(Register16bits::DE),
            0x12 => Instruction::LDR16R8(Register16bits::DE, Register8bits::A),
            0x13 => Instruction::INC16(Register16bits::DE),
            0x14 => Instruction::INC(Register8bits::D),
            0x15 => Instruction::DEC(Register8bits::D),
            0x16 => Instruction::LDR8D8(Register8bits::D),
            0x17 => Instruction::RLA,
            0x18 => Instruction::JR(JumpCondition::Always),
            0x19 => Instruction::ADD16(Register16bits::DE),
            0x1a => Instruction::LDR8R16(Register8bits::A, Register16bits::DE),
            0x1b => Instruction::DEC16(Register16bits::DE),
            0x1c => Instruction::INC(Register8bits::E),
            0x1d => Instruction::DEC(Register8bits::E),
            0x1e => Instruction::LDR8D8(Register8bits::E),
            0x1f => Instruction::RRA,
            0x20 => Instruction::JR(JumpCondition::NZ),
            0x21 => Instruction::LDR16D16(Register16bits::HL),
            0x22 => Instruction::LDHLIA,
            0x23 => Instruction::INC16(Register16bits::HL),
            0x24 => Instruction::INC(Register8bits::H),
            0x25 => Instruction::DEC(Register8bits::H),
            0x26 => Instruction::LDR8D8(Register8bits::H),
            0x27 => Instruction::DAA,
            0x28 => Instruction::JR(JumpCondition::Z),
            0x29 => Instruction::ADD16(Register16bits::HL),
            0x2a => Instruction::LDAHLI,
            0x2b => Instruction::DEC16(Register16bits::HL),
            0x2c => Instruction::INC(Register8bits::L),
            0x2d => Instruction::DEC(Register8bits::L),
            0x2e => Instruction::LDR8D8(Register8bits::L),
            0x2f => Instruction::CPL,
            0x30 => Instruction::JR(JumpCondition::NC),
            0x31 => Instruction::LDSPD16,
            0x32 => Instruction::LDHLDA,
            0x33 => Instruction::INCSP,
            0x34 => Instruction::INCHL,
            0x35 => Instruction::DECHL,
            0x36 => Instruction::LDHLD8,
            0x37 => Instruction::SCF,
            0x38 => Instruction::JR(JumpCondition::C),
            0x39 => Instruction::ADDHLSP,
            0x3a => Instruction::LDAHLD,
            0x3b => Instruction::DECSP,
            0x3c => Instruction::INC(Register8bits::A),
            0x3d => Instruction::DEC(Register8bits::A),
            0x3e => Instruction::LDR8D8(Register8bits::A),
            0x3f => Instruction::CCF,
            0x40 => Instruction::LDR8R8(Register8bits::B, Register8bits::B),
            0x41 => Instruction::LDR8R8(Register8bits::B, Register8bits::C),
            0x42 => Instruction::LDR8R8(Register8bits::B, Register8bits::D),
            0x43 => Instruction::LDR8R8(Register8bits::B, Register8bits::E),
            0x44 => Instruction::LDR8R8(Register8bits::B, Register8bits::H),
            0x45 => Instruction::LDR8R8(Register8bits::B, Register8bits::L),
            0x46 => Instruction::LDR8HL(Register8bits::B),
            0x47 => Instruction::LDR8R8(Register8bits::B, Register8bits::A),
            0x48 => Instruction::LDR8R8(Register8bits::C, Register8bits::B),
            0x49 => Instruction::LDR8R8(Register8bits::C, Register8bits::C),
            0x4a => Instruction::LDR8R8(Register8bits::C, Register8bits::D),
            0x4b => Instruction::LDR8R8(Register8bits::C, Register8bits::E),
            0x4c => Instruction::LDR8R8(Register8bits::C, Register8bits::H),
            0x4d => Instruction::LDR8R8(Register8bits::C, Register8bits::L),
            0x4e => Instruction::LDR8HL(Register8bits::C),
            0x4f => Instruction::LDR8R8(Register8bits::C, Register8bits::A),
            0x50 => Instruction::LDR8R8(Register8bits::D, Register8bits::B),
            0x51 => Instruction::LDR8R8(Register8bits::D, Register8bits::C),
            0x52 => Instruction::LDR8R8(Register8bits::D, Register8bits::D),
            0x53 => Instruction::LDR8R8(Register8bits::D, Register8bits::E),
            0x54 => Instruction::LDR8R8(Register8bits::D, Register8bits::H),
            0x55 => Instruction::LDR8R8(Register8bits::D, Register8bits::L),
            0x56 => Instruction::LDR8HL(Register8bits::D),
            0x57 => Instruction::LDR8R8(Register8bits::D, Register8bits::A),
            0x58 => Instruction::LDR8R8(Register8bits::E, Register8bits::B),
            0x59 => Instruction::LDR8R8(Register8bits::E, Register8bits::C),
            0x5a => Instruction::LDR8R8(Register8bits::E, Register8bits::D),
            0x5b => Instruction::LDR8R8(Register8bits::E, Register8bits::E),
            0x5c => Instruction::LDR8R8(Register8bits::E, Register8bits::H),
            0x5d => Instruction::LDR8R8(Register8bits::E, Register8bits::L),
            0x5e => Instruction::LDR8HL(Register8bits::E),
            0x5f => Instruction::LDR8R8(Register8bits::E, Register8bits::A),
            0x60 => Instruction::LDR8R8(Register8bits::H, Register8bits::B),
            0x61 => Instruction::LDR8R8(Register8bits::H, Register8bits::C),
            0x62 => Instruction::LDR8R8(Register8bits::H, Register8bits::D),
            0x63 => Instruction::LDR8R8(Register8bits::H, Register8bits::E),
            0x64 => Instruction::LDR8R8(Register8bits::H, Register8bits::H),
            0x65 => Instruction::LDR8R8(Register8bits::H, Register8bits::L),
            0x66 => Instruction::LDR8HL(Register8bits::H),
            0x67 => Instruction::LDR8R8(Register8bits::H, Register8bits::A),
            0x68 => Instruction::LDR8R8(Register8bits::L, Register8bits::B),
            0x69 => Instruction::LDR8R8(Register8bits::L, Register8bits::C),
            0x6a => Instruction::LDR8R8(Register8bits::L, Register8bits::D),
            0x6b => Instruction::LDR8R8(Register8bits::L, Register8bits::E),
            0x6c => Instruction::LDR8R8(Register8bits::L, Register8bits::H),
            0x6d => Instruction::LDR8R8(Register8bits::L, Register8bits::L),
            0x6e => Instruction::LDR8HL(Register8bits::L),
            0x6f => Instruction::LDR8R8(Register8bits::L, Register8bits::A),
            0x70 => Instruction::LDHLR8(Register8bits::B),
            0x71 => Instruction::LDHLR8(Register8bits::C),
            0x72 => Instruction::LDHLR8(Register8bits::D),
            0x73 => Instruction::LDHLR8(Register8bits::E),
            0x74 => Instruction::LDHLR8(Register8bits::H),
            0x75 => Instruction::LDHLR8(Register8bits::L),
            0x76 => Instruction::HALT,
            0x77 => Instruction::LDHLR8(Register8bits::A),
            0x78 => Instruction::LDR8R8(Register8bits::A, Register8bits::B),
            0x79 => Instruction::LDR8R8(Register8bits::A, Register8bits::C),
            0x7a => Instruction::LDR8R8(Register8bits::A, Register8bits::D),
            0x7b => Instruction::LDR8R8(Register8bits::A, Register8bits::E),
            0x7c => Instruction::LDR8R8(Register8bits::A, Register8bits::H),
            0x7d => Instruction::LDR8R8(Register8bits::A, Register8bits::L),
            0x7e => Instruction::LDR8HL(Register8bits::A),
            0x7f => Instruction::LDR8R8(Register8bits::A, Register8bits::A),
            0x80 => Instruction::ADD(Register8bits::B),
            0x81 => Instruction::ADD(Register8bits::C),
            0x82 => Instruction::ADD(Register8bits::D),
            0x83 => Instruction::ADD(Register8bits::E),
            0x84 => Instruction::ADD(Register8bits::H),
            0x85 => Instruction::ADD(Register8bits::L),
            0x86 => Instruction::ADDHL,
            0x87 => Instruction::ADD(Register8bits::A),
            0x88 => Instruction::ADC(Register8bits::B),
            0x89 => Instruction::ADC(Register8bits::C),
            0x8a => Instruction::ADC(Register8bits::D),
            0x8b => Instruction::ADC(Register8bits::E),
            0x8c => Instruction::ADC(Register8bits::H),
            0x8d => Instruction::ADC(Register8bits::L),
            0x8e => Instruction::ADCHL,
            0x8f => Instruction::ADC(Register8bits::A),
            0x90 => Instruction::SUB(Register8bits::B),
            0x91 => Instruction::SUB(Register8bits::C),
            0x92 => Instruction::SUB(Register8bits::D),
            0x93 => Instruction::SUB(Register8bits::E),
            0x94 => Instruction::SUB(Register8bits::H),
            0x95 => Instruction::SUB(Register8bits::L),
            0x96 => Instruction::SUBHL,
            0x97 => Instruction::SUB(Register8bits::A),
            0x98 => Instruction::SBC(Register8bits::B),
            0x99 => Instruction::SBC(Register8bits::C),
            0x9a => Instruction::SBC(Register8bits::D),
            0x9b => Instruction::SBC(Register8bits::E),
            0x9c => Instruction::SBC(Register8bits::H),
            0x9d => Instruction::SBC(Register8bits::L),
            0x9e => Instruction::SBCHL,
            0x9f => Instruction::SBC(Register8bits::A),
            0xa0 => Instruction::AND(Register8bits::B),
            0xa1 => Instruction::AND(Register8bits::C),
            0xa2 => Instruction::AND(Register8bits::D),
            0xa3 => Instruction::AND(Register8bits::E),
            0xa4 => Instruction::AND(Register8bits::H),
            0xa5 => Instruction::AND(Register8bits::L),
            0xa6 => Instruction::ANDHL,
            0xa7 => Instruction::AND(Register8bits::A),
            0xa8 => Instruction::XOR(Register8bits::B),
            0xa9 => Instruction::XOR(Register8bits::C),
            0xaa => Instruction::XOR(Register8bits::D),
            0xab => Instruction::XOR(Register8bits::E),
            0xac => Instruction::XOR(Register8bits::H),
            0xad => Instruction::XOR(Register8bits::L),
            0xae => Instruction::XORHL,
            0xaf => Instruction::XOR(Register8bits::A),
            0xb0 => Instruction::OR(Register8bits::B),
            0xb1 => Instruction::OR(Register8bits::C),
            0xb2 => Instruction::OR(Register8bits::D),
            0xb3 => Instruction::OR(Register8bits::E),
            0xb4 => Instruction::OR(Register8bits::H),
            0xb5 => Instruction::OR(Register8bits::L),
            0xb6 => Instruction::ORHL,
            0xb7 => Instruction::OR(Register8bits::A),
            0xb8 => Instruction::CP(Register8bits::B),
            0xb9 => Instruction::CP(Register8bits::C),
            0xba => Instruction::CP(Register8bits::D),
            0xbb => Instruction::CP(Register8bits::E),
            0xbc => Instruction::CP(Register8bits::H),
            0xbd => Instruction::CP(Register8bits::L),
            0xbe => Instruction::CPHL,
            0xbf => Instruction::CP(Register8bits::A),
            0xc0 => Instruction::RET(JumpCondition::NZ),
            0xc1 => Instruction::POP(Register16bits::BC),
            0xc2 => Instruction::JP(JumpCondition::NZ),
            0xc3 => Instruction::JP(JumpCondition::Always),
            0xc4 => Instruction::CALL(JumpCondition::NZ),
            0xc5 => Instruction::PUSH(Register16bits::BC),
            0xc6 => Instruction::ADDD8,
            0xc7 => Instruction::RST(0),
            0xc8 => Instruction::RET(JumpCondition::Z),
            0xc9 => Instruction::RET(JumpCondition::Always),
            0xca => Instruction::JP(JumpCondition::Z),
            0xcb => Instruction::PREFIX,
            0xcc => Instruction::CALL(JumpCondition::Z),
            0xcd => Instruction::CALL(JumpCondition::Always),
            0xce => Instruction::ADCD8,
            0xcf => Instruction::RST(0x08),
            0xd0 => Instruction::RET(JumpCondition::NC),
            0xd1 => Instruction::POP(Register16bits::DE),
            0xd2 => Instruction::JP(JumpCondition::NC),
            0xd3 => Instruction::UNUSED,
            0xd4 => Instruction::CALL(JumpCondition::NC),
            0xd5 => Instruction::PUSH(Register16bits::DE),
            0xd6 => Instruction::SUBD8,
            0xd7 => Instruction::RST(0x10),
            0xd8 => Instruction::RET(JumpCondition::C),
            0xd9 => Instruction::RETI,
            0xda => Instruction::JP(JumpCondition::C),
            0xdb => Instruction::UNUSED,
            0xdc => Instruction::CALL(JumpCondition::C),
            0xdd => Instruction::UNUSED,
            0xde => Instruction::SBCD8,
            0xdf => Instruction::RST(0x18),
            0xe0 => Instruction::LDA8A,
            0xe1 => Instruction::POP(Register16bits::HL),
            0xe2 => Instruction::LDADDRR8(Register8bits::C, Register8bits::A),
            0xe3 => Instruction::UNUSED,
            0xe4 => Instruction::UNUSED,
            0xe5 => Instruction::PUSH(Register16bits::HL),
            0xe6 => Instruction::ANDD8,
            0xe7 => Instruction::RST(0x20),
            0xe8 => Instruction::ADDSPr8,
            0xe9 => Instruction::JPHL,
            0xea => Instruction::LDA16A,
            0xeb => Instruction::UNUSED,
            0xec => Instruction::UNUSED,
            0xed => Instruction::UNUSED,
            0xee => Instruction::XORD8,
            0xef => Instruction::RST(0x28),
            0xf0 => Instruction::LDAA8,
            0xf1 => Instruction::POP(Register16bits::AF),
            0xf2 => Instruction::LDR8ADDR(Register8bits::A, Register8bits::C),
            0xf3 => Instruction::DI,
            0xf4 => Instruction::UNUSED,
            0xf5 => Instruction::PUSH(Register16bits::AF),
            0xf6 => Instruction::ORD8,
            0xf7 => Instruction::RST(0x30),
            0xf8 => Instruction::LDHLSPr8,
            0xf9 => Instruction::LDSPHL,
            0xfa => Instruction::LDAA16,
            0xfb => Instruction::EI,
            0xfc => Instruction::UNUSED,
            0xfd => Instruction::UNUSED,
            0xfe => Instruction::CPD8,
            0xff => Instruction::RST(0x38),
        }
    }

    pub fn decode_prefixed(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::RLC(Register8bits::B),
            0x01 => Instruction::RLC(Register8bits::C),
            0x02 => Instruction::RLC(Register8bits::D),
            0x03 => Instruction::RLC(Register8bits::E),
            0x04 => Instruction::RLC(Register8bits::H),
            0x05 => Instruction::RLC(Register8bits::L),
            0x06 => Instruction::RLCHL,
            0x07 => Instruction::RLC(Register8bits::A),
            0x08 => Instruction::RRC(Register8bits::B),
            0x09 => Instruction::RRC(Register8bits::C),
            0x0a => Instruction::RRC(Register8bits::D),
            0x0b => Instruction::RRC(Register8bits::E),
            0x0c => Instruction::RRC(Register8bits::H),
            0x0d => Instruction::RRC(Register8bits::L),
            0x0e => Instruction::RRCHL,
            0x0f => Instruction::RRC(Register8bits::A),
            0x10 => Instruction::RL(Register8bits::B),
            0x11 => Instruction::RL(Register8bits::C),
            0x12 => Instruction::RL(Register8bits::D),
            0x13 => Instruction::RL(Register8bits::E),
            0x14 => Instruction::RL(Register8bits::H),
            0x15 => Instruction::RL(Register8bits::L),
            0x16 => Instruction::RLHL,
            0x17 => Instruction::RL(Register8bits::A),
            0x18 => Instruction::RR(Register8bits::B),
            0x19 => Instruction::RR(Register8bits::C),
            0x1a => Instruction::RR(Register8bits::D),
            0x1b => Instruction::RR(Register8bits::E),
            0x1c => Instruction::RR(Register8bits::H),
            0x1d => Instruction::RR(Register8bits::L),
            0x1e => Instruction::RRHL,
            0x1f => Instruction::RR(Register8bits::A),
            0x20 => Instruction::SLA(Register8bits::B),
            0x21 => Instruction::SLA(Register8bits::C),
            0x22 => Instruction::SLA(Register8bits::D),
            0x23 => Instruction::SLA(Register8bits::E),
            0x24 => Instruction::SLA(Register8bits::H),
            0x25 => Instruction::SLA(Register8bits::L),
            0x26 => Instruction::SLAHL,
            0x27 => Instruction::SLA(Register8bits::A),
            0x28 => Instruction::SRA(Register8bits::B),
            0x29 => Instruction::SRA(Register8bits::C),
            0x2a => Instruction::SRA(Register8bits::D),
            0x2b => Instruction::SRA(Register8bits::E),
            0x2c => Instruction::SRA(Register8bits::H),
            0x2d => Instruction::SRA(Register8bits::L),
            0x2e => Instruction::SRAHL,
            0x2f => Instruction::SRA(Register8bits::A),
            0x30 => Instruction::SWAP(Register8bits::B),
            0x31 => Instruction::SWAP(Register8bits::C),
            0x32 => Instruction::SWAP(Register8bits::D),
            0x33 => Instruction::SWAP(Register8bits::E),
            0x34 => Instruction::SWAP(Register8bits::H),
            0x35 => Instruction::SWAP(Register8bits::L),
            0x36 => Instruction::SWAPHL,
            0x37 => Instruction::SWAP(Register8bits::A),
            0x38 => Instruction::SRL(Register8bits::B),
            0x39 => Instruction::SRL(Register8bits::C),
            0x3a => Instruction::SRL(Register8bits::D),
            0x3b => Instruction::SRL(Register8bits::E),
            0x3c => Instruction::SRL(Register8bits::H),
            0x3d => Instruction::SRL(Register8bits::L),
            0x3e => Instruction::SRLHL,
            0x3f => Instruction::SRL(Register8bits::A),
            0x40 => Instruction::BIT(0, Register8bits::B),
            0x41 => Instruction::BIT(0, Register8bits::C),
            0x42 => Instruction::BIT(0, Register8bits::D),
            0x43 => Instruction::BIT(0, Register8bits::E),
            0x44 => Instruction::BIT(0, Register8bits::H),
            0x45 => Instruction::BIT(0, Register8bits::L),
            0x46 => Instruction::BITHL(0),
            0x47 => Instruction::BIT(0, Register8bits::A),
            0x48 => Instruction::BIT(1, Register8bits::B),
            0x49 => Instruction::BIT(1, Register8bits::C),
            0x4a => Instruction::BIT(1, Register8bits::D),
            0x4b => Instruction::BIT(1, Register8bits::E),
            0x4c => Instruction::BIT(1, Register8bits::H),
            0x4d => Instruction::BIT(1, Register8bits::L),
            0x4e => Instruction::BITHL(1),
            0x4f => Instruction::BIT(1, Register8bits::A),
            0x50 => Instruction::BIT(2, Register8bits::B),
            0x51 => Instruction::BIT(2, Register8bits::C),
            0x52 => Instruction::BIT(2, Register8bits::D),
            0x53 => Instruction::BIT(2, Register8bits::E),
            0x54 => Instruction::BIT(2, Register8bits::H),
            0x55 => Instruction::BIT(2, Register8bits::L),
            0x56 => Instruction::BITHL(2),
            0x57 => Instruction::BIT(2, Register8bits::A),
            0x58 => Instruction::BIT(3, Register8bits::B),
            0x59 => Instruction::BIT(3, Register8bits::C),
            0x5a => Instruction::BIT(3, Register8bits::D),
            0x5b => Instruction::BIT(3, Register8bits::E),
            0x5c => Instruction::BIT(3, Register8bits::H),
            0x5d => Instruction::BIT(3, Register8bits::L),
            0x5e => Instruction::BITHL(3),
            0x5f => Instruction::BIT(3, Register8bits::A),
            0x60 => Instruction::BIT(4, Register8bits::B),
            0x61 => Instruction::BIT(4, Register8bits::C),
            0x62 => Instruction::BIT(4, Register8bits::D),
            0x63 => Instruction::BIT(4, Register8bits::E),
            0x64 => Instruction::BIT(4, Register8bits::H),
            0x65 => Instruction::BIT(4, Register8bits::L),
            0x66 => Instruction::BITHL(4),
            0x67 => Instruction::BIT(4, Register8bits::A),
            0x68 => Instruction::BIT(5, Register8bits::B),
            0x69 => Instruction::BIT(5, Register8bits::C),
            0x6a => Instruction::BIT(5, Register8bits::D),
            0x6b => Instruction::BIT(5, Register8bits::E),
            0x6c => Instruction::BIT(5, Register8bits::H),
            0x6d => Instruction::BIT(5, Register8bits::L),
            0x6e => Instruction::BITHL(5),
            0x6f => Instruction::BIT(5, Register8bits::A),
            0x70 => Instruction::BIT(6, Register8bits::B),
            0x71 => Instruction::BIT(6, Register8bits::C),
            0x72 => Instruction::BIT(6, Register8bits::D),
            0x73 => Instruction::BIT(6, Register8bits::E),
            0x74 => Instruction::BIT(6, Register8bits::H),
            0x75 => Instruction::BIT(6, Register8bits::L),
            0x76 => Instruction::BITHL(6),
            0x77 => Instruction::BIT(6, Register8bits::A),
            0x78 => Instruction::BIT(7, Register8bits::B),
            0x79 => Instruction::BIT(7, Register8bits::C),
            0x7a => Instruction::BIT(7, Register8bits::D),
            0x7b => Instruction::BIT(7, Register8bits::E),
            0x7c => Instruction::BIT(7, Register8bits::H),
            0x7d => Instruction::BIT(7, Register8bits::L),
            0x7e => Instruction::BITHL(7),
            0x7f => Instruction::BIT(7, Register8bits::A),
            0x80 => Instruction::RES(0, Register8bits::B),
            0x81 => Instruction::RES(0, Register8bits::C),
            0x82 => Instruction::RES(0, Register8bits::D),
            0x83 => Instruction::RES(0, Register8bits::E),
            0x84 => Instruction::RES(0, Register8bits::H),
            0x85 => Instruction::RES(0, Register8bits::L),
            0x86 => Instruction::RESHL(0),
            0x87 => Instruction::RES(0, Register8bits::A),
            0x88 => Instruction::RES(1, Register8bits::B),
            0x89 => Instruction::RES(1, Register8bits::C),
            0x8a => Instruction::RES(1, Register8bits::D),
            0x8b => Instruction::RES(1, Register8bits::E),
            0x8c => Instruction::RES(1, Register8bits::H),
            0x8d => Instruction::RES(1, Register8bits::L),
            0x8e => Instruction::RESHL(1),
            0x8f => Instruction::RES(1, Register8bits::A),
            0x90 => Instruction::RES(2, Register8bits::B),
            0x91 => Instruction::RES(2, Register8bits::C),
            0x92 => Instruction::RES(2, Register8bits::D),
            0x93 => Instruction::RES(2, Register8bits::E),
            0x94 => Instruction::RES(2, Register8bits::H),
            0x95 => Instruction::RES(2, Register8bits::L),
            0x96 => Instruction::RESHL(2),
            0x97 => Instruction::RES(2, Register8bits::A),
            0x98 => Instruction::RES(3, Register8bits::B),
            0x99 => Instruction::RES(3, Register8bits::C),
            0x9a => Instruction::RES(3, Register8bits::D),
            0x9b => Instruction::RES(3, Register8bits::E),
            0x9c => Instruction::RES(3, Register8bits::H),
            0x9d => Instruction::RES(3, Register8bits::L),
            0x9e => Instruction::RESHL(3),
            0x9f => Instruction::RES(3, Register8bits::A),
            0xa0 => Instruction::RES(4, Register8bits::B),
            0xa1 => Instruction::RES(4, Register8bits::C),
            0xa2 => Instruction::RES(4, Register8bits::D),
            0xa3 => Instruction::RES(4, Register8bits::E),
            0xa4 => Instruction::RES(4, Register8bits::H),
            0xa5 => Instruction::RES(4, Register8bits::L),
            0xa6 => Instruction::RESHL(4),
            0xa7 => Instruction::RES(4, Register8bits::A),
            0xa8 => Instruction::RES(5, Register8bits::B),
            0xa9 => Instruction::RES(5, Register8bits::C),
            0xaa => Instruction::RES(5, Register8bits::D),
            0xab => Instruction::RES(5, Register8bits::E),
            0xac => Instruction::RES(5, Register8bits::H),
            0xad => Instruction::RES(5, Register8bits::L),
            0xae => Instruction::RESHL(5),
            0xaf => Instruction::RES(5, Register8bits::A),
            0xb0 => Instruction::RES(6, Register8bits::B),
            0xb1 => Instruction::RES(6, Register8bits::C),
            0xb2 => Instruction::RES(6, Register8bits::D),
            0xb3 => Instruction::RES(6, Register8bits::E),
            0xb4 => Instruction::RES(6, Register8bits::H),
            0xb5 => Instruction::RES(6, Register8bits::L),
            0xb6 => Instruction::RESHL(6),
            0xb7 => Instruction::RES(6, Register8bits::A),
            0xb8 => Instruction::RES(7, Register8bits::B),
            0xb9 => Instruction::RES(7, Register8bits::C),
            0xba => Instruction::RES(7, Register8bits::D),
            0xbb => Instruction::RES(7, Register8bits::E),
            0xbc => Instruction::RES(7, Register8bits::H),
            0xbd => Instruction::RES(7, Register8bits::L),
            0xbe => Instruction::RESHL(7),
            0xbf => Instruction::RES(7, Register8bits::A),
            0xc0 => Instruction::SET(0, Register8bits::B),
            0xc1 => Instruction::SET(0, Register8bits::C),
            0xc2 => Instruction::SET(0, Register8bits::D),
            0xc3 => Instruction::SET(0, Register8bits::E),
            0xc4 => Instruction::SET(0, Register8bits::H),
            0xc5 => Instruction::SET(0, Register8bits::L),
            0xc6 => Instruction::SETHL(0),
            0xc7 => Instruction::SET(0, Register8bits::A),
            0xc8 => Instruction::SET(1, Register8bits::B),
            0xc9 => Instruction::SET(1, Register8bits::C),
            0xca => Instruction::SET(1, Register8bits::D),
            0xcb => Instruction::SET(1, Register8bits::E),
            0xcc => Instruction::SET(1, Register8bits::H),
            0xcd => Instruction::SET(1, Register8bits::L),
            0xce => Instruction::SETHL(1),
            0xcf => Instruction::SET(1, Register8bits::A),
            0xd0 => Instruction::SET(2, Register8bits::B),
            0xd1 => Instruction::SET(2, Register8bits::C),
            0xd2 => Instruction::SET(2, Register8bits::D),
            0xd3 => Instruction::SET(2, Register8bits::E),
            0xd4 => Instruction::SET(2, Register8bits::H),
            0xd5 => Instruction::SET(2, Register8bits::L),
            0xd6 => Instruction::SETHL(2),
            0xd7 => Instruction::SET(2, Register8bits::A),
            0xd8 => Instruction::SET(3, Register8bits::B),
            0xd9 => Instruction::SET(3, Register8bits::C),
            0xda => Instruction::SET(3, Register8bits::D),
            0xdb => Instruction::SET(3, Register8bits::E),
            0xdc => Instruction::SET(3, Register8bits::H),
            0xdd => Instruction::SET(3, Register8bits::L),
            0xde => Instruction::SETHL(3),
            0xdf => Instruction::SET(3, Register8bits::A),
            0xe0 => Instruction::SET(4, Register8bits::B),
            0xe1 => Instruction::SET(4, Register8bits::C),
            0xe2 => Instruction::SET(4, Register8bits::D),
            0xe3 => Instruction::SET(4, Register8bits::E),
            0xe4 => Instruction::SET(4, Register8bits::H),
            0xe5 => Instruction::SET(4, Register8bits::L),
            0xe6 => Instruction::SETHL(4),
            0xe7 => Instruction::SET(4, Register8bits::A),
            0xe8 => Instruction::SET(5, Register8bits::B),
            0xe9 => Instruction::SET(5, Register8bits::C),
            0xea => Instruction::SET(5, Register8bits::D),
            0xeb => Instruction::SET(5, Register8bits::E),
            0xec => Instruction::SET(5, Register8bits::H),
            0xed => Instruction::SET(5, Register8bits::L),
            0xee => Instruction::SETHL(5),
            0xef => Instruction::SET(5, Register8bits::A),
            0xf0 => Instruction::SET(6, Register8bits::B),
            0xf1 => Instruction::SET(6, Register8bits::C),
            0xf2 => Instruction::SET(6, Register8bits::D),
            0xf3 => Instruction::SET(6, Register8bits::E),
            0xf4 => Instruction::SET(6, Register8bits::H),
            0xf5 => Instruction::SET(6, Register8bits::L),
            0xf6 => Instruction::SETHL(6),
            0xf7 => Instruction::SET(6, Register8bits::A),
            0xf8 => Instruction::SET(7, Register8bits::B),
            0xf9 => Instruction::SET(7, Register8bits::C),
            0xfa => Instruction::SET(7, Register8bits::D),
            0xfb => Instruction::SET(7, Register8bits::E),
            0xfc => Instruction::SET(7, Register8bits::H),
            0xfd => Instruction::SET(7, Register8bits::L),
            0xfe => Instruction::SETHL(7),
            0xff => Instruction::SET(7, Register8bits::A),
        }
    }
}

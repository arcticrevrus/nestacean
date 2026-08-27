use std::hint::unreachable_unchecked;
use std::intrinsics::unreachable;
use std::ops::BitAnd;

use crate::Bus;
use crate::cpu::Registers;

#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    ADC(AddressingMode),
    AND(AddressingMode),
    ASL(AddressingMode),
    BCC(AddressingMode),
    BCS(AddressingMode),
    BEQ(AddressingMode),
    BIT(AddressingMode),
    BMI(AddressingMode),
    BNE(AddressingMode),
    BPL(AddressingMode),
    BRK(AddressingMode),
    BVC(AddressingMode),
    BVS(AddressingMode),
    CLC(AddressingMode),
    CLD(AddressingMode),
    CLI(AddressingMode),
    CLV(AddressingMode),
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    DEX(AddressingMode),
    DEY(AddressingMode),
    EOR(AddressingMode),
    INC(AddressingMode),
    INX(AddressingMode),
    INY(AddressingMode),
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP(AddressingMode),
    ORA(AddressingMode),
    PHA(AddressingMode),
    PHP(AddressingMode),
    PLP(AddressingMode),
    ROL(AddressingMode),
    ROR(AddressingMode),
    RTI(AddressingMode),
    RTS(AddressingMode),
    SBC(AddressingMode),
    SEC(AddressingMode),
    SED(AddressingMode),
    SEI(AddressingMode),
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    TAX(AddressingMode),
    TSX(AddressingMode),
    TXS(AddressingMode),
    TYA(AddressingMode),
}
impl Instruction {
    fn from_opbyte(opbyte: u8) -> Self {
        match opbyte {
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => Self::ADC(match opbyte {
                0x69 => AddressingMode::Immediate,
                0x65 => AddressingMode::ZeroPage,
                0x75 => AddressingMode::ZeroPageX,
                0x6D => AddressingMode::Absolute,
                0x7D => AddressingMode::AbsoluteX,
                0x79 => AddressingMode::AbsoluteY,
                0x61 => AddressingMode::IndirectIndexed,
                0x71 => AddressingMode::IndexedIndirect,
                // SAFETY: this is a nested match, outer match catches all opbytes
                _ => unsafe { unreachable_unchecked() },
            }),
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => Self::AND(match opbyte {
                0x29 => AddressingMode::Immediate,
                0x25 => AddressingMode::ZeroPage,
            }),
        }
    }
}

enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
    Implicit,
    Accumulator,
    Absolute,
    Relative,
    Indirect,
}

struct Operation {
    instruction: Instruction,
    cycles: u8,
}

use std::ops::DerefMut;

use crate::{
    cpu::{Cpu, Registers},
    mmap::MemoryMap,
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TSX,
    TXS,
    TYA,
}

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
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

#[derive(Debug)]
pub struct Operation {
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub arg1: Option<u8>,
    pub arg2: Option<u8>,
}
impl Operation {
    pub fn from_bytes(opbyte: u8, arg1: u8, arg2: u8, cpu: &mut Cpu) -> Self {
        use AddressingMode::*;
        use Instruction::*;
        let (instruction, mode) = match opbyte {
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => (
                ADC,
                match opbyte {
                    0x69 => Immediate,
                    0x65 => ZeroPage,
                    0x75 => ZeroPageX,
                    0x6D => Absolute,
                    0x7D => AbsoluteX,
                    0x79 => AbsoluteY,
                    0x61 => IndirectIndexed,
                    0x71 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => (
                AND,
                match opbyte {
                    0x29 => Immediate,
                    0x25 => ZeroPage,
                    0x35 => ZeroPageX,
                    0x2D => Absolute,
                    0x3D => AbsoluteX,
                    0x39 => AbsoluteY,
                    0x21 => IndirectIndexed,
                    0x71 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => (
                ASL,
                match opbyte {
                    0x0A => Accumulator,
                    0x06 => ZeroPage,
                    0x16 => ZeroPageX,
                    0x0E => Absolute,
                    0x1E => AbsoluteX,
                    _ => unreachable!(),
                },
            ),
            0x90 => (BCC, Relative),
            0xB0 => (BCS, Relative),
            0xF0 => (BEQ, Relative),
            0x24 => (BIT, ZeroPage),
            0x2C => (BIT, Absolute),
            0x30 => (BMI, Relative),
            0xD0 => (BNE, Relative),
            0x10 => (BPL, Relative),
            0x00 => (BRK, Implicit),
            0x50 => (BVC, Relative),
            0x70 => (BVS, Relative),
            0x18 => (CLC, Implicit),
            0xD8 => (CLD, Implicit),
            0x58 => (CLI, Implicit),
            0xB8 => (CLV, Implicit),
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => (
                CMP,
                match opbyte {
                    0xC9 => Immediate,
                    0xC5 => ZeroPage,
                    0xD5 => ZeroPageX,
                    0xCD => Absolute,
                    0xDD => AbsoluteX,
                    0xD9 => AbsoluteY,
                    0xC1 => IndirectIndexed,
                    0xD1 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),
            0xE0 => (CPX, Immediate),
            0xE4 => (CPX, ZeroPage),
            0xEC => (CPX, Absolute),
            0xC0 => (CPY, Immediate),
            0xC4 => (CPY, ZeroPage),
            0xCC => (CPY, Absolute),
            0xC6 => (DEC, ZeroPage),
            0xD6 => (DEC, ZeroPageX),
            0xCE => (DEC, Absolute),
            0xDE => (DEC, AbsoluteX),
            0xCA => (DEX, Implicit),
            0x88 => (DEY, Implicit),
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => (
                EOR,
                match opbyte {
                    0x49 => Immediate,
                    0x45 => ZeroPage,
                    0x55 => ZeroPageX,
                    0x4D => Absolute,
                    0x5D => AbsoluteX,
                    0x59 => AbsoluteY,
                    0x41 => IndirectIndexed,
                    0x51 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),
            0xE6 => (INC, ZeroPage),
            0xF6 => (INC, ZeroPageX),
            0xEE => (INC, Absolute),
            0xFE => (INC, AbsoluteX),
            0xE8 => (INX, Implicit),
            0xC8 => (INY, Implicit),
            0x4C => (JMP, Absolute),
            0x6C => (JMP, Indirect),
            0x20 => (JSR, Absolute),
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => (
                LDA,
                match opbyte {
                    0xA9 => Immediate,
                    0xA5 => ZeroPage,
                    0xB5 => ZeroPageX,
                    0xAD => Absolute,
                    0xBD => AbsoluteX,
                    0xB9 => AbsoluteY,
                    0xA1 => IndirectIndexed,
                    0xB1 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),
            0x78 => (SEI, Implicit),
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => (
                STA,
                match opbyte {
                    0x85 => ZeroPage,
                    0x95 => ZeroPageX,
                    0x8D => Absolute,
                    0x9D => AbsoluteX,
                    0x99 => AbsoluteY,
                    0x81 => IndirectIndexed,
                    0xB1 => IndexedIndirect,
                    _ => unreachable!(),
                },
            ),

            _ => todo!("Todo: implement {opbyte:02X} opbyte"),
        };
        let (arg1, arg2) = match mode {
            ZeroPageX | ZeroPageY | AbsoluteX | AbsoluteY | IndexedIndirect | IndirectIndexed
            | Immediate | ZeroPage | Relative => (Some(arg1), None),
            Absolute => (Some(arg1), Some(arg2)),
            Accumulator | Implicit | Indirect => (None, None),
        };
        Self {
            instruction,
            mode,
            arg1,
            arg2,
        }
    }

    fn get_argument(
        registers: &Registers,
        memory_map: &mut MemoryMap,
        mode: &AddressingMode,
        arg1: u8,
        arg2: u8,
    ) -> Option<u8> {
        use AddressingMode::*;
        match mode {
            ZeroPageX => Some(memory_map.read(arg1.wrapping_add(registers.x).into())),
            ZeroPageY => Some(memory_map.read(arg1.wrapping_add(registers.y).into())),
            AbsoluteX => Some(memory_map.read((arg1 as u16).wrapping_add(registers.x as u16))),
            AbsoluteY => Some(memory_map.read((arg1 as u16).wrapping_add(registers.y as u16))),
            IndexedIndirect => {
                let first_read = memory_map.read(arg1.wrapping_add(registers.x).into());
                let second_read =
                    memory_map.read((arg1.wrapping_add(registers.x).wrapping_add(1)).into());
                Some(memory_map.read((first_read as u16 + second_read as u16) * 256))
            }
            IndirectIndexed => {
                let first_read = memory_map.read(arg1.into());
                let second_read =
                    memory_map.read((arg1.wrapping_add(1)) as u16 * 256 + registers.y as u16);
                Some(memory_map.read(first_read.wrapping_add(second_read).into()))
            }
            Implicit => None,
            Accumulator => Some(registers.a),
            Immediate => Some(arg1),
            ZeroPage => Some(memory_map.read(arg1 as u16)),
            Absolute => {
                let addr = (arg1 as u16) << 8 | arg2 as u16;
                Some(memory_map.read(addr))
            }
            Relative => todo!(),
            Indirect => todo!(),
        }
    }
}

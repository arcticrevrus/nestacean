use std::ops::BitAnd;

use crate::Bus;
use crate::cpu::Registers;

#[allow(clippy::upper_case_acronyms)]
enum Instruction {
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

use crate::cpu::opcodes::Instruction::*;

struct Operation {
    instruction: Instruction,
    byte1: Option<u8>,
    byte2: Option<u8>,
    cycles: u8,
}
impl Operation {
    fn run_op(self, registers: &mut Registers, memory: u8, bus: &impl Bus) {
        match self.instruction {
            ADC => {
                registers.a = registers
                    .a
                    .wrapping_add(memory)
                    .wrapping_add(registers.p.bitand(1))
            }
            AND => registers.a = registers.a.bitand(memory),
            ASL => (),
            _ => unimplemented!(),
        }
    }
    /*
    fn from_opbyte(registers: &Registers, bus: &impl Bus) -> Self {
        let wrapped = |base, wrap| if wrap { return base + 1 } else { return base };

        fn wrapped(base: u8, wrapped: bool) -> u8 {
            if wrapped { base + 1 } else { base }
        }
        let instruction_byte = bus.read(registers.pc);
        let mut immediate = || bus.read(registers.pc.wrapping_add(1));
        let mut zero_page = || bus.read(immediate() as u16);
        let mut zpx = || bus.read((immediate().wrapping_add(registers.x)) as u16);
        let mut zpx = || bus.read((immediate().wrapping_add(registers.y)) as u16);
        let mut abs_hi = || bus.read(registers.pc.wrapping_add(1));
        let mut abs_lo = || bus.read(registers.pc + 2);
        let mut abs = || (abs_hi() << 8) as u16 | abs_lo() as u16;
        let mut abs_x = || {
            let i = abs().overflowing_add(registers.x as u16);
            (bus.read(i.0), i.1)
        };
        let mut abs_y = || {
            let i = abs().overflowing_add(registers.y as u16);
            (bus.read(i.0), i.1)
        };
        let mut abs_y = || bus.read(abs().wrapping_add(registers.y as u16));
        // (d,x) 	Indexed indirect 	val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
        // (d),y 	Indirect indexed 	val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
        let mut first_peek = || bus.read(abs_hi() as u16 % 256);
        let mut second_peek = || bus.read(abs_lo() as u16 % 256);
        let mut indirect_x = || bus.read(first_peek() as u16 + (second_peek() * 256) as u16);
        let mut y_first_peek = || bus.read(immediate().wrapping_add(1) as u16);
        let mut indirect_indexed = || {
            bus.read(
                ((immediate() as u16 + y_first_peek() as u16) * 256)
                    .wrapping_add(registers.y as u16),
            )
        };

        match instruction_byte {
            // ADC
            0x69 => Operation {
                instruction: ADC,
                byte1: Some(immediate()),
                byte2: None,
                cycles: 2,
            },
            0x65 => Operation {
                instruction: ADC,
                byte1: Some(zero_page()),
                byte2: None,
                cycles: 3,
            },
            0x75 => Operation {
                instruction: ADC,
                byte1: Some(zpx()),
                byte2: None,
                cycles: 4,
            },
            0x7D => {
                let i = abs_x();
                Operation {
                    instruction: ADC,
                    byte1: Some(i.0),
                    byte2: None,
                    cycles: wrapped(4, i.1),
                }
            }
            0x79 => {}
        };
        todo!()
    }
    */
}

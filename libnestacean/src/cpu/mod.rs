use std::time::{Duration, Instant};

use crate::{
    cart::Cart,
    cpu::opcodes::{AddressingMode, Instruction, Operation},
    mmap::MemoryMap,
};

pub(crate) mod opcodes;

pub(crate) enum CpuVersion {
    Ricoh2A03,
    Ricoh2A07,
    UMCUA6527P,
    UMCUA6527,
}
impl CpuVersion {
    fn clock(&self) -> usize {
        match self {
            Self::Ricoh2A03 => 21_447_272,
            _ => todo!(),
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
}
impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xFFFC,
            s: 0xFD,
            p: 0b00000100,
        }
    }
}
impl Registers {
    pub(crate) fn get_carry(&self) -> u8 {
        self.p & 0b0000_0001
    }
    pub(crate) fn set_carry(&mut self, set: bool) {
        if set {
            self.p |= 0b0000_0001;
        } else {
            self.p &= 0b1111_1110;
        }
    }
    pub(crate) fn toggle_carry(&mut self) {
        self.p ^= 0b0000_0001;
    }
    pub(crate) fn get_zero(&self) -> u8 {
        self.p & 0b0000_0010
    }
    pub(crate) fn set_zero(&mut self, set: bool) {
        if set {
            self.p |= 0b0000_0010;
        } else {
            self.p &= 0b1111_1101;
        }
    }
    pub(crate) fn toggle_zero(&mut self) {
        self.p ^= 0b0000_0010;
    }
    pub(crate) fn get_overflow(&self) -> u8 {
        self.p & 0b0100_0000
    }
    pub(crate) fn set_overflow(&mut self, set: bool) {
        if set {
            self.p |= 0b0100_0000;
        } else {
            self.p &= 0b1011_1111;
        }
    }
    pub(crate) fn toggle_overflow(&mut self) {
        self.p ^= 0b0100_0000;
    }

    pub(crate) fn get_negative(&self) -> u8 {
        self.p & 0b1000_0000
    }
    pub(crate) fn set_negative(&mut self, set: bool) {
        if set {
            self.p |= 0b1000_0000;
        } else {
            self.p &= 0b0111_1111;
        }
    }
    pub(crate) fn toggle_negative(&mut self) {
        self.p ^= 0b1000_0000;
    }
    pub(crate) fn pc_hi(&self) -> u8 {
        (self.pc >> 8) as u8
    }
    pub(crate) fn pc_lo(&self) -> u8 {
        (self.pc & 0xFF) as u8
    }
}

enum OpArg {
    One(u8),
    Two(u16),
}

pub struct Cpu {
    version: CpuVersion,
    pub registers: Registers,
    pub mmap: MemoryMap,
    start_time: Instant,
    cycle_count: usize,
}
impl Cpu {
    pub(crate) fn new(cart: Cart) -> Self {
        let mut registers = Registers::default();
        let mut mmap = MemoryMap::new(cart);
        let lobyte = mmap.read(0xFFFC);
        let hibyte = mmap.read(0xFFFD);
        registers.pc = ((hibyte as u16) << 8) | lobyte as u16;
        Self {
            version: CpuVersion::Ricoh2A03,
            registers,
            mmap,
            start_time: Instant::now(),
            cycle_count: 0,
        }
    }

    pub fn step(&mut self) {
        let bytes = self.fetch();
        let op = self.decode(bytes);
        dbg!(&op);
        self.execute(op);
        self.registers.pc = self.registers.pc.wrapping_add(2);
    }

    fn fetch(&mut self) -> [u8; 3] {
        let mut output = [0; 3];
        let mut i = 0;
        while i < 3 {
            output[i] = self.mmap.read(self.registers.pc.wrapping_add(i as u16));
            i += 1;
        }
        output
    }

    fn decode(&mut self, fetchbytes: [u8; 3]) -> Operation {
        let [opbyte, arg1, arg2] = fetchbytes;
        Operation::from_bytes(opbyte, arg1, arg2, self)
    }
    fn execute(&mut self, op: Operation) {
        let mut arg = None;
        match op.mode {
            AddressingMode::Implicit => (),
            AddressingMode::Immediate | AddressingMode::Relative => {
                arg = Some(OpArg::One(op.arg1.unwrap()))
            }
            AddressingMode::Absolute => {
                arg = Some(OpArg::Two(
                    ((op.arg1.unwrap() as u16) << 8) & op.arg2.unwrap() as u16,
                ))
            }
            _ => {
                eprintln!("Implement AddressingMode::{:?}", op.mode);
                todo!()
            }
        }
        match op.instruction {
            Instruction::BRK => self.brk(None),
            Instruction::CLD => self.clear_decimal(),
            Instruction::SEI => self.set_interrupt_disable(),
            Instruction::LDA => self.load_to_register_a(arg.unwrap()),
            Instruction::BPL => self.branch_if_plus(arg.unwrap()),
            Instruction::STA => self.store_a(arg.unwrap()),
            Instruction::JSR => self.jsr(arg.unwrap()),
            _ => {
                eprintln!("Implement Instruction::{:?}", op.instruction);
                todo!()
            }
        }
    }
    fn tick_clock(&mut self, count: u8) {
        let rate = self.version.clock();
        for _ in 0..count {
            let mut target_cycles =
                (self.start_time.elapsed().as_secs_f64() * rate as f64) as usize;
            while target_cycles <= self.cycle_count {
                target_cycles = (self.start_time.elapsed().as_secs_f64() * rate as f64) as usize;
                std::thread::sleep(Duration::from_micros(100));
            }
            self.cycle_count = self.cycle_count.wrapping_add(1);
        }
    }
    fn add_with_carry(&mut self, arg: u8) {
        let is_positive = self.registers.a >= 128;
        let mem_is_positive = arg >= 128;
        let (value, wrapped1) = self.registers.a.overflowing_add(arg);
        let (value, wrapped2) = value.overflowing_add(self.registers.get_carry());
        let value_is_positive = value >= 128;
        self.registers.a = value;
        self.tick_clock(1);
        self.registers.set_carry(wrapped1 | wrapped2);
        self.registers.set_zero(value == 0);
        self.registers
            .set_overflow((is_positive & mem_is_positive) & value_is_positive);
        self.registers.set_negative(value_is_positive);
        self.tick_clock(1);
    }
    fn bitwise_and(&mut self, arg: u8) {
        self.registers.a &= arg;
        self.tick_clock(1);
        self.registers.set_zero(self.registers.a == 0);
        self.registers.set_negative(self.registers.a >= 128);
        self.tick_clock(1);
    }
    fn arithmetic_shift_left(&mut self, value: Option<&mut u8>) {
        let value_ref = match value {
            Some(v) => v,
            None => &mut self.registers.a.clone(),
        };
        *value_ref <<= 1;
        self.tick_clock(1);
        self.registers.set_carry(*value_ref >= 128);
        self.registers.set_zero(*value_ref == 0);
        self.registers.set_negative(*value_ref >= 128);
        self.tick_clock(1);
    }
    fn branch_if_carry_clear(&mut self, destination: u8) {
        if self.registers.get_carry() == 0 {
            self.tick_clock(1);
            self.registers.pc = self
                .registers
                .pc
                .wrapping_add(2)
                .wrapping_add(destination as i8 as i16 as u16);
        };
        self.tick_clock(2);
    }
    fn branch_if_carry_set(registers: &mut Registers, destination: u8) {
        if registers.get_carry() == 1 {
            registers.pc = registers
                .pc
                .wrapping_add(2)
                .wrapping_add(destination as i8 as i16 as u16);
        };
    }
    fn branch_if_equal(registers: &mut Registers, destination: u8) {
        if registers.get_zero() == 0b0000_0010 {
            registers.pc = registers
                .pc
                .wrapping_add(2)
                .wrapping_add(destination as i8 as i16 as u16);
        }
    }
    fn bit_test(registers: &mut Registers, destination: u8) {
        let result = registers.a & destination;
        registers.set_zero(result == 0);

        registers.set_overflow((destination & 0b0100_0000) == 0b0100_0000);
        registers.set_negative((destination & 0b1000_0000) == 0b1000_0000);
    }
    fn branch_if_minus(registers: &mut Registers, destination: u8) {
        if registers.get_negative() == 0b1000_0000 {
            registers.pc = registers
                .pc
                .wrapping_add(2)
                .wrapping_add(destination as i8 as i16 as u16);
        }
    }
    fn branch_if_plus(&mut self, destination: OpArg) {
        match destination {
            OpArg::One(arg) => {
                if self.registers.get_negative() != 0 {
                    return;
                }
                self.registers.pc = self.registers.pc.wrapping_add(arg as i8 as u16);
            }
            _ => unreachable!(),
        }
    }
    fn brk(&mut self, arg: Option<u8>) {
        let val = self.registers.pc;
        let hi = (val >> 8) as u8;
        let lo = (val & 0xFF) as u8;
        self.mmap.write(self.registers.s as u16 + 0x100, hi);
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.mmap.write(self.registers.s as u16 + 0x100, lo);
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.mmap.write(
            self.registers.s as u16 + 0x100,
            self.registers.p | 0b0011_0000,
        );
        self.registers.s = self.registers.s.wrapping_sub(1);
        self.registers.pc = 0xFFFE;
    }
    fn set_interrupt_disable(&mut self) {
        self.registers.p &= 0b1111_1011
    }
    fn clear_decimal(&mut self) {
        self.registers.p &= 0b1111_0111
    }
    fn return_from_interrupt(registers: &mut Registers) {}
    fn load_to_register_a(&mut self, arg: OpArg) {
        match arg {
            OpArg::One(a) => self.registers.a = a,
            _ => unreachable!(),
        }
    }
    fn store_a(&mut self, arg: OpArg) {
        match arg {
            OpArg::Two(a) => self.mmap.write(a, self.registers.a),
            _ => unreachable!(),
        }
    }
    fn jsr(&mut self, arg: OpArg) {
        match arg {
            OpArg::Two(a) => {
                self.mmap
                    .write(self.registers.s as u16 + 0x0100, self.registers.pc_hi());
                self.registers.s -= 1;
                self.mmap
                    .write(self.registers.s as u16 + 0x100, self.registers.pc_lo());
                self.registers.s -= 1;
                self.registers.pc = a;
            }
            _ => unreachable!(),
        }
    }
}

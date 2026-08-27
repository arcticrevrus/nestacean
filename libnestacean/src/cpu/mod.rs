use std::sync::mpsc::{self, Receiver, Sender};

use crate::{
    cart::{Cart, Mapper},
    mmap::MemoryMap,
};

pub(crate) mod opcodes;

pub(crate) enum CpuVersion {
    Ricoh2A03,
    Ricoh2A07,
    UMCUA6527P,
    UMCUA6527,
}

pub(crate) struct Registers {
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
            pc: 0,
            s: 0xFD,
            p: 0b00000100,
        }
    }
}

pub(crate) struct Cpu {
    version: CpuVersion,
    pub registers: Registers,
    mmap: MemoryMap,
}
impl Cpu {
    pub(crate) fn new(cart: Cart) -> Self {
        Self {
            version: CpuVersion::Ricoh2A03,
            registers: Registers::default(),
            mmap: MemoryMap::new(cart),
        }
    }

    pub(crate) fn step(&mut self) {
        let bytes = self.fetch();
    }

    fn fetch(&mut self) -> [u8; 3] {
        let mut output = [0; 3];
        let mut i = 0;
        while i < 3 {
            output[i] = self.mmap.read(self.registers.pc + i as u16);
            i += 1;
        }
        output
    }

    fn decode(&mut self, fetchbytes: [u8; 3]) -> {
        let [opbyte, arg1, arg2] = fetchbytes;
    }
}

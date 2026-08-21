use std::sync::mpsc::{self, Receiver, Sender};

use crate::mmap::MemoryMap;

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
    address_bus: (Sender<u16>, Receiver<u16>),
    data_bus: (Sender<u8>, Receiver<u8>),
}
impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            version: CpuVersion::Ricoh2A03,
            registers: Registers::default(),
            address_bus: mpsc::channel(),
            data_bus: mpsc::channel(),
        }
    }

    pub(crate) fn step(&mut self, mmap: MemoryMap) {
        let opcode = mmap.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let _ = opcode; // TODO: decode/execute
    }

    fn fetch(&mut self, bus: &mut impl Bus, addr: u16) {
        let opbyte = bus.read(addr);
    }
}

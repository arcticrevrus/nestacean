mod apu;
pub mod cart;
mod cpu;
mod mmap;

use core::panic::PanicInfo;
use mmap::MemoryMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub use crate::cart::Cart;
use crate::cpu::{Cpu, CpuVersion};

trait Bus {
    fn map_addr(&mut self, address: u16) -> &mut u8;
    fn read(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>));
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>));
}

enum SystemType {
    NTSC,
    PAL,
    Dendy,
    RGB,
    Brazil,
    Argentina,
}
impl SystemType {
    fn cpu(self) -> CpuVersion {
        match self {
            Self::NTSC | Self::RGB => CpuVersion::Ricoh2A03,
            Self::PAL => CpuVersion::Ricoh2A07,
            Self::Dendy => CpuVersion::UMCUA6527P,
            Self::Brazil | Self::Argentina => CpuVersion::UMCUA6527,
        }
    }
    fn clock(self) -> f32 {
        match self {
            Self::NTSC | Self::RGB => 1.789773,
            Self::PAL => 1.662607,
            Self::Dendy => 1.773448,
            Self::Brazil => 1.787806,
            Self::Argentina => 1.791028,
        }
    }
}

struct Nes {
    cpu: Cpu,
}
impl Nes {
    fn new(cart: Cart) -> Self {
        let cart = Cart {
            file: None,
            expansion_rom: [0; 0x1FDF],
            ram: [0; 0x2000],
            rom: [0; 0x8000],
            mapper: cart::Mapper {},
        };
        Self {
            cpu: Cpu::new(cart),
        }
    }

    fn step(&mut self) {
        self.cpu.step();
    }
}

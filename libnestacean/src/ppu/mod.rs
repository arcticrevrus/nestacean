use std::sync::mpsc::{Receiver, Sender};

use crate::Bus;

#[derive(Default)]
pub(crate) enum PpuVersion {
    #[default]
    Ricoh2C02,
    Ricoh2C07,
}

#[derive(Default)]
pub(crate) struct Ppu {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u8,
    pub ppudata: u8,
    pub oamdma: u8,
    version: PpuVersion,
    odd_frame: bool,
}
impl Bus for Ppu {
    fn map_addr(&mut self, address: u16) -> &mut u8 {
        match address {
            0x2000..=0x3FFF => match (address - 0x2000) % 8 {
                0 => &mut self.ppuctrl,
                1 => &mut self.ppumask,
                2 => &mut self.ppustatus,
                3 => &mut self.oamaddr,
                4 => &mut self.oamdata,
                5 => &mut self.ppuscroll,
                6 => &mut self.ppuaddr,
                _ => &mut self.ppudata,
            },
            _ => panic!("Invaid address matched by PPU"),
        }
    }
    fn read(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        let value = self.map_addr(addr);
        let _ = data.0.send(*value);
    }
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        let value = data
            .1
            .recv()
            .expect("Attempted to read from closed data bus");
        *self.map_addr(addr) = value
    }
}

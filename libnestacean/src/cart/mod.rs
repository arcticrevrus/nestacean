use std::sync::mpsc::{Receiver, Sender};

use crate::Bus;

pub struct Mapper {}

pub(crate) struct Cart {
    pub expansion_rom: [u8; 0x1FDF],
    pub ram: [u8; 0x2000],
    pub rom: [u8; 0x8000],
    pub mapper: Mapper,
}
impl Bus for Cart {
    fn map_addr(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0..=0x401F => panic!("Invalid address access for Cart"),
            0x4020..=0x5FFF => &mut self.expansion_rom[(addr - 0x4020) as usize],
            0x6000..=0x7FFF => &mut self.ram[(addr - 0x6000) as usize],
            0x8000..=0xFFFF => &mut self.rom[(addr - 0x8000) as usize],
        }
    }
    fn read(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        data.0.send(*self.map_addr(addr));
    }
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        let data = data
            .1
            .recv()
            .expect("Attempted to read from closed data bus");
        *self.map_addr(addr) = data;
    }
}

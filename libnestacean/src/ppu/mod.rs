use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use crate::Bus;

#[derive(Default)]
pub(crate) enum PpuVersion {
    #[default]
    Ricoh2C02,
    Ricoh2C07,
}

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
    cycles: usize,
    timestamp: Instant,
}
impl Default for Ppu {
    fn default() -> Self {
        Self {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamdata: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            oamdma: 0,
            version: PpuVersion::Ricoh2C02,
            odd_frame: false,
            cycles: 0,
            timestamp: Instant::now(),
        }
    }
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
        match self.map_addr(addr) {
            0 | 1 | 5 | 6 | 7 => {
                if self.cycles < 29658 * 3 {
                    return;
                }
            }
            _ => (),
        }
        let value = data
            .1
            .recv()
            .expect("Attempted to read from closed data bus");
        *self.map_addr(addr) = value
    }
}
impl Ppu {
    pub fn step(&mut self, cycles: usize, rate: usize) {
        let rate = rate * 3;
        for _ in 0..cycles {
            let mut target_cycles = (self.timestamp.elapsed().as_secs_f64() * rate as f64) as usize;
            while target_cycles <= self.cycles {
                target_cycles = (self.timestamp.elapsed().as_secs_f64() * rate as f64) as usize;
                thread::sleep(Duration::from_micros(100));
            }
            self.cycles = self.cycles.wrapping_add(1)
        }
    }
}

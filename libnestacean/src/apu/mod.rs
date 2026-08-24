use crate::Bus;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct Pulse(pub [u8; 4]);
impl Default for Pulse {
    fn default() -> Self {
        Self([0; 4])
    }
}

pub struct Triangle(pub [u8; 4]);
impl Default for Triangle {
    fn default() -> Self {
        Self([0; 4])
    }
}

pub struct Noise(pub [u8; 4]);
impl Default for Noise {
    fn default() -> Self {
        Self([0; 4])
    }
}

pub struct Dmc(pub [u8; 4]);
impl Default for Dmc {
    fn default() -> Self {
        Self([0; 4])
    }
}

#[derive(Default)]
pub(crate) struct Apu {
    pub pulse1: Pulse,
    pub pulse2: Pulse,
    pub triangle: Triangle,
    pub noise: Noise,
    pub dmc: Dmc,
    pub status: u8,
    pub joy1: u8,
    pub frame_counter: u8,
}
impl Bus for Apu {
    fn map_addr(&mut self, address: u16) -> &mut u8 {
        match address {
            0..=4 => &mut self.pulse1.0[address as usize],
            5..=7 => &mut self.pulse2.0[(address - 4) as usize],
            8..=11 => &mut self.triangle.0[(address - 8) as usize],
            12..=15 => &mut self.noise.0[(address - 12) as usize],
            16..=19 => &mut self.dmc.0[(address - 16) as usize],
            20 => &mut self.status,
            21 => &mut self.frame_counter,
            _ => panic!("Invalid APU address"),
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

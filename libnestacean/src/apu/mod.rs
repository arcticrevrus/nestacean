use crate::Bus;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[derive(Default)]
pub struct Pulse(pub [u8; 4]);

#[derive(Default)]
pub struct Triangle(pub [u8; 4]);

#[derive(Default)]
pub struct Noise(pub [u8; 4]);

#[derive(Default)]
pub struct Dmc(pub [u8; 4]);

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
            0x4000..=0x4003 => &mut self.pulse1.0[(address - 0x4000) as usize],
            0x4004..=0x4007 => &mut self.pulse2.0[(address - 0x4004) as usize],
            0x4008..=0x400B => &mut self.triangle.0[(address - 0x4008) as usize],
            0x400C..=0x400F => &mut self.noise.0[(address - 0x400C) as usize],
            0x4010..=0x4013 => &mut self.dmc.0[(address - 0x4010) as usize],
            0x4015 => &mut self.status,
            0x4017 => &mut self.frame_counter,
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

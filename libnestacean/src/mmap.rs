use std::sync::mpsc::{self, Receiver, Sender};

use crate::{Bus, apu::Apu, cart::Cart};

#[derive(Default)]
pub(crate) struct Ppu {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,
}

struct Ram([u8; 0x800]);
impl Default for Ram {
    fn default() -> Self {
        Self([0; 0x800])
    }
}
impl Bus for Ram {
    fn map_addr(&mut self, address: u16) -> &mut u8 {
        match address {
            0x0..=0x07FF => &mut self.0[address as usize],
            _ => panic!("Invalid internal RAM address"),
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

pub(crate) struct MemoryMap {
    address: (Sender<u16>, Receiver<u16>),
    data: (Sender<u8>, Receiver<u8>),
    ram: Ram,
    ppu: Ppu,
    apu: Apu,
    apu_test: [u8; 4],
    irq_timer: [u8; 4],
    cart: Cart,
}
impl MemoryMap {
    pub(crate) fn new(cart: Cart) -> Self {
        Self {
            address: mpsc::channel(),
            data: mpsc::channel(),
            ram: Ram::default(),
            ppu: Ppu::default(),
            apu: Apu::default(),
            apu_test: [0; 4],
            irq_timer: [0; 4],
            cart,
        }
    } /*
    fn addr_mut(&mut self, address: u16) -> &mut u8 {
    match address {
    // 2KB internal RAM, mirrored 4x through 0x1FFF
    0x0000..=0x1FFF => &mut self.ram.0[(address % 0x0800) as usize],
    // PPU registers, mirrored every 8 bytes through 0x3FFF
    0x2000..=0x3FFF => match (address - 0x2000) % 8 {
    0 => &mut self.ppu.ppuctrl,
    1 => &mut self.ppu.ppumask,
    2 => &mut self.ppu.ppustatus,
    3 => &mut self.ppu.oamaddr,
    4 => &mut self.ppu.oamdata,
    5 => &mut self.ppu.ppuscroll,
    6 => &mut self.ppu.ppuaddr,
    _ => &mut self.ppu.ppudata,
    },
    0x4000..=0x4003 => &mut self.apu.pulse1.0[(address - 0x4000) as usize],
    0x4004..=0x4007 => &mut self.apu.pulse2.0[(address - 0x4004) as usize],
    0x4008..=0x400B => &mut self.apu.triangle.0[(address - 0x4008) as usize],
    0x400C..=0x400F => &mut self.apu.noise.0[(address - 0x400C) as usize],
    0x4010..=0x4013 => &mut self.apu.dmc.0[(address - 0x4010) as usize],
    0x4014 => &mut self.ppu.oamdma,
    0x4015 => &mut self.apu.status,
    0x4016 => &mut self.apu.joy1,
    0x4017 => &mut self.apu.joy1,
    0x4018..=0x401A => &mut self.apu_test[(address - 0x4018) as usize],
    // NOTE: unused address? What is this?
    0x401B => &mut self.apu_test[0],
    0x401C..=0x401F => &mut self.irq_timer[(address - 0x401C) as usize],
    0x4020..=0x5FFF => &mut self.cart.expansion_rom[(address - 0x4020) as usize],
    0x6000..=0x7FFF => &mut self.cart.ram[(address - 0x6000) as usize],
    0x8000..=0xFFFF => &mut self.cart.rom[(address - 0x8000) as usize],
    }
    }
     */
    pub fn read(&mut self, address: u16) -> u8 {
        self.address.0.send(address).unwrap();
        match address {
            0x0..=0x1FFF => self.ram.read(&self.address, &self.data),
            _ => todo!(),
        }
        self.data.1.recv().unwrap()
    }
    pub fn write(&mut self, address: u16, value: u8) {
        self.address.0.send(address).unwrap();
        self.data.0.send(value).unwrap();
        match address {
            0x0..=0x1FFF => self.ram.write(&self.address, &self.data),
            _ => todo!(),
        }
    }
}

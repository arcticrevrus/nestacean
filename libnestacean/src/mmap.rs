use std::sync::mpsc::{self, Receiver, Sender};

use crate::{Bus, cart::Cart};

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

#[derive(Default)]
pub(crate) struct Apu {
    pulse1: [u8; 4],
    pulse2: [u8; 4],
    triangle: [u8; 4],
    noise: [u8; 4],
    dmc: [u8; 4],
    status: u8,
    frame_counter: u8,
}
impl Bus for Apu {
    fn map_addr(&mut self, address: u16) -> &mut u8 {
        &mut match address {
            0..=3 => self.pulse1[address],
            4..=7 => self.pulse2[address - 4],
            8..=11 => self.triangle[address - 8],
            12..=15 => self.noise[address - 12],
            16..=19 => self.dmc[address - 16],
            20 => self.status,
            21 => self.frame_counter,
            _ => panic!("Invalid APU address"),
        }
    }
    fn read(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address.1
            .recv()
            .expect("Attempted to read from closed address bus");
        let value = self.map_addr(addr);
        let _ = data.send(*value);
    }
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address.1
            .recv()
            .expect("Attempted to read from closed address bus");
        let value = data.1.recv().expect("Attempted to read from closed data bus");
        *self.map_addr(addr) = value
    }
}

pub(crate) struct Mapper<'a> {
    address: &'a (Sender<u16>, Receiver<u16>),
    data: &'a (Sender<u8>, Receiver<u8>),
    ram: [u8; 0x800],
    ppu: Ppu,
    apu: Apu,
    snd_enable: u8,
    joy1: u8,
    joy2: u8,
    apu_test: [u8; 4],
    irq_timer: [u8; 4],
    cart: Cart
}
impl<'a> Mapper<'a> {
    fn new(cart: Cart, address: &'a (Sender<u16>, Receiver<u16>), data: &'a (Sender<u8>, Receiver<u8>)) -> Self {
        Self {
            address,
            data,
            ram: [0; 0x800],
            ppu: Ppu::default(),
            apu: Apu::default(),
            snd_enable: 0,
            joy1: 0,
            joy2: 0,
            apu_test: [0; 4],
            irq_timer: [0; 4],
            cart,
        }
    }
}
impl<'a> Bus for Mapper<'a> {
    fn map_addr(&mut self, )
}


pub(crate) struct MemoryMap {
    address: (Sender<u16>, Receiver<u16>),
    data: (Sender<u8>, Receiver<u8>),
    ram: [u8; 0x800],
    ppu: Ppu,
    apu: Apu,
    snd_enable: u8,
    joy1: u8,
    joy2: u8,
    apu_test: [u8; 4],
    irq_timer: [u8; 4],
    cart_use: [u8; 0x1FE0],
    cart_ram: [u8; 0x2000],
    cart_rom: [u8; 0x8000],
}
impl MemoryMap {
    pub(crate) fn new(cart_rom: [u8; 0x8000]) -> Self {
        Self {
            data: mpsc::channel(),
            address: mpsc::channel(),
            ram: [0; 0x800],
            ppu: Ppu::default(),
            apu: Apu::default(),
            snd_enable: 0,
            joy1: 0,
            joy2: 0,
            apu_test: [0; 4],
            irq_timer: [0; 4],
            cart_use: [0; 0x1FE0],
            cart_ram: [0; 0x2000],
            cart_rom,
        }
    }

    fn addr_mut(&mut self, address: u16) -> &mut u8 {
        match address {
            // 2KB internal RAM, mirrored 4x through 0x1FFF
            0x0000..=0x1FFF => &mut self.ram[(address % 0x0800) as usize],
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
            0x4000..=0x4003 => &mut self.apu.pulse1[(address - 0x4000) as usize],
            //0x4004..=0x4007 => &mut self.apu.pulse2[(address - 0x4004) as usize],
            //0x4008..=0x400B => &mut self.apu.triangle[(address - 0x4008) as usize],
            //0x400C..=0x400F => &mut self.apu.noise[(address - 0x400C) as usize],
            //0x4010..=0x4013 => &mut self.apu.dmc[(address - 0x4010) as usize],
            0x4014 => &mut self.ppu.oamdma,
            0x4015 => &mut self.snd_enable,
            0x4016 => &mut self.joy1,
            0x4017 => &mut self.joy2,
            0x4018..=0x401A => &mut self.apu_test[(address - 0x4018) as usize],
            // NOTE: unused address? What is this?
            0x401B => &mut self.apu_test[0],
            0x401C..=0x401F => &mut self.irq_timer[(address - 0x401C) as usize],
            0x4020..=0x5FFF => &mut self.cart_use[(address - 0x4020) as usize],
            0x6000..=0x7FFF => &mut self.cart_ram[(address - 0x6000) as usize],
            0x8000..=0xFFFF => &mut self.cart_rom[(address - 0x8000) as usize],
        }
    }
}
impl Bus for MemoryMap {
    fn map_addr(&mut self, address: u16) -> u8
    fn read(&mut self, address: (Sender<u16>, Receiver<u16), data: (Sender<u8>, Receiver<u8>)) {
        *self.addr_mut(address)
    }
    fn write(&mut self, address: u16, value: u8) {
        *self.addr_mut(address) = value;
    }
}

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
    }
    pub fn read(&mut self, address: u16) -> u8 {
        self.address.0.send(address).unwrap();
        match address {
            0x0..=0x1FFF => self.ram.read(&self.address, &self.data),
            0x2000..=0x3FFF => self.ppu.read(&self.address, &self.data),
            0x4000..=0x4017 => self.apu.read(&self.address, &self.data),
            // TODO: Maybe implement test features?
            // return 0 for now
            0x4018..=0x401F => self.data.0.send(0).unwrap(),
            0x4020..=0xFFFF => self.cart.read(&self.address, &self.data),
        }
        self.data.1.recv().unwrap()
    }
    pub fn write(&mut self, address: u16, value: u8) {
        self.address.0.send(address).unwrap();
        self.data.0.send(value).unwrap();
        match address {
            0x0..=0x1FFF => self.ram.write(&self.address, &self.data),
            0x2000..=0x3FFF => self.ppu.write(&self.address, &self.data),
            0x4000..=0x4017 => self.apu.write(&self.address, &self.data),
            //TODO: Maybe implement test features?
            // nop for now
            0x4018..=0x401F => (),
            0x4020..=0xFFFF => self.cart.write(&self.address, &self.data),
        }
    }
}

#[cfg(test)]
mod mmap_tests {
    use super::*;

    #[test]
    fn test_ram_bus() {
        let mut map = MemoryMap::new(Cart::empty());
        map.ram.0[0] = 0xFF;
        assert!(map.read(0) == 0xFF);
        map.write(0, 0xEF);
        assert!(map.read(0) == 0xEF);
        assert!(map.ram.0[0] == 0xEF);
    }

    #[test]
    fn test_ppu_bus() {
        let mut map = MemoryMap::new(Cart::empty());
        map.ppu.ppuctrl = 0xB0;
        map.ppu.ppumask = 0x0B;
        map.ppu.ppustatus = 0xFA;
        map.ppu.oamaddr = 0xCE;
        let mut arr = [0; 4];
        for i in 0x2000..=0x2003 {
            arr[i - 0x2000] = map.read(i as u16);
        }
        for i in 0x2000..=0x3FFF {
            if (i % 8) < 4 {
                let data = map.read(i as u16);
                assert!(data == arr[i % 8]);
            }
        }
        assert!(arr == [0xB0, 0x0B, 0xFA, 0xCE]);
    }

    #[test]
    fn test_apu_bus() {
        let mut map = MemoryMap::new(Cart::empty());
        let input = [0xB0, 0x0B, 0xFA, 0xCE];
        for (i, val) in input.iter().enumerate() {
            map.write(0x4000 + i as u16, *val);
            map.write(0x4004 + i as u16, *val);
            map.write(0x4008 + i as u16, *val);
            map.write(0x400C + i as u16, *val);
            map.write(0x4010 + i as u16, *val);
        }
        map.write(0x4015, 0x4E);
        map.write(0x4017, 0xE1);
        assert!(map.apu.pulse1.0 == input);
        assert!(map.apu.pulse2.0 == input);
        assert!(map.apu.triangle.0 == input);
        assert!(map.apu.noise.0 == input);
        assert!(map.apu.dmc.0 == input);
        assert!(map.apu.status == 0x4E);
        assert!(map.apu.frame_counter == 0xE1);
        //TODO: Implement read tests
    }
}

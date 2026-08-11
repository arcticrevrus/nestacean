pub struct MemoryMap<'a> {
    pub ram: [u8; 0x800],
    pub mirror1: &'a mut [u8; 0x800],
    pub mirror2: &'a mut [u8; 0x800],
    pub mirror3: &'a mut [u8; 0x800],
    pub ppu_registers: [&'a mut u8; 0x8],
    pub pulse1: &'a mut [u8; 4],
    pub pulse2: &'a mut [u8; 4],
    pub triangle: &'a mut [u8; 4],
    pub noise: &'a mut [u8; 4],
    pub dmc: &'a mut [u8; 4],
    pub oamdma: &'a mut u8,
    pub snd_enable: u8,
    pub joy1: u8,
    pub joy2: u8,
    pub apu_test: [u8; 4],
    pub irq_timer: [u8; 4],
    pub cart_use: [u8; 0x1FE0],
    pub cart_ram: [u8; 0x2000],
    pub cart_rom: &'a mut [u8; 0x8000],
}
impl<'a> MemoryMap<'a> {
    fn _from_addr(&mut self, address: u16) -> &mut u8 {
        match address {
            0x0000..=0x07FF => &mut (self.ram[address as usize]),
            0x0800..=0x0FFF => &mut (self.mirror1[(address - 0x800) as usize]),
            0x1000..=0x17FF => &mut (self.mirror2[(address - 0x1000) as usize]),
            0x1800..=0x1FFF => &mut (self.mirror3[(address - 0x1800) as usize]),
            0x2000..=0x3FFF => self.ppu_registers[((address - 0x2000) % 8) as usize],
            0x4000..=0x4003 => &mut (self.pulse1[(address - 0x4000) as usize]),
            0x4004..=0x4007 => &mut (self.pulse2[(address - 0x4004) as usize]),
            0x4008..=0x400B => &mut (self.triangle[(address - 0x4008) as usize]),
            0x400C..=0x400F => &mut (self.noise[(address - 0x400C) as usize]),
            0x4010..=0x4013 => &mut (self.dmc[(address - 0x4010) as usize]),
            0x4014 => &mut self.oamdma,
            0x4015 => &mut self.snd_enable,
            0x4016 => &mut self.joy1,
            0x4017 => &mut self.joy2,
            0x4018..=0x401A => &mut (self.apu_test[(address - 0x4018) as usize]),
            // NOTE: unused address? What is this?
            0x401B => &mut (self.apu_test[0]),
            0x401C..=0x401F => &mut (self.irq_timer[(address - 0x401C) as usize]),
            0x4020..=0x5FFF => &mut (self.cart_use[(address - 0x4020) as usize]),
            0x6000..=0x7FFF => &mut (self.cart_ram[(address - 0x6000) as usize]),
            0x8000..=0xFFFF => &mut (self.cart_rom[(address - 0x8000) as usize]),
        }
    }
    fn read(&mut self, address: u16) -> u8 {
        *self._from_addr(address)
    }
    fn write(&mut self, address: u16, value: u8) {
        *self._from_addr(address) = value
    }
}

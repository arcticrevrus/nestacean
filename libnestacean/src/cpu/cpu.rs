pub(crate) enum CpuVersion {
    Ricoh2A03,
    Ricoh2A07,
    UMCUA6527P,
    UMCUA6527,
}

pub(crate) struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
}
impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0xFD,
            p: 0b00000100,
        }
    }
}

pub(crate) struct Cpu {
    version: CpuVersion,
    pub registers: Registers,
}
impl Cpu {
    pub(crate) fn new(bus: &mut impl Bus) -> Self {
        let mut cpu = Self {
            version: CpuVersion::Ricoh2A03,
            registers: Registers::default(),
        };
        let lo = bus.read(0xFFFC) as u16;
        let hi = bus.read(0xFFFD) as u16;
        cpu.registers.pc = (hi << 8) | lo;
        cpu
    }

    pub(crate) fn step(&mut self, bus: &mut impl Bus) {
        let opcode = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let _ = opcode; // TODO: decode/execute
    }

    fn fetch(&mut self, bus: &mut impl Bus, addr: u16) {
        let opbyte = bus.read(addr);
    }
}

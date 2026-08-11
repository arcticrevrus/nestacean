mod mmap;

use mmap::MemoryMap;
use std::sync::Mutex;

enum CpuVersion {
    Ricoh2A03,
    Ricoh2A07,
    UMCUA6527P,
    UMCUA6527,
}

enum SystemType {
    NTSC,
    PAL,
    Dendy,
    RGB,
    Brazil,
    Argentina,
}
impl SystemType {
    fn cpu(self) -> CpuVersion {
        match self {
            Self::NTSC | Self::RGB => CpuVersion::Ricoh2A03,
            Self::PAL => CpuVersion::Ricoh2A07,
            Self::Dendy => CpuVersion::UMCUA6527P,
            Self::Brazil | Self::Argentina => CpuVersion::UMCUA6527,
        }
    }
    fn clock(self) -> f32 {
        match self {
            Self::NTSC | Self::RGB => 1.789773,
            Self::PAL => 1.662607,
            Self::Dendy => 1.773448,
            Self::Brazil => 1.787806,
            Self::Argentina => 1.791028,
        }
    }
}

#[derive(Default)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
}

struct Cpu<'a> {
    version: CpuVersion,
    registers: Registers,
    mem: Mutex<MemoryMap<'a>>,
}
impl<'a> Cpu<'a> {
    fn new(mmap: &'a mut Mutex<MemoryMap<'a>>) -> Self {
        Self {
            version: CpuVersion::Ricoh2A03,
            registers: Registers::default(),
            mem: mmap,
        }
    }
}

#[derive(Default)]
struct Ppu {
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
struct Apu {
    pulse1: [u8; 4],
    pulse2: [u8; 4],
    triangle: [u8; 4],
    noise: [u8; 4],
    dmc: [u8; 4],
    status: u8,
    frame_counter: u8,
}

struct Nes<'a> {
    cpu: Cpu<'a>,
    memmap: Mutex<MemoryMap<'a>>,
    ppu: &'a mut Ppu,
    apu: &'a mut Apu,
}
impl<'a> Nes<'a> {
    fn new(cart: &mut [u8; 0x8000], apu: &mut Apu, ppu: &mut Ppu) -> Self {
        let mut ram = [0; 0x800];
        let mut snd_enable = 0;
        let mut joy1 = 0;
        let mut joy2 = 0;
        let mut apu_test = [0; 0x4];
        let mut irq_timer = [0; 0x4];
        let mut cart_use = [0; 0x1FE0];
        let mut cart_ram = [0; 0x2000];

        let mut mem = MemoryMap {
            ram,
            mirror1: &mut ram,
            mirror2: &mut ram,
            mirror3: &mut ram,
            ppu_registers: [
                &mut ppu.ppuctrl,
                &mut ppu.ppumask,
                &mut ppu.ppustatus,
                &mut ppu.oamaddr,
                &mut ppu.oamdata,
                &mut ppu.ppuscroll,
                &mut ppu.ppuaddr,
                &mut ppu.ppudata,
            ],
            pulse1: &mut apu.pulse1,
            pulse2: &mut apu.pulse2,
            triangle: &mut apu.triangle,
            noise: &mut apu.noise,
            dmc: &mut apu.dmc,
            oamdma: &mut ppu.oamdma,
            snd_enable,
            joy1,
            joy2,
            apu_test,
            irq_timer,
            cart_use,
            cart_ram,
            cart_rom: cart,
        };
        let mut mmap = Mutex::new(mem);
        Self {
            cpu: Cpu::new(&mut mmap),
            memmap: mmap,
            ppu,
            apu,
        }
    }
}

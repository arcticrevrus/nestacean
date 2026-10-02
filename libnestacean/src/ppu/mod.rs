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

#[derive(PartialEq)]
enum NtscRegion {
    Hsync,
    BackPorch,
    ColorBurst,
    Pulse,
    LeftBorder,
    Active,
    RightBorder,
    FrontPorch,
    BottomBorder,
    Vblank,
    VblankPulse,
    VSyncSerration,
}
impl NtscRegion {
    #[inline(always)]
    fn get(row: u16, column: u16) -> (Self, Option<Self>) {
        use NtscRegion::*;
        let mut cb = None;
        let main = match row {
            0..=239 => match column {
                277..302 => Hsync,
                302..306 => BackPorch,
                306..321 => {
                    cb = Some(ColorBurst);
                    BackPorch
                }
                321..326 => BackPorch,
                326 => Pulse,
                327..342 => LeftBorder,
                0..257 => Active,
                257..268 => RightBorder,
                268..277 => FrontPorch,
                _ => unreachable!(),
            },
            240..=241 => match column {
                277..302 => Hsync,
                302..306 => BackPorch,
                306..321 => {
                    cb = Some(ColorBurst);
                    BackPorch
                }
                321..326 => BackPorch,
                326 => Pulse,
                327..342 | 0..268 => BottomBorder,
                268..277 => FrontPorch,
                _ => unreachable!(),
            },
            242..=244 => match column {
                277..302 => Hsync,
                302..306 => BackPorch,
                306..321 => {
                    cb = Some(ColorBurst);
                    BackPorch
                }
                326..342 | 0..277 => Vblank,
                _ => unreachable!(),
            },
            245..=247 => match column {
                277..342 | 0..254 => VblankPulse,
                254..277 => VSyncSerration,
                _ => unreachable!(),
            },
            248..=261 => match column {
                277..302 => Hsync,
                302..306 => BackPorch,
                306..321 => {
                    cb = Some(ColorBurst);
                    BackPorch
                }
                321..326 => BackPorch,
                326..342 | 0..277 => Vblank,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        (main, cb)
    }
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
    row: u16,
    column: u16,
    vblank_read: bool,
    reset: bool,
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
            row: 0,
            column: 0,
            vblank_read: false,
            reset: false,
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
        if addr == 0x2002 {
            self.clear_vblank();
        }
    }
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        match (addr - 0x2000) % 8 {
            0 | 1 | 5 | 6 => {
                if self.cycles < 29658 * 3 && self.reset {
                    _ = data
                        .1
                        .recv()
                        .expect("Attempted to read from a closed data bus");
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
            self.cycles = self.cycles.wrapping_add(1);
            self.next_dot();
        }
    }
    fn next_dot(&mut self) {
        self.column += 1;
        if self.column > 340 {
            self.column = 0;
            self.row = if self.row == 261 { 0 } else { self.row + 1 };
        }
        match (self.row, self.column) {
            (241, 1) => self.set_vblank(),
            (261, 1) => self.clear_vblank(),
            _ => {}
        }
    }
    fn set_vblank(&mut self) {
        self.ppustatus |= 0b1000_0000;
    }
    fn clear_vblank(&mut self) {
        self.ppustatus &= 0b0111_1111;
    }
    #[inline]
    fn render_active(&self) -> bool {
        NtscRegion::get(self.row, self.column).0 == NtscRegion::Active
    }
}

use std::{
    io::Read,
    os::unix::fs::FileExt,
    path::Path,
    sync::mpsc::{Receiver, Sender},
};

use crate::Bus;

pub enum Mapper {
    Nrom,
}

#[derive(Debug)]
pub struct RomHeader {
    pub magic_constant: [u8; 4],
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub flags_8: u8,
    pub flags_9: u8,
    pub flags_10: u8,
    pub padding: [u8; 5],
}

#[derive(Debug)]
pub struct RomFile {
    pub header: RomHeader,
    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub pc_inst_rom: Option<Vec<u8>>,
    pub pc_prom: Option<Vec<u8>>,
}
impl RomFile {
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path)?;

        let mut h = [0; size_of::<RomHeader>()];
        file.read_exact(&mut h)?;
        let header = RomFile::verify_header(h).expect("Error validating File Header");

        let trainer: Option<[u8; 512]> = if h[6] & (1 << 3) != 0 {
            todo!();
        } else {
            None
        };

        let prg_rom_offset = match trainer {
            Some(_) => size_of::<RomHeader>() + 1,
            None => size_of::<RomHeader>(),
        };
        let mut prg_rom = vec![0; 16384 * header.prg_rom_size as usize];
        file.read_at(&mut prg_rom, prg_rom_offset as u64)?;
        let prg_rom = prg_rom;

        let mut chr_rom = vec![0; 8192 * header.chr_rom_size as usize];
        file.read_at(&mut chr_rom, (prg_rom_offset + prg_rom.len()) as u64)?;
        let chr_rom = chr_rom;

        Ok(Self {
            header,
            trainer,
            prg_rom,
            chr_rom,
            pc_inst_rom: None,
            pc_prom: None,
        })
    }
    fn verify_header(header: [u8; 16]) -> Option<RomHeader> {
        let magic_constant = [0x4E, 0x45, 0x53, 0x1A];
        if header[0..=3] != magic_constant {
            return None;
        };
        Some(RomHeader {
            magic_constant,
            prg_rom_size: header[4],
            chr_rom_size: header[5],
            flags_6: header[6],
            flags_7: header[7],
            flags_8: header[8],
            flags_9: header[9],
            flags_10: header[10],
            padding: [0; 5],
        })
    }
}

pub struct Cart {
    pub file: Option<RomFile>,
    pub expansion_rom: [u8; 0x1FDF],
    pub ram: [u8; 0x2000],
    pub rom: [u8; 0x8000],
    pub ppu: [u8; 0x2000],
    pub mapper: Mapper,
}
impl Bus for Cart {
    fn map_addr(&mut self, addr: u16) -> &mut u8 {
        match addr {
            0x0..=0x401F => panic!("Invalid address access for Cart"),
            0x4020..=0x5FFF => &mut self.expansion_rom[(addr - 0x4020) as usize],
            0x6000..=0x7FFF => &mut self.ram[(addr - 0x6000) as usize],
            0x8000..=0xFFFF => &mut self.rom[(addr - 0x8000) as usize],
        }
    }
    fn read(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        let _ = data.0.send(*self.map_addr(addr));
    }
    fn write(&mut self, address: &(Sender<u16>, Receiver<u16>), data: &(Sender<u8>, Receiver<u8>)) {
        let addr = address
            .1
            .recv()
            .expect("Attempted to read from closed address bus");
        let data = data
            .1
            .recv()
            .expect("Attempted to read from closed data bus");
        *self.map_addr(addr) = data;
    }
}
impl Cart {
    pub fn empty() -> Self {
        Self {
            file: None,
            expansion_rom: [0; 0x1FDF],
            ram: [0; 0x2000],
            rom: [0; 0x8000],
            ppu: [0; 0x2000],
            mapper: Mapper::Nrom,
        }
    }
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        // Naive implementation with hard coded nrom mapper
        // no save support
        let file = Some(RomFile::from_file(path)?);
        let mapper = Mapper::Nrom;
        let hi_rom_start = file.as_ref().unwrap().prg_rom.len().saturating_sub(0x4000);
        let lo_rom = &file.as_ref().unwrap().prg_rom[0..0x4000];
        let hi_rom = &file.as_ref().unwrap().prg_rom[hi_rom_start..];
        let mut rom: [u8; 0x8000] = [0; 0x8000];
        for (i, byte) in lo_rom.iter().enumerate() {
            rom[i] = *byte;
        }
        for (i, byte) in hi_rom.iter().enumerate() {
            let i = i + 0x4000;
            rom[i] = *byte;
        }

        let ram = [0; 0x2000];
        let expansion_rom = [0; 0x1FDF];
        let mut ppu = [0; 0x2000];
        for (i, byte) in file.as_ref().unwrap().chr_rom.iter().enumerate() {
            ppu[i] = *byte;
        }
        Ok(Self {
            file,
            expansion_rom,
            ram,
            rom,
            ppu,
            mapper,
        })
    }
}

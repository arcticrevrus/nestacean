use std::path::Path;

use libnestacean::{self, Cart, cart::RomFile};

fn main() {
    let rom_file = "/Users/revrus/Downloads/test.nes";
    if let Ok(file) = RomFile::from_file(Path::new(rom_file)) {
        print!("[");
        for (i, byte) in file.chr_rom.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{byte:02X}");
        }
        print!("]");
        println!();
    };
}

use std::path::Path;

use libnestacean::{self, Cart, Nes, cart::RomFile};

fn main() {
    let rom_file = "../test.nes";
    let cart = Cart::from_file(Path::new(rom_file)).unwrap();
    let mut nes = Nes::new(cart);
    let byte = nes.cpu.mmap.read(nes.cpu.registers.pc);
    for b in nes.cpu.mmap.cart.rom {
        print!("{b:02X} ");
    }
    println!("]");
}

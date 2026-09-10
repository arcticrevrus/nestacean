use std::path::Path;

use libnestacean::{self, Cart, Nes, cart::RomFile};

fn main() {
    let rom_file = "../test.nes";
    let cart = Cart::from_file(Path::new(rom_file)).unwrap();
    let mut nes = Nes::new(cart);
    loop {
        nes.cpu.step()
    }
}

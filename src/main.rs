mod gameboy;
use crate::gameboy::GameBoy;
use std::fs;

fn main() {

    let mut gameboy: GameBoy = GameBoy::new();
    let rom = fs::read("./roms/tetris.gb");
    match rom {
        Ok(rom) => {
            gameboy.set_memory(rom);
        },
        Err(_) => {
            println!("No file");
        },
    }
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

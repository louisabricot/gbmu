mod gameboy;
use crate::gameboy::GameBoy;

fn main() {
    let mut GameBoy: GameBoy = GameBoy::new();
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

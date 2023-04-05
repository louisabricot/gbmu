mod graphics;
mod hardware;
use graphics::Graphics;

fn main() {
    let mut graphics: Graphics = Graphics::new();
    graphics.render();
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod lcd;

fn main() {
    lcd::render();
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

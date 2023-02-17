use std::env;
use std::fs;

mod hardware;
use crate::hardware::cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("not enough arguments");
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    let content = fs::read(file_path).expect("Should have been able to read the file");

    let mut mycpu = Cpu::new(0);
    let endpc = mycpu.run(content, 200);
    println!("{}", endpc);
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

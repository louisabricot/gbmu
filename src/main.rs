use std::env;
use std::fs;

mod hardware;
use hardware::cpu::Cpu;
use hardware::memory::Memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("not enough arguments");
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    let _content = fs::read(file_path).expect("Should have been able to read the file");

    let mut _memory = Memory::new(_content);
    let mut _mycpu = Cpu::new(_memory);
    let vec = _mycpu.disassemble(300, 0);
    for line in vec {
        println!("{}", line);
    }
}

#[cfg(test)]
mod main_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//! Joypad Input
//! The eight gameboy buttons/direction key are arranged in form of a 2x4
//! matrix.  
//! Select either button or direction key by writing to this register, then
//! read out bit 0-3.  
//! 
//! | 7    | 6    | 5      |  4        | 3         | 2         |    1   |  0     |
//! |------|------|--------|-----------|-----------|-----------|--------|--------|
//! | X    |  X   | Button | Direction | Down/Start| Up/Select | Left/B | Right/A| 


const ACTION: u8 = 0b00100000;
const DIRECTION: u8 = 0b00010000;

pub enum Action {
  Start,
  Select,
  A,
  B
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub enum Output {
  Action,
  Direction
}

pub struct Joypad {
    start: bool,
    select: bool,
    left: bool,
    right: bool,
    up: bool,
    b: bool,
    a: bool,
}

impl Joypad {

}

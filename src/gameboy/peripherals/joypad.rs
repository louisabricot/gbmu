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

    pub fn update(&mut self, p1: u8) {
      let button = match (select_buttons(p1)) {
        Output::Action => todo, 
        Output::Direction => 
      }
    }

    fn select_buttons(p1: u8) -> Option<Output> {
      let select_pins = 0b00110000;
      match p1 & select_pins {
        ACTION => Some(Output::Action),
        DIRECTION => Some(Output::Direction),
        _ => None,
      }
    }
}

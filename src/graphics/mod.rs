//! Graphics module include windows, GUI elements and graphics controllers for
//! the GameBoy
//!
//! # Example
//! ```
//! use graphics::Graphics;
//!
//! let mut graphics: Graphics = Graphics::new();
//! graphics.render();
//! ```

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use std::time::Duration;

use super::gameboy::GameBoy;

mod controller;
mod debugger;
mod gui;
mod lcd;

use debugger::Debugger;
use lcd::Lcd;

pub struct Graphics {
    /// Sdl context provide by sdl2
    sdl_context: Sdl,
    /// LCD Window rendering GameBoy screen
    pub lcd: Lcd,
    /// Debugger Window providing options for the GameBoy emulator
    pub debugger: Debugger,
    pub gameboy: Option<GameBoy>,
}

impl Graphics {
    /// Create a new Graphics object from a sdl2 context
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let lcd = Lcd::new(&sdl_context);
        let (mut x, y) = lcd.canvas().window().position();
        x += lcd.canvas().window().size().0 as i32 + 10;
        let debugger = Debugger::new(&sdl_context, x, y);
        Self {
            sdl_context,
            lcd,
            debugger,
            gameboy: None,
        }
    }

    /// Render LCD and Debugger Windows, loop and trigger GUI buttons events
    pub fn render(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::Window {
                        win_event: WindowEvent::Close,
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode, .. } => {
                        if let Some(button) = self.lcd.keypress(keycode.unwrap().name()) {
                            button.clone().action(self)
                        }
                    }
                    Event::MouseButtonDown {
                        window_id, x, y, ..
                    } => {
                        if self.debugger.get_window_id() == window_id {
                            if let Some(button) = self.debugger.click(x, y) {
                                if button.active() {
                                    button.clone().action(self)
                                }
                            }
                        } else if self.lcd.get_window_id() == window_id {
                            if let Some(button) = self.lcd.click(x, y) {
                                if button.active() {
                                    button.clone().action(self)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            self.lcd.print_frame();
            match &self.gameboy {
                Some(gameboy) => self.debugger.print_frame(
                    self.print_registers(),
                    self.get_flags(),
                    gameboy.disassemble(
                        self.debugger.instructions().get_nb_lines() as u16,
                        gameboy.get_program_counter(),
                    ),
                ),
                None => self.debugger.print_frame(vec![], vec![], vec![]),
            };
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn print_registers(&self) -> Vec<String> {
        let mut registers = Vec::new();
        if let Some(gameboy) = &self.gameboy {
            registers = gameboy.get_registers();
        }
        registers
    }

    fn get_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if let Some(gameboy) = &self.gameboy {
            flags = gameboy.get_flags();
        }
        flags
    }
}

//! Graphics module include windows, GUI elements and graphics controllers for
//! the GameBoy
//!
//! # Example
//! ```
//! use graphics::Graphics;
//!
//! fn main() {
//!     let mut graphics: Graphics = Graphics::new();
//!     graphics.render();
//! }
//! ```

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use std::time::Duration;

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
            self.debugger.print_frame();
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

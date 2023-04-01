extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

mod button;
mod controller;
mod debugger;
mod lcd;
mod utils;

use debugger::Debugger;
use lcd::Lcd;

pub struct Graphics {
    sdl_context: Sdl,
    pub lcd: Lcd,
    pub debugger: Debugger,
}

impl Graphics {
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

    pub fn render(&mut self) {
        self.debugger.print_frame();

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
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
            std::thread::sleep(Duration::from_millis(10));
            self.lcd.print_frame();
        }
    }
}

extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod debugger;
mod lcd;

use debugger::Debugger;
use lcd::Lcd;

pub fn render() {
    let sdl_context = sdl2::init().unwrap();
    let mut lcd = Lcd::new(&sdl_context);
    let (mut x, y) = lcd.canvas().window().position();
    x += lcd.canvas().window().size().0 as i32 + 10;
    let mut debugger = Debugger::new(&sdl_context, x, y);

    let mut x: u32 = 0;
    let mut y: u32 = 0;
    while y < lcd.get_height() {
        let c1: u8 = (((x * 255) / lcd.get_width()) % 255) as u8;
        let c2: u8 = (((y * 255) / lcd.get_height()) % 255) as u8;
        lcd.set_pixel(x, y, Color::RGB(c1, c2, 255 - c1));
        x += 1;
        if x >= lcd.get_width() {
            x = 0;
            y += 1;
        }
    }

    lcd.print_frame();
    debugger.print_frame();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    window_id, x, y, ..
                } => {
                    if debugger.get_window_id() == window_id {
                        if let Some(button) = debugger.click(x, y) {
                            button.action()
                        }
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

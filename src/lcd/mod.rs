extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

/// LCD width
const SCREEN_WIDTH: u32 = 166;
/// LCD height
const SCREEN_HEIGHT: u32 = 144;

/// Ratio used to render the LCD window
const PIXEL_SIZE: u32 = 4;

/// Represent the Gameboy LCD window
pub struct LCD {
    canvas: Canvas<Window>,
}

impl LCD {
    /// Create a new LCD using the sdl_context given
    fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "gbmu",
                SCREEN_WIDTH * PIXEL_SIZE,
                SCREEN_HEIGHT * PIXEL_SIZE,
            )
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Self { canvas: canvas }
    }

    /// Set a pixel at position (x, y) to a given color
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Rect::new(
                (x * PIXEL_SIZE) as i32,
                (y * PIXEL_SIZE) as i32,
                PIXEL_SIZE,
                PIXEL_SIZE,
            ))
            .unwrap();
    }

    /// Print the actual frame into the LCD window
    fn print_frame(&mut self) {
        self.canvas.present();
    }

    /// Get width of the LCD screen
    fn get_width(&self) -> u32 {
        self.canvas.window().size().0 / PIXEL_SIZE
    }

    /// Get height of the LCD screen
    fn get_height(&self) -> u32 {
        self.canvas.window().size().1 / PIXEL_SIZE
    }
}

pub fn render() {
    let sdl_context = sdl2::init().unwrap();
    let mut lcd = LCD::new(&sdl_context);

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
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

extern crate sdl2;

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
    pub fn new(sdl_context: &Sdl) -> Self {
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
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
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
    pub fn print_frame(&mut self) {
        self.canvas.present();
    }

    /// Get width of the LCD screen
    pub fn get_width(&self) -> u32 {
        self.canvas.window().size().0 / PIXEL_SIZE
    }

    /// Get height of the LCD screen
    pub fn get_height(&self) -> u32 {
        self.canvas.window().size().1 / PIXEL_SIZE
    }
}

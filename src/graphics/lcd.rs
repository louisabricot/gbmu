extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;
use sdl2::video::WindowPos::{Centered, Positioned};
use sdl2::Sdl;

use std::include_bytes;

use super::button::Button;
use super::utils::get_texture_rect;

/// LCD width
const SCREEN_WIDTH: u32 = 166;
/// LCD height
const SCREEN_HEIGHT: u32 = 144;

/// Ratio used to render the LCD window
const PIXEL_SIZE: u32 = 4;

/// Represent the Gameboy LCD window
pub struct Lcd {
    canvas: Canvas<Window>,
    _buttons: Vec<Button>,
}

impl Lcd {
    /// Create a new LCD using the sdl_context given
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let mut window = video_subsystem
            .window(
                "gbmu",
                SCREEN_WIDTH * PIXEL_SIZE,
                SCREEN_HEIGHT * PIXEL_SIZE,
            )
            .build()
            .unwrap();
        window.set_position(Centered, Positioned(0));
        let canvas = window.into_canvas().build().unwrap();
        let buttons = vec![Button::new(0, 0, 0, 0, "".to_string())];
        Self {
            canvas,
            _buttons: buttons,
        }
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

    // Render transparent joystick on canvas
    pub fn render_joystick(&mut self) -> Result<(), String> {
        self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let layout_gameboy: &[u8] = include_bytes!("../../assets/layout_gameboy.png");

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.load_texture_bytes(layout_gameboy)?;
        texture.set_alpha_mod(64);
        let TextureQuery { width, height, .. } = texture.query();
        let target = get_texture_rect(
            0,
            0,
            width,
            height,
            self.canvas.window().size().0,
            self.canvas.window().size().1,
            true,
        );
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        self.canvas.copy(&texture, None, Some(target).unwrap())?;
        Ok(())
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

    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }
}

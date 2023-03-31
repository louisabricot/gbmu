extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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

const JOYSTICK_TEXTURE_WIDTH: u32 = 843;
const JOYSTICK_TEXTURE_HEIGHT: u32 = 433;

/// Represent the Gameboy LCD window
pub struct Lcd {
    canvas: Canvas<Window>,
    joystick: Joystick,
    buttons: Vec<Button>,
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
        let joystick = Joystick::new(&canvas);
        let ratio_width: f32 = joystick.rect.width() as f32 / JOYSTICK_TEXTURE_WIDTH as f32;
        let ratio_height: f32 = joystick.rect.height() as f32 / JOYSTICK_TEXTURE_HEIGHT as f32;
        let buttons = vec![
            Button::new(
                // A
                joystick.rect().x + (690.0 * ratio_width) as i32,
                joystick.rect().y + (25.0 * ratio_height) as i32,
                (130.0 * ratio_width) as u32,
                (130.0 * ratio_height) as u32,
                40,
                "O".to_string(),
                true,
            ),
            Button::new(
                // B
                joystick.rect().x + (540.0 * ratio_width) as i32,
                joystick.rect().y + (95.0 * ratio_height) as i32,
                (130.0 * ratio_width) as u32,
                (130.0 * ratio_height) as u32,
                40,
                "K".to_string(),
                true,
            ),
            Button::new(
                // UP
                joystick.rect().x + (105.0 * ratio_width) as i32,
                joystick.rect().y + (22.0 * ratio_height) as i32,
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "W".to_string(),
                true,
            ),
            Button::new(
                // LEFT
                joystick.rect().x + (26.0 * ratio_width) as i32,
                joystick.rect().y + (102.0 * ratio_height) as i32,
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "A".to_string(),
                true,
            ),
            Button::new(
                // DOWN
                joystick.rect().x + (105.0 * ratio_width) as i32,
                joystick.rect().y + (182.0 * ratio_height) as i32,
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "S".to_string(),
                true,
            ),
            Button::new(
                // RIGHT
                joystick.rect().x + (185.0 * ratio_width) as i32,
                joystick.rect().y + (102.0 * ratio_height) as i32,
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "D".to_string(),
                true,
            ),
            Button::new(
                // START
                joystick.rect().x + (220.0 * ratio_width) as i32,
                joystick.rect().y + (320.0 * ratio_height) as i32,
                (135.0 * ratio_width) as u32,
                (85.0 * ratio_height) as u32,
                30,
                "V".to_string(),
                false,
            ),
            Button::new(
                // SELECT
                joystick.rect().x + (380.0 * ratio_width) as i32,
                joystick.rect().y + (320.0 * ratio_height) as i32,
                (135.0 * ratio_width) as u32,
                (85.0 * ratio_height) as u32,
                30,
                "B".to_string(),
                false,
            ),
        ];
        Self {
            canvas,
            joystick,
            buttons,
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
    pub fn render_joystick(&mut self) {
        for button in &mut self.buttons {
            match button.draw(&mut self.canvas, None, Color::RGBA(0, 0, 0, 64)) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        match self.joystick.draw(&mut self.canvas) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    }

    pub fn keypress(&self, name: String) -> Option<&Button> {
        self.buttons.iter().find(|&button| button.text().eq(&name))
    }

    pub fn click(&self, x: i32, y: i32) -> Option<&Button> {
        self.buttons
            .iter()
            .find(|&button| button.rect().contains_point(Point::new(x, y)))
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

    /// Get canvas
    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    /// Get the window id from canvas
    pub fn get_window_id(&self) -> u32 {
        self.canvas.window().id()
    }
}

struct Joystick {
    rect: Rect,
}

impl Joystick {
    fn new(canvas: &Canvas<Window>) -> Self {
        let layout_gameboy: &[u8] = include_bytes!("../../assets/layout_gameboy.png");

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture_bytes(layout_gameboy).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = get_texture_rect(
            0,
            0,
            width,
            height,
            canvas.window().size().0,
            canvas.window().size().1,
            true,
        );

        Self { rect: target }
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let layout_gameboy: &[u8] = include_bytes!("../../assets/layout_gameboy.png");

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.load_texture_bytes(layout_gameboy).unwrap();
        texture.set_alpha_mod(64);
        canvas.copy(&texture, None, Some(self.rect).unwrap())
    }
}

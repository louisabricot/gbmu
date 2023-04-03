//! LCD Window
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::video::WindowPos::{Centered, Positioned};
use sdl2::Sdl;

use super::gui::button::Button;
use super::gui::joystick::{Joystick, JOYSTICK_TEXTURE_HEIGHT, JOYSTICK_TEXTURE_WIDTH};

/// LCD width
const SCREEN_WIDTH: u32 = 166;
/// LCD height
const SCREEN_HEIGHT: u32 = 144;

/// Ratio used to render the LCD window
const PIXEL_SIZE: u32 = 4;

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
        let ratio_width: f32 = joystick.rect().width() as f32 / JOYSTICK_TEXTURE_WIDTH as f32;
        let ratio_height: f32 = joystick.rect().height() as f32 / JOYSTICK_TEXTURE_HEIGHT as f32;
        let buttons = vec![
            Button::new(
                // A
                (
                    joystick.rect().x + (690.0 * ratio_width) as i32,
                    joystick.rect().y + (25.0 * ratio_height) as i32,
                ),
                (130.0 * ratio_width) as u32,
                (130.0 * ratio_height) as u32,
                40,
                "O".to_string(),
                true,
                None,
            ),
            Button::new(
                // B
                (
                    joystick.rect().x + (540.0 * ratio_width) as i32,
                    joystick.rect().y + (95.0 * ratio_height) as i32,
                ),
                (130.0 * ratio_width) as u32,
                (130.0 * ratio_height) as u32,
                40,
                "K".to_string(),
                true,
                None,
            ),
            Button::new(
                // UP
                (
                    joystick.rect().x + (105.0 * ratio_width) as i32,
                    joystick.rect().y + (22.0 * ratio_height) as i32,
                ),
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "W".to_string(),
                true,
                None,
            ),
            Button::new(
                // LEFT
                (
                    joystick.rect().x + (26.0 * ratio_width) as i32,
                    joystick.rect().y + (102.0 * ratio_height) as i32,
                ),
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "A".to_string(),
                true,
                None,
            ),
            Button::new(
                // DOWN
                (
                    joystick.rect().x + (105.0 * ratio_width) as i32,
                    joystick.rect().y + (182.0 * ratio_height) as i32,
                ),
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "S".to_string(),
                true,
                None,
            ),
            Button::new(
                // RIGHT
                (
                    joystick.rect().x + (185.0 * ratio_width) as i32,
                    joystick.rect().y + (102.0 * ratio_height) as i32,
                ),
                (80.0 * ratio_width) as u32,
                (80.0 * ratio_height) as u32,
                40,
                "D".to_string(),
                true,
                None,
            ),
            Button::new(
                // START
                (
                    joystick.rect().x + (220.0 * ratio_width) as i32,
                    joystick.rect().y + (320.0 * ratio_height) as i32,
                ),
                (135.0 * ratio_width) as u32,
                (85.0 * ratio_height) as u32,
                30,
                "V".to_string(),
                false,
                None,
            ),
            Button::new(
                // SELECT
                (
                    joystick.rect().x + (380.0 * ratio_width) as i32,
                    joystick.rect().y + (320.0 * ratio_height) as i32,
                ),
                (135.0 * ratio_width) as u32,
                (85.0 * ratio_height) as u32,
                30,
                "B".to_string(),
                false,
                None,
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
    fn render_joystick(&mut self) {
        if !self.joystick.hidden() {
            for button in &mut self.buttons {
                match button.draw(&mut self.canvas, None, Color::RGBA(0, 0, 0, 128)) {
                    Ok(()) => (),
                    Err(e) => println!("{}", e),
                }
            }
            match self.joystick.draw(&mut self.canvas) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
    }

    /// Return a button if the name is matching
    pub fn keypress(&mut self, name: String) -> Option<&mut Button> {
        self.buttons
            .iter_mut()
            .find(|button| button.text().eq(&name))
    }

    /// Return a button if exists at a given position
    pub fn click(&mut self, x: i32, y: i32) -> Option<&mut Button> {
        self.buttons
            .iter_mut()
            .find(|button| button.rect().contains_point(Point::new(x, y)))
    }

    /// Print the actual frame into the LCD window
    pub fn print_frame(&mut self) {
        self.canvas.clear();
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        while y < self.get_height() {
            let c1: u8 = (((x * 255) / self.get_width()) % 255) as u8;
            let c2: u8 = (((y * 255) / self.get_height()) % 255) as u8;
            self.set_pixel(x, y, Color::RGB(c1, c2, 255 - c1));
            x += 1;
            if x >= self.get_width() {
                x = 0;
                y += 1;
            }
        }
        self.render_joystick();
        self.canvas.present();
    }

    /// Get width of the LCD screen
    pub fn get_width(&self) -> u32 {
        self.canvas.window().size().0 / PIXEL_SIZE
    }

    /// Show joystick and activate click on buttons
    pub fn show_joystick(&mut self) {
        self.joystick.show();
        for button in &mut self.buttons {
            button.activate();
        }
    }

    /// Hide joystick and deactive click on buttons
    pub fn hide_joystick(&mut self) {
        self.joystick.hide();
        for button in &mut self.buttons {
            button.deactivate();
        }
    }

    /// Get height of the LCD screen
    pub fn get_height(&self) -> u32 {
        self.canvas.window().size().1 / PIXEL_SIZE
    }

    /// Get canvas
    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    /// Get joystick
    pub fn joystick(&self) -> &Joystick {
        &self.joystick
    }

    /// Get the window id from canvas
    pub fn get_window_id(&self) -> u32 {
        self.canvas.window().id()
    }
}

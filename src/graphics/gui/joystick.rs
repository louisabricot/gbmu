//! GUI Joystick
//!
//! # Example
//!
//! ```
//! extern crate sdl2;
//!
//! use crate::graphics::gui::Joystick;
//!
//!
//! fn main() {
//!     let sdl_context = sdl2::init().unwrap();
//!     let video_subsystem = sdl_context.video().unwrap();
//!     let mut window = video_subsystem
//!         .window(
//!             "Window",
//!             1280,
//!             768,
//!         )
//!         .build()
//!         .unwrap();
//!     let canvas = window.into_canvas().build().unwrap();
//!     let joystick = Joystick::new(&canvas);
//! }
//! ```

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

use super::utils::get_texture_rect;

pub const JOYSTICK_TEXTURE_WIDTH: u32 = 843;
pub const JOYSTICK_TEXTURE_HEIGHT: u32 = 433;

/// Represent a GUI Joystick
pub struct Joystick {
    /// `sdl2::rect::Rect` containing the joystick's texture
    rect: Rect,
    /// Hide or not the joystick
    hidden: bool,
}

impl Joystick {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let layout_gameboy: &[u8] = get_layout!();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture_bytes(layout_gameboy).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = get_texture_rect(
            (0, 0),
            width,
            height,
            canvas.window().size().0,
            canvas.window().size().1,
            true,
        );

        Self {
            rect: target,
            hidden: false,
        }
    }

    pub fn hide(&mut self) {
        self.hidden = true
    }

    pub fn show(&mut self) {
        self.hidden = false
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let layout_gameboy: &[u8] = get_layout!();

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.load_texture_bytes(layout_gameboy).unwrap();
        texture.set_alpha_mod(64);
        canvas.copy(&texture, None, Some(self.rect).unwrap())
    }
}

macro_rules! get_layout {
    () => {
        include_bytes!("../../../assets/layout_gameboy.png")
    };
}

use get_layout;

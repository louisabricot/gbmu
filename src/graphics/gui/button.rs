//! GUI Button
//!
//! # Example
//!
//! ```
//! use crate::graphics::Graphics;
//! use crate::graphics::gui::button::Button;
//!
//! fn ok_handler(_graphics: &mut Graphics) {
//!     println!("Ok !");
//! }
//!
//! let button: Button = Button::new(
//!     (0, 0),
//!     100,
//!     35,
//!     10,
//!     "OK".to_string(),
//!     true,
//!     Some(ok_handler),
//! );
//! ```

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

use super::super::Graphics;
use super::utils::{get_font, get_texture_rect};

/// Represent a GUI Button
#[derive(Clone)]
pub struct Button {
    /// `sdl2::rect::Rect` to locate and draw button
    rect: Rect,
    /// Text to print inside the button
    text: String,
    /// Text font size
    line_height: u32,
    /// Tell if the text is centered inside the button
    centered_text: bool,
    /// Tell if the button is active or not, could be use in parent class to
    /// check for actions
    active: bool,
    /// Action to execute
    action: Option<fn(&mut Graphics)>,
}

impl Button {
    pub fn new(
        (x, y): (i32, i32),
        width: u32,
        height: u32,
        line_height: u32,
        text: String,
        centered: bool,
        action: Option<fn(&mut Graphics)>,
    ) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            text,
            line_height,
            centered_text: centered,
            active: true,
            action,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        bg_color: Option<Color>,
        font_color: Color,
    ) -> Result<(), String> {
        match bg_color {
            None => (),
            Some(color) => {
                canvas.set_draw_color(color);
                canvas.fill_rect(self.rect)?;
            }
        }
        let ttf_context: Sdl2TtfContext = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: Font = get_font(&ttf_context)?;

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font.render(self.text.as_str()).solid(font_color).unwrap();
        let mut texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        texture.set_alpha_mod(font_color.rgba().3);
        let TextureQuery { width, height, .. } = texture.query();
        let mut pos_y = self.rect.y();
        if self.centered_text {
            pos_y += (self.rect.height() as i32 - self.line_height as i32) / 2;
        }
        let target = get_texture_rect(
            (self.rect.x(), pos_y),
            width,
            height,
            self.rect.width(),
            self.line_height,
            self.centered_text,
        );
        canvas.copy(&texture, None, Some(target).unwrap())?;
        Ok(())
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true
    }

    pub fn deactivate(&mut self) {
        self.active = false
    }

    pub fn action(&mut self, graphics: &mut Graphics) {
        match self.action {
            None => println!("{}", self.text),
            Some(action) => action(graphics),
        }
    }
}

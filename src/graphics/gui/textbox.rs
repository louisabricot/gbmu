//! GUI Textbox
//!
//! # Example
//!
//! ```
//! use crate::graphics::gui::textbox::TextBox;
//!
//! let textbox = TextBox::new(0, 0, 300, 400);
//! ```

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;

use super::utils::{get_font, get_texture_rect};

const PADDING_TEXTBOX: u32 = 5;
const INTERLINE_TEXTBOX: u32 = 2;
const LINE_HEIGHT_TEXTBOX: u32 = 10;

const COLOR_BACKGROUND: Color = Color::RGB(22, 27, 35);

/// Represent a GUI TextBox
pub struct TextBox {
    /// `sdl2::rect::Rect` to draw textbox
    rect: Rect,
    /// Padding between text and rect
    padding: u32,
    /// Interline size in px
    interline: u32,
    /// Font size
    line_height: u32,
}

impl TextBox {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            padding: PADDING_TEXTBOX,
            interline: INTERLINE_TEXTBOX,
            line_height: LINE_HEIGHT_TEXTBOX,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, lines: Vec<&str>) -> Result<(), String> {
        canvas.set_draw_color(COLOR_BACKGROUND);
        canvas.fill_rect(self.rect)?;

        let ttf_context: Sdl2TtfContext = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: Font = get_font(&ttf_context)?;

        for (index, line) in lines.iter().enumerate() {
            // render a surface, and convert it to a texture bound to the canvas
            let surface = font.render(line).solid(Color::WHITE).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let target = get_texture_rect(
                (
                    self.rect.x() + self.padding as i32,
                    self.rect.y()
                        + (index * (self.line_height + self.interline) as usize) as i32
                        + self.padding as i32,
                ),
                width,
                height,
                self.rect.width(),
                self.line_height,
                false,
            );
            canvas.copy(&texture, None, Some(target).unwrap())?;
        }
        Ok(())
    }

    pub fn get_nb_lines(&self) -> u32 {
        (self.rect.height() - self.padding * 2) / (self.line_height + self.interline)
    }
}

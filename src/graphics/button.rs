use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use sdl2::video::Window;

use super::utils::get_texture_rect;
use super::Graphics;

#[derive(Clone)]
pub struct Button {
    rect: Rect,
    text: String,
    line_height: u32,
    centered_text: bool,
    active: bool,
    action: Option<fn(&mut Graphics)>,
}

impl Button {
    pub fn new(
        pos: (i32, i32),
        width: u32,
        height: u32,
        line_height: u32,
        text: String,
        centered: bool,
        action: Option<fn(&mut Graphics)>,
    ) -> Self {
        Self {
            rect: Rect::new(pos.0, pos.1, width, height),
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
        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: &[u8] = include_bytes!("../../assets/gameboy.ttf");
        let font: Font = ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 128)?;

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
            self.rect.x(),
            pos_y,
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

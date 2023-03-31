use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use sdl2::video::Window;

use super::utils::get_texture_rect;

pub struct Button {
    rect: Rect,
    text: String,
    active: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: String) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            text,
            active: true,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(self.rect)?;
        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: &[u8] = include_bytes!("../../assets/gameboy.ttf");
        let font: Font = ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 128)?;

        // render a surface, and convert it to a texture bound to the canvas
        let line_height = 15;
        let surface = font.render(self.text.as_str()).solid(Color::WHITE).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = get_texture_rect(
            self.rect.x(),
            self.rect.y() + (line_height / 2) as i32,
            width,
            height,
            self.rect.width(),
            line_height,
            true,
        );
        canvas.copy(&texture, None, Some(target).unwrap())?;
        Ok(())
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn _activate(&mut self) {
        self.active = true
    }

    pub fn _deactivate(&mut self) {
        self.active = false
    }

    pub fn action(&self) {
        if self.active {
            println!("{}", self.text);
        }
    }
}

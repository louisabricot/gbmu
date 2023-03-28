extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::Sdl;

use std::include_bytes;

const SPACE_SZ: u32 = 15;
const BTN_HEIGHT: u32 = 35;
const REG_HEIGHT: u32 = 200;
const PRG_HEIGHT: u32 = 400;

/// Debugger width
const SCREEN_WIDTH: u32 = 300;
/// Debugger height
const SCREEN_HEIGHT: u32 = SPACE_SZ * 6 + BTN_HEIGHT * 3 + REG_HEIGHT + PRG_HEIGHT;

/// Represent the Debugger window
pub struct Debugger {
    canvas: Canvas<Window>,
    boxes: Vec<TextBox>,
    buttons: Vec<Button>,
}

impl Debugger {
    /// Create a new LCD using the sdl_context given
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("debugger", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RED);

        let mut boxes = Vec::new();
        let mut buttons = Vec::new();

        // Load - Save - Reset
        let labels = vec!["Load", "Save", "Reset"];
        let nb_buttons = 3;
        let btn_width = (SCREEN_WIDTH - SPACE_SZ * (nb_buttons + 1)) / nb_buttons;
        let btn_height = BTN_HEIGHT;

        for i in 0..nb_buttons {
            let x = i * btn_width + (i + 1) * SPACE_SZ;
            let y = SPACE_SZ;
            buttons.push(Button::new(
                x as i32,
                y as i32,
                btn_width,
                btn_height,
                labels[i as usize].to_string(),
            ));
        }

        // Registers
        let x = SPACE_SZ;
        let y = SPACE_SZ * 2 + BTN_HEIGHT;
        let width = SCREEN_WIDTH - SPACE_SZ * 2;
        let height = REG_HEIGHT;
        boxes.push(TextBox::new(x as i32, y as i32, width, height));

        // Program execution
        let x = SPACE_SZ;
        let y = SPACE_SZ * 3 + BTN_HEIGHT + REG_HEIGHT;
        let width = SCREEN_WIDTH - SPACE_SZ * 2;
        let height = PRG_HEIGHT;
        boxes.push(TextBox::new(x as i32, y as i32, width, height));

        // Play - Pause - Step
        let labels = vec!["Play", "Pause", "Step"];
        let nb_buttons = 3;
        let btn_width = (SCREEN_WIDTH - SPACE_SZ * (nb_buttons + 1)) / nb_buttons;
        let btn_height = BTN_HEIGHT;

        for i in 0..nb_buttons {
            let x = i * btn_width + (i + 1) * SPACE_SZ;
            let y = SPACE_SZ * 4 + BTN_HEIGHT + REG_HEIGHT + PRG_HEIGHT;
            buttons.push(Button::new(
                x as i32,
                y as i32,
                btn_width,
                btn_height,
                labels[i as usize].to_string(),
            ));
        }

        // Speed slider
        let x = SPACE_SZ;
        let y = SPACE_SZ * 5 + BTN_HEIGHT * 2 + REG_HEIGHT + PRG_HEIGHT;
        let width = SCREEN_WIDTH - SPACE_SZ * 2;
        let height = BTN_HEIGHT;
        buttons.push(Button::new(
            x as i32,
            y as i32,
            width,
            height,
            "Speed".to_string(),
        ));

        Self {
            canvas,
            boxes,
            buttons,
        }
    }

    pub fn click(&self, x: i32, y: i32) -> Option<&Button> {
        self.buttons
            .iter()
            .find(|&button| button.rect.contains_point(Point::new(x, y)))
    }

    /// Print the actual frame into the Debugger window
    pub fn print_frame(&mut self) {
        for button in &mut self.buttons {
            match button.draw(&mut self.canvas) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        for textbox in &mut self.boxes {
            match textbox.draw(&mut self.canvas, vec!["abc", "def", "ghi"]) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        self.canvas.present();
    }

    /// Get the window id from canvas
    pub fn get_window_id(&self) -> u32 {
        self.canvas.window().id()
    }
}

pub struct Button {
    rect: Rect,
    text: String,
}

impl Button {
    fn new(x: i32, y: i32, width: u32, height: u32, text: String) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            text,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(self.rect)?;
        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: &[u8] = include_bytes!("../../assets/gameboy.ttf");
        let font: Font = ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 64)?;

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font.render(self.text.as_str()).solid(Color::WHITE).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let target = Rect::new(
            self.rect.x(),
            self.rect.y(),
            self.rect.width(),
            self.rect.height(),
        );
        canvas.copy(&texture, None, Some(target).unwrap())?;
        Ok(())
    }

    pub fn action(&self) {
        println!("{}", self.text);
    }
}

pub struct TextBox {
    rect: Rect,
}

impl TextBox {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, lines: Vec<&str>) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(self.rect)?;

        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: &[u8] = include_bytes!("../../assets/gameboy.ttf");
        let font: Font = ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 64)?;

        let line_height = self.rect.height() / lines.len() as u32;
        for (index, line) in lines.iter().enumerate() {
            // render a surface, and convert it to a texture bound to the canvas
            let surface = font.render(line).solid(Color::WHITE).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let target = Rect::new(
                self.rect.x(),
                self.rect.y() + (index * line_height as usize) as i32,
                self.rect.width(),
                line_height,
            );
            canvas.copy(&texture, None, Some(target).unwrap())?;
        }
        Ok(())
    }
}

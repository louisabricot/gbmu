extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::Sdl;

use std::include_bytes;

use super::button::Button;
use super::controller::toggle_overlay;
use super::utils::get_texture_rect;

const SPACE_SZ: u32 = 15;
const BTN_HEIGHT: u32 = 35;
const REG_HEIGHT: u32 = 200;
const PRG_HEIGHT: u32 = 400;

/// Debugger width
const SCREEN_WIDTH: u32 = 300;
/// Debugger height
const SCREEN_HEIGHT: u32 = SPACE_SZ * 7 + BTN_HEIGHT * 4 + REG_HEIGHT + PRG_HEIGHT;

/// Represent the Debugger window
pub struct Debugger {
    canvas: Canvas<Window>,
    boxes: Vec<TextBox>,
    buttons: Vec<Button>,
}

impl Debugger {
    /// Create a new LCD using the sdl_context given
    pub fn new(sdl_context: &Sdl, x: i32, y: i32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("debugger", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position(x, y)
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
                (x as i32, y as i32),
                btn_width,
                btn_height,
                10,
                labels[i as usize].to_string(),
                true,
                None,
            ));
        }

        let x = SPACE_SZ;
        let btn_width = SCREEN_WIDTH - SPACE_SZ * 2;
        let y = SPACE_SZ * 2 + BTN_HEIGHT;
        buttons.push(Button::new(
            (x as i32, y as i32),
            btn_width,
            btn_height,
            10,
            "Hide - Show Overlay".to_string(),
            true,
            Some(toggle_overlay),
        ));

        // Registers
        let x = SPACE_SZ;
        let y = SPACE_SZ * 3 + BTN_HEIGHT * 2;
        let width = SCREEN_WIDTH - SPACE_SZ * 2;
        let height = REG_HEIGHT;
        boxes.push(TextBox::new(x as i32, y as i32, width, height));

        // Program execution
        let x = SPACE_SZ;
        let y = SPACE_SZ * 4 + BTN_HEIGHT * 2 + REG_HEIGHT;
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
            let y = SPACE_SZ * 5 + BTN_HEIGHT * 2 + REG_HEIGHT + PRG_HEIGHT;
            buttons.push(Button::new(
                (x as i32, y as i32),
                btn_width,
                btn_height,
                10,
                labels[i as usize].to_string(),
                true,
                None,
            ));
        }

        // Speed slider
        let x = SPACE_SZ;
        let y = SPACE_SZ * 6 + BTN_HEIGHT * 3 + REG_HEIGHT + PRG_HEIGHT;
        let width = SCREEN_WIDTH - SPACE_SZ * 2;
        let height = BTN_HEIGHT;
        buttons.push(Button::new(
            (x as i32, y as i32),
            width,
            height,
            10,
            "Speed".to_string(),
            true,
            None,
        ));

        Self {
            canvas,
            boxes,
            buttons,
        }
    }

    pub fn click(&mut self, x: i32, y: i32) -> Option<&mut Button> {
        self.buttons
            .iter_mut()
            .find(|button| button.rect().contains_point(Point::new(x, y)))
    }

    /// Print the actual frame into the Debugger window
    pub fn print_frame(&mut self) {
        for button in &mut self.buttons {
            match button.draw(&mut self.canvas, Some(Color::RED), Color::WHITE) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        match self.boxes[0].draw(
            &mut self.canvas,
            vec![
                "AB: 0xffff",
                "     A: 0xff",
                "     B: 0xff",
                "CD: 0xffff",
                "     C: 0xff",
                "     D: 0xff",
                "EF: 0xffff",
                "     E: 0xff",
                "     F: 0xff",
                "HL: 0xffff",
                "     H: 0xff",
                "     L: 0xff",
                "SP: 0xffff",
                "PC: 0xffff",
            ],
        ) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        match self.boxes[1].draw(
            &mut self.canvas,
            vec!["0xffff: LD AB, 0xdead"; self.boxes[1].get_nb_lines() as usize],
        ) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        self.canvas.present();
    }

    /// Get the window id from canvas
    pub fn get_window_id(&self) -> u32 {
        self.canvas.window().id()
    }
}

const PADDING_TEXTBOX: u32 = 5;
const INTERLINE_TEXTBOX: u32 = 2;
const LINE_HEIGHT_TEXTBOX: u32 = 10;

pub struct TextBox {
    rect: Rect,
    padding: u32,
    interline: u32,
    line_height: u32,
}

impl TextBox {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            padding: PADDING_TEXTBOX,
            interline: INTERLINE_TEXTBOX,
            line_height: LINE_HEIGHT_TEXTBOX,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, lines: Vec<&str>) -> Result<(), String> {
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(self.rect)?;

        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font: &[u8] = include_bytes!("../../assets/gameboy.ttf");
        let font: Font = ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 128)?;

        for (index, line) in lines.iter().enumerate() {
            // render a surface, and convert it to a texture bound to the canvas
            let surface = font.render(line).solid(Color::WHITE).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let target = get_texture_rect(
                self.rect.x() + self.padding as i32,
                self.rect.y()
                    + (index * (self.line_height + self.interline) as usize) as i32
                    + self.padding as i32,
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

    fn get_nb_lines(&self) -> u32 {
        (self.rect.height() - self.padding * 2) / (self.line_height + self.interline)
    }
}

//! Debugger Window
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use super::controller::{load_rom, toggle_overlay};
use super::gui::button::Button;
use super::gui::textbox::TextBox;
use super::Graphics;

/// Spaces between GUI elements
const SPACE_SZ: u32 = 15;
/// Button height
const BTN_HEIGHT: u32 = 35;
/// Registers TextBox height
const REG_HEIGHT: u32 = 200;
/// Program execution TextBox height
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
        let funcs: Vec<Option<fn(&mut Graphics)>> = vec![Some(load_rom), None, None];
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
                funcs[i as usize],
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

        // Registers 1
        let x = SPACE_SZ;
        let y = SPACE_SZ * 3 + BTN_HEIGHT * 2;
        let width = (SCREEN_WIDTH - SPACE_SZ * 2) / 2;
        let height = REG_HEIGHT;
        boxes.push(TextBox::new(x as i32, y as i32, width, height));

        // Registers 2
        let width = (SCREEN_WIDTH - SPACE_SZ * 2) / 2;
        let height = REG_HEIGHT;
        let x = SPACE_SZ + width;
        let y = SPACE_SZ * 3 + BTN_HEIGHT * 2;
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

    /// Return a button if exists at a given position
    pub fn click(&mut self, x: i32, y: i32) -> Option<&mut Button> {
        self.buttons
            .iter_mut()
            .find(|button| button.rect().contains_point(Point::new(x, y)))
    }

    /// Print the actual frame into the Debugger window
    pub fn print_frame(&mut self, registers: Vec<String>, instructions: Vec<String>) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        for button in &mut self.buttons {
            match button.draw(&mut self.canvas, Some(Color::RED), Color::WHITE) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
        }
        let regs = registers.iter().map(|s| s.as_ref()).collect();
        match self.boxes[0].draw(
            &mut self.canvas,
            regs,
        ) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        match self.boxes[1].draw(&mut self.canvas, vec!["F: 0b1111"]) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        let instr = instructions.iter().map(|s| s.as_ref()).collect();
        match self.boxes[2].draw(
            &mut self.canvas,
            instr,
        ) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
        self.canvas.present();
    }

    pub fn registers(&self) -> &TextBox {
        &self.boxes[0]
    }

    pub fn instructions(&self) -> &TextBox {
        &self.boxes[2]
    }

    /// Get the window id from canvas
    pub fn get_window_id(&self) -> u32 {
        self.canvas.window().id()
    }
}

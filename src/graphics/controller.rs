//! Controller contains handling function for buttons in the LCD or Debugger Window
use native_dialog::{FileDialog, MessageDialog, MessageType};

use std::fs;

use super::super::gameboy::GameBoy;
use super::super::gameboy::memory::Memory;

use super::Graphics;

/// Toggle overlay on the LCD Window
pub fn toggle_overlay(graphics: &mut Graphics) {
    if graphics.lcd.joystick().hidden() {
        graphics.lcd.show_joystick();
    } else {
        graphics.lcd.hide_joystick();
    }
}

pub fn step(graphics: &mut Graphics) {
    if let Some(gameboy) = &mut graphics.gameboy {
        gameboy.step();
    }
}

/// Open a FileDialog then load a Rom into memory
pub fn load_rom(graphics: &mut Graphics) {
    let path = match FileDialog::new()
        .add_filter("rom", &["gb"])
        .show_open_single_file()
    {
        Ok(path) => path,
        Err(error) => {
            MessageDialog::new()
                .set_title("Error")
                .set_type(MessageType::Error)
                .set_text(format!("Could not open FileDialog:\n {}", error).as_str())
                .show_alert()
                .ok();
            return;
        }
    };
    let path = match path {
        Some(path) => path,
        None => return, // Canceled dialog
    };
    let content = match fs::read(path) {
        Ok(content) => content,
        Err(error) => {
            MessageDialog::new()
                .set_title("Error")
                .set_type(MessageType::Error)
                .set_text(format!("Could not read:\n {}", error).as_str())
                .show_alert()
                .ok();
            return;
        }
    };
    let cartridge = Cartridge::new(content);
    graphics.gameboy = Some(GameBoy::new(cartridge));
    if let Some(gameboy) = &mut graphics.gameboy {
        gameboy.boot();
    }
}

extern crate native_dialog;

use native_dialog::{FileDialog, MessageDialog, MessageType};

use std::fs;

use super::Graphics;

pub fn toggle_overlay(graphics: &mut Graphics) {
    if graphics.lcd.joystick().hidden() {
        graphics.lcd.show_joystick();
    } else {
        graphics.lcd.hide_joystick();
    }
}

pub fn load_rom(_graphics: &mut Graphics) {
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
    let _content = match fs::read(path) {
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
}

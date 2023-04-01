use super::Graphics;

pub fn toggle_overlay(graphics: &mut Graphics) {
    if graphics.lcd.joystick().hidden() {
        graphics.lcd.show_joystick();
    } else {
        graphics.lcd.hide_joystick();
    }
}

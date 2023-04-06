//! Collection of utilitary functions used by GUI elements
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

/// Get gameboy font for GUI elements
pub fn get_font<'r>(ttf_context: &Sdl2TtfContext) -> Result<Font<'_, 'r>, String> {
    let font: &[u8] = include_bytes!("../../../assets/gameboy.ttf");
    ttf_context.load_font_from_rwops(RWops::from_bytes(font)?, 128)
}

/// Return a sdl2 Rect to scale texture
///
/// # Arguments
///
/// * `(x, y)` - Position of the parent rect
/// * `rect_width` - Texture's width
/// * `rect_height` - Texture's height
/// * `const_width` - Parent's width
/// * `const_height` - Parent's height
/// * `centered` - Center the texture inside his parent
pub fn get_texture_rect(
    (x, y): (i32, i32),
    rect_width: u32,
    rect_height: u32,
    cons_width: u32,
    cons_height: u32,
    centered: bool,
) -> Rect {
    let width_ratio = rect_width as f32 / cons_width as f32;
    let height_ratio = rect_height as f32 / cons_height as f32;

    let (width, height) = if width_ratio > 1f32 || height_ratio > 1f32 {
        if width_ratio > height_ratio {
            // Scaling down
            let height = (rect_height as f32 / width_ratio) as i32;
            (cons_width as i32, height)
        } else {
            // Scaling up
            let width = (rect_width as f32 / height_ratio) as i32;
            (width, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let (x, y) = if centered {
        (
            x + (cons_width as i32 - width) / 2,
            y + (cons_height as i32 - height) / 2,
        )
    } else {
        (x, y)
    };
    Rect::new(x, y, width as u32, height as u32)
}

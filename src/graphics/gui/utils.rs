//! Collection of utilitary functions used by GUI elements
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

/// Get gameboy font for GUI elements
pub fn get_font<'ttf, 'r>(ttf_context: &'ttf Sdl2TtfContext) -> Result<Font<'ttf, 'r>, String> {
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
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            // Scaling down
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            // Scaling up
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let (cx, cy) = if centered {
        (
            x + (cons_width as i32 - w) / 2,
            y + (cons_height as i32 - h) / 2,
        )
    } else {
        (x, y)
    };
    Rect::new(cx, cy, w as u32, h as u32)
}

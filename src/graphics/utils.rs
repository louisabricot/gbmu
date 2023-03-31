use sdl2::rect::Rect;

/// Return a sdl2 Rect to scale texture
pub fn get_texture_rect(
    x: i32,
    y: i32,
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
    Rect::new(cx as i32, cy as i32, w as u32, h as u32)
}

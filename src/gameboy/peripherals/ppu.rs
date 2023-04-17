const DEFAULT_TILEMAP: u16 = 0x9800;

struct PPU {
    // palette
    // layers: background, window and object

}

// FIFO pixel fetcher
// fetches a row of 8 background or window pixels and queues them up to be
// mixed with sprite pixels

// 5 steps

/// This step determines which background/window tile to fetch pixel from.
/// By default the tilemap used is the one at $9800 but certain conditions can
/// change that.
fn get_tile() {}

/// Checks the LCDC.4 for which tilemap to use.
/// CGB: checks which VRAM bank to use and check if the tile is flipped
/// vertically.
/// If the PPU's access to VRAM is blocked, then tile data is $FF.
fn get_tile_data_low() {}

/// Same as data_low but increment tile address by one.  
/// Pushes a row of background/window pixels to the FIFO. 
fn get_tile_data_high() {}

/// Does nothing
fn sleep() {}

/// Pushes a row of background/window pixels to the FIFO. 
/// Since tiles are 8 pixels wide, a "row" of pixels is 8 pixels from the tile
/// to be rendered based on the X and Y coordinates calculated in the previous
/// steps. Only pushes if FIFO is empty.  
fn push() {}

fn draw_screen() {}

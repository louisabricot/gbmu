pub struct OAM {
    table: Vec<Sprite>,
}

bitflags! {
    struct Attribute: u8 {
        const BG_WINDOW: u8 = 0b1000_0000;
        const Y_FLIP: u8 = 0b0100_0000;
        const X_FLIP: u8 = 0b0010_0000;
        const DMG_PALETTE_NB: u8 = 0b0001_0000;
        const TILE_WRAM: u8 = 0b0000_1000;
        const CGB_PALETTE_NB: u8 = 0b0000_0111;
    }
}

pub struct Sprite {
    y: u8,
    x: u8,
    tile_index: u8,
    attribute: Attribute,
}

impl Sprite {
    pub fn new(x: u8, y: u8, tile_index: u8, attribute: Attribute) -> Self {
        Self {
            x,
            y,
            tile_index,
            attribute,
        }
    }

    /// OAM attributes bit 7 will grant OBJ priority when clear, not set.  
    pub fn is_OBJ_priority(&self) -> bool {
        !self.attribute.contains(Attribute::BG_WINDOW)
    }
}

impl OAM {
    
    
}

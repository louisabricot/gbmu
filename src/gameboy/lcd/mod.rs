//8KB of build-int LCD display ram DMG
//16KB of build-int LCD display ram DMG

/// Pixels are grouped in 8x8 tiles
/// A tile assigns a color ID to each of its pixels


/// When a tile is used in background or window, the color ids are associated
/// with a palette
/// When a tile is used in an object, color id 0 refers to transparency

/// A single tile is represented by a u128
/// A row in a tile is represented by a u16

pub enum Color {
  White,
  LightGray,
  DarkGrey,
  Black
}

pub enum ColorId {
  Zero,
  One,
  Two,
  Three
}

//TODO: See CGB palette memory.



/// 
/// The GameBoy has three layers, from back to front: Backgroumd, Window and
/// Object. 
/// Both the background and window layers share the same tile data table.  
pub enum Layer {

  /// The *background* is composed of a tilemap which contains references to
  /// tiles. 
  Background,

  /// The *window* layer is on top of the `background` and often serves to fix
  /// status bar on the top-left position.  
  Window,

  /// 
  Object,
}

impl PPU {
    

    /// Tiles are obtained from the Tile Data Table using either of the two
    /// addressing modes which can be selected via the LCDC register.  
    get_tile(&self, base: u16) {
      
    } 

    pub fn get_row_color_id(&self, row: u16)
    {
      let [hi, lo] = row.to_le_bytes();

      for i in 0..u8::BITS {
          println!("color id is {}", self.get_pixel_color_id(hi, lo, i));

      }
    }

    /// Returns the color id for the pixel at *index*.  
    pub fn get_pixel_color_id(&self, hi: u8, lo: u8 index: u8) -> u8 {
      let mut color_id = match {
        hi & (1 << index) != 0 => color |= 0b10,
        lo & (1 << index) != 0 => color |= 0b01,
      }
      color_id
    }

    pub fn get_color(&self, id: u8) -> u8 {
        
    }
}

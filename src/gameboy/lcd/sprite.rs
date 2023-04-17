/// Sprites can be either 8x8 or 8x16 pixels.  
pub struct Sprite {
  
  /// Sprite's vertical position on the screen + 16.  
  /// `16` corresponds to the height of the 8x16-pixel sprite.  
  y: u8,

  /// Sprite's horizontal position on the screen + 8.  
  x: u8,

  /// In 8x8 mode, this byte specifies the sprites' tile index from the memory
  /// area at 0x8000-0x8FFF.  
  /// In 8x16 mode, this byte specifies the index of the first tile of the
  /// sprite.  
  tile_index: u8,

  /// 
  attributes: Attributes,

}

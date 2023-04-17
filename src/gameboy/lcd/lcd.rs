/// LCD Controller register encodes information about the LCD:
/// 
/// |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0  |
/// |-----|-----|-----|-----|-----|-----|-----|-----|
/// | CO  | WCA | WO  |BGCH | BCA | OBJ | OBJ | DIS |



pub struct LcdController {
  const BG_DISPLAY: u8 = 0b1;
  const OBJ_ON: u8 = 0b10;
  const OBJ_BLOCK_COMPOSITION: u8 = 0b100;
  const BG_CODE_ARE: u8 = 0b1000;
  const BG_CHARACTER_DATA: U8 0b10000;
  const WINDOWING_ON: u8 = 0b100000;
  const WINDOW_CODE_AREA: u8 = 0b1000000;
  const LCD_CONTROLLER_OPERATION_STOP: u8 = 0b10000000;
  
  flags: u8;
}

impl LcdController {

  
}
pub struct Lcd {

  controller: LcdController,
}

use bitflags::bitflags;

pub mod control_reg;
pub mod oam_address;
pub mod ppu_address;
pub mod scroll;

bitflags! {
  // 7  bit  0
  // ---- ----
  // BGRs bMmG
  // |||| ||||
  // |||| |||+- Greyscale (0: normal color, 1: greyscale)
  // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
  // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
  // |||| +---- 1: Enable background rendering
  // |||+------ 1: Enable sprite rendering
  // ||+------- Emphasize red (green on PAL/Dendy)
  // |+-------- Emphasize green (red on PAL/Dendy)
  // +--------- Emphasize blue
  pub struct PPUMASK: u8 {
    const GRAYSCALE         = 0b00000001;
    const LEFT_BACKGROUND   = 0b00000010;
    const LEFT_SPRITE       = 0b00000100;
    const RENDER_BACKGROUND = 0b00001000;
    const RENDER_SPRITE     = 0b00010000;
    const EMPHASIS_RED      = 0b00100000;
    const EMPHASIS_GREEN    = 0b01000000;
    const EMPHASIS_BLUE     = 0b10000000;
  }
}

bitflags! {
  // 7  bit  0
  // ---- ----
  // VSOx xxxx
  // |||| ||||
  // |||+-++++- (PPU open bus or 2C05 PPU identifier)
  // ||+------- Sprite overflow flag
  // |+-------- Sprite 0 hit flag
  // +--------- Vblank flag, cleared on read. Unreliable; see below.
  pub struct PPUSTATUS: u8 {
    const OPEN_0          = 0b00000001;
    const OPEN_1          = 0b00000010;
    const OPEN_2          = 0b00000100;
    const OPEN_3          = 0b00001000;
    const OPEN_4          = 0b00010000;
    const SPRITE_OVERFLOW = 0b00100000;
    const SPRITE_0_HIT    = 0b01000000;
    const VBLANK          = 0b10000000;
  }
}

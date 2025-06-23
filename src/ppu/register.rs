use bitflags::bitflags;

pub struct AddressRegister(u8, u8);

impl AddressRegister {
  pub fn new() -> Self {
    AddressRegister(0, 0)
  }
  fn set(&mut self, data: u16) {
    self.0 = (data >> 8) as u8;
    self.1 = (data & 0xff) as u8;
  }

  pub fn update(&mut self, data: u8, w_flag: &mut bool) {
    if *w_flag {
      self.1 = data;
    } else {
      self.0 = data;
    }

    if self.get() > 0x3fff {
      self.set(self.get() & 0b11111111111111);
    }
    *w_flag = !*w_flag;
  }

  pub fn increment(&mut self, inc: u8) {
    let (value, overflow) = self.1.overflowing_add(inc);
    self.1 = value;
    if overflow {
      self.0 = self.0.wrapping_add(1);
    }
    if self.get() > 0x3fff {
      self.set(self.get() & 0x3fff); //mirror down addr above 0x3fff
    }
  }

  pub fn get(&self) -> u16 {
    ((self.0 as u16) << 8) | (self.1 as u16)
  }
}

bitflags! {
  // 7  bit  0
  // ---- ----
  // VPHB SINN
  // |||| ||||
  // |||| ||++- Base nametable address
  // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
  // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
  // |||| |     (0: add 1, going across; 1: add 32, going down)
  // |||| +---- Sprite pattern table address for 8x8 sprites
  // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
  // |||+------ Background pattern table address (0: $0000; 1: $1000)
  // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
  // |+-------- PPU master/slave select
  // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
  // +--------- Generate an NMI at the start of the
  //            vertical blanking interval (0: off; 1: on)
  pub struct ControlRegister: u8 {
    const NAMETABLE1              = 0b00000001;
    const NAMETABLE2              = 0b00000010;
    const VRAM_ADD_INCREMENT      = 0b00000100;
    const SPRITE_PATTERN_ADDR     = 0b00001000;
    const BACKROUND_PATTERN_ADDR  = 0b00010000;
    const SPRITE_SIZE             = 0b00100000;
    const MASTER_SLAVE_SELECT     = 0b01000000;
    const GENERATE_NMI            = 0b10000000;
  }
}

impl ControlRegister {
  pub fn new() -> Self {
    ControlRegister::from_bits_truncate(0b00000000)
  }

  pub fn vram_addr_increment(&self) -> u8 {
    if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
      1
    } else {
      32
    }
  }

  pub fn update(&mut self, data: u8) {
    *self = ControlRegister::from_bits_truncate(data);
  }
}

pub struct PPUADDRESS(u8, u8);

impl PPUADDRESS {
  pub fn new() -> Self {
    PPUADDRESS(0, 0)
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

impl Default for PPUADDRESS {
  fn default() -> Self {
    PPUADDRESS::new()
  }
}

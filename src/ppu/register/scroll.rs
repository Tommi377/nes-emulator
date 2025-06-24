pub struct PPUSCROLL(u8, u8);

impl PPUSCROLL {
  pub fn new() -> Self {
    PPUSCROLL(0, 0)
  }

  pub fn update(&mut self, data: u8, w_flag: &mut bool) {
    if *w_flag {
      self.1 = data;
    } else {
      self.0 = data;
    }

    *w_flag = !*w_flag;
  }

  pub fn get(&self) -> u16 {
    ((self.0 as u16) << 8) | (self.1 as u16)
  }
}

impl Default for PPUSCROLL {
  fn default() -> Self {
    PPUSCROLL::new()
  }
}

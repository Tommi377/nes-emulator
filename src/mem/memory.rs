pub trait Memory {
  fn mem_read_u8(&self, addr: u16) -> u8;

  fn mem_write_u8(&mut self, addr: u16, data: u8);

  fn mem_read_u16(&self, addr: u16) -> u16 {
    let lo = self.mem_read_u8(addr) as u16;
    let hi = self.mem_read_u8(addr + 1) as u16;
    (hi << 8) | lo
  }

  fn mem_write_u16(&mut self, addr: u16, data: u16) {
    let lo = data as u8;
    let hi = (data >> 8) as u8;
    self.mem_write_u8(addr, lo);
    self.mem_write_u8(addr + 1, hi);
  }
}

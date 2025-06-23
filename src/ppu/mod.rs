use crate::{mem::rom::Mirroring, ppu::register::*};

pub mod register;

pub struct PPU {
  pub chr_rom: Vec<u8>,        // $0000–$1FFF (8KB CHR ROM)
  pub vram: [u8; 2048],        // $2000–$2FFF (2KB VRAM, mirrored to 4KB)
  pub palette_table: [u8; 32], // $3F00–$3FFF (32 bytes for palettes, mirrored)
  pub oam_data: [u8; 256],

  pub mirroring: Mirroring,

  addr: AddressRegister,
  ctrl: ControlRegister,

  // Internal registers
  v_reg: u16,  // Current VRAM address (15 bits)
  t_reg: u16,  // Temporary VRAM address (15 bits)
  x_reg: u8,   // Fine X scroll (3 bits)
  w_reg: bool, // Write toggle (0 or 1)

  internal_data_buf: u8,
}

impl PPU {
  pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    PPU {
      chr_rom: chr_rom,
      mirroring: mirroring,
      vram: [0; 2048],
      oam_data: [0; 64 * 4],
      palette_table: [0; 32],
      addr: AddressRegister::new(),
      ctrl: ControlRegister::new(),
      v_reg: 0,
      t_reg: 0,
      x_reg: 0,
      w_reg: false,
      internal_data_buf: 0,
    }
  }

  fn increment_vram_addr(&mut self) {
    self.addr.increment(self.ctrl.vram_addr_increment());
  }

  pub fn write_to_ppu_addr(&mut self, value: u8) {
    self.addr.update(value, &mut self.w_reg);
  }

  pub fn write_to_ctrl(&mut self, value: u8) {
    self.ctrl.update(value);
  }

  pub fn write_to_data(&mut self, value: u8) {
    let addr = self.addr.get();
    self.increment_vram_addr();

    match addr {
      0..=0x1fff => {
        println!("attempt to write to chr rom space {}", addr)
      }
      0x2000..=0x2fff => {
        let vram_addr = self.mirror_vram_addr(addr);
        self.vram[vram_addr as usize] = value;
      }
      0x3000..=0x3eff => panic!(
        "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
        addr
      ),
      0x3f00..=0x3fff => {
        self.palette_table[(addr - 0x3f00) as usize] = value;
      }
      _ => panic!("unexpected access to mirrored space {}", addr),
    }
  }

  pub fn read_data(&mut self) -> u8 {
    let addr = self.addr.get();
    self.increment_vram_addr();

    match addr {
      0..=0x1fff => {
        let result = self.internal_data_buf;
        self.internal_data_buf = self.chr_rom[addr as usize];
        result
      }
      0x2000..=0x2fff => {
        let result = self.internal_data_buf;
        self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
        result
      }
      0x3000..=0x3eff => panic!(
        "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
        addr
      ),
      0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
      _ => panic!("unexpected access to mirrored space {}", addr),
    }
  }

  fn mirror_vram_addr(&self, addr: u16) -> u16 {
    let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
    let vram_index = mirrored_vram - 0x2000; // to vram vector
    let name_table = vram_index / 0x400; // to the name table index
    match (&self.mirroring, name_table) {
      (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
      (Mirroring::Horizontal, 2) => vram_index - 0x400,
      (Mirroring::Horizontal, 1) => vram_index - 0x400,
      (Mirroring::Horizontal, 3) => vram_index - 0x800,
      _ => vram_index,
    }
  }
}

#[cfg(test)]
mod ppu_tests {
  use super::*;
  use crate::mem::rom::Mirroring;

  fn create_test_ppu(mirroring: Mirroring) -> PPU {
    let chr_rom = vec![0x42; 0x2000]; // 8KB CHR ROM filled with 0x42
    PPU::new(chr_rom, mirroring)
  }

  #[test]
  fn test_ppu_new() {
    let chr_rom = vec![0x12, 0x34, 0x56, 0x78];
    let ppu = PPU::new(chr_rom.clone(), Mirroring::Horizontal);

    assert_eq!(ppu.chr_rom, chr_rom);
    assert_eq!(ppu.mirroring, Mirroring::Horizontal);
    assert_eq!(ppu.vram.len(), 2048);
    assert_eq!(ppu.oam_data.len(), 256);
    assert_eq!(ppu.palette_table.len(), 32);
    assert_eq!(ppu.v_reg, 0);
    assert_eq!(ppu.t_reg, 0);
    assert_eq!(ppu.x_reg, 0);
    assert_eq!(ppu.w_reg, false);
    assert_eq!(ppu.internal_data_buf, 0);

    // Check that arrays are zero-initialized
    assert!(ppu.vram.iter().all(|&x| x == 0));
    assert!(ppu.oam_data.iter().all(|&x| x == 0));
    assert!(ppu.palette_table.iter().all(|&x| x == 0));
  }

  #[test]
  fn test_write_to_ctrl() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Test writing different control values
    ppu.write_to_ctrl(0b10110101);
    assert_eq!(ppu.ctrl.bits(), 0b10110101);

    ppu.write_to_ctrl(0x00);
    assert_eq!(ppu.ctrl.bits(), 0x00);
    ppu.write_to_ctrl(0xFF);
    assert_eq!(ppu.ctrl.bits(), 0xFF);
  }

  #[test]
  fn test_write_to_ppu_addr() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    // Address should now be 0x2000
    assert_eq!(ppu.addr.get(), 0x2000);
  }

  #[test]
  fn test_write_to_vram() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Set address to VRAM space
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    // Write data to VRAM
    ppu.write_to_data(0xAB);

    // The data should be written to VRAM at the mirrored address
    let mirrored_addr = ppu.mirror_vram_addr(0x2000);
    assert_eq!(ppu.vram[mirrored_addr as usize], 0xAB);
  }

  #[test]
  fn test_write_to_palette() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Set address to palette space
    ppu.write_to_ppu_addr(0x3F);
    ppu.write_to_ppu_addr(0x10);

    // Write data to palette
    ppu.write_to_data(0xCD);

    // Check that palette was updated
    assert_eq!(ppu.palette_table[0x10], 0xCD);
  }

  #[test]
  fn test_read_from_chr_rom() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Set address to CHR ROM space
    ppu.write_to_ppu_addr(0x00);
    ppu.write_to_ppu_addr(0x10);

    // First read should return buffered data (0)
    let first_read = ppu.read_data();
    assert_eq!(first_read, 0);

    // Second read should return the CHR ROM data
    let second_read = ppu.read_data();
    assert_eq!(second_read, 0x42); // Our test CHR ROM is filled with 0x42
  }

  #[test]
  fn test_read_from_vram() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Write some data to VRAM first
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);
    ppu.write_to_data(0xEF);

    // Reset address to read from the same location
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    // First read should return buffered data (0)
    let first_read = ppu.read_data();
    assert_eq!(first_read, 0);

    // Second read should return the VRAM data
    let second_read = ppu.read_data();
    assert_eq!(second_read, 0xEF);
  }

  #[test]
  fn test_read_from_palette() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Write to palette first
    ppu.write_to_ppu_addr(0x3F);
    ppu.write_to_ppu_addr(0x05);
    ppu.write_to_data(0x99);

    // Reset address to read from palette
    ppu.write_to_ppu_addr(0x3F);
    ppu.write_to_ppu_addr(0x05);

    // Palette reads are immediate (no buffering)
    let palette_data = ppu.read_data();
    assert_eq!(palette_data, 0x99);
  }

  #[test]
  fn test_mirror_vram_addr_vertical() {
    let ppu = create_test_ppu(Mirroring::Vertical);

    // Test vertical mirroring patterns
    // Nametable 0 (0x2000-0x23FF) -> 0x0000-0x03FF
    assert_eq!(ppu.mirror_vram_addr(0x2000), 0x0000);
    assert_eq!(ppu.mirror_vram_addr(0x23FF), 0x03FF);

    // Nametable 1 (0x2400-0x27FF) -> 0x0400-0x07FF
    assert_eq!(ppu.mirror_vram_addr(0x2400), 0x0400);
    assert_eq!(ppu.mirror_vram_addr(0x27FF), 0x07FF);

    // Nametable 2 (0x2800-0x2BFF) mirrors to 0x0000-0x03FF (same as NT0)
    assert_eq!(ppu.mirror_vram_addr(0x2800), 0x0000);
    assert_eq!(ppu.mirror_vram_addr(0x2BFF), 0x03FF);

    // Nametable 3 (0x2C00-0x2FFF) mirrors to 0x0400-0x07FF (same as NT1)
    assert_eq!(ppu.mirror_vram_addr(0x2C00), 0x0400);
    assert_eq!(ppu.mirror_vram_addr(0x2FFF), 0x07FF);
  }

  #[test]
  fn test_mirror_vram_addr_horizontal() {
    let ppu = create_test_ppu(Mirroring::Horizontal);

    // Test horizontal mirroring patterns
    // Nametable 0 (0x2000-0x23FF) -> 0x0000-0x03FF
    assert_eq!(ppu.mirror_vram_addr(0x2000), 0x0000);
    assert_eq!(ppu.mirror_vram_addr(0x23FF), 0x03FF);

    // Nametable 1 (0x2400-0x27FF) mirrors to 0x0000-0x03FF (same as NT0)
    assert_eq!(ppu.mirror_vram_addr(0x2400), 0x0000);
    assert_eq!(ppu.mirror_vram_addr(0x27FF), 0x03FF);

    // Nametable 2 (0x2800-0x2BFF) -> 0x0400-0x07FF
    assert_eq!(ppu.mirror_vram_addr(0x2800), 0x0400);
    assert_eq!(ppu.mirror_vram_addr(0x2BFF), 0x07FF);

    // Nametable 3 (0x2C00-0x2FFF) mirrors to 0x0400-0x07FF (same as NT2)
    assert_eq!(ppu.mirror_vram_addr(0x2C00), 0x0400);
    assert_eq!(ppu.mirror_vram_addr(0x2FFF), 0x07FF);
  }

  #[test]
  fn test_vram_addr_increment() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Test increment by 1 (default)
    ppu.write_to_ctrl(0x00); // VRAM increment = 1
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    let initial_addr = ppu.addr.get();
    ppu.write_to_data(0x11);
    let after_write_addr = ppu.addr.get();

    assert_eq!(after_write_addr, initial_addr + 1);

    // Test increment by 32
    ppu.write_to_ctrl(0x04); // VRAM increment = 32
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    let initial_addr = ppu.addr.get();
    ppu.write_to_data(0x22);
    let after_write_addr = ppu.addr.get();

    assert_eq!(after_write_addr, initial_addr + 32);
  }

  #[test]
  fn test_multiple_address_writes() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Test multiple address register writes
    ppu.write_to_ppu_addr(0x21);
    ppu.write_to_ppu_addr(0x34);
    assert_eq!(ppu.addr.get(), 0x2134);

    // Next write should affect high byte again
    ppu.write_to_ppu_addr(0x25);
    ppu.write_to_ppu_addr(0x67);
    assert_eq!(ppu.addr.get(), 0x2567);
  }

  #[test]
  fn test_consecutive_data_operations() {
    let mut ppu = create_test_ppu(Mirroring::Vertical);

    // Test consecutive writes
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    for i in 0..10 {
      ppu.write_to_data(i);
    }

    // Reset address and read back
    ppu.write_to_ppu_addr(0x20);
    ppu.write_to_ppu_addr(0x00);

    // First read is buffered
    let _ = ppu.read_data();

    // Next reads should return the written values
    for i in 0..10 {
      let value = ppu.read_data();
      assert_eq!(value, i);
    }
  }
}

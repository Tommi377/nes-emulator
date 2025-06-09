mod op;

use crate::{cpu::op::{AddressingMode, OP}, utils::set_bit};

pub struct CPU {
  pub pc: u16,
  pub status: u8,
  pub reg_a: u8,
  pub reg_x: u8,
  pub reg_y: u8,
  memory: [u8; 0xFFFF],
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      pc: 0,
      status: 0,
      reg_a: 0,
      reg_x: 0,
      reg_y: 0,
      memory: [0; 0xFFFF],
    }
  }
  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run();
  }

  pub fn load(&mut self, program: Vec<u8>) {
    self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program);
    self.mem_write_u16(0xFFFC, 0x8000);
  }

  pub fn reset(&mut self) {
    self.reg_a = 0;
    self.reg_x = 0;
    self.reg_y = 0;
    self.status = 0;

    self.pc = self.mem_read_u16(0xFFFC);
  }

  pub fn run(&mut self) {
    loop {
      let opcode: OP = self.mem_read_pc_u8().into();

      match opcode.op {
        "LDA" => self.lda(&opcode.mode),
        "TAX" => self.tax(),
        "INX" => self.inx(),
        "BRK" => {
          return;
        }
        _ => panic!("Unknown opcode: {} at PC: 0x{:04X}", opcode.op, self.pc),
      }
    }
  }

  fn get_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
    match addressing_mode {
      AddressingMode::Immediate => {
        self.pc += 1;
        self.pc - 1
      },
      AddressingMode::ZeroPage => self.mem_read_pc_u8() as u16,
      AddressingMode::ZeroPage_X => self.mem_read_pc_u8().wrapping_add(self.reg_x) as u16,
      AddressingMode::ZeroPage_Y => self.mem_read_pc_u8().wrapping_add(self.reg_y) as u16,
      AddressingMode::Absolute => self.mem_read_pc_u16(),
      AddressingMode::Absolute_X => self.mem_read_pc_u16().wrapping_add(self.reg_x as u16),
      AddressingMode::Absolute_Y => self.mem_read_pc_u16().wrapping_add(self.reg_y as u16),
      AddressingMode::Indirect_X => {
        let ptr = self.mem_read_pc_u8().wrapping_add(self.reg_x);
        let lo = self.mem_read_u8(ptr as u16) as u16;
        let hi = self.mem_read_u8(ptr.wrapping_add(1) as u16) as u16;
        hi << 8 | lo
      }
      AddressingMode::Indirect_Y =>  {
        let ptr = self.mem_read_pc_u8();
        let lo = self.mem_read_u8(ptr as u16) as u16;
        let hi = self.mem_read_u8((ptr).wrapping_add(1) as u16) as u16;
        let deref_base = hi << 8 | lo;
        let deref = deref_base.wrapping_add(self.reg_y as u16);
        deref
      }
      AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", addressing_mode),
    }
  }

  fn mem_read_u8(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn mem_read_pc_u8(&mut self) -> u8 {
    let value = self.mem_read_u8(self.pc);
    self.pc += 1;
    value
  }

  fn mem_write_u8(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }

  fn mem_read_u16(&self, addr: u16) -> u16 {
    let lo = self.mem_read_u8(addr) as u16;
    let hi = self.mem_read_u8(addr + 1) as u16;
    (hi << 8) | lo
  }

  fn mem_read_pc_u16(&mut self) -> u16 {
    let value = self.mem_read_u16(self.pc);
    self.pc += 2;
    value
  }


  fn mem_write_u16(&mut self, addr: u16, data: u16) {
    let lo = (data & 0b1111_1111) as u8;
    let hi = (data >> 8) as u8;
    self.mem_write_u8(addr, lo);
    self.mem_write_u8(addr + 1, hi);
  }

  fn lda(&mut self, addressing_mode: &AddressingMode) {
    let addr= self.get_address(addressing_mode);
    self.reg_a = self.mem_read_u8(addr);
    self.update_zero_and_negative_flags(self.reg_a);
  }

  fn tax(&mut self) {
    self.reg_x = self.reg_a;
    self.update_zero_and_negative_flags(self.reg_x);
  }

  fn inx(&mut self) {
    self.reg_x = self.reg_x.wrapping_add(1);
    self.update_zero_and_negative_flags(self.reg_x);
  }
  
  fn update_zero_and_negative_flags(&mut self, result: u8) {
    self.status = set_bit(self.status, StatusFlag::Zero as u8, result == 0);
    self.status = set_bit(self.status, StatusFlag::Negative as u8, result & 0b1000_0000 != 0);
  }
}

#[allow(dead_code)]
#[repr(u8)]
pub enum StatusFlag {
  Carry = 0b0000_0001,
  Zero = 0b0000_0010,
  InterruptDisable = 0b0000_0100,
  Decimal = 0b0000_1000,
  Break = 0b0001_0000,
  // Status flag 0b0010_0000 does nothing
  Overflow = 0b0100_0000,
  Negative = 0b1000_0000
}

#[cfg(test)]
mod test {
  use super::*;

  // Memory tests
  #[test]
  fn test_load_program() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xa9, 0x05, 0x00]);
    assert_eq!(&cpu.memory[0x8000..0x8003], &[0xa9, 0x05, 0x00]);
  }

  #[test]
  fn test_mem_write_and_read_u8() {
    let mut cpu = CPU::new();
    let addr: u16 = 0x0000;
    let data: u8 = 0xFF;
    assert!(cpu.memory[addr as usize] == 0);
    cpu.mem_write_u8(addr, data);
    assert!(cpu.memory[addr as usize] == data);
    let value = cpu.mem_read_u8(addr);
    assert_eq!(value, data);
  }

  #[test]
  fn test_mem_write_and_read_u16() {
    let mut cpu = CPU::new();
    let addr: u16 = 0x0000;
    let data: u16 = 0x1234;
    cpu.mem_write_u16(addr, data);
    let value = cpu.mem_read_u16(addr);
    assert_eq!(value, data);
  }

  // LDA Tests
  #[test]
  fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa5_lda_zero_page() {
      let mut cpu = CPU::new();
      cpu.mem_write_u8(0x10, 0x55);
      cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
      assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xa5_lda_zero_page_x() {
      let mut cpu = CPU::new();

      let ptr: u8 = 0x10;
      let offset: u8 = 0x10;
      let data: u8 = 0x55;

      cpu.mem_write_u8((ptr + offset) as u16, data);

      cpu.load(vec![0xb5, ptr, 0x00]);
      cpu.reset();

      cpu.reg_x = offset;
      cpu.run();

      assert_eq!(cpu.reg_a, data);
  }


  #[test]
  fn test_0xa5_lda_absolute() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1234, 0x55);
    cpu.load_and_run(vec![0xad, 0x34, 0x12, 0x00]);
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xa5_lda_absolute_x() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xbd, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_x = 0x01; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xa5_lda_absolute_y() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xb9, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_y = 0x01; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xa5_lda_indirect_x() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u16((indir_ptr + offset) as u16, ptr);
    cpu.mem_write_u8(ptr, data);

    cpu.load(vec![0xa1, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.run();

    assert_eq!(cpu.reg_a, data);
  }

  #[test]
  fn test_0xa5_lda_indirect_y() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u16(indir_ptr as u16, ptr);
    cpu.mem_write_u8(ptr + offset as u16, data);

    cpu.load(vec![0xb1, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_y = offset;
    cpu.run();

    assert_eq!(cpu.reg_a, data);
  }

  #[test]
  fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa9_lda_neg_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }

  // TAX Tests
  #[test]
  fn test_0xaa_tax_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.reg_a = 5;
    cpu.load(vec![0xaa, 0x00]);
    cpu.run();
    assert_eq!(cpu.reg_x, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }
 
   #[test]
  fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.reg_a = 0;
    cpu.load(vec![0xaa, 0x00]);
    cpu.run();
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xaa_tax_neg_flag() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.reg_a = 255;
    cpu.load(vec![0xaa, 0x00]);
    cpu.run();
    assert!(cpu.status & 0b1000_0000 != 0);
  }

  // INX Tests
  #[test]
  fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.reg_x = 0xff;
    cpu.load(vec![0xe8, 0xe8, 0x00]);
    cpu.run();
    assert_eq!(cpu.reg_x, 1)
  }

  // General Instruction tests
  #[test]
  fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.reg_x, 0xc1)
  }
}
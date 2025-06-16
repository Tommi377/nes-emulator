pub mod opcode;

use crate::{
  cpu::opcode::{OP, opcode_table::AddressingMode},
  mem::{Memory, bus::Bus},
  utils::set_bit,
};

pub struct CPU {
  pub pc: u16,
  pub status: u8,
  pub stack: u8,
  pub reg_a: u8,
  pub reg_x: u8,
  pub reg_y: u8,
  pub bus: Bus,
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      pc: 0,
      status: 0,
      reg_a: 0,
      reg_x: 0,
      reg_y: 0,
      stack: 0xFF,
      bus: Bus::new(),
    }
  }
  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run();
  }

  pub fn load(&mut self, program: Vec<u8>) {
    self.load_at(program, 0x0000);
  }

  pub fn load_at(&mut self, program: Vec<u8>, start_address: usize) {
    for i in 0..(program.len() as u16) {
      self.mem_write_u8((start_address as u16) + i, program[i as usize]);
    }
    self.mem_write_u16(0xFFFC, start_address as u16);
    self.reset();
  }

  pub fn reset(&mut self) {
    self.reg_a = 0;
    self.reg_x = 0;
    self.reg_y = 0;
    self.status = 0;

    self.pc = self.mem_read_u16(0xFFFC);
  }

  pub fn run(&mut self) {
    self.run_with_callback(|_| {});
  }

  pub fn run_with_callback<F>(&mut self, mut callback: F)
  where
    F: FnMut(&mut CPU),
  {
    while (self.status & StatusFlag::Break as u8) == 0 {
      callback(self);
      let opcode: OP = self.mem_read_pc_u8().into();
      opcode.execute(self);
    }
  }

  fn mem_read_pc_u8(&mut self) -> u8 {
    let value = self.mem_read_u8(self.pc);
    self.pc += 1;
    value
  }

  fn mem_read_pc_u16(&mut self) -> u16 {
    let value = self.mem_read_u16(self.pc);
    self.pc += 2;
    value
  }

  fn get_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
    match addressing_mode {
      AddressingMode::Immediate => {
        self.pc += 1;
        self.pc - 1
      }
      AddressingMode::ZeroPage => self.mem_read_pc_u8() as u16,
      AddressingMode::ZeroPage_X => self.mem_read_pc_u8().wrapping_add(self.reg_x) as u16,
      AddressingMode::ZeroPage_Y => self.mem_read_pc_u8().wrapping_add(self.reg_y) as u16,
      AddressingMode::Absolute => self.mem_read_pc_u16(),
      AddressingMode::Absolute_X => self.mem_read_pc_u16().wrapping_add(self.reg_x as u16),
      AddressingMode::Absolute_Y => self.mem_read_pc_u16().wrapping_add(self.reg_y as u16),
      AddressingMode::Indirect => {
        let ptr = self.mem_read_pc_u16();
        let lo = self.mem_read_u8(ptr) as u16;
        let hi = self.mem_read_u8(ptr & 0xFF00 | ((ptr as u8).wrapping_add(1) as u16)) as u16; // Replicate the page boundary bug in the original 6502
        hi << 8 | lo
      }
      AddressingMode::Indirect_X => {
        let ptr = self.mem_read_pc_u8().wrapping_add(self.reg_x);
        let lo = self.mem_read_u8(ptr as u16) as u16;
        let hi = self.mem_read_u8(ptr.wrapping_add(1) as u16) as u16;
        hi << 8 | lo
      }
      AddressingMode::Indirect_Y => {
        let ptr = self.mem_read_pc_u8();
        let lo = self.mem_read_u8(ptr as u16) as u16;
        let hi = self.mem_read_u8((ptr).wrapping_add(1) as u16) as u16;
        let deref_base = hi << 8 | lo;
        let deref = deref_base.wrapping_add(self.reg_y as u16);
        deref
      }
      AddressingMode::Accumulator => panic!("mode {:?} is not an address", addressing_mode),
      _ => panic!("mode {:?} is not supported", addressing_mode),
    }
  }

  fn get_address_and_value(&mut self, mode: &AddressingMode) -> (u16, u8) {
    let address = self.get_address(&mode);
    let value = self.mem_read_u8(address);
    (address, value)
  }

  fn get_stack_address(&self) -> u16 {
    0x0100 | self.stack as u16
  }

  fn stack_push_value_u8(&mut self, value: u8) {
    self.mem_write_u8(self.get_stack_address(), value);
    self.stack = self.stack.wrapping_sub(1);
  }

  fn stack_pull_value_u8(&mut self) -> u8 {
    self.stack = self.stack.wrapping_add(1);
    self.mem_read_u8(self.get_stack_address())
  }

  fn stack_push_value_u16(&mut self, value: u16) {
    let [lo, hi] = value.to_le_bytes();
    self.stack_push_value_u8(hi);
    self.stack_push_value_u8(lo);
  }

  fn stack_pull_value_u16(&mut self) -> u16 {
    let lo = self.stack_pull_value_u8();
    let hi = self.stack_pull_value_u8();
    u16::from_le_bytes([lo, hi])
  }

  fn get_flag(&self, flag: StatusFlag) -> bool {
    (self.status & (flag as u8)) != 0
  }

  fn set_flag(&mut self, flag: StatusFlag, value: bool) {
    self.status &= !(flag as u8);
    if value {
      self.status |= flag as u8;
    }
  }

  fn branch(&mut self, condition: bool) {
    let offset = self.mem_read_pc_u8() as i8;
    if condition {
      let jump_addr = self.pc.wrapping_add(offset as u16);
      self.pc = jump_addr;
    }
  }

  fn update_zero_and_negative_flags(&mut self, result: u8) {
    self.status = set_bit(self.status, StatusFlag::Zero as u8, result == 0);
    self.status = set_bit(
      self.status,
      StatusFlag::Negative as u8,
      result & 0b1000_0000 != 0,
    );
  }
}

impl Memory for CPU {
  fn mem_read_u8(&self, addr: u16) -> u8 {
    self.bus.mem_read_u8(addr)
  }

  fn mem_write_u8(&mut self, addr: u16, data: u8) {
    self.bus.mem_write_u8(addr, data)
  }
  fn mem_read_u16(&self, pos: u16) -> u16 {
    self.bus.mem_read_u16(pos)
  }

  fn mem_write_u16(&mut self, pos: u16, data: u16) {
    self.bus.mem_write_u16(pos, data)
  }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum StatusFlag {
  Carry = 0b0000_0001,
  Zero = 0b0000_0010,
  InterruptDisable = 0b0000_0100,
  Decimal = 0b0000_1000,
  Break = 0b0001_0000,
  // Status flag 0b0010_0000 does nothing
  Overflow = 0b0100_0000,
  Negative = 0b1000_0000,
}

#[cfg(test)]
mod memory_test {
  use super::*;

  // Memory tests
  #[test]
  fn test_load_program() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.mem_read_u8(0x0000), 0xa9);
    assert_eq!(cpu.mem_read_u8(0x0001), 0x05);
    assert_eq!(cpu.mem_read_u8(0x0002), 0x00);
  }

  #[test]
  fn test_mem_write_and_read_u8() {
    let mut cpu = CPU::new();
    let addr: u16 = 0x0000;
    let data: u8 = 0xFF;
    assert_eq!(cpu.mem_read_u8(addr), 0);
    cpu.mem_write_u8(addr, data);
    assert_eq!(cpu.mem_read_u8(addr), data);
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

  // General Instruction tests
  #[test]
  fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.reg_x, 0xc1)
  }
}

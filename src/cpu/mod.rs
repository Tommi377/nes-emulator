pub mod opcode;

use std::fmt::{Debug, Formatter};

use crate::{
  cpu::opcode::{OP, opcode_table::AddressingMode},
  mem::{Memory, bus::Bus, rom::Rom},
  utils::set_bit,
};

const INIT_STACK_POINTER: u8 = 0xFF;
const PC_START_ADDRESS: u16 = 0xFFFC;

pub struct CPU {
  pub pc: u16,
  pub status: u8,
  pub stack: u8,
  pub reg_a: u8,
  pub reg_x: u8,
  pub reg_y: u8,
  pub bus: Bus,
}

impl Default for CPU {
  fn default() -> Self {
    Self::new()
  }
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      pc: 0,
      status: 0b00100100,
      reg_a: 0,
      reg_x: 0,
      reg_y: 0,
      stack: INIT_STACK_POINTER,
      bus: Bus::new(),
    }
  }
  pub fn load_and_run(&mut self, ram: Vec<u8>) {
    self.load(ram);
    self.reset();
    self.run();
  }

  pub fn load(&mut self, ram: Vec<u8>) {
    self.insert_rom(Rom::from_pc(0x0000));
    self.load_at(ram, 0x0000);
  }

  pub fn load_at(&mut self, program: Vec<u8>, start_address: usize) {
    for i in 0..(program.len() as u16) {
      self.mem_write_u8((start_address as u16) + i, program[i as usize]);
    }
    self.reset();
  }

  pub fn insert_rom(&mut self, rom: Rom) {
    self.bus.insert_rom(rom);
  }

  pub fn reset(&mut self) {
    self.reg_a = 0;
    self.reg_x = 0;
    self.reg_y = 0;
    self.status = 0b00100100;
    self.stack = INIT_STACK_POINTER;

    self.pc = self.mem_read_u16(PC_START_ADDRESS);
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

        deref_base.wrapping_add(self.reg_y as u16)
      }
      AddressingMode::Accumulator => panic!("mode {:?} is not an address", addressing_mode),
      _ => panic!("mode {:?} is not supported", addressing_mode),
    }
  }

  fn try_get_address(&mut self, mode: &AddressingMode) -> Option<u16> {
    match mode {
      AddressingMode::Relative | AddressingMode::NoneAddressing | AddressingMode::Accumulator => {
        None
      }
      _ => Some(self.get_address(mode)),
    }
  }

  fn get_address_and_value(&mut self, mode: &AddressingMode) -> (u16, u8) {
    let address = self.get_address(mode);
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

impl Debug for CPU {
  fn fmt(&self, f: &mut Formatter<'_>) -> ::core::fmt::Result {
    let op: OP = self.mem_read_u8(self.pc).into();

    let pc_str = format!("{:04X}", self.pc);

    let instructions = (0..op.bytes)
      .map(|i| self.mem_read_u8(self.pc.wrapping_add(i as u16)))
      .collect::<Vec<u8>>();

    let code_str = instructions
      .iter()
      .map(|byte| format!("{:02X}", byte))
      .collect::<Vec<String>>()
      .join(" ");

    let ins_str = format!(
      "{: >4} {}",
      op.name,
      match op.mode {
        AddressingMode::Immediate => format!("#${:02X}", instructions[1]),
        AddressingMode::ZeroPage => format!(
          "${:02X} = {:02X}",
          instructions[1],
          self.mem_read_u8(instructions[1] as u16)
        ),
        AddressingMode::ZeroPage_X => {
          let addr = instructions[1].wrapping_add(self.reg_x);
          format!(
            "${:02X},X @ {:02X} = {:02X}",
            instructions[1],
            addr,
            self.mem_read_u8(addr as u16)
          )
        }
        AddressingMode::ZeroPage_Y => {
          let addr = instructions[1].wrapping_add(self.reg_y);
          format!(
            "${:02X},Y @ {:02X} = {:02X}",
            instructions[1],
            addr,
            self.mem_read_u8(addr as u16)
          )
        }
        AddressingMode::Absolute => {
          if op.name == "JMP" || op.name == "JSR" {
            format!(
              "${:04X}",
              u16::from_le_bytes([instructions[1], instructions[2]])
            )
          } else {
            let addr = u16::from_le_bytes([instructions[1], instructions[2]]);
            format!("${:04X} = {:02X}", addr, self.mem_read_u8(addr))
          }
        }
        AddressingMode::Absolute_X => {
          let addr = u16::from_le_bytes([instructions[1], instructions[2]]);
          let addr_final = addr.wrapping_add(self.reg_x as u16);
          format!(
            "${:04X},X @ {:04X} = {:02X}",
            addr,
            addr_final,
            self.mem_read_u8(addr_final)
          )
        }
        AddressingMode::Absolute_Y => {
          let addr = u16::from_le_bytes([instructions[1], instructions[2]]);
          let addr_final = addr.wrapping_add(self.reg_y as u16);
          format!(
            "${:04X},Y @ {:04X} = {:02X}",
            addr,
            addr_final,
            self.mem_read_u8(addr_final)
          )
        }
        AddressingMode::Indirect => {
          let ptr = u16::from_le_bytes([instructions[1], instructions[2]]);
          let lo = self.mem_read_u8(ptr) as u16;
          let hi = self.mem_read_u8(ptr & 0xFF00 | ((ptr as u8).wrapping_add(1) as u16)) as u16; // Replicate the page boundary bug in the original 6502
          let ptr_2 = hi << 8 | lo;
          format!("(${:04X}) = {:04X}", ptr, ptr_2,)
        }
        AddressingMode::Indirect_X => {
          let ptr = instructions[1].wrapping_add(self.reg_x);
          let lo = self.mem_read_u8(ptr as u16) as u16;
          let hi = self.mem_read_u8(ptr.wrapping_add(1) as u16) as u16;
          let ptr_final = hi << 8 | lo;
          format!(
            "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
            instructions[1],
            ptr,
            ptr_final,
            self.mem_read_u8(ptr_final),
          )
        }
        AddressingMode::Indirect_Y => {
          let lo = self.mem_read_u8(instructions[1] as u16) as u16;
          let hi = self.mem_read_u8(instructions[1].wrapping_add(1) as u16) as u16;
          let ptr = hi << 8 | lo;
          let ptr_final = ptr.wrapping_add(self.reg_y as u16);
          format!(
            "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
            instructions[1],
            ptr,
            ptr_final,
            self.mem_read_u8(ptr_final),
          )
        }
        AddressingMode::Relative => {
          let offset = instructions[1] as i8;
          let jump_addr = self.pc.wrapping_add(offset as u16 + 2);
          format!("${:04X}", jump_addr)
        }
        AddressingMode::Accumulator => "A".to_string(),
        _ => "".to_string(),
      }
    );

    let reg_str = format!(
      "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
      self.reg_a, self.reg_x, self.reg_y, self.status, self.stack
    );

    write!(f, "{:5} {:8} {:32} {}", pc_str, code_str, ins_str, reg_str)
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

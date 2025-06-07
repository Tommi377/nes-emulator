use num_enum::TryFromPrimitive;

use crate::utils::set_bit;

pub struct CPU {
  pub pc: u8,
  pub status: u8,
  pub reg_a: u8,
  pub reg_x: u8,
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      pc: 0,
      status: 0,
      reg_a: 0,
      reg_x: 0,
    }
  }
  
  pub fn interpret(&mut self, program: Vec<u8>) {
    self.pc = 0;
    
    loop {
      let opcode_number = self.get_and_increment_pc(&program);
      let opcode = OpCode::try_from(opcode_number).unwrap();


      match opcode {
        OpCode::LDA_IMMEDIATE => {
          let param = self.get_and_increment_pc(&program);
          self.lda(param);
        }
        OpCode::TAX_IMPLIED => {
          self.tax();
        }
        OpCode::BRK_IMPLIED => {
          return;
        }
        _ => todo!()
      }
    }
  }

  fn lda(&mut self, value: u8) {
    self.reg_a = value;
    self.update_zero_and_negative_flags(self.reg_a);
  }

  fn tax(&mut self) {
    self.reg_x = self.reg_a;
    self.update_zero_and_negative_flags(self.reg_x);
  }
  
  fn update_zero_and_negative_flags(&mut self, result: u8) {
    self.status = set_bit(self.status, StatusFlag::Zero as u8, result == 0);
    self.status = set_bit(self.status, StatusFlag::Negative as u8, result & 0b1000_0000 != 0);
  }

  fn get_and_increment_pc(&mut self, program: &Vec<u8>) -> u8 {
    let value = program[self.pc as usize];
    self.pc += 1;
    value
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa9_lda_neg_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }

  #[test]
  fn test_0xaa_tax_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.reg_a = 5;
    cpu.interpret(vec![0xaa, 0x00]);
    assert_eq!(cpu.reg_x, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }
 
   #[test]
  fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0;
    cpu.interpret(vec![0xaa, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xaa_tax_neg_flag() {
    let mut cpu = CPU::new();
    cpu.reg_a = 255;
    cpu.interpret(vec![0xaa, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
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

#[allow(non_camel_case_types)]
#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
  LDA_IMMEDIATE = 0xA9,
  TAX_IMPLIED = 0xAA,
  INX_IMPLIED = 0xE8,
  BRK_IMPLIED = 0x00,
}
use crate::{cpu::{StatusFlag, CPU}, utils::set_bit};

#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub static OPCODE_TABLE: [Option<OP>; 256] = {
  use AddressingMode::*;

  let mut table: [Option<OP>; 256] = [None; 256];
  table[0xA9] = Some(OP { code: 0xA9, op: "LDA", mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA5] = Some(OP { code: 0xA5, op: "LDA", mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB5] = Some(OP { code: 0xB5, op: "LDA", mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAD] = Some(OP { code: 0xAD, op: "LDA", mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBD] = Some(OP { code: 0xBD, op: "LDA", mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xB9] = Some(OP { code: 0xB9, op: "LDA", mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA1] = Some(OP { code: 0xA1, op: "LDA", mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB1] = Some(OP { code: 0xB1, op: "LDA", mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });
  table[0xAA] = Some(OP { code: 0xAA, op: "TAX", mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xE8] = Some(OP { code: 0xE8, op: "INX", mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x00] = Some(OP { code: 0x00, op: "BRK", mode: NoneAddressing,  bytes: 1, cycles: 7 });
  table
};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct OP {
  pub code: u8,
  pub op: &'static str,
  pub mode: AddressingMode,
  pub bytes: u8,
  pub cycles: u8
}

impl OP {
  pub fn execute(&self, cpu: &mut CPU) {
    match self.op {
      "LDA" => Self::lda(cpu, &self.mode),
      "TAX" => Self::tax(cpu),
      "INX" => Self::inx(cpu),
      "BRK" => Self::brk(cpu),
      _ => panic!("Unknown opcode: {} at PC: 0x{:04X}", self.op, cpu.pc),
    }
  }


  fn lda(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let addr= cpu.get_address(addressing_mode);
    cpu.reg_a = cpu.mem_read_u8(addr);
    cpu.update_zero_and_negative_flags(cpu.reg_a);
  }

  fn sta(cpu: &mut CPU, mode: &AddressingMode) {
    let addr = cpu.get_address(mode);
    cpu.mem_write_u8(addr, cpu.reg_a);
  }

  fn tax(cpu: &mut CPU) {
    cpu.reg_x = cpu.reg_a;
    cpu.update_zero_and_negative_flags(cpu.reg_x);
  }

  fn inx(cpu: &mut CPU) {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    cpu.update_zero_and_negative_flags(cpu.reg_x);
  }

  fn brk(cpu: &mut CPU) {
    cpu.status = set_bit(cpu.status, StatusFlag::Break as u8, true);
  }
}

impl From<u8> for OP {
  fn from(value: u8) -> Self {
    OPCODE_TABLE[value as usize].unwrap_or_else(|| {
      panic!("Opcode 0x{:02X} not found in opcode table", value);
    })
  }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

#[cfg(test)]
mod test {
  use super::*;

  // LDA Tests
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
}
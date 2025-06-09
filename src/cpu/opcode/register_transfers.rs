use crate::cpu::{opcode::{opcode_table::AddressingMode}, CPU};

pub(crate) fn tax(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_x = cpu.reg_a;
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}

pub(crate) fn tay(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_y = cpu.reg_a;
  cpu.update_zero_and_negative_flags(cpu.reg_y);
}

pub(crate) fn tsx(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_x = cpu.reg_sp;
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}

pub(crate) fn txa(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_a = cpu.reg_x;
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn txs(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_sp = cpu.reg_x;
  cpu.update_zero_and_negative_flags(cpu.reg_sp);
}

pub(crate) fn tya(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_a = cpu.reg_y;
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

#[cfg(test)]
mod transfer_test {
  use super::*;
  #[test]
  fn test_0xaa_tax_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();
  
    cpu.reg_a = 5;
    cpu.run();

    assert_eq!(cpu.reg_x, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();

    cpu.reg_a = 0;
    cpu.run();
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xaa_tax_neg_flag() {
    let mut cpu = CPU::new();
  
    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();

    cpu.reg_a = 255;
    cpu.run();
    assert!(cpu.status & 0b1000_0000 != 0);
  }

  #[test]
  fn test_0xa8_tay_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xa8, 0x00]);
    cpu.reset();

    cpu.reg_a = 5;
    cpu.run();
  
    assert_eq!(cpu.reg_y, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }
  
  #[test]
  fn test_0xba_tsx_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xba, 0x00]);
    cpu.reset();

    cpu.reg_sp = 5;
    cpu.run();
  
    assert_eq!(cpu.reg_x, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0x8a_txa_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8a, 0x00]);
    cpu.reset();

    cpu.reg_x = 5;
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0x9a_txs_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x9a, 0x00]);
    cpu.reset();

    cpu.reg_x = 5;
    cpu.run();
  
    assert_eq!(cpu.reg_sp, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0x98_tya_transfer() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x98, 0x00]);
    cpu.reset();

    cpu.reg_y = 5;
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }
}
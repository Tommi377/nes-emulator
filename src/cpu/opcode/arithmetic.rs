use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

pub(crate) fn adc(cpu: &mut CPU, mode: AddressingMode) {
  let address = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(address);
  cpu_addition_with_carry(cpu, value);
}

pub(crate) fn sbc(cpu: &mut CPU, mode: AddressingMode) {
  let address = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(address);
  let complement_value = value ^ 0xFF;
  cpu_addition_with_carry(cpu, complement_value);
}

fn cpu_addition_with_carry(cpu: &mut CPU, value: u8) {
  let carry_in = if cpu.get_flag(StatusFlag::Carry) {
    1u8
  } else {
    0
  };

  let (temp_result, temp_carry) = cpu.reg_a.overflowing_add(value);
  let (result, carry_from_carry) = temp_result.overflowing_add(carry_in);
  let carry_flag = temp_carry || carry_from_carry;

  let overflow_flag = (value ^ result) & (cpu.reg_a ^ result) & 0b1000_0000 != 0;

  cpu.reg_a = result;

  cpu.status &= !(StatusFlag::Carry as u8 | StatusFlag::Overflow as u8);
  if carry_flag {
    cpu.status |= StatusFlag::Carry as u8;
  }
  if overflow_flag {
    cpu.status |= StatusFlag::Overflow as u8;
  }
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

#[cfg(test)]
mod adc_tests {
  use super::*;
  use crate::cpu::StatusFlag;

  #[test]
  fn test_adc_basic_addition() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x30;
    cpu.mem_write_u8(0x10, 0x30);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10); // Address for zero page mode

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x60);
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_adc_with_carry_in() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x50;
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag
    cpu.mem_write_u8(0x10, 0x30);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x81); // 0x50 + 0x30 + 1 = 0x81
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_adc_carry_out() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0xFF;
    cpu.mem_write_u8(0x10, 0x01);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00); // 0xFF + 0x01 = 0x100, result is 0x00
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero should be set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_adc_overflow_positive() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x7F; // +127
    cpu.mem_write_u8(0x10, 0x01); // +1
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x80); // Result is -128 in two's complement
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0); // Overflow should be set
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0);
  }

  #[test]
  fn test_adc_overflow_negative() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x80; // -128
    cpu.mem_write_u8(0x10, 0xFF); // -1
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x7F); // Result is +127
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0); // Overflow should be set
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (0x80 + 0xFF = 0x17F)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_adc_no_overflow_different_signs() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x80; // -128
    cpu.mem_write_u8(0x10, 0x7F); // +127
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    adc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0xFF); // -1
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0); // No overflow when adding different signs
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0);
  }

  #[test]
  fn test_adc_immediate_mode() {
    let mut cpu = crate::cpu::CPU::new();
    cpu.reg_a = 0x10;
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x20); // Immediate value

    adc(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0x30);
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }
}

#[cfg(test)]
mod sbc_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

  #[test]
  fn test_sbc_basic_subtraction() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x50; // 80
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0x30); // 48
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10); // Address for zero page mode

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x20); // 80 - 48 = 32
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_sbc_with_borrow() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x50; // 80
    cpu.status &= !(StatusFlag::Carry as u8); // Clear carry flag (borrow)
    cpu.mem_write_u8(0x10, 0x30); // 48
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x1F); // 80 - 48 - 1 = 31
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow occurred)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_sbc_underflow() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x30; // 48
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0x50); // 80
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0xE0); // 48 - 80 = -32 (wraps to 224)
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be clear (borrow occurred)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative result
  }

  #[test]
  fn test_sbc_zero_result() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x50; // 80
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0x50); // 80
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00); // 80 - 80 = 0
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag should be set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_sbc_overflow_positive_to_negative() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x7F; // +127
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0xFF); // -1 (255)
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x80); // +127 - (-1) = +128, but wraps to -128
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be clear (borrow occurred)
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0); // Overflow should be set
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative result
  }

  #[test]
  fn test_sbc_overflow_negative_to_positive() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x80; // -128
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0x01); // +1
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x7F); // -128 - 1 = -129, but wraps to +127
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0); // Overflow should be set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Positive result
  }

  #[test]
  fn test_sbc_no_overflow_same_signs() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x50; // +80
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0x30); // +48
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x20); // +80 - 48 = +32
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0); // No overflow
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Positive result
  }

  #[test]
  fn test_sbc_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x30; // 48
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10); // Immediate value 16

    sbc(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0x20); // 48 - 16 = 32
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_sbc_with_borrow_chain() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x00; // 0
    cpu.status &= !(StatusFlag::Carry as u8); // Clear carry flag (borrow)
    cpu.mem_write_u8(0x10, 0x01); // 1
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0xFE); // 0 - 1 - 1 = -2 (wraps to 254)
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be clear (borrow occurred)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative result
  }

  #[test]
  fn test_sbc_boundary_values() {
    let mut cpu = CPU::new();

    // Test subtracting 0 with carry set
    cpu.reg_a = 0x42;
    cpu.status |= StatusFlag::Carry as u8;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x42); // 66 - 0 = 66
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set

    // Reset for next test
    cpu.pc = 0x8000;

    // Test subtracting 0 with carry clear (borrow)
    cpu.reg_a = 0x42;
    cpu.status &= !(StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x41); // 66 - 0 - 1 = 65
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set
  }

  #[test]
  fn test_sbc_max_values() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0xFF; // 255
    cpu.status |= StatusFlag::Carry as u8; // Set carry flag (no borrow)
    cpu.mem_write_u8(0x10, 0xFF); // 255
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    sbc(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00); // 255 - 255 = 0
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry should be set (no borrow)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag should be set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }
}

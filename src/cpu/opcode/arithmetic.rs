use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

pub(crate) fn adc(cpu: &mut CPU, mode: AddressingMode) {
  let (_, value) = cpu.get_address_and_value(&mode);
  cpu_addition_with_carry(cpu, value);
}

pub(crate) fn sbc(cpu: &mut CPU, mode: AddressingMode) {
  let (_, value) = cpu.get_address_and_value(&mode);
  let complement_value = value ^ 0xFF;
  cpu_addition_with_carry(cpu, complement_value);
}

pub(crate) fn cmp(cpu: &mut CPU, mode: AddressingMode) {
  let (_, value) = cpu.get_address_and_value(&mode);
  cpu.set_flag(StatusFlag::Carry, cpu.reg_a >= value);
  cpu.set_flag(StatusFlag::Zero, cpu.reg_a == value);
  cpu.set_flag(
    StatusFlag::Negative,
    cpu.reg_a.wrapping_sub(value) & 0b1000_0000 != 0,
  );
}

pub(crate) fn cpx(cpu: &mut CPU, mode: AddressingMode) {
  let (_, value) = cpu.get_address_and_value(&mode);
  cpu.set_flag(StatusFlag::Carry, cpu.reg_x >= value);
  cpu.set_flag(StatusFlag::Zero, cpu.reg_x == value);
  cpu.set_flag(
    StatusFlag::Negative,
    cpu.reg_x.wrapping_sub(value) & 0b1000_0000 != 0,
  );
}

pub(crate) fn cpy(cpu: &mut CPU, mode: AddressingMode) {
  let (_, value) = cpu.get_address_and_value(&mode);
  cpu.set_flag(StatusFlag::Carry, cpu.reg_y >= value);
  cpu.set_flag(StatusFlag::Zero, cpu.reg_y == value);
  cpu.set_flag(
    StatusFlag::Negative,
    cpu.reg_y.wrapping_sub(value) & 0b1000_0000 != 0,
  );
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

#[cfg(test)]
mod cmp_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

  #[test]
  fn test_cmp_equal_values() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x50;
    cpu.mem_write_u8(0x10, 0x50);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x50); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_a >= value)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set (reg_a == value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (result is 0)
  }

  #[test]
  fn test_cmp_reg_a_greater() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x80;
    cpu.mem_write_u8(0x10, 0x30);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x80); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_a >= value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_a != value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (positive result)
  }

  #[test]
  fn test_cmp_reg_a_less() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x30;
    cpu.mem_write_u8(0x10, 0x80);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x30); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (reg_a < value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_a != value)
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x30 - 0x80 = 0xB0)
  }

  #[test]
  fn test_cmp_boundary_cases() {
    let mut cpu = CPU::new();

    // Test comparing 0x00 with 0x00
    cpu.reg_a = 0x00;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear

    // Test comparing 0xFF with 0xFF
    cpu.reg_a = 0xFF;
    cpu.mem_write_u8(0x10, 0xFF);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cmp_wrap_around() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x00;
    cpu.mem_write_u8(0x10, 0x01);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cmp(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (0 < 1)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x00 - 0x01 = 0xFF)
  }

  #[test]
  fn test_cmp_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x42;
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x42);

    cmp(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0x42); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }
}

#[cfg(test)]
mod cpx_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

  #[test]
  fn test_cpx_equal_values() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0x50;
    cpu.mem_write_u8(0x10, 0x50);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_x, 0x50); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_x >= value)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set (reg_x == value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (result is 0)
  }

  #[test]
  fn test_cpx_reg_x_greater() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0x80;
    cpu.mem_write_u8(0x10, 0x30);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_x, 0x80); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_x >= value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_x != value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (positive result)
  }

  #[test]
  fn test_cpx_reg_x_less() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0x30;
    cpu.mem_write_u8(0x10, 0x80);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_x, 0x30); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (reg_x < value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_x != value)
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x30 - 0x80 = 0xB0)
  }

  #[test]
  fn test_cpx_boundary_cases() {
    let mut cpu = CPU::new();

    // Test comparing 0x00 with 0x00
    cpu.reg_x = 0x00;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear

    // Test comparing 0xFF with 0xFF
    cpu.reg_x = 0xFF;
    cpu.mem_write_u8(0x10, 0xFF);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cpx_wrap_around() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0x00;
    cpu.mem_write_u8(0x10, 0x01);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_x, 0x00); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (0 < 1)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x00 - 0x01 = 0xFF)
  }

  #[test]
  fn test_cpx_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_x = 0x42;
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x42);

    cpx(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_x, 0x42); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cpx_different_from_reg_a() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x10; // Different from reg_x
    cpu.reg_x = 0x20;
    cpu.mem_write_u8(0x10, 0x15);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpx(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x10); // reg_a should not change
    assert_eq!(cpu.reg_x, 0x20); // reg_x should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (0x20 >= 0x15)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }
}

#[cfg(test)]
mod cpy_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

  #[test]
  fn test_cpy_equal_values() {
    let mut cpu = CPU::new();
    cpu.reg_y = 0x50;
    cpu.mem_write_u8(0x10, 0x50);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0x50); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_y >= value)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set (reg_y == value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (result is 0)
  }

  #[test]
  fn test_cpy_reg_y_greater() {
    let mut cpu = CPU::new();
    cpu.reg_y = 0x80;
    cpu.mem_write_u8(0x10, 0x30);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0x80); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (reg_y >= value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_y != value)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (positive result)
  }

  #[test]
  fn test_cpy_reg_y_less() {
    let mut cpu = CPU::new();
    cpu.reg_y = 0x30;
    cpu.mem_write_u8(0x10, 0x80);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0x30); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (reg_y < value)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear (reg_y != value)
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x30 - 0x80 = 0xB0)
  }

  #[test]
  fn test_cpy_boundary_cases() {
    let mut cpu = CPU::new();

    // Test comparing 0x00 with 0x00
    cpu.reg_y = 0x00;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear

    // Test comparing 0xFF with 0xFF
    cpu.reg_y = 0xFF;
    cpu.mem_write_u8(0x10, 0xFF);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cpy_wrap_around() {
    let mut cpu = CPU::new();
    cpu.reg_y = 0x00;
    cpu.mem_write_u8(0x10, 0x01);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0x00); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (0 < 1)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0x00 - 0x01 = 0xFF)
  }

  #[test]
  fn test_cpy_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_y = 0x42;
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x42);

    cpy(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_y, 0x42); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cpy_different_from_other_regs() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x10; // Different from reg_y
    cpu.reg_x = 0x15; // Different from reg_y
    cpu.reg_y = 0x20;
    cpu.mem_write_u8(0x10, 0x15);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x10); // reg_a should not change
    assert_eq!(cpu.reg_x, 0x15); // reg_x should not change
    assert_eq!(cpu.reg_y, 0x20); // reg_y should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (0x20 >= 0x15)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear
  }

  #[test]
  fn test_cpy_extreme_values() {
    let mut cpu = CPU::new();

    // Test 0xFF compared to 0x00 (maximum vs minimum)
    cpu.reg_y = 0xFF;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0xFF); // Register should not change
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry set (0xFF >= 0x00)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative set (0xFF - 0x00 = 0xFF)

    // Test 0x00 compared to 0xFF (minimum vs maximum)
    cpu.reg_y = 0x00;
    cpu.mem_write_u8(0x10, 0xFF);
    cpu.pc = 0x8000;
    cpu.mem_write_u8(0x8000, 0x10);

    cpy(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_y, 0x00); // Register should not change
    assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Carry clear (0x00 < 0xFF)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Zero clear
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative clear (0x00 - 0xFF = 0x01)
  }
}

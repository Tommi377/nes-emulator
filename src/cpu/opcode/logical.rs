use crate::{
  bus::memory::Memory,
  cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode},
};

pub(crate) fn and(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr);
  cpu.reg_a &= value;
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn eor(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr);
  cpu.reg_a ^= value;
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn ora(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr);
  cpu.reg_a |= value;
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn bit(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr);
  let result = cpu.reg_a & value;

  cpu.status &= !(StatusFlag::Zero as u8 | StatusFlag::Overflow as u8 | StatusFlag::Negative as u8);
  if result == 0 {
    cpu.status |= StatusFlag::Zero as u8;
  }
  if value & 0b0100_0000 != 0 {
    cpu.status |= StatusFlag::Overflow as u8;
  }
  if value & 0b1000_0000 != 0 {
    cpu.status |= StatusFlag::Negative as u8;
  }
}

#[cfg(test)]
mod logical_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag};

  // AND Tests
  #[test]
  fn test_and_basic_operation() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b1010_1010);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10); // Zero page address

    and(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0b1010_0000); // Result of AND operation
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // Not zero
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative (bit 7 set)
  }

  #[test]
  fn test_and_zero_result() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b0000_1111);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    and(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00);
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Negative flag clear
  }

  #[test]
  fn test_and_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1100_1100;
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0b1010_1010); // Immediate value

    and(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0b1000_1000);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_and_preserves_other_flags() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x55;
    cpu.status = StatusFlag::Carry as u8 | StatusFlag::Overflow as u8; // Set other flags
    cpu.mem_write_u8(0x10, 0xAA);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    and(&mut cpu, AddressingMode::ZeroPage);

    // Check that carry and overflow flags are preserved
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  // EOR Tests
  #[test]
  fn test_eor_basic_operation() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b1010_1010);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    eor(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0b0101_1010); // Result of XOR operation
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_eor_same_value_gives_zero() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1010_1010;
    cpu.mem_write_u8(0x10, 0b1010_1010);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    eor(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00);
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_eor_negative_result() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b0111_1111;
    cpu.mem_write_u8(0x10, 0b1111_1111);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    eor(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0b1000_0000);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative flag set
  }

  #[test]
  fn test_eor_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0xFF;
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x0F); // Immediate value

    eor(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0xF0);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  // ORA Tests
  #[test]
  fn test_ora_basic_operation() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b0000_1111);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    ora(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0b1111_1111); // Result of OR operation
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative flag set
  }

  #[test]
  fn test_ora_zero_with_zero() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x00;
    cpu.mem_write_u8(0x10, 0x00);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    ora(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x00);
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_ora_immediate_mode() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1010_1010;
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0b0101_0101); // Immediate value

    ora(&mut cpu, AddressingMode::Immediate);

    assert_eq!(cpu.reg_a, 0b1111_1111);
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  #[test]
  fn test_ora_no_change() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_1111;
    cpu.mem_write_u8(0x10, 0b1010_1010);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    ora(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0b1111_1111); // No change since all bits already set
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
  }

  // BIT Tests
  #[test]
  fn test_bit_basic_operation() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b1010_1010);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    bit(&mut cpu, AddressingMode::ZeroPage);

    // Accumulator should not change
    assert_eq!(cpu.reg_a, 0b1111_0000);
    // Zero flag should be clear (result of AND is not zero)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    // Negative flag should be set (bit 7 of memory value is set)
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
    // Overflow flag should be clear (bit 6 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_bit_zero_result() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1111_0000;
    cpu.mem_write_u8(0x10, 0b0000_1111);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    bit(&mut cpu, AddressingMode::ZeroPage);

    // Accumulator should not change
    assert_eq!(cpu.reg_a, 0b1111_0000);
    // Zero flag should be set (result of AND is zero)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0);
    // Negative flag should be clear (bit 7 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    // Overflow flag should be clear (bit 6 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_bit_overflow_flag() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0xFF;
    cpu.mem_write_u8(0x10, 0b0100_0000); // Bit 6 set, bit 7 clear
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    bit(&mut cpu, AddressingMode::ZeroPage);

    // Accumulator should not change
    assert_eq!(cpu.reg_a, 0xFF);
    // Zero flag should be clear (result of AND is not zero)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    // Negative flag should be clear (bit 7 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    // Overflow flag should be set (bit 6 of memory value is set)
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_bit_both_overflow_and_negative() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0xFF;
    cpu.mem_write_u8(0x10, 0b1100_0000); // Both bit 6 and 7 set
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    bit(&mut cpu, AddressingMode::ZeroPage);

    // Accumulator should not change
    assert_eq!(cpu.reg_a, 0xFF);
    // Zero flag should be clear (result of AND is not zero)
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    // Negative flag should be set (bit 7 of memory value is set)
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
    // Overflow flag should be set (bit 6 of memory value is set)
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0);
  }

  #[test]
  fn test_bit_clears_flags() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x00;
    cpu.status = 0xFF; // Set all flags initially
    cpu.mem_write_u8(0x10, 0b0011_1111); // Bits 6 and 7 clear
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    bit(&mut cpu, AddressingMode::ZeroPage);

    // Zero flag should be set (result of AND is zero)
    assert_ne!(cpu.status & StatusFlag::Zero as u8, 0);
    // Negative flag should be clear (bit 7 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    // Overflow flag should be clear (bit 6 of memory value is clear)
    assert_eq!(cpu.status & StatusFlag::Overflow as u8, 0);
    // Other flags should be preserved
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0);
    assert_ne!(cpu.status & StatusFlag::InterruptDisable as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Decimal as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Break as u8, 0);
  }

  #[test]
  fn test_bit_immediate_mode_not_used() {
    // BIT instruction typically doesn't use immediate mode in real 6502
    // This test ensures our implementation works with absolute addressing
    let mut cpu = CPU::new();
    cpu.reg_a = 0b1010_1010;
    cpu.mem_write_u16(0x1234, 0b1100_0011);
    cpu.pc = 0x0600;
    cpu.mem_write_u16(0x0600, 0x1234); // Absolute address

    bit(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.reg_a, 0b1010_1010); // Accumulator unchanged
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // AND result is not zero
    assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Bit 7 set in memory
    assert_ne!(cpu.status & StatusFlag::Overflow as u8, 0); // Bit 6 set in memory
  }

  // Edge case tests
  #[test]
  fn test_logical_operations_with_different_addressing_modes() {
    let mut cpu = CPU::new();

    // Test with Zero Page X
    cpu.reg_a = 0xAA;
    cpu.reg_x = 0x05;
    cpu.mem_write_u8(0x15, 0x55); // 0x10 + 0x05
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    and(&mut cpu, AddressingMode::ZeroPage_X);
    assert_eq!(cpu.reg_a, 0x00); // 0xAA & 0x55 = 0x00
  }

  #[test]
  fn test_flag_combinations() {
    let mut cpu = CPU::new();

    // Test that operations properly update only the flags they should
    cpu.reg_a = 0x7F;
    cpu.status = StatusFlag::Carry as u8 | StatusFlag::Decimal as u8;
    cpu.mem_write_u8(0x10, 0x01);
    cpu.pc = 0x0600;
    cpu.mem_write_u8(0x0600, 0x10);

    ora(&mut cpu, AddressingMode::ZeroPage);

    assert_eq!(cpu.reg_a, 0x7F); // 0x7F | 0x01 = 0x7F
    // Zero and Negative flags should be updated, others preserved
    assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Preserved
    assert_ne!(cpu.status & StatusFlag::Decimal as u8, 0); // Preserved
  }
}

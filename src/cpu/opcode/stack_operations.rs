use crate::cpu::{CPU, opcode::opcode_table::AddressingMode};

pub(crate) fn pha(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.mem_write_u8(cpu.get_stack_address(), cpu.reg_a);
  cpu.stack = cpu.stack.wrapping_sub(1);
}

pub(crate) fn php(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.mem_write_u8(cpu.get_stack_address(), cpu.status);
  cpu.stack = cpu.stack.wrapping_sub(1);
}

pub(crate) fn pla(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.stack = cpu.stack.wrapping_add(1);
  cpu.reg_a = cpu.mem_read_u8(cpu.get_stack_address());
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn plp(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.stack = cpu.stack.wrapping_add(1);
  cpu.status = cpu.mem_read_u8(cpu.get_stack_address());
}

#[cfg(test)]
mod stack_operations_test {
  use super::*;
  use crate::cpu::CPU;

  #[test]
  fn test_pha_push_accumulator() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x42;
    cpu.stack = 0xFF;

    pha(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.mem_read_u8(0x01FF), 0x42);
    assert_eq!(cpu.stack, 0xFE);
  }

  #[test]
  fn test_pha_stack_wrapping() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x33;
    cpu.stack = 0x00; // At bottom of stack

    pha(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.mem_read_u8(0x0100), 0x33);
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_php_push_processor_status() {
    let mut cpu = CPU::new();
    cpu.status = 0b1010_0101;
    cpu.stack = 0xFF;

    php(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.mem_read_u8(0x01FF), 0b1010_0101);
    assert_eq!(cpu.stack, 0xFE);
  }

  #[test]
  fn test_php_stack_wrapping() {
    let mut cpu = CPU::new();
    cpu.status = 0b1100_0011;
    cpu.stack = 0x00;

    php(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.mem_read_u8(0x0100), 0b1100_0011);
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_pla_pull_accumulator() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0x99);

    pla(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.reg_a, 0x99);
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_pla_stack_wrapping() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFF;
    cpu.mem_write_u8(0x0100, 0x77);

    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that accumulator was loaded from stack
    assert_eq!(cpu.reg_a, 0x77);
    // Check that stack pointer wrapped to 0x00
    assert_eq!(cpu.stack, 0x00);
  }

  #[test]
  fn test_plp_pull_processor_status() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0b0110_1001);

    plp(&mut cpu, AddressingMode::NoneAddressing);

    // Check that status was loaded from stack
    assert_eq!(cpu.status, 0b0110_1001);
    // Check that stack pointer was incremented
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_plp_stack_wrapping() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFF;
    cpu.mem_write_u8(0x0100, 0b1111_0000);

    plp(&mut cpu, AddressingMode::NoneAddressing);

    // Check that status was loaded from stack
    assert_eq!(cpu.status, 0b1111_0000);
    // Check that stack pointer wrapped to 0x00
    assert_eq!(cpu.stack, 0x00);
  }

  #[test]
  fn test_push_pull_accumulator_round_trip() {
    let mut cpu = CPU::new();
    let original_value = 0xAB;
    cpu.reg_a = original_value;
    cpu.stack = 0xFF;

    // Push accumulator to stack
    pha(&mut cpu, AddressingMode::NoneAddressing);
    // Modify accumulator
    cpu.reg_a = 0x00;
    // Pull accumulator from stack
    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that original value was restored
    assert_eq!(cpu.reg_a, original_value);
    // Check that stack pointer is back to original position
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_push_pull_status_round_trip() {
    let mut cpu = CPU::new();
    let original_status = 0b1001_0110;
    cpu.status = original_status;
    cpu.stack = 0xFF;

    // Push status to stack
    php(&mut cpu, AddressingMode::NoneAddressing);
    // Modify status
    cpu.status = 0x00;
    // Pull status from stack
    plp(&mut cpu, AddressingMode::NoneAddressing);

    // Check that original status was restored
    assert_eq!(cpu.status, original_status);
    // Check that stack pointer is back to original position
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_multiple_pushes_and_pulls() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFF;

    // Push multiple values
    cpu.reg_a = 0x11;
    pha(&mut cpu, AddressingMode::NoneAddressing);

    cpu.status = 0x22;
    php(&mut cpu, AddressingMode::NoneAddressing);

    cpu.reg_a = 0x33;
    pha(&mut cpu, AddressingMode::NoneAddressing);

    // Stack should now be at 0xFC
    assert_eq!(cpu.stack, 0xFC);

    // Pull values in reverse order (LIFO)
    pla(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.reg_a, 0x33);

    plp(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.status, 0x22);

    pla(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.reg_a, 0x11);

    // Stack should be back to original position
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_pla_sets_zero_flag() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0x00); // Push zero value
    cpu.status = 0b0000_0000; // Clear all flags initially

    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that accumulator is zero
    assert_eq!(cpu.reg_a, 0x00);
    // Check that zero flag is set (bit 1)
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
    // Check that negative flag is clear (bit 7)
    assert_eq!(cpu.status & 0b1000_0000, 0b0000_0000);
  }

  #[test]
  fn test_pla_sets_negative_flag() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0x80); // Push negative value (bit 7 set)
    cpu.status = 0b0000_0010; // Set zero flag initially

    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that accumulator has the negative value
    assert_eq!(cpu.reg_a, 0x80);
    // Check that negative flag is set (bit 7)
    assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000);
    // Check that zero flag is cleared (bit 1)
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0000);
  }

  #[test]
  fn test_pla_clears_zero_and_negative_flags() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0x42); // Push positive non-zero value
    cpu.status = 0b1000_0010; // Set both zero and negative flags initially

    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that accumulator has the correct value
    assert_eq!(cpu.reg_a, 0x42);
    // Check that both zero and negative flags are cleared
    assert_eq!(cpu.status & 0b1000_0010, 0b0000_0000);
  }

  #[test]
  fn test_pla_preserves_other_flags() {
    let mut cpu = CPU::new();
    cpu.stack = 0xFE;
    cpu.mem_write_u8(0x01FF, 0x7F); // Push positive value
    // Set all other flags except zero and negative
    cpu.status = 0b0111_1101; // Carry, Interrupt, Decimal, Break, Overflow flags set

    pla(&mut cpu, AddressingMode::NoneAddressing);

    // Check that accumulator has the correct value
    assert_eq!(cpu.reg_a, 0x7F);
    // Check that other flags are preserved (all except zero and negative)
    // Zero flag should be clear, negative flag should be clear
    assert_eq!(cpu.status & 0b0111_1101, 0b0111_1101);
    assert_eq!(cpu.status & 0b1000_0010, 0b0000_0000);
  }

  #[test]
  fn test_pla_flag_combinations() {
    let mut cpu = CPU::new();

    // Test case 1: Pull 0xFF (negative, non-zero)
    cpu.stack = 0xFC;
    cpu.mem_write_u8(0x01FD, 0xFF);
    cpu.status = 0b0000_0010; // Start with zero flag set

    pla(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.reg_a, 0xFF);
    assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000); // Negative flag set
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0000); // Zero flag clear

    // Test case 2: Pull 0x01 (positive, non-zero)
    cpu.mem_write_u8(0x01FE, 0x01);
    cpu.status = 0b1000_0010; // Start with both flags set

    pla(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.reg_a, 0x01);
    assert_eq!(cpu.status & 0b1000_0000, 0b0000_0000); // Negative flag clear
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0000); // Zero flag clear

    // Test case 3: Pull 0x00 (zero)
    cpu.mem_write_u8(0x01FF, 0x00);
    cpu.status = 0b1000_0000; // Start with negative flag set

    pla(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.reg_a, 0x00);
    assert_eq!(cpu.status & 0b1000_0000, 0b0000_0000); // Negative flag clear
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010); // Zero flag set
  }
}

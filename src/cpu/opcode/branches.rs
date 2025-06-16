use crate::cpu::{CPU, opcode::opcode_table::AddressingMode};

pub(crate) fn bcs(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(cpu.get_flag(crate::cpu::StatusFlag::Carry));
}

pub(crate) fn bcc(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(!cpu.get_flag(crate::cpu::StatusFlag::Carry));
}

pub(crate) fn beq(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(cpu.get_flag(crate::cpu::StatusFlag::Zero));
}

pub(crate) fn bne(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(!cpu.get_flag(crate::cpu::StatusFlag::Zero));
}

pub(crate) fn bmi(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(cpu.get_flag(crate::cpu::StatusFlag::Negative));
}

pub(crate) fn bpl(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(!cpu.get_flag(crate::cpu::StatusFlag::Negative));
}

pub(crate) fn bvs(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(cpu.get_flag(crate::cpu::StatusFlag::Overflow));
}

pub(crate) fn bvc(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.branch(!cpu.get_flag(crate::cpu::StatusFlag::Overflow));
}

#[cfg(test)]
mod branch_tests {
  use super::*;
  use crate::cpu::{CPU, StatusFlag};
  use crate::mem::Memory;

  // Helper function to set up CPU with a given PC and status flags
  fn setup_cpu_with_flags(pc: u16, status: u8) -> CPU {
    let mut cpu = CPU::new();
    cpu.pc = pc;
    cpu.status = status;
    cpu
  }

  // BCS (Branch if Carry Set) Tests
  #[test]
  fn test_bcs_carry_set_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0600, 0x10); // Positive offset of +16

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0611); // 0x6000 + 1 (PC increment) + 0x10
  }

  #[test]
  fn test_bcs_carry_set_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0620, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0620, 0xF0); // Negative offset of -16 (0xF0 = -16 in signed i8)

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0611); // 0x0620 + 1 (PC increment) - 16
  }

  #[test]
  fn test_bcs_carry_clear_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set
    cpu.mem_write_u8(0x0600, 0x10); // Offset (should be ignored)

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bcs_carry_set_zero_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0600, 0x00); // Zero offset

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // 0x0600 + 1 (PC increment) + 0
  }

  // BCC (Branch if Carry Clear) Tests
  #[test]
  fn test_bcc_carry_clear_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Carry clear)
    cpu.mem_write_u8(0x0600, 0x10); // Positive offset of +16

    bcc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0611); // 0x0600 + 1 (PC increment) + 0x10
  }

  #[test]
  fn test_bcc_carry_set_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0600, 0x10); // Offset (should be ignored)

    bcc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bcc_carry_clear_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0620, 0x00); // No flags set (Carry clear)
    cpu.mem_write_u8(0x0620, 0xE0); // Negative offset of -32

    bcc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // 0x0620 + 1 (PC increment) - 32
  }

  // BEQ (Branch if Equal/Zero Set) Tests
  #[test]
  fn test_beq_zero_set_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Zero as u8);
    cpu.mem_write_u8(0x0600, 0x08); // Positive offset of +8

    beq(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0609); // 0x0600 + 1 (PC increment) + 0x08
  }

  #[test]
  fn test_beq_zero_clear_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Zero clear)
    cpu.mem_write_u8(0x0600, 0x08); // Offset (should be ignored)

    beq(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_beq_zero_set_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0610, StatusFlag::Zero as u8);
    cpu.mem_write_u8(0x0610, 0xF8); // Negative offset of -8

    beq(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0609); // 0x0610 + 1 (PC increment) - 8
  }

  // BNE (Branch if Not Equal/Zero Clear) Tests
  #[test]
  fn test_bne_zero_clear_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Zero clear)
    cpu.mem_write_u8(0x0600, 0x05); // Positive offset of +5

    bne(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0606); // 0x0600 + 1 (PC increment) + 0x05
  }

  #[test]
  fn test_bne_zero_set_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Zero as u8);
    cpu.mem_write_u8(0x0600, 0x05); // Offset (should be ignored)

    bne(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bne_zero_clear_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0615, 0x00); // No flags set (Zero clear)
    cpu.mem_write_u8(0x0615, 0xF5); // Negative offset of -11

    bne(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060B); // 0x0615 + 1 (PC increment) - 11
  }

  // BMI (Branch if Minus/Negative Set) Tests
  #[test]
  fn test_bmi_negative_set_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Negative as u8);
    cpu.mem_write_u8(0x0600, 0x0C); // Positive offset of +12

    bmi(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060D); // 0x0600 + 1 (PC increment) + 0x0C
  }

  #[test]
  fn test_bmi_negative_clear_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Negative clear)
    cpu.mem_write_u8(0x0600, 0x0C); // Offset (should be ignored)

    bmi(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bmi_negative_set_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0620, StatusFlag::Negative as u8);
    cpu.mem_write_u8(0x0620, 0xEC); // Negative offset of -20

    bmi(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060D); // 0x0620 + 1 (PC increment) - 20
  }

  // BPL (Branch if Plus/Negative Clear) Tests
  #[test]
  fn test_bpl_negative_clear_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Negative clear)
    cpu.mem_write_u8(0x0600, 0x07); // Positive offset of +7

    bpl(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0608); // 0x0600 + 1 (PC increment) + 0x07
  }

  #[test]
  fn test_bpl_negative_set_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Negative as u8);
    cpu.mem_write_u8(0x0600, 0x07); // Offset (should be ignored)

    bpl(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bpl_negative_clear_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0610, 0x00); // No flags set (Negative clear)
    cpu.mem_write_u8(0x0610, 0xF9); // Negative offset of -7

    bpl(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060A); // 0x0610 + 1 (PC increment) - 7
  }

  // BVS (Branch if Overflow Set) Tests
  #[test]
  fn test_bvs_overflow_set_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Overflow as u8);
    cpu.mem_write_u8(0x0600, 0x14); // Positive offset of +20

    bvs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0615); // 0x0600 + 1 (PC increment) + 0x14
  }

  #[test]
  fn test_bvs_overflow_clear_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Overflow clear)
    cpu.mem_write_u8(0x0600, 0x14); // Offset (should be ignored)

    bvs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bvs_overflow_set_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0630, StatusFlag::Overflow as u8);
    cpu.mem_write_u8(0x0630, 0xE8); // Negative offset of -24

    bvs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0619); // 0x0630 + 1 (PC increment) - 24
  }

  // BVC (Branch if Overflow Clear) Tests
  #[test]
  fn test_bvc_overflow_clear_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0x00); // No flags set (Overflow clear)
    cpu.mem_write_u8(0x0600, 0x0A); // Positive offset of +10

    bvc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060B); // 0x0600 + 1 (PC increment) + 0x0A
  }

  #[test]
  fn test_bvc_overflow_set_no_branch() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Overflow as u8);
    cpu.mem_write_u8(0x0600, 0x0A); // Offset (should be ignored)

    bvc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 to skip offset byte when condition is false
  }

  #[test]
  fn test_bvc_overflow_clear_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0615, 0x00); // No flags set (Overflow clear)
    cpu.mem_write_u8(0x0615, 0xF6); // Negative offset of -10

    bvc(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x060C); // 0x0615 + 1 (PC increment) - 10
  }

  #[test]
  fn test_branch_max_positive_offset() {
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0600, 0x7F); // Maximum positive offset (+127)

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0680); // 0x0600 + 1 + 127
  }

  #[test]
  fn test_branch_max_negative_offset() {
    let mut cpu = setup_cpu_with_flags(0x0680, StatusFlag::Carry as u8);
    cpu.mem_write_u8(0x0680, 0x80); // Maximum negative offset (-128)

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0601); // 0x0680 + 1 - 128
  }

  // Test that branch instructions don't affect other registers or flags
  #[test]
  fn test_branch_preserves_registers_and_other_flags() {
    let mut cpu = setup_cpu_with_flags(0x0600, 0xFF); // All flags set
    cpu.reg_a = 0x42;
    cpu.reg_x = 0x55;
    cpu.reg_y = 0x66;
    cpu.stack = 0xFD;
    cpu.mem_write_u8(0x0600, 0x10);

    let original_status = cpu.status;

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    // Branch should only affect PC, not other registers or status flags
    assert_eq!(cpu.reg_a, 0x42);
    assert_eq!(cpu.reg_x, 0x55);
    assert_eq!(cpu.reg_y, 0x66);
    assert_eq!(cpu.stack, 0xFD);
    assert_eq!(cpu.status, original_status);
  }

  // Test all combinations of multiple flags
  #[test]
  fn test_branch_with_multiple_flags_set() {
    // Test BCS when both Carry and Zero are set
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Carry as u8 | StatusFlag::Zero as u8);
    cpu.mem_write_u8(0x0600, 0x05);

    bcs(&mut cpu, AddressingMode::NoneAddressing);

    assert_eq!(cpu.pc, 0x0606); // Should branch based on Carry flag
  }

  #[test]
  fn test_branch_instructions_independence() {
    // Test that each branch instruction only cares about its specific flag
    // Carry is clear, Zero and Negative are set
    let mut cpu = setup_cpu_with_flags(0x0600, StatusFlag::Zero as u8 | StatusFlag::Negative as u8);
    cpu.mem_write_u8(0x0600, 0x05);

    // BCC should branch (Carry is clear) even though other flags are set
    bcc(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.pc, 0x0606);

    // Reset PC and test BEQ
    cpu.pc = 0x0600;
    beq(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.pc, 0x0606);

    // Reset PC and test BMI
    cpu.pc = 0x0600;
    bmi(&mut cpu, AddressingMode::NoneAddressing);
    assert_eq!(cpu.pc, 0x0606);
  }
}

use crate::{
  cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode},
  utils::set_bit,
};

pub(crate) fn brk(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.status = set_bit(cpu.status, StatusFlag::Break as u8, true);
}

pub(crate) fn nop(_cpu: &mut CPU, _mode: AddressingMode) {}

pub(crate) fn rti(cpu: &mut CPU, _mode: AddressingMode) {
  let value = cpu.stack_pull_value_u8();
  cpu.status &= 0b0011_0000; // Clear all flags except B and extra bit
  cpu.status |= value & 0b1100_1111; // The B flag and extra bit are ignored.
  cpu.pc = cpu.stack_pull_value_u16();
}

#[cfg(test)]
mod system_functions_tests {
  use super::*;
  use crate::mem::Memory;

  #[test]
  fn test_brk_sets_break_flag() {
    let mut cpu = CPU::new();
    assert_eq!(cpu.status & StatusFlag::Break as u8, 0);

    brk(&mut cpu, AddressingMode::NoneAddressing);

    assert_ne!(cpu.status & StatusFlag::Break as u8, 0);
  }

  #[test]
  fn test_brk_preserves_other_flags() {
    let mut cpu = CPU::new();
    // Set some flags before BRK
    cpu.status = StatusFlag::Carry as u8 | StatusFlag::Zero as u8 | StatusFlag::Negative as u8;
    let initial_status = cpu.status;

    brk(&mut cpu, AddressingMode::NoneAddressing);

    // All original flags should remain, plus Break flag should be set
    assert_eq!(cpu.status, initial_status | StatusFlag::Break as u8);
  }

  #[test]
  fn test_brk_preserves_registers() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x42;
    cpu.reg_x = 0x55;
    cpu.reg_y = 0x66;
    cpu.pc = 0x1234;
    cpu.stack = 0xFD;

    brk(&mut cpu, AddressingMode::NoneAddressing);

    // BRK should not affect registers
    assert_eq!(cpu.reg_a, 0x42);
    assert_eq!(cpu.reg_x, 0x55);
    assert_eq!(cpu.reg_y, 0x66);
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.stack, 0xFD);
  }

  #[test]
  fn test_nop_does_nothing() {
    let mut cpu = CPU::new();
    // Set up initial state
    cpu.reg_a = 0x42;
    cpu.reg_x = 0x55;
    cpu.reg_y = 0x66;
    cpu.pc = 0x1234;
    cpu.stack = 0xFD;
    cpu.status = 0b1010_1010;

    // Save initial state
    let initial_a = cpu.reg_a;
    let initial_x = cpu.reg_x;
    let initial_y = cpu.reg_y;
    let initial_pc = cpu.pc;
    let initial_stack = cpu.stack;
    let initial_status = cpu.status;

    nop(&mut cpu, AddressingMode::NoneAddressing);

    // NOP should not change anything
    assert_eq!(cpu.reg_a, initial_a);
    assert_eq!(cpu.reg_x, initial_x);
    assert_eq!(cpu.reg_y, initial_y);
    assert_eq!(cpu.pc, initial_pc);
    assert_eq!(cpu.stack, initial_stack);
    assert_eq!(cpu.status, initial_status);
  }

  #[test]
  fn test_nop_multiple_calls() {
    let mut cpu = CPU::new();
    let initial_state = (
      cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.pc, cpu.stack, cpu.status,
    );

    // Multiple NOP calls should do nothing
    for _ in 0..10 {
      nop(&mut cpu, AddressingMode::NoneAddressing);
    }

    let final_state = (
      cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.pc, cpu.stack, cpu.status,
    );
    assert_eq!(initial_state, final_state);
  }

  #[test]
  fn test_rti_restores_status_and_pc() {
    let mut cpu = CPU::new();

    // Set up stack as if an interrupt occurred
    cpu.stack = 0xFC; // After status and PC were pushed
    cpu.mem_write_u8(0x01FD, 0b1010_1010); // Status to restore
    cpu.mem_write_u16(0x01FE, 0x3456); // PC to restore

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // Status should be restored with bits 4 and 5 preserved from current status
    // Original: 0b1010_1010, but bits 4,5 are preserved (initially 0), so result is 0b1010_1010
    assert_eq!(cpu.status, 0b1010_1010);
    // PC should be restored
    assert_eq!(cpu.pc, 0x3456);
    // Stack should be incremented by 3 (1 for status + 2 for PC)
    assert_eq!(cpu.stack, 0xFF);
  }

  #[test]
  fn test_rti_ignores_break_and_extra_bits() {
    let mut cpu = CPU::new();

    // Set up stack with status that has B and extra bit set
    cpu.stack = 0xFC;
    cpu.mem_write_u8(0x01FD, 0b0011_0000); // Only B and extra bit set
    cpu.mem_write_u16(0x01FE, 0x1000);

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // B flag and extra bit from stack are ignored, but current CPU's bits 4,5 are preserved
    // CPU starts with status 0b00100100, so bit 5 (value 32) is preserved
    assert_eq!(cpu.status, 0b0010_0000); // Only bit 5 preserved from initial CPU status
  }

  #[test]
  fn test_rti_preserves_registers() {
    let mut cpu = CPU::new();
    cpu.reg_a = 0x42;
    cpu.reg_x = 0x55;
    cpu.reg_y = 0x66;

    // Set up stack
    cpu.stack = 0xFC;
    cpu.mem_write_u8(0x01FD, 0b1100_0000);
    cpu.mem_write_u16(0x01FE, 0x2000);

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // RTI should not affect A, X, Y registers
    assert_eq!(cpu.reg_a, 0x42);
    assert_eq!(cpu.reg_x, 0x55);
    assert_eq!(cpu.reg_y, 0x66);
  }

  #[test]
  fn test_rti_with_all_flags_set() {
    let mut cpu = CPU::new();

    cpu.stack = 0xFC;
    // Set all flags except B and extra bit (which should be ignored)
    cpu.mem_write_u8(0x01FD, 0b1111_1111);
    cpu.mem_write_u16(0x01FE, 0x4000);

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // All flags should be set except B and extra bit, plus bit 5 preserved from CPU initial status
    // 0b1100_1111 (stack) | 0b0010_0000 (preserved bit 5) = 0b1110_1111
    assert_eq!(cpu.status, 0b1110_1111);
    assert_eq!(cpu.pc, 0x4000);
  }

  #[test]
  fn test_rti_with_no_flags_set() {
    let mut cpu = CPU::new();

    cpu.stack = 0xFC;
    cpu.mem_write_u8(0x01FD, 0b0000_0000);
    cpu.mem_write_u16(0x01FE, 0x5000);

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // No flags from stack, but bit 5 preserved from CPU initial status
    assert_eq!(cpu.status, 0b0010_0000);
    assert_eq!(cpu.pc, 0x5000);
  }

  #[test]
  fn test_rti_stack_wrapping() {
    let mut cpu = CPU::new();

    // Set up stack near overflow
    cpu.stack = 0xFD; // Will wrap when pulling 3 bytes
    cpu.mem_write_u8(0x01FE, 0b1010_0101); // Status
    cpu.mem_write_u8(0x01FF, 0x00); // PC low byte
    cpu.mem_write_u8(0x0100, 0x60); // PC high byte (wraps around)

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // Stack status: 0b1010_0101, preserve CPU bits 4,5: 0b0010_0000
    // Final: 0b0010_0000 | (0b1010_0101 & 0b1100_1111) = 0b0010_0000 | 0b1000_0101 = 0b1010_0101
    assert_eq!(cpu.status, 0b1010_0101);
    assert_eq!(cpu.pc, 0x6000);
    assert_eq!(cpu.stack, 0x00); // Should wrap to 0x00
  }

  #[test]
  fn test_rti_preserves_stack_memory() {
    let mut cpu = CPU::new();

    // Set up some memory values in stack area
    cpu.mem_write_u8(0x01F0, 0xAB);
    cpu.mem_write_u8(0x01F1, 0xCD);

    cpu.stack = 0xFC;
    cpu.mem_write_u8(0x01FD, 0b0010_0100);
    cpu.mem_write_u16(0x01FE, 0x7000);

    rti(&mut cpu, AddressingMode::NoneAddressing);

    // Memory should be unchanged (except for the stack locations that were read)
    assert_eq!(cpu.mem_read_u8(0x01F0), 0xAB);
    assert_eq!(cpu.mem_read_u8(0x01F1), 0xCD);
    // Status should be stack bits plus preserved bit 5: 0b0000_0100 | 0b0010_0000 = 0b0010_0100
    assert_eq!(cpu.status, 0b0010_0100);
  }
}

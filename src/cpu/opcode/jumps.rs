use crate::cpu::{CPU, opcode::opcode_table::AddressingMode};

pub(crate) fn jmp(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.pc = addr;
}

#[cfg(test)]
mod jmp_tests {
  use super::*;
  use crate::cpu::{CPU, opcode::opcode_table::AddressingMode};

  #[test]
  fn test_jmp_absolute() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x1234); // Jump to address 0x1234

    jmp(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.pc, 0x1234);
  }

  #[test]
  fn test_jmp_absolute_zero_address() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x0000); // Jump to address 0x0000

    jmp(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.pc, 0x0000);
  }

  #[test]
  fn test_jmp_absolute_max_address() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0xFFFF); // Jump to address 0xFFFF

    jmp(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.pc, 0xFFFF);
  }

  #[test]
  fn test_jmp_indirect() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x1000); // Pointer to address 0x1000
    cpu.mem_write_u16(0x1000, 0x5678); // Target address stored at 0x1000

    jmp(&mut cpu, AddressingMode::Indirect);

    assert_eq!(cpu.pc, 0x5678);
  }

  #[test]
  fn test_jmp_indirect_page_boundary_bug() {
    // This test simulates the famous 6502 JMP indirect bug
    // When the indirect address is at a page boundary (e.g., 0x10FF),
    // the high byte should be read from 0x1000, not 0x1100
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x10FF); // Pointer to address 0x10FF (page boundary)
    cpu.mem_write_u8(0x10FF, 0x34); // Low byte of target address
    cpu.mem_write_u8(0x1000, 0x12); // High byte of target address (should be read from 0x1000, not 0x1100)
    cpu.mem_write_u8(0x1100, 0x56); // This should NOT be used as high byte

    jmp(&mut cpu, AddressingMode::Indirect);

    // The result should be 0x1234, not 0x5634
    assert_eq!(cpu.pc, 0x1234);
  }

  #[test]
  fn test_jmp_indirect_zero_pointer() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x0000); // Pointer to address 0x0000
    cpu.mem_write_u16(0x0000, 0xABCD); // Target address stored at 0x0000

    jmp(&mut cpu, AddressingMode::Indirect);

    assert_eq!(cpu.pc, 0xABCD);
  }

  #[test]
  fn test_jmp_indirect_max_pointer() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0xFFFE); // Pointer to address 0xFFFE
    cpu.mem_write_u16(0xFFFE, 0x1357); // Target address stored at 0xFFFE

    jmp(&mut cpu, AddressingMode::Indirect);

    assert_eq!(cpu.pc, 0x1357);
  }

  #[test]
  fn test_jmp_absolute_with_different_initial_pc() {
    let mut cpu = CPU::new();
    cpu.pc = 0x2000; // Different starting PC
    cpu.mem_write_u16(0x2000, 0x8888); // Jump to address 0x8888

    jmp(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.pc, 0x8888);
  }

  #[test]
  fn test_jmp_preserves_other_registers() {
    let mut cpu = CPU::new();
    // Set up initial register state
    cpu.reg_a = 0x42;
    cpu.reg_x = 0x55;
    cpu.reg_y = 0x66;
    cpu.status = 0b1010_1010;
    cpu.stack = 0xFD;

    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x3000); // Jump to address 0x3000

    jmp(&mut cpu, AddressingMode::Absolute);

    // JMP should only affect PC, not other registers or status
    assert_eq!(cpu.pc, 0x3000);
    assert_eq!(cpu.reg_a, 0x42);
    assert_eq!(cpu.reg_x, 0x55);
    assert_eq!(cpu.reg_y, 0x66);
    assert_eq!(cpu.status, 0b1010_1010);
    assert_eq!(cpu.stack, 0xFD);
  }

  #[test]
  fn test_jmp_can_jump_to_same_location() {
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x8000); // Jump to same address (infinite loop)

    jmp(&mut cpu, AddressingMode::Absolute);

    assert_eq!(cpu.pc, 0x8000);
  }

  #[test]
  fn test_jmp_indirect_chain() {
    // Test multiple levels of indirection (though this would require multiple JMP instructions)
    let mut cpu = CPU::new();
    cpu.pc = 0x8000;
    cpu.mem_write_u16(0x8000, 0x2000); // First indirect pointer
    cpu.mem_write_u16(0x2000, 0x4000); // Target address

    jmp(&mut cpu, AddressingMode::Indirect);

    assert_eq!(cpu.pc, 0x4000);
  }
}

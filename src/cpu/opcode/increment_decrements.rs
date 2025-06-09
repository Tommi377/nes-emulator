use crate::cpu::{opcode::{optable::AddressingMode}, CPU};

pub(crate) fn inx(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_x = cpu.reg_x.wrapping_add(1);
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}

#[cfg(test)]
mod inx_test {
  use super::*;
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
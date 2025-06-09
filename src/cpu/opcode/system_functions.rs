use crate::{
  cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode},
  utils::set_bit,
};

pub(crate) fn brk(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.status = set_bit(cpu.status, StatusFlag::Break as u8, true);
}

#[cfg(test)]
mod brk_test {
  use super::*;

  #[test]
  fn test_brk_status() {
    let mut cpu = CPU::new();
    assert_eq!(cpu.status & StatusFlag::Break as u8, 0);
    cpu.load_and_run(vec![0x00]);
    assert_ne!(cpu.status & StatusFlag::Break as u8, 0);
  }
}

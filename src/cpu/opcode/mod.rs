use crate::cpu::{
  CPU,
  opcode::opcode_table::{AddressingMode, OPCODE_TABLE},
};

pub mod arithmetic;
pub mod branches;
pub mod combined_ops;
pub mod increment_decrements;
pub mod jumps;
pub mod load_store;
pub mod logical;
pub mod opcode_table;
pub mod register_transfers;
pub mod shifts;
pub mod stack_operations;
pub mod status_flag_changes;
pub mod system_functions;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct OP {
  pub code: u8,
  pub name: &'static str,
  pub op: fn(&mut CPU, AddressingMode),
  pub mode: AddressingMode,
  pub bytes: u8,
  pub cycles: u8,
}

impl OP {
  pub fn execute(&self, cpu: &mut CPU) {
    (self.op)(cpu, self.mode);
  }
}

impl From<u8> for OP {
  fn from(value: u8) -> Self {
    OPCODE_TABLE[value as usize].unwrap_or_else(|| {
      panic!("Opcode 0x{:02X} not found in opcode table", value);
    })
  }
}

#[cfg(test)]
mod opcode_test {
  use super::*;

  #[test]
  fn test_opcode_found() {
    let op: OP = 0x00.into();
    assert!(op.code == 0x00);
  }

  #[test]
  fn test_opcode_not_found() {
    let result = std::panic::catch_unwind(|| {
      let _op: OP = 0xFF.into(); // Non-existent opcode
    });
    assert!(result.is_err());
  }
}

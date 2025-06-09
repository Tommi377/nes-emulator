use crate::{cpu::{opcode::optable::{AddressingMode, OPCODE_TABLE}, CPU}};

pub mod optable;
pub mod register_transfers;
pub mod increment_decrements;
pub mod system_functions;
pub mod load_store;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct OP {
  pub code: u8,
  pub op: fn(&mut CPU, AddressingMode),
  pub mode: AddressingMode,
  pub bytes: u8,
  pub cycles: u8
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
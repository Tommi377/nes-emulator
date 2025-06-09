use crate::cpu::opcode::{increment_decrements::*, load_store::*, register_transfers::*, system_functions::*, OP};


#[allow(non_camel_case_types)]
#[allow(dead_code)]

// Instruction Set for the Obelisk 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html

pub(crate) static OPCODE_TABLE: [Option<OP>; 256] = {
  use AddressingMode::*;

  let mut table: [Option<OP>; 256] = [None; 256];
  // LDA Instructions
  table[0xA9] = Some(OP { code: 0xA9, op: lda, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA5] = Some(OP { code: 0xA5, op: lda, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB5] = Some(OP { code: 0xB5, op: lda, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAD] = Some(OP { code: 0xAD, op: lda, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBD] = Some(OP { code: 0xBD, op: lda, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xB9] = Some(OP { code: 0xB9, op: lda, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA1] = Some(OP { code: 0xA1, op: lda, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB1] = Some(OP { code: 0xB1, op: lda, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  // STA Instructions
  table[0x85] = Some(OP { code: 0x85, op: sta, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x95] = Some(OP { code: 0x95, op: sta, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8D] = Some(OP { code: 0x8D, op: sta, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x9D] = Some(OP { code: 0x9D, op: sta, mode: Absolute_X,      bytes: 3, cycles: 5 });
  table[0x99] = Some(OP { code: 0x99, op: sta, mode: Absolute_Y,      bytes: 3, cycles: 5 });
  table[0x81] = Some(OP { code: 0x81, op: sta, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x91] = Some(OP { code: 0x91, op: sta, mode: Indirect_Y,      bytes: 2, cycles: 6 });

  // Non-Memory Addressing Instructions
  table[0xE8] = Some(OP { code: 0xE8, op: inx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x00] = Some(OP { code: 0x00, op: brk, mode: NoneAddressing,  bytes: 1, cycles: 7 });

  // Transfer Instructions
  table[0xAA] = Some(OP { code: 0xAA, op: tax, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xA8] = Some(OP { code: 0xA8, op: tay, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xBA] = Some(OP { code: 0xBA, op: tsx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x8A] = Some(OP { code: 0x8A, op: txa, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x9A] = Some(OP { code: 0x9A, op: txs, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x98] = Some(OP { code: 0x98, op: tya, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  
  table
};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPage_X,
  ZeroPage_Y,
  Absolute,
  Absolute_X,
  Absolute_Y,
  Indirect_X,
  Indirect_Y,
  NoneAddressing,
}
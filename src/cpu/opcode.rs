use crate::cpu::op::OP;

#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub static OPCODE_TABLE: [Option<OP>; 256] = {
  use AddressingMode::*;

  let mut table: [Option<OP>; 256] = [None; 256];
  // LDA Instructions
  table[0xA9] = Some(OP { code: 0xA9, op: "LDA", mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA5] = Some(OP { code: 0xA5, op: "LDA", mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB5] = Some(OP { code: 0xB5, op: "LDA", mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAD] = Some(OP { code: 0xAD, op: "LDA", mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBD] = Some(OP { code: 0xBD, op: "LDA", mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xB9] = Some(OP { code: 0xB9, op: "LDA", mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA1] = Some(OP { code: 0xA1, op: "LDA", mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB1] = Some(OP { code: 0xB1, op: "LDA", mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });
  table[0xAA] = Some(OP { code: 0xAA, op: "TAX", mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // STA Instructions
  table[0x85] = Some(OP { code: 0x85, op: "STA", mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x95] = Some(OP { code: 0x95, op: "STA", mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8D] = Some(OP { code: 0x8D, op: "STA", mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x9D] = Some(OP { code: 0x9D, op: "STA", mode: Absolute_X,      bytes: 3, cycles: 5 });
  table[0x99] = Some(OP { code: 0x99, op: "STA", mode: Absolute_Y,      bytes: 3, cycles: 5 });
  table[0x81] = Some(OP { code: 0x81, op: "STA", mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x91] = Some(OP { code: 0x91, op: "STA", mode: Indirect_Y,      bytes: 2, cycles: 6 });

  // Non-Memory Addressing Instructions
  table[0xE8] = Some(OP { code: 0xE8, op: "INX", mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x00] = Some(OP { code: 0x00, op: "BRK", mode: NoneAddressing,  bytes: 1, cycles: 7 });
  
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
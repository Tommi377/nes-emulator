use crate::cpu::opcode::{
  OP, arithmetic::*, increment_decrements::*, load_store::*, register_transfers::*,
  status_flag_changes::*, system_functions::*,
};

// Instruction Set for the Obelisk 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[rustfmt::skip]
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

  // LDX Instructions
  table[0xA2] = Some(OP { code: 0xA2, op: ldx, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA6] = Some(OP { code: 0xA6, op: ldx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB6] = Some(OP { code: 0xB6, op: ldx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0xAE] = Some(OP { code: 0xAE, op: ldx, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBE] = Some(OP { code: 0xBE, op: ldx, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  // LDY Instructions
  table[0xA0] = Some(OP { code: 0xA0, op: ldy, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA4] = Some(OP { code: 0xA4, op: ldy, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB4] = Some(OP { code: 0xB4, op: ldy, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAC] = Some(OP { code: 0xAC, op: ldy, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBC] = Some(OP { code: 0xBC, op: ldy, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  // STA Instructions
  table[0x85] = Some(OP { code: 0x85, op: sta, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x95] = Some(OP { code: 0x95, op: sta, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8D] = Some(OP { code: 0x8D, op: sta, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x9D] = Some(OP { code: 0x9D, op: sta, mode: Absolute_X,      bytes: 3, cycles: 5 });
  table[0x99] = Some(OP { code: 0x99, op: sta, mode: Absolute_Y,      bytes: 3, cycles: 5 });
  table[0x81] = Some(OP { code: 0x81, op: sta, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x91] = Some(OP { code: 0x91, op: sta, mode: Indirect_Y,      bytes: 2, cycles: 6 });

  // STX Instructions
  table[0x86] = Some(OP { code: 0x86, op: stx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x96] = Some(OP { code: 0x96, op: stx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0x8E] = Some(OP { code: 0x8E, op: stx, mode: Absolute,        bytes: 3, cycles: 4 });

  // STY Instructions
  table[0x84] = Some(OP { code: 0x84, op: sty, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x94] = Some(OP { code: 0x94, op: sty, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8C] = Some(OP { code: 0x8C, op: sty, mode: Absolute,        bytes: 3, cycles: 4 });

  // Transfer Instructions
  table[0xAA] = Some(OP { code: 0xAA, op: tax, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xA8] = Some(OP { code: 0xA8, op: tay, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xBA] = Some(OP { code: 0xBA, op: tsx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x8A] = Some(OP { code: 0x8A, op: txa, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x9A] = Some(OP { code: 0x9A, op: txs, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x98] = Some(OP { code: 0x98, op: tya, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // Arithmetic Instructions
  table[0x69] = Some(OP { code: 0x69, op: adc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x65] = Some(OP { code: 0x65, op: adc, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x75] = Some(OP { code: 0x75, op: adc, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x6D] = Some(OP { code: 0x6D, op: adc, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x7D] = Some(OP { code: 0x7D, op: adc, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x79] = Some(OP { code: 0x79, op: adc, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x61] = Some(OP { code: 0x61, op: adc, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x71] = Some(OP { code: 0x71, op: adc, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  // Increment/Decrement Instructions
  table[0xE6] = Some(OP { code: 0xE6, op: inc, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xF6] = Some(OP { code: 0xF6, op: inc, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xEE] = Some(OP { code: 0xEE, op: inc, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xFE] = Some(OP { code: 0xFE, op: inc, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xE8] = Some(OP { code: 0xE8, op: inx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xC8] = Some(OP { code: 0xC8, op: iny, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  table[0xC6] = Some(OP { code: 0xC6, op: dec, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xD6] = Some(OP { code: 0xD6, op: dec, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xCE] = Some(OP { code: 0xCE, op: dec, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xDE] = Some(OP { code: 0xDE, op: dec, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xCA] = Some(OP { code: 0xCA, op: dex, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x88] = Some(OP { code: 0x88, op: dey, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  
  // Status Flag Changes
  table[0x18] = Some(OP { code: 0x18, op: clc, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xD8] = Some(OP { code: 0xD8, op: cld, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x58] = Some(OP { code: 0x58, op: cli, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xB8] = Some(OP { code: 0xB8, op: clv, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x38] = Some(OP { code: 0x38, op: sec, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xF8] = Some(OP { code: 0xF8, op: sed, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x78] = Some(OP { code: 0x78, op: sei, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // System Instructions
  table[0x00] = Some(OP { code: 0x00, op: brk, mode: NoneAddressing,  bytes: 1, cycles: 7 });

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

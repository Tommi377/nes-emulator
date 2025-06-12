use crate::cpu::opcode::{
  OP, arithmetic::*, increment_decrements::*, load_store::*, logical::*, register_transfers::*,
  shifts::*, stack_operations::*, status_flag_changes::*, system_functions::*,
};

// Instruction Set for the Obelisk 6502 CPU
// https://www.nesdev.org/obelisk-6502-guide/reference.html

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[rustfmt::skip]
pub(crate) static OPCODE_TABLE: [Option<OP>; 256] = {
  use AddressingMode::*;

  let mut table: [Option<OP>; 256] = [None; 256];

  // Load Instructions
  table[0xA9] = Some(OP { code: 0xA9, op: lda, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA5] = Some(OP { code: 0xA5, op: lda, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB5] = Some(OP { code: 0xB5, op: lda, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAD] = Some(OP { code: 0xAD, op: lda, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBD] = Some(OP { code: 0xBD, op: lda, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xB9] = Some(OP { code: 0xB9, op: lda, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA1] = Some(OP { code: 0xA1, op: lda, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB1] = Some(OP { code: 0xB1, op: lda, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0xA2] = Some(OP { code: 0xA2, op: ldx, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA6] = Some(OP { code: 0xA6, op: ldx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB6] = Some(OP { code: 0xB6, op: ldx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0xAE] = Some(OP { code: 0xAE, op: ldx, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBE] = Some(OP { code: 0xBE, op: ldx, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  table[0xA0] = Some(OP { code: 0xA0, op: ldy, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA4] = Some(OP { code: 0xA4, op: ldy, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB4] = Some(OP { code: 0xB4, op: ldy, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAC] = Some(OP { code: 0xAC, op: ldy, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBC] = Some(OP { code: 0xBC, op: ldy, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  // Store Instructions
  table[0x85] = Some(OP { code: 0x85, op: sta, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x95] = Some(OP { code: 0x95, op: sta, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8D] = Some(OP { code: 0x8D, op: sta, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x9D] = Some(OP { code: 0x9D, op: sta, mode: Absolute_X,      bytes: 3, cycles: 5 });
  table[0x99] = Some(OP { code: 0x99, op: sta, mode: Absolute_Y,      bytes: 3, cycles: 5 });
  table[0x81] = Some(OP { code: 0x81, op: sta, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x91] = Some(OP { code: 0x91, op: sta, mode: Indirect_Y,      bytes: 2, cycles: 6 });

  table[0x86] = Some(OP { code: 0x86, op: stx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x96] = Some(OP { code: 0x96, op: stx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0x8E] = Some(OP { code: 0x8E, op: stx, mode: Absolute,        bytes: 3, cycles: 4 });

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

  // Logical Instructions
  table[0x29] = Some(OP { code: 0x29, op: and, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x25] = Some(OP { code: 0x25, op: and, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x35] = Some(OP { code: 0x35, op: and, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x2D] = Some(OP { code: 0x2D, op: and, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x3D] = Some(OP { code: 0x3D, op: and, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x39] = Some(OP { code: 0x39, op: and, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x21] = Some(OP { code: 0x21, op: and, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x31] = Some(OP { code: 0x31, op: and, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x49] = Some(OP { code: 0x49, op: eor, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x45] = Some(OP { code: 0x45, op: eor, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x55] = Some(OP { code: 0x55, op: eor, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x4D] = Some(OP { code: 0x4D, op: eor, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x5D] = Some(OP { code: 0x5D, op: eor, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x59] = Some(OP { code: 0x59, op: eor, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x41] = Some(OP { code: 0x41, op: eor, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x51] = Some(OP { code: 0x51, op: eor, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x09] = Some(OP { code: 0x09, op: ora, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x05] = Some(OP { code: 0x05, op: ora, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x15] = Some(OP { code: 0x15, op: ora, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x0D] = Some(OP { code: 0x0D, op: ora, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x1D] = Some(OP { code: 0x1D, op: ora, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x19] = Some(OP { code: 0x19, op: ora, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x01] = Some(OP { code: 0x01, op: ora, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x11] = Some(OP { code: 0x11, op: ora, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x24] = Some(OP { code: 0x24, op: bit, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x2C] = Some(OP { code: 0x2C, op: bit, mode: Absolute,        bytes: 3, cycles: 4 });

  // Arithmetic Instructions
  table[0x69] = Some(OP { code: 0x69, op: adc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x65] = Some(OP { code: 0x65, op: adc, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x75] = Some(OP { code: 0x75, op: adc, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x6D] = Some(OP { code: 0x6D, op: adc, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x7D] = Some(OP { code: 0x7D, op: adc, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x79] = Some(OP { code: 0x79, op: adc, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x61] = Some(OP { code: 0x61, op: adc, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x71] = Some(OP { code: 0x71, op: adc, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  /* TODO
  table[0xE9] = Some(OP { code: 0xE9, op: sbc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xE5] = Some(OP { code: 0xE5, op: sbc, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xF5] = Some(OP { code: 0xF5, op: sbc, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xED] = Some(OP { code: 0xED, op: sbc, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xFD] = Some(OP { code: 0xFD, op: sbc, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xF9] = Some(OP { code: 0xF9, op: sbc, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xE1] = Some(OP { code: 0xE1, op: sbc, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xF1] = Some(OP { code: 0xF1, op: sbc, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  // Compare Instructions
  table[0xC9] = Some(OP { code: 0xC9, op: cmp, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xC5] = Some(OP { code: 0xC5, op: cmp, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xD5] = Some(OP { code: 0xD5, op: cmp, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xCD] = Some(OP { code: 0xCD, op: cmp, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xDD] = Some(OP { code: 0xDD, op: cmp, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xD9] = Some(OP { code: 0xD9, op: cmp, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xC1] = Some(OP { code: 0xC1, op: cmp, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xD1] = Some(OP { code: 0xD1, op: cmp, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0xE0] = Some(OP { code: 0xE0, op: cpx, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xE4] = Some(OP { code: 0xE4, op: cpx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xEC] = Some(OP { code: 0xEC, op: cpx, mode: Absolute,        bytes: 3, cycles: 4 });

  table[0xC0] = Some(OP { code: 0xC0, op: cpy, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xC4] = Some(OP { code: 0xC4, op: cpy, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xCC] = Some(OP { code: 0xCC, op: cpy, mode: Absolute,        bytes: 3, cycles: 4 });
  */

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

  // Shift Instructions
  table[0x0A] = Some(OP { code: 0x0A, op: asl, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x06] = Some(OP { code: 0x06, op: asl, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x16] = Some(OP { code: 0x16, op: asl, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x0E] = Some(OP { code: 0x0E, op: asl, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x1E] = Some(OP { code: 0x1E, op: asl, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x4A] = Some(OP { code: 0x4A, op: lsr, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x46] = Some(OP { code: 0x46, op: lsr, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x56] = Some(OP { code: 0x56, op: lsr, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x4E] = Some(OP { code: 0x4E, op: lsr, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x5E] = Some(OP { code: 0x5E, op: lsr, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x2A] = Some(OP { code: 0x2A, op: rol, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x26] = Some(OP { code: 0x26, op: rol, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x36] = Some(OP { code: 0x36, op: rol, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x2E] = Some(OP { code: 0x2E, op: rol, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x3E] = Some(OP { code: 0x3E, op: rol, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x6A] = Some(OP { code: 0x6A, op: ror, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x66] = Some(OP { code: 0x66, op: ror, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x76] = Some(OP { code: 0x76, op: ror, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x6E] = Some(OP { code: 0x6E, op: ror, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x7E] = Some(OP { code: 0x7E, op: ror, mode: Absolute_X,      bytes: 3, cycles: 7 });

  /* TODO
  // Jump Instructions
  table[0x4C] = Some(OP { code: 0x4C, op: jmp, mode: Absolute,        bytes: 3, cycles: 3 });
  table[0x6C] = Some(OP { code: 0x6C, op: jmp, mode: Indirect,        bytes: 3, cycles: 5 });

  table[0x20] = Some(OP { code: 0x20, op: jsr, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x60] = Some(OP { code: 0x60, op: rts, mode: NoneAddressing,  bytes: 1, cycles: 6 });
  */

  /* TODO
  // Branch Instructions
  table[0x90] = Some(OP { code: 0x90, op: bcc, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xB0] = Some(OP { code: 0xB0, op: bcs, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xF0] = Some(OP { code: 0xF0, op: beq, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x30] = Some(OP { code: 0x30, op: bmi, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xD0] = Some(OP { code: 0xD0, op: bne, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x10] = Some(OP { code: 0x10, op: bpl, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x50] = Some(OP { code: 0x50, op: bvc, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x70] = Some(OP { code: 0x70, op: bvs, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  */
  
  // Status Flag Changes
  table[0x18] = Some(OP { code: 0x18, op: clc, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xD8] = Some(OP { code: 0xD8, op: cld, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x58] = Some(OP { code: 0x58, op: cli, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xB8] = Some(OP { code: 0xB8, op: clv, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x38] = Some(OP { code: 0x38, op: sec, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xF8] = Some(OP { code: 0xF8, op: sed, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x78] = Some(OP { code: 0x78, op: sei, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // Stack Operations
  table[0x48] = Some(OP { code: 0x48, op: pha, mode: NoneAddressing,  bytes: 1, cycles: 3 });
  table[0x08] = Some(OP { code: 0x08, op: php, mode: NoneAddressing,  bytes: 1, cycles: 3 });
  table[0x68] = Some(OP { code: 0x68, op: pla, mode: NoneAddressing,  bytes: 1, cycles: 4 });
  table[0x28] = Some(OP { code: 0x28, op: plp, mode: NoneAddressing,  bytes: 1, cycles: 4 });

  // System Instructions
  table[0x00] = Some(OP { code: 0x00, op: brk, mode: NoneAddressing,  bytes: 1, cycles: 7 });

  table
};

#[derive(Debug, Clone, Copy, PartialEq)]
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
  Indirect,
  Indirect_X,
  Indirect_Y,
  Accumulator,
  Relative,
  NoneAddressing,
}

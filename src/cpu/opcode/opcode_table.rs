use crate::cpu::opcode::{
    OP, arithmetic::*, branches::*, combined_ops::*, increment_decrements::*, jumps::*,
    load_store::*, logical::*, register_transfers::*, rmw::*, shifts::*, stack_operations::*,
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

  // Load Instructions
  table[0xA9] = Some(OP { code: 0xA9, name: "LDA", op: lda, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA5] = Some(OP { code: 0xA5, name: "LDA", op: lda, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB5] = Some(OP { code: 0xB5, name: "LDA", op: lda, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAD] = Some(OP { code: 0xAD, name: "LDA", op: lda, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBD] = Some(OP { code: 0xBD, name: "LDA", op: lda, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xB9] = Some(OP { code: 0xB9, name: "LDA", op: lda, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA1] = Some(OP { code: 0xA1, name: "LDA", op: lda, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB1] = Some(OP { code: 0xB1, name: "LDA", op: lda, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0xA2] = Some(OP { code: 0xA2, name: "LDX", op: ldx, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA6] = Some(OP { code: 0xA6, name: "LDX", op: ldx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB6] = Some(OP { code: 0xB6, name: "LDX", op: ldx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0xAE] = Some(OP { code: 0xAE, name: "LDX", op: ldx, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBE] = Some(OP { code: 0xBE, name: "LDX", op: ldx, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  table[0xA0] = Some(OP { code: 0xA0, name: "LDY", op: ldy, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xA4] = Some(OP { code: 0xA4, name: "LDY", op: ldy, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB4] = Some(OP { code: 0xB4, name: "LDY", op: ldy, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xAC] = Some(OP { code: 0xAC, name: "LDY", op: ldy, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBC] = Some(OP { code: 0xBC, name: "LDY", op: ldy, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  // Store Instructions
  table[0x85] = Some(OP { code: 0x85, name: "STA", op: sta, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x95] = Some(OP { code: 0x95, name: "STA", op: sta, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8D] = Some(OP { code: 0x8D, name: "STA", op: sta, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x9D] = Some(OP { code: 0x9D, name: "STA", op: sta, mode: Absolute_X,      bytes: 3, cycles: 5 });
  table[0x99] = Some(OP { code: 0x99, name: "STA", op: sta, mode: Absolute_Y,      bytes: 3, cycles: 5 });
  table[0x81] = Some(OP { code: 0x81, name: "STA", op: sta, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x91] = Some(OP { code: 0x91, name: "STA", op: sta, mode: Indirect_Y,      bytes: 2, cycles: 6 });

  table[0x86] = Some(OP { code: 0x86, name: "STX", op: stx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x96] = Some(OP { code: 0x96, name: "STX", op: stx, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0x8E] = Some(OP { code: 0x8E, name: "STX", op: stx, mode: Absolute,        bytes: 3, cycles: 4 });

  table[0x84] = Some(OP { code: 0x84, name: "STY", op: sty, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x94] = Some(OP { code: 0x94, name: "STY", op: sty, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x8C] = Some(OP { code: 0x8C, name: "STY", op: sty, mode: Absolute,        bytes: 3, cycles: 4 });

  // Transfer Instructions
  table[0xAA] = Some(OP { code: 0xAA, name: "TAX", op: tax, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xA8] = Some(OP { code: 0xA8, name: "TAY", op: tay, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xBA] = Some(OP { code: 0xBA, name: "TSX", op: tsx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x8A] = Some(OP { code: 0x8A, name: "TXA", op: txa, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x9A] = Some(OP { code: 0x9A, name: "TXS", op: txs, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x98] = Some(OP { code: 0x98, name: "TYA", op: tya, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // Logical Instructions
  table[0x29] = Some(OP { code: 0x29, name: "AND", op: and, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x25] = Some(OP { code: 0x25, name: "AND", op: and, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x35] = Some(OP { code: 0x35, name: "AND", op: and, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x2D] = Some(OP { code: 0x2D, name: "AND", op: and, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x3D] = Some(OP { code: 0x3D, name: "AND", op: and, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x39] = Some(OP { code: 0x39, name: "AND", op: and, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x21] = Some(OP { code: 0x21, name: "AND", op: and, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x31] = Some(OP { code: 0x31, name: "AND", op: and, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x49] = Some(OP { code: 0x49, name: "EOR", op: eor, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x45] = Some(OP { code: 0x45, name: "EOR", op: eor, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x55] = Some(OP { code: 0x55, name: "EOR", op: eor, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x4D] = Some(OP { code: 0x4D, name: "EOR", op: eor, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x5D] = Some(OP { code: 0x5D, name: "EOR", op: eor, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x59] = Some(OP { code: 0x59, name: "EOR", op: eor, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x41] = Some(OP { code: 0x41, name: "EOR", op: eor, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x51] = Some(OP { code: 0x51, name: "EOR", op: eor, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x09] = Some(OP { code: 0x09, name: "ORA", op: ora, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x05] = Some(OP { code: 0x05, name: "ORA", op: ora, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x15] = Some(OP { code: 0x15, name: "ORA", op: ora, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x0D] = Some(OP { code: 0x0D, name: "ORA", op: ora, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x1D] = Some(OP { code: 0x1D, name: "ORA", op: ora, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x19] = Some(OP { code: 0x19, name: "ORA", op: ora, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x01] = Some(OP { code: 0x01, name: "ORA", op: ora, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x11] = Some(OP { code: 0x11, name: "ORA", op: ora, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0x24] = Some(OP { code: 0x24, name: "BIT", op: bit, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x2C] = Some(OP { code: 0x2C, name: "BIT", op: bit, mode: Absolute,        bytes: 3, cycles: 4 });

  // Arithmetic Instructions
  table[0x69] = Some(OP { code: 0x69, name: "ADC", op: adc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x65] = Some(OP { code: 0x65, name: "ADC", op: adc, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x75] = Some(OP { code: 0x75, name: "ADC", op: adc, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x6D] = Some(OP { code: 0x6D, name: "ADC", op: adc, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x7D] = Some(OP { code: 0x7D, name: "ADC", op: adc, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x79] = Some(OP { code: 0x79, name: "ADC", op: adc, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x61] = Some(OP { code: 0x61, name: "ADC", op: adc, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0x71] = Some(OP { code: 0x71, name: "ADC", op: adc, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0xE9] = Some(OP { code: 0xE9, name: "SBC", op: sbc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xE5] = Some(OP { code: 0xE5, name: "SBC", op: sbc, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xF5] = Some(OP { code: 0xF5, name: "SBC", op: sbc, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xED] = Some(OP { code: 0xED, name: "SBC", op: sbc, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xFD] = Some(OP { code: 0xFD, name: "SBC", op: sbc, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xF9] = Some(OP { code: 0xF9, name: "SBC", op: sbc, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xE1] = Some(OP { code: 0xE1, name: "SBC", op: sbc, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xF1] = Some(OP { code: 0xF1, name: "SBC", op: sbc, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  // Compare Instructions
  table[0xC9] = Some(OP { code: 0xC9, name: "CMP", op: cmp, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xC5] = Some(OP { code: 0xC5, name: "CMP", op: cmp, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xD5] = Some(OP { code: 0xD5, name: "CMP", op: cmp, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xCD] = Some(OP { code: 0xCD, name: "CMP", op: cmp, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xDD] = Some(OP { code: 0xDD, name: "CMP", op: cmp, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xD9] = Some(OP { code: 0xD9, name: "CMP", op: cmp, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xC1] = Some(OP { code: 0xC1, name: "CMP", op: cmp, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xD1] = Some(OP { code: 0xD1, name: "CMP", op: cmp, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  table[0xE0] = Some(OP { code: 0xE0, name: "CPX", op: cpx, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xE4] = Some(OP { code: 0xE4, name: "CPX", op: cpx, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xEC] = Some(OP { code: 0xEC, name: "CPX", op: cpx, mode: Absolute,        bytes: 3, cycles: 4 });

  table[0xC0] = Some(OP { code: 0xC0, name: "CPY", op: cpy, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xC4] = Some(OP { code: 0xC4, name: "CPY", op: cpy, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xCC] = Some(OP { code: 0xCC, name: "CPY", op: cpy, mode: Absolute,        bytes: 3, cycles: 4 });

  // Increment/Decrement Instructions
  table[0xE6] = Some(OP { code: 0xE6, name: "INC", op: inc, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xF6] = Some(OP { code: 0xF6, name: "INC", op: inc, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xEE] = Some(OP { code: 0xEE, name: "INC", op: inc, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xFE] = Some(OP { code: 0xFE, name: "INC", op: inc, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xE8] = Some(OP { code: 0xE8, name: "INX", op: inx, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xC8] = Some(OP { code: 0xC8, name: "INY", op: iny, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  table[0xC6] = Some(OP { code: 0xC6, name: "DEC", op: dec, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xD6] = Some(OP { code: 0xD6, name: "DEC", op: dec, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xCE] = Some(OP { code: 0xCE, name: "DEC", op: dec, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xDE] = Some(OP { code: 0xDE, name: "DEC", op: dec, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xCA] = Some(OP { code: 0xCA, name: "DEX", op: dex, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x88] = Some(OP { code: 0x88, name: "DEY", op: dey, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // Shift Instructions
  table[0x0A] = Some(OP { code: 0x0A, name: "ASL", op: asl, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x06] = Some(OP { code: 0x06, name: "ASL", op: asl, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x16] = Some(OP { code: 0x16, name: "ASL", op: asl, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x0E] = Some(OP { code: 0x0E, name: "ASL", op: asl, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x1E] = Some(OP { code: 0x1E, name: "ASL", op: asl, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x4A] = Some(OP { code: 0x4A, name: "LSR", op: lsr, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x46] = Some(OP { code: 0x46, name: "LSR", op: lsr, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x56] = Some(OP { code: 0x56, name: "LSR", op: lsr, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x4E] = Some(OP { code: 0x4E, name: "LSR", op: lsr, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x5E] = Some(OP { code: 0x5E, name: "LSR", op: lsr, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x2A] = Some(OP { code: 0x2A, name: "ROL", op: rol, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x26] = Some(OP { code: 0x26, name: "ROL", op: rol, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x36] = Some(OP { code: 0x36, name: "ROL", op: rol, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x2E] = Some(OP { code: 0x2E, name: "ROL", op: rol, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x3E] = Some(OP { code: 0x3E, name: "ROL", op: rol, mode: Absolute_X,      bytes: 3, cycles: 7 });

  table[0x6A] = Some(OP { code: 0x6A, name: "ROR", op: ror, mode: Accumulator,     bytes: 1, cycles: 2 });
  table[0x66] = Some(OP { code: 0x66, name: "ROR", op: ror, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x76] = Some(OP { code: 0x76, name: "ROR", op: ror, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x6E] = Some(OP { code: 0x6E, name: "ROR", op: ror, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x7E] = Some(OP { code: 0x7E, name: "ROR", op: ror, mode: Absolute_X,      bytes: 3, cycles: 7 });

  // Jump Instructions
  table[0x4C] = Some(OP { code: 0x4C, name: "JMP", op: jmp, mode: Absolute,        bytes: 3, cycles: 3 });
  table[0x6C] = Some(OP { code: 0x6C, name: "JMP", op: jmp, mode: Indirect,        bytes: 3, cycles: 5 });

  table[0x20] = Some(OP { code: 0x20, name: "JSR", op: jsr, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x60] = Some(OP { code: 0x60, name: "RTS", op: rts, mode: NoneAddressing,  bytes: 1, cycles: 6 });

  // Branch Instructions
  table[0x90] = Some(OP { code: 0x90, name: "BCC", op: bcc, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xB0] = Some(OP { code: 0xB0, name: "BCS", op: bcs, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xF0] = Some(OP { code: 0xF0, name: "BEQ", op: beq, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x30] = Some(OP { code: 0x30, name: "BMI", op: bmi, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0xD0] = Some(OP { code: 0xD0, name: "BNE", op: bne, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x10] = Some(OP { code: 0x10, name: "BPL", op: bpl, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x50] = Some(OP { code: 0x50, name: "BVC", op: bvc, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  table[0x70] = Some(OP { code: 0x70, name: "BVS", op: bvs, mode: Relative,        bytes: 2, cycles: 2 /* +1 if branch succeeds +2 if to a new page */ });
  
  // Status Flag Changes
  table[0x18] = Some(OP { code: 0x18, name: "CLC", op: clc, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xD8] = Some(OP { code: 0xD8, name: "CLD", op: cld, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x58] = Some(OP { code: 0x58, name: "CLI", op: cli, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xB8] = Some(OP { code: 0xB8, name: "CLV", op: clv, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x38] = Some(OP { code: 0x38, name: "SEC", op: sec, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xF8] = Some(OP { code: 0xF8, name: "SED", op: sed, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x78] = Some(OP { code: 0x78, name: "SEI", op: sei, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // Stack Operations
  table[0x48] = Some(OP { code: 0x48, name: "PHA", op: pha, mode: NoneAddressing,  bytes: 1, cycles: 3 });
  table[0x08] = Some(OP { code: 0x08, name: "PHP", op: php, mode: NoneAddressing,  bytes: 1, cycles: 3 });
  table[0x68] = Some(OP { code: 0x68, name: "PLA", op: pla, mode: NoneAddressing,  bytes: 1, cycles: 4 });
  table[0x28] = Some(OP { code: 0x28, name: "PLP", op: plp, mode: NoneAddressing,  bytes: 1, cycles: 4 });

  // System Instructions
  table[0x00] = Some(OP { code: 0x00, name: "BRK", op: brk, mode: NoneAddressing,  bytes: 1, cycles: 7 });
  table[0xEA] = Some(OP { code: 0xEA, name: "NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x40] = Some(OP { code: 0x40, name: "RTI", op: rti, mode: NoneAddressing,  bytes: 1, cycles: 6 });

  //////////////////////////////
  //    Unofficial Opcodes    //
  //////////////////////////////

  // NOP - Unofficial
  table[0x1A] = Some(OP { code: 0x1A, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x3A] = Some(OP { code: 0x3A, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x5A] = Some(OP { code: 0x5A, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0x7A] = Some(OP { code: 0x7A, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xDA] = Some(OP { code: 0xDA, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });
  table[0xFA] = Some(OP { code: 0xFA, name: "*NOP", op: nop, mode: NoneAddressing,  bytes: 1, cycles: 2 });

  // SKB - Unofficial
  table[0x80] = Some(OP { code: 0x80, name: "*NOP", op: nop, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x82] = Some(OP { code: 0x82, name: "*NOP", op: nop, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x89] = Some(OP { code: 0x89, name: "*NOP", op: nop, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xC2] = Some(OP { code: 0xC2, name: "*NOP", op: nop, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xE2] = Some(OP { code: 0xE2, name: "*NOP", op: nop, mode: Immediate,       bytes: 2, cycles: 2 });

  // IGN - Unofficial
  table[0x04] = Some(OP { code: 0x04, name: "*NOP", op: nop, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x44] = Some(OP { code: 0x44, name: "*NOP", op: nop, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x64] = Some(OP { code: 0x64, name: "*NOP", op: nop, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x14] = Some(OP { code: 0x14, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x34] = Some(OP { code: 0x34, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x54] = Some(OP { code: 0x54, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x74] = Some(OP { code: 0x74, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xD4] = Some(OP { code: 0xD4, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0xF4] = Some(OP { code: 0xF4, name: "*NOP", op: nop, mode: ZeroPage_X,      bytes: 2, cycles: 4 });
  table[0x0C] = Some(OP { code: 0x0C, name: "*NOP", op: nop, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x1C] = Some(OP { code: 0x1C, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x3C] = Some(OP { code: 0x3C, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x5C] = Some(OP { code: 0x5C, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0x7C] = Some(OP { code: 0x7C, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xDC] = Some(OP { code: 0xDC, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xFC] = Some(OP { code: 0xFC, name: "*NOP", op: nop, mode: Absolute_X,      bytes: 3, cycles: 4 /* +1 if page crossed */ });

  // Combined Instructions
  table[0x4B] = Some(OP { code: 0x4B, name: "*ALR", op: alr, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x0B] = Some(OP { code: 0x0B, name: "*ANC", op: anc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x2B] = Some(OP { code: 0x2B, name: "*ANC", op: anc, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0x6B] = Some(OP { code: 0x6B, name: "*ARR", op: arr, mode: Immediate,       bytes: 2, cycles: 2 });
  table[0xCB] = Some(OP { code: 0xCB, name: "*AXS", op: axs, mode: Immediate,       bytes: 2, cycles: 2 });

  // LAX
  table[0xA7] = Some(OP { code: 0xA7, name: "*LAX", op: lax, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0xB7] = Some(OP { code: 0xB7, name: "*LAX", op: lax, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0xAF] = Some(OP { code: 0xAF, name: "*LAX", op: lax, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0xBF] = Some(OP { code: 0xBF, name: "*LAX", op: lax, mode: Absolute_Y,      bytes: 3, cycles: 4 /* +1 if page crossed */ });
  table[0xA3] = Some(OP { code: 0xA3, name: "*LAX", op: lax, mode: Indirect_X,      bytes: 2, cycles: 6 });
  table[0xB3] = Some(OP { code: 0xB3, name: "*LAX", op: lax, mode: Indirect_Y,      bytes: 2, cycles: 5 /* +1 if page crossed */ });

  // SAX
  table[0x87] = Some(OP { code: 0x87, name: "*SAX", op: sax, mode: ZeroPage,        bytes: 2, cycles: 3 });
  table[0x97] = Some(OP { code: 0x97, name: "*SAX", op: sax, mode: ZeroPage_Y,      bytes: 2, cycles: 4 });
  table[0x8F] = Some(OP { code: 0x8F, name: "*SAX", op: sax, mode: Absolute,        bytes: 3, cycles: 4 });
  table[0x83] = Some(OP { code: 0x83, name: "*SAX", op: sax, mode: Indirect_X,      bytes: 2, cycles: 6 });
  
  // DCP
  table[0xC7] = Some(OP { code: 0xC7, name: "*DCP", op: dcp, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xD7] = Some(OP { code: 0xD7, name: "*DCP", op: dcp, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xCF] = Some(OP { code: 0xCF, name: "*DCP", op: dcp, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xDF] = Some(OP { code: 0xDF, name: "*DCP", op: dcp, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xDB] = Some(OP { code: 0xDB, name: "*DCP", op: dcp, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0xC3] = Some(OP { code: 0xC3, name: "*DCP", op: dcp, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0xD3] = Some(OP { code: 0xD3, name: "*DCP", op: dcp, mode: Indirect_Y,      bytes: 2, cycles: 8 });

  // ISC/ISB
  table[0xE7] = Some(OP { code: 0xE7, name: "*ISB", op: isc, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0xF7] = Some(OP { code: 0xF7, name: "*ISB", op: isc, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0xEF] = Some(OP { code: 0xEF, name: "*ISB", op: isc, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0xFF] = Some(OP { code: 0xFF, name: "*ISB", op: isc, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0xFB] = Some(OP { code: 0xFB, name: "*ISB", op: isc, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0xE3] = Some(OP { code: 0xE3, name: "*ISB", op: isc, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0xF3] = Some(OP { code: 0xF3, name: "*ISB", op: isc, mode: Indirect_Y,      bytes: 2, cycles: 8 });

  // RLA
  table[0x27] = Some(OP { code: 0x27, name: "*RLA", op: rla, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x37] = Some(OP { code: 0x37, name: "*RLA", op: rla, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x2F] = Some(OP { code: 0x2F, name: "*RLA", op: rla, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x3F] = Some(OP { code: 0x3F, name: "*RLA", op: rla, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0x3B] = Some(OP { code: 0x3B, name: "*RLA", op: rla, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0x23] = Some(OP { code: 0x23, name: "*RLA", op: rla, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0x33] = Some(OP { code: 0x33, name: "*RLA", op: rla, mode: Indirect_Y,      bytes: 2, cycles: 8 });

  // RRA
  table[0x67] = Some(OP { code: 0x67, name: "*RRA", op: rra, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x77] = Some(OP { code: 0x77, name: "*RRA", op: rra, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x6F] = Some(OP { code: 0x6F, name: "*RRA", op: rra, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x7F] = Some(OP { code: 0x7F, name: "*RRA", op: rra, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0x7B] = Some(OP { code: 0x7B, name: "*RRA", op: rra, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0x63] = Some(OP { code: 0x63, name: "*RRA", op: rra, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0x73] = Some(OP { code: 0x73, name: "*RRA", op: rra, mode: Indirect_Y,      bytes: 2, cycles: 8 });

  // SLO
  table[0x07] = Some(OP { code: 0x07, name: "*SLO", op: slo, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x17] = Some(OP { code: 0x17, name: "*SLO", op: slo, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x0F] = Some(OP { code: 0x0F, name: "*SLO", op: slo, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x1F] = Some(OP { code: 0x1F, name: "*SLO", op: slo, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0x1B] = Some(OP { code: 0x1B, name: "*SLO", op: slo, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0x03] = Some(OP { code: 0x03, name: "*SLO", op: slo, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0x13] = Some(OP { code: 0x13, name: "*SLO", op: slo, mode: Indirect_Y,      bytes: 2, cycles: 8 });
  
  // SRE
  table[0x47] = Some(OP { code: 0x47, name: "*SRE", op: sre, mode: ZeroPage,        bytes: 2, cycles: 5 });
  table[0x57] = Some(OP { code: 0x57, name: "*SRE", op: sre, mode: ZeroPage_X,      bytes: 2, cycles: 6 });
  table[0x4F] = Some(OP { code: 0x4F, name: "*SRE", op: sre, mode: Absolute,        bytes: 3, cycles: 6 });
  table[0x5F] = Some(OP { code: 0x5F, name: "*SRE", op: sre, mode: Absolute_X,      bytes: 3, cycles: 7 });
  table[0x5B] = Some(OP { code: 0x5B, name: "*SRE", op: sre, mode: Absolute_Y,      bytes: 3, cycles: 7 });
  table[0x43] = Some(OP { code: 0x43, name: "*SRE", op: sre, mode: Indirect_X,      bytes: 2, cycles: 8 });
  table[0x53] = Some(OP { code: 0x53, name: "*SRE", op: sre, mode: Indirect_Y,      bytes: 2, cycles: 8 });

  // Duplicated
  table[0xEB] = Some(OP { code: 0xEB, name: "*SBC", op: sbc, mode: Immediate,       bytes: 2, cycles: 2 });

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

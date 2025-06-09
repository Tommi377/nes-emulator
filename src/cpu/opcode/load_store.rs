use crate::cpu::{opcode::opcode_table::AddressingMode, CPU};

pub(crate) fn lda(cpu: &mut CPU, mode: AddressingMode) {
  let addr= cpu.get_address(&mode);
  cpu.reg_a = cpu.mem_read_u8(addr);
  cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn ldx(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.reg_x = cpu.mem_read_u8(addr);
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}

pub(crate) fn ldy(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.reg_y = cpu.mem_read_u8(addr);
  cpu.update_zero_and_negative_flags(cpu.reg_y);
}

pub(crate) fn sta(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.mem_write_u8(addr, cpu.reg_a);
}

pub(crate) fn stx(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.mem_write_u8(addr, cpu.reg_x);
}

pub(crate) fn sty(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  cpu.mem_write_u8(addr, cpu.reg_y);
}

#[cfg(test)]
mod lda_test {
  use super::*;

  #[test]
  fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa5_lda_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x10, 0x55);
    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xb5_lda_zero_page_x() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u8((ptr + offset) as u16, data);

    cpu.load(vec![0xb5, ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.run();

    assert_eq!(cpu.reg_a, data);
  }

  #[test]
  fn test_0xad_lda_absolute() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1234, 0x55);
    cpu.load_and_run(vec![0xad, 0x34, 0x12, 0x00]);
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xbd_lda_absolute_x() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xbd, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_x = 0x01; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xb9_lda_absolute_y() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xb9, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_y = 0x01; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_a, 0x55);
  }

  #[test]
  fn test_0xa1_lda_indirect_x() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u16((indir_ptr + offset) as u16, ptr);
    cpu.mem_write_u8(ptr, data);

    cpu.load(vec![0xa1, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.run();

    assert_eq!(cpu.reg_a, data);
  }

  #[test]
  fn test_0xb1_lda_indirect_y() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u16(indir_ptr as u16, ptr);
    cpu.mem_write_u8(ptr + offset as u16, data);

    cpu.load(vec![0xb1, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_y = offset;
    cpu.run();

    assert_eq!(cpu.reg_a, data);
  }

  #[test]
  fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa9_lda_neg_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }
}

#[cfg(test)]
mod ldx_test {
  use super::*;

  #[test]
  fn test_0xa2_ldx_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.reg_x, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa6_ldx_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x10, 0x55);
    cpu.load_and_run(vec![0xa6, 0x10, 0x00]);
    assert_eq!(cpu.reg_x, 0x55);
  }

  #[test]
  fn test_0xb6_ldx_zero_page_y() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u8((ptr + offset) as u16, data);

    cpu.load(vec![0xb6, ptr, 0x00]);
    cpu.reset();

    cpu.reg_y = offset;
    cpu.run();

    assert_eq!(cpu.reg_x, data);
  }

  #[test]
  fn test_0xae_ldx_absolute() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1234, 0x55);
    cpu.load_and_run(vec![0xae, 0x34, 0x12, 0x00]);
    assert_eq!(cpu.reg_x, 0x55);
  }

  #[test]
  fn test_0xbe_ldx_absolute_y() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xbe, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_y = 1; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_x, 0x55);
  }

  #[test]
  fn test_0xa2_ldx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa2_ldx_neg_flag () {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }
}

#[cfg(test)]
mod ldy_test {
  use super::*;

  #[test]
  fn test_0xa0_ldy_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
    assert_eq!(cpu.reg_y, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa4_ldy_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x10, 0x55);
    cpu.load_and_run(vec![0xa4, 0x10, 0x00]);
    assert_eq!(cpu.reg_y, 0x55);
  }

  #[test]
  fn test_0xb4_ldy_zero_page_x() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.mem_write_u8((ptr + offset) as u16, data);

    cpu.load(vec![0xb4, ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.run();

    assert_eq!(cpu.reg_y, data);
  }

  #[test]
  fn test_0xac_ldy_absolute() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1234, 0x55);
    cpu.load_and_run(vec![0xac, 0x34, 0x12, 0x00]);
    assert_eq!(cpu.reg_y, 0x55);
  }

  #[test]
  fn test_0xbc_ldy_absolute_x() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0xbc, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_x = 1; // Offset
    cpu.run();
  
    assert_eq!(cpu.reg_y, 0x55);
  }

  #[test]
  fn test_0xa0_ldy_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa0_ldy_neg_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }
}

#[cfg(test)]
mod sta_test {
  use super::*;

  #[test]
  fn test_0x85_sta_zero_page() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x85, 0x10, 0x00]);
    cpu.reset();

    cpu.reg_a = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x10), 0x55);
  }

  #[test]
  fn test_0x95_sta_zero_page_x() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.load(vec![0x95, ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.reg_a = 0x55;

    cpu.run();
    assert_eq!(cpu.mem_read_u8((ptr + offset) as u16), data);
  }

  #[test]
  fn test_0x8d_sta_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8d, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_a = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x1234), 0x55);
  }

  #[test]
  fn test_0x9d_sta_absolute_x() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x9d, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_x = 0x01; // Offset
    cpu.reg_a = 0x55; // Data to store
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x1235), 0x55);
  }

  #[test]
  fn test_0x99_sta_absolute_y() {
    let mut cpu = CPU::new();
    cpu.mem_write_u8(0x1235, 0x55);
    cpu.load(vec![0x99, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_y = 0x01; // Offset
    cpu.reg_a = 0x55; // Data to store
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x1235), 0x55);
  }

  #[test]
  fn test_0x81_sta_indirect_x() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;

    cpu.mem_write_u16((indir_ptr + offset) as u16, ptr);

    cpu.load(vec![0x81, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.reg_a = 0x55; // Data to store
    cpu.run();

    assert_eq!(cpu.mem_read_u8(ptr), 0x55);
  }

  #[test]
  fn test_0x91_sta_indirect_y() {
    let mut cpu = CPU::new();

    let indir_ptr: u8 = 0x10;
    let ptr: u16 = 0x1234;
    let offset: u8 = 0x10;

    cpu.mem_write_u16(indir_ptr as u16, ptr);

    cpu.load(vec![0x91, indir_ptr, 0x00]);
    cpu.reset();

    cpu.reg_y = offset;
    cpu.reg_a = 0x55; // Data to store
    cpu.run();

    assert_eq!(cpu.mem_read_u8(ptr + offset as u16), 0x55);
  }
}

#[cfg(test)]
mod stx_test {
  use super::*;

  #[test]
  fn test_0x86_stx_zero_page() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x86, 0x10, 0x00]);
    cpu.reset();

    cpu.reg_x = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x10), 0x55);
  }

  #[test]
  fn test_0x96_stx_zero_page_y() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.load(vec![0x96, ptr, 0x00]);
    cpu.reset();

    cpu.reg_y = offset;
    cpu.reg_x = data;

    cpu.run();
    assert_eq!(cpu.mem_read_u8((ptr + offset) as u16), data);
  }

  #[test]
  fn test_0x8e_stx_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8e, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_x = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x1234), 0x55);
  }
}

#[cfg(test)]
mod sty_test {
  use super::*;

  #[test]
  fn test_0x84_sty_zero_page() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x84, 0x10, 0x00]);
    cpu.reset();

    cpu.reg_y = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x10), 0x55);
  }

  #[test]
  fn test_0x94_sty_zero_page_x() {
    let mut cpu = CPU::new();

    let ptr: u8 = 0x10;
    let offset: u8 = 0x10;
    let data: u8 = 0x55;

    cpu.load(vec![0x94, ptr, 0x00]);
    cpu.reset();

    cpu.reg_x = offset;
    cpu.reg_y = data;

    cpu.run();
    assert_eq!(cpu.mem_read_u8((ptr + offset) as u16), data);
  }

  #[test]
  fn test_0x8c_sty_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8c, 0x34, 0x12, 0x00]);
    cpu.reset();

    cpu.reg_y = 0x55;
    cpu.run();
  
    assert_eq!(cpu.mem_read_u8(0x1234), 0x55);
  }
}
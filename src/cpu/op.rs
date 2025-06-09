use crate::{cpu::{opcode::{AddressingMode, OPCODE_TABLE}, StatusFlag, CPU}, utils::set_bit};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct OP {
  pub code: u8,
  pub op: &'static str,
  pub mode: AddressingMode,
  pub bytes: u8,
  pub cycles: u8
}

impl OP {
  pub fn execute(&self, cpu: &mut CPU) {
    match self.op {
      "LDA" => Self::lda(cpu, &self.mode),
      "STA" => Self::sta(cpu, &self.mode),
      "TAX" => Self::tax(cpu),
      "INX" => Self::inx(cpu),
      "BRK" => Self::brk(cpu),
      _ => panic!("Unknown opcode: {} at PC: 0x{:04X}", self.op, cpu.pc),
    }
  }


  fn lda(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let addr= cpu.get_address(addressing_mode);
    cpu.reg_a = cpu.mem_read_u8(addr);
    cpu.update_zero_and_negative_flags(cpu.reg_a);
  }

  fn sta(cpu: &mut CPU, addressing_mode: &AddressingMode) {
    let addr = cpu.get_address(addressing_mode);
    cpu.mem_write_u8(addr, cpu.reg_a);
  }

  fn tax(cpu: &mut CPU) {
    cpu.reg_x = cpu.reg_a;
    cpu.update_zero_and_negative_flags(cpu.reg_x);
  }

  fn inx(cpu: &mut CPU) {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    cpu.update_zero_and_negative_flags(cpu.reg_x);
  }

  fn brk(cpu: &mut CPU) {
    cpu.status = set_bit(cpu.status, StatusFlag::Break as u8, true);
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
mod test {
  use super::*;

  mod lda_tests {
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

  mod sta_tests {
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

  mod tax_tests {
    use super::*;
    #[test]
    fn test_0xaa_tax_immediate_load_data() {
      let mut cpu = CPU::new();
      cpu.pc = 0x8000;
      cpu.reg_a = 5;
      cpu.load(vec![0xaa, 0x00]);
      cpu.run();
      assert_eq!(cpu.reg_x, 0x05);
      assert!(cpu.status & 0b0000_0010 == 0b00);
      assert!(cpu.status & 0b1000_0000 == 0);
    }
  
    #[test]
    fn test_0xaa_tax_zero_flag() {
      let mut cpu = CPU::new();
      cpu.pc = 0x8000;
      cpu.reg_a = 0;
      cpu.load(vec![0xaa, 0x00]);
      cpu.run();
      assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_neg_flag() {
      let mut cpu = CPU::new();
      cpu.pc = 0x8000;
      cpu.reg_a = 255;
      cpu.load(vec![0xaa, 0x00]);
      cpu.run();
      assert!(cpu.status & 0b1000_0000 != 0);
    }
  }

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

  mod brk_test {
    use super::*;
    #[test]
    fn test_inx_overflow() {
      let mut cpu = CPU::new();
      assert_eq!(cpu.status & StatusFlag::Break as u8, 0);
      cpu.load_and_run(vec![0x00]);
      assert_ne!(cpu.status & StatusFlag::Break as u8, 0);
    }
  }
}
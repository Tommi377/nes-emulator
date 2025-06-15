use crate::{
  bus::memory::Memory,
  cpu::{CPU, opcode::opcode_table::AddressingMode},
};

pub(crate) fn inc(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr).wrapping_add(1);
  cpu.mem_write_u8(addr, value);
  cpu.update_zero_and_negative_flags(value);
}

pub(crate) fn inx(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_x = cpu.reg_x.wrapping_add(1);
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}

pub(crate) fn iny(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_y = cpu.reg_y.wrapping_add(1);
  cpu.update_zero_and_negative_flags(cpu.reg_y);
}

pub(crate) fn dec(cpu: &mut CPU, mode: AddressingMode) {
  let addr = cpu.get_address(&mode);
  let value = cpu.mem_read_u8(addr).wrapping_sub(1);
  cpu.mem_write_u8(addr, value);
  cpu.update_zero_and_negative_flags(value);
}
pub(crate) fn dex(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_x = cpu.reg_x.wrapping_sub(1);
  cpu.update_zero_and_negative_flags(cpu.reg_x);
}
pub(crate) fn dey(cpu: &mut CPU, _mode: AddressingMode) {
  cpu.reg_y = cpu.reg_y.wrapping_sub(1);
  cpu.update_zero_and_negative_flags(cpu.reg_y);
}
#[cfg(test)]
mod increment_decrements_tests {
  use super::*;
  use crate::cpu::{StatusFlag, opcode::opcode_table::AddressingMode};

  // INC Tests
  mod inc_tests {
    use super::*;

    #[test]
    fn test_inc_zero_page() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x05);
      inc(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x06);
      assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
      assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    }

    #[test]
    fn test_inc_zero_page_x() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.reg_x = 0x05;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage base address
      cpu.mem_write_u8(0x15, 0x10); // 0x10 + 0x05 = 0x15
      inc(&mut cpu, AddressingMode::ZeroPage_X);
      assert_eq!(cpu.mem_read_u8(0x15), 0x11);
      assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
      assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
    }

    #[test]
    fn test_inc_absolute() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u16(0x0600, 0x1234); // Absolute address
      cpu.mem_write_u8(0x1234, 0x7F);
      inc(&mut cpu, AddressingMode::Absolute);
      assert_eq!(cpu.mem_read_u8(0x1234), 0x80);
      assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
      assert_ne!(cpu.status & StatusFlag::Negative as u8, 0);
    }

    #[test]
    fn test_inc_absolute_x() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.reg_x = 0x10;
      cpu.mem_write_u16(0x0600, 0x1234); // Absolute base address
      cpu.mem_write_u8(0x1244, 0xFE); // 0x1234 + 0x10 = 0x1244
      inc(&mut cpu, AddressingMode::Absolute_X);
      assert_eq!(cpu.mem_read_u8(0x1244), 0xFF);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inc_overflow() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0xFF);
      inc(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inc_to_negative() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x7F);
      inc(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x80);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inc_to_zero() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0xFF);
      inc(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }

  // INX Tests
  mod inx_tests {
    use super::*;

    #[test]
    fn test_inx_basic() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x05;
      inx(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x06);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inx_overflow() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0xFF;
      inx(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inx_to_negative() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x7F;
      inx(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x80);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inx_from_zero() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x00;
      inx(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x01);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_inx_multiple_overflow() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0xFF;
      inx(&mut cpu, AddressingMode::NoneAddressing);
      inx(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x01);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }

  // INY Tests
  mod iny_tests {
    use super::*;

    #[test]
    fn test_iny_basic() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x05;
      iny(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x06);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_iny_overflow() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0xFF;
      iny(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_iny_to_negative() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x7F;
      iny(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x80);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_iny_from_zero() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x00;
      iny(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x01);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_iny_multiple_overflow() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0xFF;
      iny(&mut cpu, AddressingMode::NoneAddressing);
      iny(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x01);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }

  // DEC Tests
  mod dec_tests {
    use super::*;

    #[test]
    fn test_dec_zero_page() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x05);
      dec(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x04);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_zero_page_x() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.reg_x = 0x05;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage base address
      cpu.mem_write_u8(0x15, 0x10); // 0x10 + 0x05 = 0x15
      dec(&mut cpu, AddressingMode::ZeroPage_X);
      assert_eq!(cpu.mem_read_u8(0x15), 0x0F);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_absolute() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u16(0x0600, 0x1234); // Absolute address
      cpu.mem_write_u8(0x1234, 0x80);
      dec(&mut cpu, AddressingMode::Absolute);
      assert_eq!(cpu.mem_read_u8(0x1234), 0x7F);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_absolute_x() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.reg_x = 0x10;
      cpu.mem_write_u16(0x0600, 0x1234); // Absolute base address
      cpu.mem_write_u8(0x1244, 0x02); // 0x1234 + 0x10 = 0x1244
      dec(&mut cpu, AddressingMode::Absolute_X);
      assert_eq!(cpu.mem_read_u8(0x1244), 0x01);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_underflow() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x00);
      dec(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0xFF);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_to_zero() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x01);
      dec(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dec_to_positive() {
      let mut cpu = CPU::new();
      cpu.pc = 0x0600;
      cpu.mem_write_u8(0x0600, 0x10); // ZeroPage address
      cpu.mem_write_u8(0x10, 0x80);
      dec(&mut cpu, AddressingMode::ZeroPage);
      assert_eq!(cpu.mem_read_u8(0x10), 0x7F);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }

  // DEX Tests
  mod dex_tests {
    use super::*;

    #[test]
    fn test_dex_basic() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x05;
      dex(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x04);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dex_underflow() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x00;
      dex(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0xFF);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dex_to_zero() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x01;
      dex(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dex_to_positive() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x80;
      dex(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0x7F);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dex_multiple_underflow() {
      let mut cpu = CPU::new();
      cpu.reg_x = 0x00;
      dex(&mut cpu, AddressingMode::NoneAddressing);
      dex(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_x, 0xFE);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }

  // DEY Tests
  mod dey_tests {
    use super::*;

    #[test]
    fn test_dey_basic() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x05;
      dey(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x04);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dey_underflow() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x00;
      dey(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0xFF);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dey_to_zero() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x01;
      dey(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x00);
      assert_ne!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dey_to_positive() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x80;
      dey(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0x7F);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_eq!(cpu.status & (StatusFlag::Negative as u8), 0);
    }

    #[test]
    fn test_dey_multiple_underflow() {
      let mut cpu = CPU::new();
      cpu.reg_y = 0x00;
      dey(&mut cpu, AddressingMode::NoneAddressing);
      dey(&mut cpu, AddressingMode::NoneAddressing);
      assert_eq!(cpu.reg_y, 0xFE);
      assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
      assert_ne!(cpu.status & (StatusFlag::Negative as u8), 0);
    }
  }
}

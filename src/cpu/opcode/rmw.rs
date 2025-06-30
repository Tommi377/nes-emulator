use crate::{
    cpu::{
        CPU, StatusFlag,
        opcode::{arithmetic::cpu_addition_with_carry, opcode_table::AddressingMode},
    },
    mem::Memory,
};

pub(crate) fn dcp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr).wrapping_sub(1);
    cpu.mem_write_u8(addr, value);
    cpu.set_flag(StatusFlag::Carry, cpu.reg_a >= value);
    cpu.set_flag(StatusFlag::Zero, cpu.reg_a == value);
    cpu.set_flag(
        StatusFlag::Negative,
        cpu.reg_a.wrapping_sub(value) & 0b1000_0000 != 0,
    )
}

pub(crate) fn isc(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr).wrapping_add(1);
    cpu.mem_write_u8(addr, value);
    cpu_addition_with_carry(cpu, value ^ 0xFF);
}

pub(crate) fn rla(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr);
    let mut result = value << 1;
    if cpu.get_flag(StatusFlag::Carry) {
        result += 1
    }
    cpu.mem_write_u8(addr, result);

    cpu.reg_a &= result;
    cpu.set_flag(StatusFlag::Carry, (value & 0b1000_0000) != 0);
    cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn rra(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr);
    let mut result = value >> 1;
    if cpu.get_flag(StatusFlag::Carry) {
        result += 0b1000_0000;
    }
    cpu.mem_write_u8(addr, result);
    cpu.set_flag(StatusFlag::Carry, value & 0b0000_0001 != 0);
    cpu_addition_with_carry(cpu, result);
}

pub(crate) fn slo(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr);
    let result = value << 1;
    cpu.mem_write_u8(addr, result);

    cpu.reg_a |= result;
    cpu.set_flag(StatusFlag::Carry, (value & 0b1000_0000) != 0);
    cpu.update_zero_and_negative_flags(cpu.reg_a);
}

pub(crate) fn sre(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr);
    let result = value >> 1;
    cpu.mem_write_u8(addr, result);

    cpu.reg_a ^= result;
    cpu.set_flag(StatusFlag::Carry, value & 0b0000_0001 != 0);
    cpu.update_zero_and_negative_flags(cpu.reg_a);
}

#[cfg(test)]
mod rmw_tests {
    use super::*;
    use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

    // DCP (DEC + CMP) Tests
    mod dcp_tests {
        use super::*;

        #[test]
        fn test_dcp_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x05;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10); // Zero page address
            cpu.mem_write_u8(0x10, 0x03); // Value to decrement

            dcp(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x02); // Memory decremented
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // A >= decremented value (5 >= 2)
            assert_eq!(cpu.status & StatusFlag::Zero as u8, 0); // A != decremented value
            assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Positive result
        }

        #[test]
        fn test_dcp_equal_comparison() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x04;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x05); // Will become 0x04 after decrement

            dcp(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x04);
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // A >= decremented value
            assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // A == decremented value
            assert_eq!(cpu.status & StatusFlag::Negative as u8, 0);
        }

        #[test]
        fn test_dcp_less_than_comparison() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x02;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x05); // Will become 0x04 after decrement

            dcp(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x04);
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // A < decremented value
            assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
            assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative result
        }

        #[test]
        fn test_dcp_underflow() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x50;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x00); // Will underflow to 0xFF

            dcp(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0xFF);
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // A < 0xFF
            assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
            assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Result of 0x50 - 0xFF is positive (0x51)
        }
    }

    // ISC (INC + SBC) Tests
    mod isc_tests {
        use super::*;

        #[test]
        fn test_isc_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x10;
            cpu.set_flag(StatusFlag::Carry, true); // SBC needs carry set for normal operation
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x05); // Will become 0x06 after increment

            isc(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x06); // Memory incremented
            assert_eq!(cpu.reg_a, 0x0A); // 0x10 - 0x06 = 0x0A
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // No borrow
        }

        #[test]
        fn test_isc_with_borrow() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x05;
            cpu.set_flag(StatusFlag::Carry, true);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x09); // Will become 0x0A after increment

            isc(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x0A);
            assert_eq!(cpu.reg_a, 0xFB); // 0x05 - 0x0A = 0xFB (with borrow)
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Borrow occurred
            assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Negative result
        }

        #[test]
        fn test_isc_overflow() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0xFF;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0xFF); // Will overflow to 0x00

            isc(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x00); // Memory overflowed
        }
    }

    // RLA (ROL + AND) Tests
    mod rla_tests {
        use super::*;

        #[test]
        fn test_rla_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1111_0000;
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0100_0001); // Will become 0b1000_0010 after ROL

            rla(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b1000_0010); // Memory rotated left
            assert_eq!(cpu.reg_a, 0b1000_0000); // A & rotated value
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 7 was 0
            assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Result is negative
        }

        #[test]
        fn test_rla_with_carry_in() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1111_1111;
            cpu.set_flag(StatusFlag::Carry, true);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0100_0000); // Will become 0b1000_0001 after ROL

            rla(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b1000_0001);
            assert_eq!(cpu.reg_a, 0b1000_0001); // A & rotated value
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 7 was 0
        }

        #[test]
        fn test_rla_with_carry_out() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1010_1010;
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1000_0001); // Will become 0b0000_0010 after ROL

            rla(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0000_0010);
            assert_eq!(cpu.reg_a, 0b0000_0010); // A & rotated value
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 7 was 1
        }

        #[test]
        fn test_rla_zero_result() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b0000_0000;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1111_1111);

            rla(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.reg_a, 0x00);
            assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
        }
    }

    // RRA (ROR + ADC) Tests
    mod rra_tests {
        use super::*;

        #[test]
        fn test_rra_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x10;
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0000_0110); // Will become 0b0000_0011 after ROR

            rra(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0000_0011); // Memory rotated right
            assert_eq!(cpu.reg_a, 0x13); // 0x10 + 0x03 = 0x13
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 0 was 0
        }

        #[test]
        fn test_rra_with_carry_in() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x10;
            cpu.set_flag(StatusFlag::Carry, true);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0000_0100); // Will become 0b1000_0010 after ROR

            rra(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b1000_0010);
            assert_eq!(cpu.reg_a, 0x92); // 0x10 + 0x82 = 0x92
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 0 was 0
        }

        #[test]
        fn test_rra_with_carry_out() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x10;
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0000_0101); // Will become 0b0000_0010 after ROR

            rra(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0000_0010);
            // Note: The carry flag here is from the addition operation, not the rotation
            // 0x10 + 0x02 = 0x12, which doesn't produce carry
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // No carry from addition
        }

        #[test]
        fn test_rra_rotation_carry_out() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0xF0;
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1111_1111); // Will become 0b0111_1111 after ROR, produces carry

            rra(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0111_1111);
            // 0xF0 + 0x7F + 1(carry from rotation) = 0x170, which sets carry flag
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Carry from addition
        }
    }

    // SLO (ASL + ORA) Tests
    mod slo_tests {
        use super::*;

        #[test]
        fn test_slo_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1111_0000;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b0100_0001); // Will become 0b1000_0010 after ASL

            slo(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b1000_0010); // Memory shifted left
            assert_eq!(cpu.reg_a, 0b1111_0010); // A | shifted value
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 7 was 0
            assert_ne!(cpu.status & StatusFlag::Negative as u8, 0); // Result is negative
        }

        #[test]
        fn test_slo_with_carry() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b0000_1111;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1000_0001); // Will become 0b0000_0010 after ASL

            slo(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0000_0010);
            assert_eq!(cpu.reg_a, 0b0000_1111); // A | shifted value
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 7 was 1
        }

        #[test]
        fn test_slo_zero_input() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0x00;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0x00);

            slo(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0x00);
            assert_eq!(cpu.reg_a, 0x00);
            assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
        }
    }

    // SRE (LSR + EOR) Tests
    mod sre_tests {
        use super::*;

        #[test]
        fn test_sre_basic_operation() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1111_0000;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1000_0110); // Will become 0b0100_0011 after LSR

            sre(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0100_0011); // Memory shifted right
            assert_eq!(cpu.reg_a, 0b1011_0011); // A ^ shifted value
            assert_eq!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 0 was 0
        }

        #[test]
        fn test_sre_with_carry() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b1111_0000;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1000_0111); // Will become 0b0100_0011 after LSR

            sre(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0100_0011);
            assert_eq!(cpu.reg_a, 0b1011_0011); // A ^ shifted value
            assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // Original bit 0 was 1
        }

        #[test]
        fn test_sre_zero_result() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b0100_0001;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1000_0010); // Will become 0b0100_0001 after LSR

            sre(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0100_0001);
            assert_eq!(cpu.reg_a, 0x00); // A ^ shifted value = 0
            assert_ne!(cpu.status & StatusFlag::Zero as u8, 0); // Zero flag set
        }

        #[test]
        fn test_sre_negative_result() {
            let mut cpu = CPU::new();
            cpu.reg_a = 0b0000_0000;
            cpu.pc = 0x0600;
            cpu.mem_write_u8(0x0600, 0x10);
            cpu.mem_write_u8(0x10, 0b1111_1110); // Will become 0b0111_1111 after LSR

            sre(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.mem_read_u8(0x10), 0b0111_1111);
            assert_eq!(cpu.reg_a, 0b0111_1111); // A ^ shifted value
            assert_eq!(cpu.status & StatusFlag::Negative as u8, 0); // Positive result
        }
    }

    // Test different addressing modes
    #[test]
    fn test_addressing_modes() {
        let mut cpu = CPU::new();

        // Test absolute addressing
        cpu.reg_a = 0x50;
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x34); // Low byte
        cpu.mem_write_u8(0x0601, 0x12); // High byte
        cpu.mem_write_u8(0x1234, 0x10);

        dcp(&mut cpu, AddressingMode::Absolute);

        assert_eq!(cpu.mem_read_u8(0x1234), 0x0F);
        assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // 0x50 >= 0x0F
    }

    #[test]
    fn test_edge_cases() {
        let mut cpu = CPU::new();

        // Test with maximum values
        cpu.reg_a = 0xFF;
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10);
        cpu.mem_write_u8(0x10, 0xFF);

        dcp(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(cpu.mem_read_u8(0x10), 0xFE);
        assert_ne!(cpu.status & StatusFlag::Carry as u8, 0); // 0xFF >= 0xFE
        assert_eq!(cpu.status & StatusFlag::Zero as u8, 0);
    }
}

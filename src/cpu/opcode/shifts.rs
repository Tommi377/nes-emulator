use crate::{
    cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode},
    mem::Memory,
};

pub(crate) fn asl(cpu: &mut CPU, mode: AddressingMode) {
    let (value, addr) = resolve_value_and_address(cpu, mode);

    let result = value << 1;

    match addr {
        Some(addr) => cpu.mem_write_u8(addr, result),
        None => cpu.reg_a = result,
    }

    cpu.set_flag(StatusFlag::Carry, value & 0b1000_0000 != 0);
    cpu.update_zero_and_negative_flags(result);
}

pub(crate) fn lsr(cpu: &mut CPU, mode: AddressingMode) {
    let (value, addr) = resolve_value_and_address(cpu, mode);

    let result = value >> 1;

    match addr {
        Some(addr) => cpu.mem_write_u8(addr, result),
        None => cpu.reg_a = result,
    }

    cpu.set_flag(StatusFlag::Carry, value & 0b0000_0001 != 0);
    cpu.update_zero_and_negative_flags(result);
}

pub(crate) fn rol(cpu: &mut CPU, mode: AddressingMode) {
    let (value, addr) = resolve_value_and_address(cpu, mode);

    let mut result = value << 1;
    if cpu.get_flag(StatusFlag::Carry) {
        result += 1
    }

    match addr {
        Some(addr) => cpu.mem_write_u8(addr, result),
        None => cpu.reg_a = result,
    }

    cpu.set_flag(StatusFlag::Carry, value & 0b1000_0000 != 0);
    cpu.update_zero_and_negative_flags(result);
}

pub(crate) fn ror(cpu: &mut CPU, mode: AddressingMode) {
    let (value, addr) = resolve_value_and_address(cpu, mode);

    let mut result = value >> 1;
    if cpu.get_flag(StatusFlag::Carry) {
        result += 0b1000_0000;
    }

    match addr {
        Some(addr) => cpu.mem_write_u8(addr, result),
        None => cpu.reg_a = result,
    }

    cpu.set_flag(StatusFlag::Carry, value & 0b0000_0001 != 0);
    cpu.update_zero_and_negative_flags(result);
}

fn resolve_value_and_address(cpu: &mut CPU, mode: AddressingMode) -> (u8, Option<u16>) {
    if mode == AddressingMode::Accumulator {
        (cpu.reg_a, None)
    } else {
        let addr = cpu.get_address(&mode);
        (cpu.mem_read_u8(addr), Some(addr))
    }
}

#[cfg(test)]
mod shift_tests {
    use super::*;
    use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};

    // ASL (Arithmetic Shift Left) Tests
    #[test]
    fn test_asl_accumulator_basic() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b0100_0010; // 66

        asl(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1000_0100); // 132
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true); // bit 7 is set
    }

    #[test]
    fn test_asl_accumulator_with_carry() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1100_0010; // 194

        asl(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1000_0100); // 132
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // bit 7 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_asl_accumulator_zero_result() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0000; // 128

        asl(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10); // Zero page address
        cpu.mem_write_u8(0x10, 0b0011_0011); // 51

        asl(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(cpu.mem_read_u8(0x10), 0b0110_0110); // 102
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    // LSR (Logical Shift Right) Tests
    #[test]
    fn test_lsr_accumulator_basic() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0100; // 132

        lsr(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b0100_0010); // 66
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false); // bit 7 is always 0 after LSR
    }

    #[test]
    fn test_lsr_accumulator_with_carry() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0101; // 133 (odd number)

        lsr(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b0100_0010); // 66
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // bit 0 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_lsr_accumulator_zero_result() {
        let mut cpu = CPU::new();
        cpu.reg_a = 1;

        lsr(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_lsr_memory() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10); // Zero page address
        cpu.mem_write_u8(0x10, 0b1100_1100); // 204

        lsr(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(cpu.mem_read_u8(0x10), 0b0110_0110); // 102
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    // ROL (Rotate Left) Tests
    #[test]
    fn test_rol_accumulator_basic_no_carry() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b0100_0010; // 66
        cpu.set_flag(StatusFlag::Carry, false);

        rol(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1000_0100); // 132
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_rol_accumulator_with_carry_in() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b0100_0010; // 66
        cpu.set_flag(StatusFlag::Carry, true); // Carry in

        rol(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1000_0101); // 133
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_rol_accumulator_with_carry_out() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1100_0010; // 194
        cpu.set_flag(StatusFlag::Carry, false);

        rol(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1000_0100); // 132
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Bit 7 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_rol_accumulator_full_rotation() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0001; // 129
        cpu.set_flag(StatusFlag::Carry, true); // Carry in

        rol(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b0000_0011); // 3
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Bit 7 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_rol_memory() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10); // Zero page address
        cpu.mem_write_u8(0x10, 0b0011_0011); // 51
        cpu.set_flag(StatusFlag::Carry, true);

        rol(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(cpu.mem_read_u8(0x10), 0b0110_0111); // 103
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    // ROR (Rotate Right) Tests
    #[test]
    fn test_ror_accumulator_basic_no_carry() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0100; // 132
        cpu.set_flag(StatusFlag::Carry, false);

        ror(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b0100_0010); // 66
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_ror_accumulator_with_carry_in() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0100; // 132
        cpu.set_flag(StatusFlag::Carry, true); // Carry in

        ror(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1100_0010); // 194
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_ror_accumulator_with_carry_out() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0101; // 133 (odd)
        cpu.set_flag(StatusFlag::Carry, false);

        ror(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b0100_0010); // 66
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Bit 0 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
    }

    #[test]
    fn test_ror_accumulator_full_rotation() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0b1000_0001; // 129
        cpu.set_flag(StatusFlag::Carry, true); // Carry in

        ror(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(cpu.reg_a, 0b1100_0000); // 192
        assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Bit 0 was set
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_ror_memory() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10); // Zero page address
        cpu.mem_write_u8(0x10, 0b1100_1100); // 204
        cpu.set_flag(StatusFlag::Carry, true);

        ror(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(cpu.mem_read_u8(0x10), 0b1110_0110); // 230
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    // Edge case tests
    #[test]
    fn test_all_operations_with_zero() {
        let mut cpu = CPU::new();

        // ASL with 0
        cpu.reg_a = 0;
        asl(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);

        // LSR with 0
        cpu.reg_a = 0;
        lsr(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);

        // ROL with 0 and no carry
        cpu.reg_a = 0;
        cpu.set_flag(StatusFlag::Carry, false);
        rol(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);

        // ROL with 0 and carry
        cpu.reg_a = 0;
        cpu.set_flag(StatusFlag::Carry, true);
        rol(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 1);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);

        // ROR with 0 and no carry
        cpu.reg_a = 0;
        cpu.set_flag(StatusFlag::Carry, false);
        ror(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), true);

        // ROR with 0 and carry
        cpu.reg_a = 0;
        cpu.set_flag(StatusFlag::Carry, true);
        ror(&mut cpu, AddressingMode::Accumulator);
        assert_eq!(cpu.reg_a, 0b1000_0000); // 128
        assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
        assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
    }

    #[test]
    fn test_resolve_value_and_address_accumulator() {
        let mut cpu = CPU::new();
        cpu.reg_a = 0x42;

        let (value, addr) = resolve_value_and_address(&mut cpu, AddressingMode::Accumulator);

        assert_eq!(value, 0x42);
        assert_eq!(addr, None);
    }

    #[test]
    fn test_resolve_value_and_address_memory() {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.mem_write_u8(0x0600, 0x10); // Zero page address
        cpu.mem_write_u8(0x10, 0x42); // Value at that address

        let (value, addr) = resolve_value_and_address(&mut cpu, AddressingMode::ZeroPage);

        assert_eq!(value, 0x42);
        assert_eq!(addr, Some(0x10));
    }
}

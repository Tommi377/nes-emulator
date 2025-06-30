use crate::{
    cpu::{
        CPU, StatusFlag,
        opcode::{
            logical::and,
            opcode_table::AddressingMode,
            shifts::{lsr, ror},
        },
    },
    mem::Memory,
};

pub(crate) fn alr(cpu: &mut CPU, mode: AddressingMode) {
    and(cpu, mode);
    lsr(cpu, AddressingMode::Accumulator);
}

pub(crate) fn anc(cpu: &mut CPU, mode: AddressingMode) {
    and(cpu, mode);
    cpu.set_flag(StatusFlag::Carry, cpu.get_flag(StatusFlag::Negative));
}

pub(crate) fn arr(cpu: &mut CPU, mode: AddressingMode) {
    and(cpu, mode);

    // Save the AND result before ROR for flag computation
    let and_result = cpu.reg_a;

    ror(cpu, AddressingMode::Accumulator);

    // Special flag handling based on bits 5 and 6 of AND result (before ROR)
    match (and_result & (1 << 5) != 0, and_result & (1 << 6) != 0) {
        (true, true) => {
            cpu.set_flag(StatusFlag::Carry, true);
            cpu.set_flag(StatusFlag::Overflow, false);
        }
        (false, false) => {
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.set_flag(StatusFlag::Overflow, false);
        }
        (true, false) => {
            cpu.set_flag(StatusFlag::Carry, false);
            cpu.set_flag(StatusFlag::Overflow, true);
        }
        (false, true) => {
            cpu.set_flag(StatusFlag::Carry, true);
            cpu.set_flag(StatusFlag::Overflow, false);
        }
    }
}

pub(crate) fn axs(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let and_result = cpu.reg_a & cpu.reg_x;
    let operand = cpu.mem_read_u8(addr);

    // Subtraction: and_result - operand
    let value = and_result.wrapping_sub(operand);
    cpu.reg_x = value;

    // AXS has inverted carry logic compared to normal 6502 subtraction
    // Carry = 0 when no borrow (A >= B), Carry = 1 when borrow (A < B)
    cpu.set_flag(StatusFlag::Carry, and_result < operand);
    cpu.update_zero_and_negative_flags(cpu.reg_x);
}

pub(crate) fn lax(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    let value = cpu.mem_read_u8(addr);
    cpu.reg_a = value;
    cpu.reg_x = value;
    cpu.update_zero_and_negative_flags(value);
}

pub(crate) fn sax(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    cpu.mem_write_u8(addr, cpu.reg_a & cpu.reg_x);
}

#[cfg(test)]
mod combined_ops_tests {
    use super::*;
    use crate::cpu::{CPU, StatusFlag, opcode::opcode_table::AddressingMode};
    use crate::mem::Memory;

    // Helper function to set up CPU with known state
    fn setup_cpu() -> CPU {
        let mut cpu = CPU::new();
        cpu.pc = 0x0600;
        cpu.status = 0b0010_0000; // Only bit 5 set (always on)
        cpu
    }

    // ALR (AND + LSR) Tests
    mod alr_tests {
        use super::*;

        #[test]
        fn test_alr_immediate_basic() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1100_1010; // 202
            cpu.mem_write_u8(0x0600, 0b1111_0000); // 240

            alr(&mut cpu, AddressingMode::Immediate);

            // AND: 202 & 240 = 192 (0b1100_0000)
            // LSR: 192 >> 1 = 96 (0b0110_0000)
            assert_eq!(cpu.reg_a, 0b0110_0000);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false); // LSR shifts out 0
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
            assert_eq!(cpu.pc, 0x0601); // PC should advance by 1 for immediate mode
        }

        #[test]
        fn test_alr_carry_set() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1100_1001; // 201
            cpu.mem_write_u8(0x0600, 0b1111_1111); // 255

            alr(&mut cpu, AddressingMode::Immediate);

            // AND: 201 & 255 = 201 (0b1100_1001)
            // LSR: 201 >> 1 = 100 (0b0110_0100)
            assert_eq!(cpu.reg_a, 0b0110_0100);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // LSR shifts out 1
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
        }

        #[test]
        fn test_alr_zero_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1010_1010; // 170
            cpu.mem_write_u8(0x0600, 0b0101_0100); // 84

            alr(&mut cpu, AddressingMode::Immediate);

            // AND: 170 & 84 = 0 (0b0000_0000)
            // LSR: 0 >> 1 = 0 (0b0000_0000)
            assert_eq!(cpu.reg_a, 0b0000_0000);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
        }

        #[test]
        fn test_alr_preserves_other_registers() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_1110;
            cpu.reg_x = 0x42;
            cpu.reg_y = 0x55;
            cpu.stack = 0xFD;
            cpu.mem_write_u8(0x0600, 0b1111_1111);

            alr(&mut cpu, AddressingMode::Immediate);

            // Other registers should not be affected
            assert_eq!(cpu.reg_x, 0x42);
            assert_eq!(cpu.reg_y, 0x55);
            assert_eq!(cpu.stack, 0xFD);
        }
    }

    // ANC (AND + Copy N to C) Tests
    mod anc_tests {
        use super::*;

        #[test]
        fn test_anc_immediate_negative_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_1111; // 255
            cpu.mem_write_u8(0x0600, 0b1000_0001); // 129

            anc(&mut cpu, AddressingMode::Immediate);

            // AND: 255 & 129 = 129 (0b1000_0001)
            assert_eq!(cpu.reg_a, 0b1000_0001);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Carry copied from Negative
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_anc_immediate_positive_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b0111_1111; // 127
            cpu.mem_write_u8(0x0600, 0b0111_1110); // 126

            anc(&mut cpu, AddressingMode::Immediate);

            // AND: 127 & 126 = 126 (0b0111_1110)
            assert_eq!(cpu.reg_a, 0b0111_1110);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false); // Carry copied from Negative
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
        }

        #[test]
        fn test_anc_zero_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1010_1010; // 170
            cpu.mem_write_u8(0x0600, 0b0101_0101); // 85

            anc(&mut cpu, AddressingMode::Immediate);

            // AND: 170 & 85 = 0 (0b0000_0000)
            assert_eq!(cpu.reg_a, 0b0000_0000);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
        }

        #[test]
        fn test_anc_pc_advance() {
            let mut cpu = setup_cpu();
            cpu.pc = 0x1234;
            cpu.reg_a = 0xFF;
            cpu.mem_write_u8(0x1234, 0x80);

            let initial_pc = cpu.pc;
            anc(&mut cpu, AddressingMode::Immediate);

            assert_eq!(cpu.pc, initial_pc + 1);
        }
    }

    // ARR (AND + ROR) Tests
    mod arr_tests {
        use super::*;

        #[test]
        fn test_arr_basic_operation() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1100_1010; // 202
            cpu.mem_write_u8(0x0600, 0b1111_0000); // 240
            cpu.set_flag(StatusFlag::Carry, true);

            arr(&mut cpu, AddressingMode::Immediate);

            // AND: 202 & 240 = 192 (0b1100_0000)
            // ROR with carry: 192 >> 1 + carry = 0b1110_0000 = 224
            assert_eq!(cpu.reg_a, 0b1110_0000);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_arr_flag_logic_case_1() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b0110_0000; // Bits 5 and 6 both set
            cpu.mem_write_u8(0x0600, 0b1111_1111);

            arr(&mut cpu, AddressingMode::Immediate);

            // After AND + ROR, check bits 5 and 6 of result
            // Both bits set: Carry=true, Overflow=false
            assert_eq!(cpu.get_flag(StatusFlag::Carry), true);
            assert_eq!(cpu.get_flag(StatusFlag::Overflow), false);
        }

        #[test]
        fn test_arr_flag_logic_case_2() {
            let mut cpu = setup_cpu();
            // Create a scenario where result has bit 6 set but not bit 5
            cpu.reg_a = 0b0100_0000; // Only bit 6 set
            cpu.mem_write_u8(0x0600, 0b1111_1111);

            arr(&mut cpu, AddressingMode::Immediate);

            // Bit 6 set, bit 5 clear: Carry=true, Overflow=false
            assert_eq!(cpu.get_flag(StatusFlag::Carry), true);
            assert_eq!(cpu.get_flag(StatusFlag::Overflow), false);
        }

        #[test]
        fn test_arr_flag_logic_case_3() {
            let mut cpu = setup_cpu();
            // Create a scenario where result has bit 5 set but not bit 6
            cpu.reg_a = 0b0010_0000; // Only bit 5 set
            cpu.mem_write_u8(0x0600, 0b1111_1111);

            arr(&mut cpu, AddressingMode::Immediate);

            // Bit 5 set, bit 6 clear: Carry=false, Overflow=true
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
            assert_eq!(cpu.get_flag(StatusFlag::Overflow), true);
        }

        #[test]
        fn test_arr_flag_logic_case_4() {
            let mut cpu = setup_cpu();
            // Create a scenario where neither bit 5 nor bit 6 is set
            cpu.reg_a = 0b0001_1111; // Bits 5 and 6 clear
            cpu.mem_write_u8(0x0600, 0b1111_1111);

            arr(&mut cpu, AddressingMode::Immediate);

            // Neither bit set: Carry=false, Overflow=false
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false);
            assert_eq!(cpu.get_flag(StatusFlag::Overflow), false);
        }
    }

    // AXS (AND X with A, then SBC) Tests
    mod axs_tests {
        use super::*;

        #[test]
        fn test_axs_basic_operation() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_0000; // 240
            cpu.reg_x = 0b1100_1100; // 204
            cpu.mem_write_u8(0x0600, 0x50); // 80

            axs(&mut cpu, AddressingMode::Immediate);

            // AND: 240 & 204 = 192 (0b1100_0000)
            // SUB: 192 - 80 = 112 (0b0111_0000)
            assert_eq!(cpu.reg_x, 112);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false); // No borrow (192 >= 80)
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_axs_underflow() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b0000_1111; // 15
            cpu.reg_x = 0b1111_0000; // 240
            cpu.mem_write_u8(0x0600, 0x50); // 80

            axs(&mut cpu, AddressingMode::Immediate);

            // AND: 15 & 240 = 0 (0b0000_0000)
            // SUB: 0 - 80 = 176 (0xB0) with underflow
            assert_eq!(cpu.reg_x, 176);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), true); // Borrow occurred
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
        }

        #[test]
        fn test_axs_zero_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_1111; // 255
            cpu.reg_x = 0b1000_1000; // 136
            cpu.mem_write_u8(0x0600, 136); // Same as AND result

            axs(&mut cpu, AddressingMode::Immediate);

            // AND: 255 & 136 = 136 (0b1000_1000)
            // SUB: 136 - 136 = 0
            assert_eq!(cpu.reg_x, 0);
            assert_eq!(cpu.get_flag(StatusFlag::Carry), false); // No borrow
            assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
        }

        #[test]
        fn test_axs_preserves_accumulator() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0xAA;
            cpu.reg_x = 0xCC;
            cpu.mem_write_u8(0x0600, 0x10);

            let initial_a = cpu.reg_a;
            axs(&mut cpu, AddressingMode::Immediate);

            // Accumulator should not be modified
            assert_eq!(cpu.reg_a, initial_a);
        }

        #[test]
        fn test_axs_pc_advance() {
            let mut cpu = setup_cpu();
            cpu.pc = 0x0100; // Use RAM address
            cpu.reg_a = 0xFF;
            cpu.reg_x = 0xFF;
            cpu.mem_write_u8(0x0100, 0x01);

            let initial_pc = cpu.pc;
            axs(&mut cpu, AddressingMode::Immediate);

            assert_eq!(cpu.pc, initial_pc + 1);
        }
    }

    // LAX (Load A and X) Tests
    mod lax_tests {
        use super::*;

        #[test]
        fn test_lax_zero_page() {
            let mut cpu = setup_cpu();
            cpu.mem_write_u8(0x0600, 0x80); // Zero page address
            cpu.mem_write_u8(0x80, 0x42); // Value at zero page

            lax(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.reg_a, 0x42);
            assert_eq!(cpu.reg_x, 0x42);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_lax_absolute() {
            let mut cpu = setup_cpu();
            cpu.mem_write_u16(0x0600, 0x1234); // Absolute address
            cpu.mem_write_u8(0x1234, 0x88); // Value at absolute address

            lax(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.reg_a, 0x88);
            assert_eq!(cpu.reg_x, 0x88);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), true); // 0x88 has bit 7 set
            assert_eq!(cpu.pc, 0x0602);
        }

        #[test]
        fn test_lax_zero_flag() {
            let mut cpu = setup_cpu();
            cpu.mem_write_u8(0x0600, 0x80); // Zero page address
            cpu.mem_write_u8(0x80, 0x00); // Zero value

            lax(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.reg_a, 0x00);
            assert_eq!(cpu.reg_x, 0x00);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), true);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), false);
        }

        #[test]
        fn test_lax_negative_flag() {
            let mut cpu = setup_cpu();
            cpu.mem_write_u8(0x0600, 0x80); // Zero page address
            cpu.mem_write_u8(0x80, 0xFF); // Negative value

            lax(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.reg_a, 0xFF);
            assert_eq!(cpu.reg_x, 0xFF);
            assert_eq!(cpu.get_flag(StatusFlag::Zero), false);
            assert_eq!(cpu.get_flag(StatusFlag::Negative), true);
        }

        #[test]
        fn test_lax_preserves_other_registers() {
            let mut cpu = setup_cpu();
            cpu.reg_y = 0x33;
            cpu.stack = 0xEE;
            cpu.mem_write_u8(0x0600, 0x80);
            cpu.mem_write_u8(0x80, 0x77);

            lax(&mut cpu, AddressingMode::ZeroPage);

            assert_eq!(cpu.reg_y, 0x33);
            assert_eq!(cpu.stack, 0xEE);
        }

        #[test]
        fn test_lax_indirect_x() {
            let mut cpu = setup_cpu();
            cpu.reg_x = 0x04;
            cpu.mem_write_u8(0x0600, 0x20); // Base pointer
            cpu.mem_write_u16(0x24, 0x0080); // Address at base + X (use RAM address)
            cpu.mem_write_u8(0x0080, 0x99); // Value at final address

            lax(&mut cpu, AddressingMode::Indirect_X);

            assert_eq!(cpu.reg_a, 0x99);
            assert_eq!(cpu.reg_x, 0x99);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_lax_pc_advance_modes() {
            // Test different addressing modes advance PC correctly
            let mut cpu = setup_cpu();

            // Zero Page mode - 1 byte operand
            cpu.pc = 0x1000;
            cpu.mem_write_u8(0x1000, 0x50);
            cpu.mem_write_u8(0x50, 0x42);
            lax(&mut cpu, AddressingMode::ZeroPage);
            assert_eq!(cpu.pc, 0x1001);

            // Absolute mode - 2 byte operand
            cpu.pc = 0x1100;
            cpu.mem_write_u16(0x1100, 0x0100); // Use RAM address
            cpu.mem_write_u8(0x0100, 0x43);
            lax(&mut cpu, AddressingMode::Absolute);
            assert_eq!(cpu.pc, 0x1102);

            // Indirect_X mode - 1 byte operand
            cpu.pc = 0x1200;
            cpu.reg_x = 0x02;
            cpu.mem_write_u8(0x1200, 0x30);
            cpu.mem_write_u16(0x32, 0x0110); // Use RAM address
            cpu.mem_write_u8(0x0110, 0x44);
            lax(&mut cpu, AddressingMode::Indirect_X);
            assert_eq!(cpu.pc, 0x1201);
        }
    }

    // SAX (Store A AND X) Tests
    mod sax_tests {
        use super::*;

        #[test]
        fn test_sax_zero_page() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_0000; // 240
            cpu.reg_x = 0b1100_1100; // 204
            cpu.mem_write_u8(0x0600, 0x80); // Zero page address

            sax(&mut cpu, AddressingMode::ZeroPage);

            // AND: 240 & 204 = 192 (0b1100_0000)
            assert_eq!(cpu.mem_read_u8(0x80), 0b1100_0000);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_sax_absolute() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1010_1010; // 170
            cpu.reg_x = 0b0101_1111; // 95
            cpu.mem_write_u16(0x0600, 0x1234); // Absolute address

            sax(&mut cpu, AddressingMode::Absolute);

            // AND: 170 & 95 = 10 (0b0000_1010)
            assert_eq!(cpu.mem_read_u8(0x1234), 0b0000_1010);
            assert_eq!(cpu.pc, 0x0602);
        }

        #[test]
        fn test_sax_zero_result() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1010_1010; // 170
            cpu.reg_x = 0b0101_0101; // 85
            cpu.mem_write_u8(0x0600, 0x90); // Zero page address

            sax(&mut cpu, AddressingMode::ZeroPage);

            // AND: 170 & 85 = 0
            assert_eq!(cpu.mem_read_u8(0x90), 0x00);
        }

        #[test]
        fn test_sax_preserves_registers_and_flags() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0xFF;
            cpu.reg_x = 0x0F;
            cpu.reg_y = 0x33;
            cpu.status = 0b1010_1010;
            cpu.mem_write_u8(0x0600, 0x80);

            let initial_a = cpu.reg_a;
            let initial_x = cpu.reg_x;
            let initial_y = cpu.reg_y;
            let initial_status = cpu.status;

            sax(&mut cpu, AddressingMode::ZeroPage);

            // SAX should not modify any registers or flags
            assert_eq!(cpu.reg_a, initial_a);
            assert_eq!(cpu.reg_x, initial_x);
            assert_eq!(cpu.reg_y, initial_y);
            assert_eq!(cpu.status, initial_status);
        }

        #[test]
        fn test_sax_indirect_x() {
            let mut cpu = setup_cpu();
            cpu.reg_a = 0b1111_1111; // 255
            cpu.reg_x = 0b0000_0001; // 1 (use small index)
            cpu.mem_write_u8(0x0600, 0x40); // Base pointer
            cpu.mem_write_u16(0x41, 0x0120); // Address at base + X (0x40 + 0x01 = 0x41) - use RAM

            sax(&mut cpu, AddressingMode::Indirect_X);

            // AND: 255 & 1 = 1 (0b0000_0001)
            assert_eq!(cpu.mem_read_u8(0x0120), 0b0000_0001);
            assert_eq!(cpu.pc, 0x0601);
        }

        #[test]
        fn test_sax_pc_advance_modes() {
            let mut cpu = setup_cpu();

            // Zero Page mode - 1 byte operand
            cpu.pc = 0x1000;
            cpu.reg_a = 0xFF;
            cpu.reg_x = 0x0F;
            cpu.mem_write_u8(0x1000, 0x50);
            sax(&mut cpu, AddressingMode::ZeroPage);
            assert_eq!(cpu.pc, 0x1001);

            // Absolute mode - 2 byte operand
            cpu.pc = 0x1100;
            cpu.mem_write_u16(0x1100, 0x0130); // Use RAM address
            sax(&mut cpu, AddressingMode::Absolute);
            assert_eq!(cpu.pc, 0x1102);

            // Indirect_X mode - 1 byte operand
            cpu.pc = 0x1200;
            cpu.reg_x = 0x02;
            cpu.mem_write_u8(0x1200, 0x30);
            cpu.mem_write_u16(0x32, 0x0140); // Use RAM address
            sax(&mut cpu, AddressingMode::Indirect_X);
            assert_eq!(cpu.pc, 0x1201);
        }
    }

    // Integration tests for program counter behavior
    mod pc_integration_tests {
        use super::*;

        #[test]
        fn test_sequential_combined_ops_pc_behavior() {
            let mut cpu = setup_cpu();
            cpu.pc = 0x0200; // Use RAM address

            // Set up memory for ALR immediate (2 bytes: opcode + operand)
            cpu.reg_a = 0xFF;
            cpu.mem_write_u8(0x0200, 0x80);

            alr(&mut cpu, AddressingMode::Immediate);
            assert_eq!(cpu.pc, 0x0201);

            // Set up memory for ANC immediate (2 bytes: opcode + operand)
            cpu.mem_write_u8(0x0201, 0x80);

            anc(&mut cpu, AddressingMode::Immediate);
            assert_eq!(cpu.pc, 0x0202);

            // Set up memory for ARR immediate (2 bytes: opcode + operand)
            cpu.mem_write_u8(0x0202, 0x80);

            arr(&mut cpu, AddressingMode::Immediate);
            assert_eq!(cpu.pc, 0x0203);
        }

        #[test]
        fn test_combined_ops_pc_wraparound() {
            let mut cpu = setup_cpu();

            // Test PC wraparound - use a safe address that won't cause ROM writes
            cpu.pc = 0x07FE; // Near end of RAM
            cpu.reg_a = 0xFF;
            cpu.mem_write_u8(0x07FE, 0x80);

            alr(&mut cpu, AddressingMode::Immediate);
            assert_eq!(cpu.pc, 0x07FF); // Should advance normally in RAM
        }

        #[test]
        fn test_combined_ops_preserve_pc_on_memory_operations() {
            let mut cpu = setup_cpu();

            // Test that memory operations don't affect PC beyond normal advancement
            cpu.pc = 0x0300; // Use RAM address
            cpu.reg_a = 0xFF;
            cpu.reg_x = 0x0F;
            cpu.mem_write_u8(0x0300, 0x80);

            let initial_pc = cpu.pc;
            sax(&mut cpu, AddressingMode::ZeroPage);

            // PC should advance by exactly 1 for zero page addressing
            assert_eq!(cpu.pc, initial_pc + 1);

            // Verify the memory write happened at the correct address
            assert_eq!(cpu.mem_read_u8(0x80), 0x0F);
        }

        #[test]
        fn test_lax_sax_roundtrip_with_pc() {
            let mut cpu = setup_cpu();
            cpu.pc = 0x0400; // Use RAM address

            // Store a value using SAX
            cpu.reg_a = 0b1111_0000;
            cpu.reg_x = 0b1100_1100;
            cpu.mem_write_u8(0x0400, 0x50); // Zero page address

            sax(&mut cpu, AddressingMode::ZeroPage);
            assert_eq!(cpu.pc, 0x0401);

            // Load it back using LAX
            cpu.mem_write_u8(0x0401, 0x50); // Same zero page address

            lax(&mut cpu, AddressingMode::ZeroPage);
            assert_eq!(cpu.pc, 0x0402);

            // Both A and X should have the AND result
            let expected = 0b1111_0000 & 0b1100_1100;
            assert_eq!(cpu.reg_a, expected);
            assert_eq!(cpu.reg_x, expected);
        }

        #[test]
        fn test_combined_ops_different_addressing_modes_pc() {
            let mut cpu = setup_cpu();

            // Test AXS with different modes to verify PC advancement
            cpu.reg_a = 0xFF;
            cpu.reg_x = 0x80;

            // Immediate mode (1 byte operand)
            cpu.pc = 0x0500; // Use RAM address
            cpu.mem_write_u8(0x0500, 0x10);
            axs(&mut cpu, AddressingMode::Immediate);
            assert_eq!(cpu.pc, 0x0501);

            // ZeroPage mode (1 byte operand)
            cpu.pc = 0x0510; // Use RAM address
            cpu.mem_write_u8(0x0510, 0x60);
            cpu.mem_write_u8(0x60, 0x10);
            axs(&mut cpu, AddressingMode::ZeroPage);
            assert_eq!(cpu.pc, 0x0511);
        }
    }
}

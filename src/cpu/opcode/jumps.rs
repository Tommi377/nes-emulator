use crate::cpu::{CPU, opcode::AddressingMode};

pub(crate) fn jmp(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    cpu.pc = addr;
}

pub(crate) fn jsr(cpu: &mut CPU, mode: AddressingMode) {
    let addr = cpu.get_address(&mode);
    cpu.stack_push_value_u16(cpu.pc.wrapping_sub(1));
    cpu.pc = addr;
}

pub(crate) fn rts(cpu: &mut CPU, _mode: AddressingMode) {
    let addr = cpu.stack_pull_value_u16();
    cpu.pc = addr.wrapping_add(1);
}

#[cfg(test)]
mod jump_tests {
    use super::*;
    use crate::{cpu::CPU, mem::Memory};
    mod jmp_tests {
        use super::*;

        #[test]
        fn test_jmp_absolute() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x1234); // Jump to address 0x1234

            jmp(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x1234);
        }

        #[test]
        fn test_jmp_absolute_zero_address() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0000); // Jump to address 0x0000

            jmp(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x0000);
        }

        #[test]
        fn test_jmp_absolute_max_address() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0xFFFF); // Jump to address 0xFFFF

            jmp(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0xFFFF);
        }

        #[test]
        fn test_jmp_indirect() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x1000); // Pointer to address 0x1000
            cpu.mem_write_u16(0x1000, 0x5678); // Target address stored at 0x1000

            jmp(&mut cpu, AddressingMode::Indirect);

            assert_eq!(cpu.pc, 0x5678);
        }

        #[test]
        fn test_jmp_indirect_page_boundary_bug() {
            // This test simulates the famous 6502 JMP indirect bug
            // When the indirect address is at a page boundary (e.g., 0x10FF),
            // the high byte should be read from 0x1000, not 0x1100
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x10FF); // Pointer to address 0x10FF (page boundary)
            cpu.mem_write_u8(0x10FF, 0x34); // Low byte of target address
            cpu.mem_write_u8(0x1000, 0x12); // High byte of target address (should be read from 0x1000, not 0x1100)
            cpu.mem_write_u8(0x1100, 0x56); // This should NOT be used as high byte

            jmp(&mut cpu, AddressingMode::Indirect);

            // The result should be 0x1234, not 0x5634
            assert_eq!(cpu.pc, 0x1234);
        }

        #[test]
        fn test_jmp_indirect_zero_pointer() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0000); // Pointer to address 0x0000
            cpu.mem_write_u16(0x0000, 0x0123); // Target address stored at 0x0000

            jmp(&mut cpu, AddressingMode::Indirect);

            assert_eq!(cpu.pc, 0x0123);
        }

        #[test]
        fn test_jmp_indirect_max_pointer() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0123); // Pointer to address 0xFFFE
            cpu.mem_write_u16(0x0123, 0x0321); // Target address stored at 0xFFFE

            jmp(&mut cpu, AddressingMode::Indirect);

            assert_eq!(cpu.pc, 0x0321);
        }

        #[test]
        fn test_jmp_absolute_with_different_initial_pc() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0020; // Different starting PC
            cpu.mem_write_u16(0x0020, 0x8888); // Jump to address 0x8888

            jmp(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x8888);
        }

        #[test]
        fn test_jmp_preserves_other_registers() {
            let mut cpu = CPU::new();
            // Set up initial register state
            cpu.reg_a = 0x42;
            cpu.reg_x = 0x55;
            cpu.reg_y = 0x66;
            cpu.status = 0b1010_1010;
            cpu.stack = 0xFD;

            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x3000); // Jump to address 0x3000

            jmp(&mut cpu, AddressingMode::Absolute);

            // JMP should only affect PC, not other registers or status
            assert_eq!(cpu.pc, 0x3000);
            assert_eq!(cpu.reg_a, 0x42);
            assert_eq!(cpu.reg_x, 0x55);
            assert_eq!(cpu.reg_y, 0x66);
            assert_eq!(cpu.status, 0b1010_1010);
            assert_eq!(cpu.stack, 0xFD);
        }

        #[test]
        fn test_jmp_can_jump_to_same_location() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0600); // Jump to same address (infinite loop)

            jmp(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x0600);
        }

        #[test]
        fn test_jmp_indirect_chain() {
            // Test multiple levels of indirection (though this would require multiple JMP instructions)
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0050); // First indirect pointer
            cpu.mem_write_u16(0x0050, 0x0150); // Target address

            jmp(&mut cpu, AddressingMode::Indirect);

            assert_eq!(cpu.pc, 0x0150);
        }
    }

    mod jsr_tests {
        use super::*;
        use crate::cpu::CPU;

        #[test]
        fn test_jsr_absolute() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0xFF; // Initialize stack pointer
            cpu.mem_write_u16(0x0600, 0x3000); // Jump to subroutine at 0x3000

            jsr(&mut cpu, AddressingMode::Absolute);

            // PC should be set to the subroutine address
            assert_eq!(cpu.pc, 0x3000);
            // Stack pointer should be decremented by 2 (16-bit push)
            assert_eq!(cpu.stack, 0xFD);
            // Return address should be PC after reading the target (0x0602 - 1 = 0x0601)
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x0601);
        }

        #[test]
        fn test_jsr_pushes_correct_return_address() {
            let mut cpu = CPU::new();
            cpu.pc = 0x1234;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x1234, 0x5678); // Target address

            jsr(&mut cpu, AddressingMode::Absolute);

            // PC should jump to target
            assert_eq!(cpu.pc, 0x5678);
            // Return address should be PC after reading target (0x1236 - 1 = 0x1235)
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x1235);
        }

        #[test]
        fn test_jsr_stack_wrapping() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0x01; // Near bottom of stack
            cpu.mem_write_u16(0x0600, 0x2000);

            jsr(&mut cpu, AddressingMode::Absolute);

            // Stack should wrap around
            assert_eq!(cpu.stack, 0xFF);
            // Return address should be stored at wrapped location (0x0602 - 1 = 0x0601)
            assert_eq!(cpu.mem_read_u16(0x0100), 0x0601);
        }

        #[test]
        fn test_jsr_zero_address() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x0600, 0x0000); // Jump to address 0x0000

            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x0000);
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x0601);
        }

        #[test]
        fn test_jsr_max_address() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x0600, 0xFFFF); // Jump to max address

            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0xFFFF);
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x0601);
        }

        #[test]
        fn test_jsr_preserves_registers_and_flags() {
            let mut cpu = CPU::new();
            // Set up initial register and flag state
            cpu.reg_a = 0x42;
            cpu.reg_x = 0x55;
            cpu.reg_y = 0x66;
            cpu.status = 0b1010_1010;

            cpu.pc = 0x0600;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x0600, 0x3000);

            jsr(&mut cpu, AddressingMode::Absolute);

            // JSR should only affect PC and stack, not other registers or status
            assert_eq!(cpu.pc, 0x3000);
            assert_eq!(cpu.reg_a, 0x42);
            assert_eq!(cpu.reg_x, 0x55);
            assert_eq!(cpu.reg_y, 0x66);
            assert_eq!(cpu.status, 0b1010_1010);
        }

        #[test]
        fn test_jsr_multiple_calls_stack_behavior() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFF;

            // First JSR call
            cpu.pc = 0x0600;
            cpu.mem_write_u16(0x0600, 0x0300);
            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x0300);
            assert_eq!(cpu.stack, 0xFD);
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x0601);

            // Second JSR call (nested subroutine)
            cpu.pc = 0x0300;
            cpu.mem_write_u16(0x0300, 0x0400);
            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x0400);
            assert_eq!(cpu.stack, 0xFB);
            assert_eq!(cpu.mem_read_u16(0x01FC), 0x0301);
            // First return address should still be there
            assert_eq!(cpu.mem_read_u16(0x01FE), 0x0601);
        }

        #[test]
        fn test_jsr_with_different_initial_stack_positions() {
            let mut cpu = CPU::new();

            // Test with stack at 0x80
            cpu.pc = 0x1000;
            cpu.stack = 0x80;
            cpu.mem_write_u16(0x1000, 0x2000);

            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x2000);
            assert_eq!(cpu.stack, 0x7E);
            assert_eq!(cpu.mem_read_u16(0x017F), 0x1001);
        }
    }

    mod rts_tests {
        use super::*;
        use crate::cpu::CPU;

        #[test]
        fn test_rts_basic() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFD; // Stack as if JSR was called
            cpu.mem_write_u16(0x01FE, 0x0601); // Return address on stack

            rts(&mut cpu, AddressingMode::NoneAddressing);

            // PC should be return address + 1
            assert_eq!(cpu.pc, 0x0602);
            // Stack pointer should be incremented by 2
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_rts_with_zero_return_address() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFD;
            cpu.mem_write_u16(0x01FE, 0x0000); // Return to address 0x0000

            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0x0001); // 0x0000 + 1
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_rts_with_max_return_address() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFD;
            cpu.mem_write_u16(0x01FE, 0xFFFE); // Return to near-max address

            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0xFFFF); // 0xFFFE + 1
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_rts_with_address_wrapping() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFD;
            cpu.mem_write_u16(0x01FE, 0xFFFF); // Max address

            rts(&mut cpu, AddressingMode::NoneAddressing);

            // Should wrap around to 0x0000
            assert_eq!(cpu.pc, 0x0000); // 0xFFFF + 1 wraps to 0x0000
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_rts_stack_wrapping() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFF; // Near top of stack
            cpu.mem_write_u16(0x0100, 0x1234); // Return address at wrapped location

            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0x1235); // 0x1234 + 1
            assert_eq!(cpu.stack, 0x01); // Stack wraps to 0x01
        }

        #[test]
        fn test_rts_preserves_registers_and_flags() {
            let mut cpu = CPU::new();
            // Set up initial register and flag state
            cpu.reg_a = 0x42;
            cpu.reg_x = 0x55;
            cpu.reg_y = 0x66;
            cpu.status = 0b1010_1010;

            cpu.stack = 0xFD;
            cpu.mem_write_u16(0x01FE, 0x2000);

            rts(&mut cpu, AddressingMode::NoneAddressing);

            // RTS should only affect PC and stack, not other registers or status
            assert_eq!(cpu.pc, 0x2001);
            assert_eq!(cpu.reg_a, 0x42);
            assert_eq!(cpu.reg_x, 0x55);
            assert_eq!(cpu.reg_y, 0x66);
            assert_eq!(cpu.status, 0b1010_1010);
        }

        #[test]
        fn test_rts_nested_subroutines() {
            let mut cpu = CPU::new();

            // Simulate two nested JSR calls
            cpu.stack = 0xFB; // Stack after two JSR calls
            cpu.mem_write_u16(0x01FC, 0x3001); // Inner subroutine return
            cpu.mem_write_u16(0x01FE, 0x0601); // Outer subroutine return

            // First RTS (return from inner subroutine)
            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0x3002); // 0x3001 + 1
            assert_eq!(cpu.stack, 0xFD);

            // Second RTS (return from outer subroutine)
            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0x0602); // 0x0601 + 1
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_rts_different_stack_positions() {
            let mut cpu = CPU::new();

            // Test with stack at different position
            cpu.stack = 0x7E; // After JSR from stack position 0x80
            cpu.mem_write_u16(0x017F, 0x1001); // Return address

            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.pc, 0x1002); // 0x1001 + 1
            assert_eq!(cpu.stack, 0x80);
        }
    }

    mod jsr_rts_integration_tests {
        use super::*;
        use crate::cpu::CPU;

        #[test]
        fn test_jsr_rts_round_trip() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x0600, 0x3000); // Subroutine address

            // Call subroutine
            jsr(&mut cpu, AddressingMode::Absolute);

            assert_eq!(cpu.pc, 0x3000);
            assert_eq!(cpu.stack, 0xFD);

            // Return from subroutine
            rts(&mut cpu, AddressingMode::NoneAddressing);

            // Should return to the instruction after the JSR (0x0602)
            assert_eq!(cpu.pc, 0x0602); // 0x0601 + 1
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_multiple_jsr_rts_calls() {
            let mut cpu = CPU::new();
            cpu.pc = 0x0600;
            cpu.stack = 0xFF;

            // First JSR
            cpu.mem_write_u16(0x0600, 0x0300);
            jsr(&mut cpu, AddressingMode::Absolute);
            assert_eq!(cpu.pc, 0x0300);
            assert_eq!(cpu.stack, 0xFD);

            // Second JSR (nested)
            cpu.pc = 0x0300;
            cpu.mem_write_u16(0x0300, 0x0400);
            jsr(&mut cpu, AddressingMode::Absolute);
            assert_eq!(cpu.pc, 0x0400);
            assert_eq!(cpu.stack, 0xFB);

            // First RTS (return from nested subroutine)
            rts(&mut cpu, AddressingMode::NoneAddressing);
            assert_eq!(cpu.pc, 0x0302); // Return to first subroutine
            assert_eq!(cpu.stack, 0xFD);

            // Second RTS (return from first subroutine)
            rts(&mut cpu, AddressingMode::NoneAddressing);
            assert_eq!(cpu.pc, 0x0602); // Return to main program
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_jsr_rts_preserves_state() {
            let mut cpu = CPU::new();
            // Set up initial state
            cpu.reg_a = 0xAA;
            cpu.reg_x = 0xBB;
            cpu.reg_y = 0xCC;
            cpu.status = 0b1100_0011;
            cpu.pc = 0x0600;
            cpu.stack = 0xFF;
            cpu.mem_write_u16(0x0600, 0x2000);

            // Save initial state
            let initial_a = cpu.reg_a;
            let initial_x = cpu.reg_x;
            let initial_y = cpu.reg_y;
            let initial_status = cpu.status;

            // JSR and RTS should not affect registers or flags
            jsr(&mut cpu, AddressingMode::Absolute);
            rts(&mut cpu, AddressingMode::NoneAddressing);

            assert_eq!(cpu.reg_a, initial_a);
            assert_eq!(cpu.reg_x, initial_x);
            assert_eq!(cpu.reg_y, initial_y);
            assert_eq!(cpu.status, initial_status);
            assert_eq!(cpu.pc, 0x0602);
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_deep_nesting_jsr_rts() {
            let mut cpu = CPU::new();
            cpu.stack = 0xFF;

            // Simulate deep nesting (4 levels)
            let addresses = [0x0100, 0x0200, 0x0300, 0x0400, 0x0500];

            // Call 4 nested subroutines
            for i in 0..4 {
                cpu.pc = addresses[i];
                cpu.mem_write_u16(addresses[i], addresses[i + 1]);
                jsr(&mut cpu, AddressingMode::Absolute);
            }

            // Should be 4 levels deep
            assert_eq!(cpu.pc, 0x0500);
            assert_eq!(cpu.stack, 0xF7); // 0xFF - (4 * 2)

            // Return from all 4 levels
            for i in (0..4).rev() {
                rts(&mut cpu, AddressingMode::NoneAddressing);
                assert_eq!(cpu.pc, addresses[i] + 2);
            }

            // Should be back to original stack position
            assert_eq!(cpu.stack, 0xFF);
        }

        #[test]
        fn test_jsr_rts_with_stack_boundary_conditions() {
            let mut cpu = CPU::new();

            // Test near stack boundary
            cpu.pc = 0x0600;
            cpu.stack = 0x01; // Very close to stack underflow
            cpu.mem_write_u16(0x0600, 0x3000);

            jsr(&mut cpu, AddressingMode::Absolute);
            assert_eq!(cpu.stack, 0xFF); // Should wrap around

            rts(&mut cpu, AddressingMode::NoneAddressing);
            assert_eq!(cpu.stack, 0x01); // Should wrap back
            assert_eq!(cpu.pc, 0x0602);
        }
    }
}

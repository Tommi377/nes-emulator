use crate::cpu::{CPU, StatusFlag, opcode::AddressingMode};

pub(crate) fn clc(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::Carry, false);
}

pub(crate) fn cld(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::Decimal, false);
}

pub(crate) fn cli(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::InterruptDisable, false);
}

pub(crate) fn clv(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::Overflow, false);
}

pub(crate) fn sec(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::Carry, true);
}

pub(crate) fn sed(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::Decimal, true);
}

pub(crate) fn sei(cpu: &mut CPU, _mode: AddressingMode) {
    cpu.set_flag(StatusFlag::InterruptDisable, true);
}

#[cfg(test)]
mod status_flag_changes_test {
    use super::*;
    use crate::cpu::CPU;

    #[test]
    fn test_clc() {
        let mut cpu = CPU::new();
        cpu.status |= StatusFlag::Carry as u8;
        clc(&mut cpu, AddressingMode::NoneAddressing);
        assert_eq!(cpu.status & (StatusFlag::Carry as u8), 0);
    }

    #[test]
    fn test_cld() {
        let mut cpu = CPU::new();
        cpu.status |= StatusFlag::Decimal as u8;
        cld(&mut cpu, AddressingMode::NoneAddressing);
        assert_eq!(cpu.status & (StatusFlag::Decimal as u8), 0);
    }

    #[test]
    fn test_cli() {
        let mut cpu = CPU::new();
        cpu.status |= StatusFlag::InterruptDisable as u8;
        cli(&mut cpu, AddressingMode::NoneAddressing);
        assert_eq!(cpu.status & (StatusFlag::InterruptDisable as u8), 0);
    }

    #[test]
    fn test_clv() {
        let mut cpu = CPU::new();
        cpu.status |= StatusFlag::Overflow as u8;
        clv(&mut cpu, AddressingMode::NoneAddressing);
        assert_eq!(cpu.status & (StatusFlag::Overflow as u8), 0);
    }

    #[test]
    fn test_sec() {
        let mut cpu = CPU::new();
        sec(&mut cpu, AddressingMode::NoneAddressing);
        assert_ne!(cpu.status & (StatusFlag::Carry as u8), 0);
    }

    #[test]
    fn test_sed() {
        let mut cpu = CPU::new();
        sed(&mut cpu, AddressingMode::NoneAddressing);
        assert_ne!(cpu.status & (StatusFlag::Decimal as u8), 0);
    }

    #[test]
    fn test_sei() {
        let mut cpu = CPU::new();
        sei(&mut cpu, AddressingMode::NoneAddressing);
        assert_ne!(cpu.status & (StatusFlag::InterruptDisable as u8), 0);
    }
}

use crate::cpu::CPU;

mod cpu;

fn main() {
    let mut cpu = CPU::new();
    println!("Register A: {}", cpu.reg_a);
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    println!("Register A: {}", cpu.reg_a);
}


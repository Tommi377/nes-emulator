use nes_emulator::cpu::CPU;

fn main() {
  let mut cpu = CPU::new();
  println!("Register A: {}", cpu.reg_a);
  cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
  println!("Register A: {}", cpu.reg_a);
}

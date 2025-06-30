use std::{env, fs};

use nes_emulator::{cpu::CPU, mem::rom::Rom};

const DEFAULT_FILE_PATH: &str = "nestest.nes";

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_FILE_PATH);
    let raw = fs::read(file_path).expect("Should have been able to read the file");

    let rom = Rom::new(&raw).unwrap();
    let mut cpu = CPU::new();
    cpu.insert_rom(rom);
    cpu.reset();
    cpu.pc = 0xC000; // Set the program counter to a specific address for testing
    cpu.stack = 0xFD; // Set the stack pointer to a specific value for testing

    cpu.run_with_callback(move |cpu: &mut CPU| {
        println!("{}", cpu.print_state());
    });
}

use nes_emulator::cpu::CPU;
use nes_emulator::mem::Memory;
use nes_emulator::mem::rom::Rom;
use rand::Rng;
use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use std::env;
use std::fs;

const DEFAULT_FILE_PATH: &str = "snake.nes";

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).map(|s| s.as_str()).unwrap_or(DEFAULT_FILE_PATH);
    let raw = fs::read(file_path).expect("Should have been able to read the file");

    let rom = Rom::new(&raw).unwrap();
    let mut cpu = CPU::new();
    cpu.insert_rom(rom);
    cpu.reset();

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::rng();

    cpu.run_with_callback(move |cpu: &mut CPU| {
        handle_user_input(cpu, &mut event_pump);
        println!("{:?}", cpu);
        cpu.mem_write_u8(0xfe, rng.random_range(1..16));

        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    });
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.mem_write_u8(0xff, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.mem_write_u8(0xff, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.mem_write_u8(0xff, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.mem_write_u8(0xff, 0x64);
            }
            _ => { /* do nothing */ }
        }
    }
}

fn color(byte: u8) -> sdl2::pixels::Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x0600 {
        let color_idx = cpu.mem_read_u8(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

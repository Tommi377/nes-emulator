use crate::mem::{Memory, rom::Rom};

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;
const PRG_START: u16 = 0x8000;
const END: u16 = 0xFFFF;

pub struct Bus {
  cpu_ram: [u8; 2048],
  rom: Option<Rom>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
  pub fn new() -> Self {
    Bus {
      cpu_ram: [0; 2048],
      rom: None,
    }
  }

  pub fn from_rom(rom: Rom) -> Self {
    Bus {
      cpu_ram: [0; 2048],
      rom: Some(rom),
    }
  }

  pub fn insert_rom(&mut self, rom: Rom) {
    self.rom = Some(rom);
  }
}

impl Memory for Bus {
  fn mem_read_u8(&self, addr: u16) -> u8 {
    match addr {
      RAM_START..=RAM_END => {
        let mirror_down_addr = addr & 0b00000111_11111111;
        self.cpu_ram[mirror_down_addr as usize]
      }
      PPU_START..=PPU_END => {
        let _mirror_down_addr = addr & 0b00100000_00000111;
        todo!("PPU is not supported yet (tried to access at {})", addr);
      }
      PRG_START..=END => self.read_prg_rom(addr),
      _ => {
        println!("Ignoring mem access at {}", addr);
        0
      }
    }
  }

  fn mem_write_u8(&mut self, addr: u16, data: u8) {
    match addr {
      RAM_START..=RAM_END => {
        let mem_addr = addr & 0b11111111111;
        self.cpu_ram[mem_addr as usize] = data;
      }
      PPU_START..=PPU_END => {
        let _mem_addr = addr & 0b00100000_00000111;
        todo!("PPU is not supported yet");
      }
      PRG_START..=END => panic!("Attempt to write to Cartridge ROM space"),
      _ => println!("Ignoring mem write-access at {}", addr),
    }
  }
}

impl Bus {
  fn read_prg_rom(&self, mut addr: u16) -> u8 {
    match &self.rom {
      Some(rom) => {
        addr -= 0x8000;
        if rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
          addr %= 0x4000;
        }
        rom.prg_rom[addr as usize]
      }
      None => {
        panic!("Trying to read ROM without a cartridge")
      }
    }
  }
}

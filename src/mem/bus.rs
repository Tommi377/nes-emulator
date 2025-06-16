use crate::mem::Memory;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;

pub struct Bus {
  cpu_ram: [u8; 2048],
  pc: u16,
}

impl Bus {
  pub fn new() -> Self {
    Bus {
      cpu_ram: [0; 2048],
      pc: 0,
    }
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
        todo!("PPU is not supported yet")
      }
      0xFFFC => self.pc as u8,
      0xFFFD => (self.pc >> 8) as u8,
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
      0xFFFC => self.pc = u16::from_le_bytes([data, (self.pc >> 8) as u8]),
      0xFFFD => self.pc = u16::from_le_bytes([self.pc as u8, data]),
      _ => {
        println!("Ignoring mem write-access at {}", addr);
      }
    }
  }
}

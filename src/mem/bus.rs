use crate::{
    mem::{Memory, rom::Rom},
    ppu::PPU,
};

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_START: u16 = 0x2000;
const PPU_END: u16 = 0x3FFF;
const PRG_START: u16 = 0x8000;
const END: u16 = 0xFFFF;

pub struct Bus {
    cpu_ram: [u8; 2048],
    rom: Option<Rom>,
    ppu: Option<PPU>,
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
            ppu: None,
        }
    }

    pub fn from_rom(rom: Rom) -> Self {
        let ppu = PPU::new(rom.chr_rom.clone(), rom.screen_mirroring);
        Bus {
            cpu_ram: [0; 2048],
            rom: Some(rom),
            ppu: Some(ppu),
        }
    }

    pub fn insert_rom(&mut self, rom: Rom) {
        let ppu = PPU::new(rom.chr_rom.clone(), rom.screen_mirroring);
        self.rom = Some(rom);
        self.ppu = Some(ppu);
    }

    pub fn tick(&mut self, count: u32) {
        if let Some(ppu) = &mut self.ppu {
            ppu.tick(count * 3);
        }
    }

    pub(crate) fn poll_nmi_status(&mut self) -> bool {
        if let Some(ppu) = self.ppu.as_mut() {
            if ppu.get_nmi_flag() {
                ppu.clear_nmi_flag();
                return true;
            }
        }
        false
    }
}

impl Memory for Bus {
    fn mem_read_u8(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_ram[mirror_down_addr as usize]
            }
            PPU_START..=PPU_END => self
                .ppu
                .as_mut()
                .map(|ppu| match addr & 0b00100000_00000111 {
                    0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                        panic!("Attempt to read from write-only PPU address {:x}", addr);
                    }
                    0x2007 => ppu.read_data(),
                    _ => panic!("PPU register read not implemented for address {:x}", addr),
                })
                .unwrap_or_else(|| {
                    panic!("Attempt to read from PPU without a PPU instance");
                }),
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
            PPU_START..=PPU_END => self
                .ppu
                .as_mut()
                .map(|ppu| match addr & 0b00100000_00000111 {
                    0x2000 => {
                        ppu.write_to_ctrl(data);
                    }
                    0x2006 => {
                        ppu.write_to_ppu_addr(data);
                    }
                    0x2007 => {
                        ppu.write_to_data(data);
                    }
                    _ => panic!("PPU register write not implemented for address {:x}", addr),
                })
                .unwrap_or_else(|| {
                    panic!("Attempt to write to PPU without a PPU instance");
                }),
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

#[cfg(test)]
mod bus_tests {
    use super::super::bus::Bus;
    use super::super::{Memory, rom::Rom};

    #[test]
    fn test_bus_new() {
        let bus = Bus::new();

        // Test reading from uninitialized RAM returns 0
        let mut bus = bus;
        assert_eq!(bus.mem_read_u8(0x0000), 0);
        assert_eq!(bus.mem_read_u8(0x0100), 0);
        assert_eq!(bus.mem_read_u8(0x07FF), 0);
    }

    #[test]
    fn test_bus_ram_read_write() {
        let mut bus = Bus::new();

        // Test basic RAM operations
        bus.mem_write_u8(0x0000, 0x42);
        assert_eq!(bus.mem_read_u8(0x0000), 0x42);

        bus.mem_write_u8(0x0100, 0xFF);
        assert_eq!(bus.mem_read_u8(0x0100), 0xFF);

        // Test different RAM addresses
        bus.mem_write_u8(0x07FF, 0xAA);
        assert_eq!(bus.mem_read_u8(0x07FF), 0xAA);
    }

    #[test]
    fn test_bus_ram_mirroring() {
        let mut bus = Bus::new();

        // RAM is mirrored every 2KB (0x800 bytes)
        bus.mem_write_u8(0x0000, 0x12);
        assert_eq!(bus.mem_read_u8(0x0800), 0x12); // First mirror
        assert_eq!(bus.mem_read_u8(0x1000), 0x12); // Second mirror
        assert_eq!(bus.mem_read_u8(0x1800), 0x12); // Third mirror

        // Test writing to mirrored address affects original
        bus.mem_write_u8(0x0801, 0x34);
        assert_eq!(bus.mem_read_u8(0x0001), 0x34);
        assert_eq!(bus.mem_read_u8(0x1001), 0x34);
        assert_eq!(bus.mem_read_u8(0x1801), 0x34);
    }

    #[test]
    fn test_bus_16_bit_operations() {
        let mut bus = Bus::new();

        // Test 16-bit read/write
        bus.mem_write_u16(0x0000, 0x1234);
        assert_eq!(bus.mem_read_u16(0x0000), 0x1234);

        // Test little-endian byte order
        assert_eq!(bus.mem_read_u8(0x0000), 0x34); // Low byte
        assert_eq!(bus.mem_read_u8(0x0001), 0x12); // High byte
    }

    #[test]
    fn test_bus_from_rom() {
        // Create a test ROM
        let rom_data = create_test_rom_data();
        let rom = Rom::new(&rom_data).unwrap();

        let mut bus = Bus::from_rom(rom);

        // Test that bus was created with ROM
        // We can test this by reading from ROM space
        let prg_data = bus.mem_read_u8(0x8000);
        assert_eq!(prg_data, 0xAA); // Test ROM data should be 0xAA
    }

    #[test]
    fn test_bus_insert_rom() {
        let mut bus = Bus::new();

        let rom_data = create_test_rom_data();
        let rom = Rom::new(&rom_data).unwrap();

        bus.insert_rom(rom);

        // Test reading from ROM space
        let prg_data = bus.mem_read_u8(0x8000);
        assert_eq!(prg_data, 0xAA);
    }

    #[test]
    fn test_bus_rom_read_operations() {
        let mut bus = Bus::new();

        let rom_data = create_test_rom_data();
        let rom = Rom::new(&rom_data).unwrap();
        bus.insert_rom(rom);

        // Test reading different ROM addresses
        assert_eq!(bus.mem_read_u8(0x8000), 0xAA);
        assert_eq!(bus.mem_read_u8(0x8001), 0xAA);
        assert_eq!(bus.mem_read_u8(0xFFFF), 0xAA);
    }

    #[test]
    #[should_panic(expected = "Attempt to write to Cartridge ROM space")]
    fn test_bus_rom_write_protection() {
        let mut bus = Bus::new();

        let rom_data = create_test_rom_data();
        let rom = Rom::new(&rom_data).unwrap();
        bus.insert_rom(rom);

        // Writing to ROM space should panic
        bus.mem_write_u8(0x8000, 0x42);
    }

    #[test]
    #[should_panic(expected = "Trying to read ROM without a cartridge")]
    fn test_bus_rom_read_without_cartridge() {
        let mut bus = Bus::new();

        // Reading from ROM space without a cartridge should panic
        bus.mem_read_u8(0x8000);
    }

    #[test]
    fn test_bus_tick_with_ppu() {
        let rom_data = create_test_rom_data();
        let rom = Rom::new(&rom_data).unwrap();
        let mut bus = Bus::from_rom(rom);

        // Tick with PPU should work (PPU gets 3x the cycles)
        bus.tick(1);
        bus.tick(100);

        // No direct way to test PPU cycle count due to private fields,
        // but we can test that it doesn't panic
    }

    // Helper function to create test ROM data
    fn create_test_rom_data() -> Vec<u8> {
        let mut rom_data = Vec::new();

        // NES header
        rom_data.extend_from_slice(b"NES\x1A"); // NES tag
        rom_data.push(1); // PRG ROM size (1 * 16KB)
        rom_data.push(1); // CHR ROM size (1 * 8KB)
        rom_data.push(0); // Control byte 1
        rom_data.push(0); // Control byte 2
        rom_data.extend_from_slice(&[0; 8]); // Unused header bytes

        // PRG ROM data (16KB filled with 0xAA)
        rom_data.extend_from_slice(&vec![0xAA; 16 * 1024]);

        // CHR ROM data (8KB filled with 0xBB)
        rom_data.extend_from_slice(&vec![0xBB; 8 * 1024]);

        rom_data
    }
}

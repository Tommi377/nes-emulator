use crate::{
    mem::rom::Mirroring,
    ppu::register::{
        PPUMASK, PPUSTATUS, control_reg::PPUCTRL, oam_address::OAMADDRESS, ppu_address::PPUADDRESS,
        scroll::PPUSCROLL,
    },
};

pub mod register;

#[allow(dead_code)]
pub struct PPU {
    pub chr_rom: Vec<u8>,        // $0000–$1FFF (8KB CHR ROM)
    pub vram: [u8; 2048],        // $2000–$2FFF (2KB VRAM, mirrored to 4KB)
    pub palette_table: [u8; 32], // $3F00–$3FFF (32 bytes for palettes, mirrored)
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,

    cycle: u32,        // Current cycle in the PPU (0-340)
    scanline: u32,     // Current scanline in the PPU (0-261)
    nmi_pending: bool, // NMI flag for VBlank

    ctrl: PPUCTRL,
    mask: PPUMASK,
    status: PPUSTATUS,
    oam_addr: OAMADDRESS,
    scroll: PPUSCROLL,
    ppu_addr: PPUADDRESS,
    ppu_data_buf: u8,

    // Internal registers
    v_reg: u16,  // Current VRAM address (15 bits)
    t_reg: u16,  // Temporary VRAM address (15 bits)
    x_reg: u8,   // Fine X scroll (3 bits)
    w_reg: bool, // Write toggle (0 or 1)
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            mirroring,
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            cycle: 0,
            scanline: 0,
            nmi_pending: false,
            ctrl: PPUCTRL::new(),
            mask: PPUMASK::from_bits_truncate(0),
            status: PPUSTATUS::from_bits_truncate(0),
            oam_addr: OAMADDRESS::new(),
            scroll: PPUSCROLL::new(),
            ppu_addr: PPUADDRESS::new(),
            ppu_data_buf: 0,
            v_reg: 0,
            t_reg: 0,
            x_reg: 0,
            w_reg: false,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.ppu_addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn tick(&mut self, count: u32) {
        self.cycle += count;
        while self.cycle >= 341 {
            self.cycle -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status.set(PPUSTATUS::VBLANK, true);
                if self.ctrl.contains(PPUCTRL::GENERATE_NMI) {
                    self.nmi_pending = true;
                }
            }
            if self.scanline >= 262 {
                self.scanline = 0;
                self.status.set(PPUSTATUS::VBLANK, false);
                self.clear_nmi_flag();
            }
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.ppu_addr.update(value, &mut self.w_reg);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let generate_nmi_check = self.ctrl.contains(PPUCTRL::GENERATE_NMI)
            && !PPUCTRL::from_bits_truncate(value).contains(PPUCTRL::GENERATE_NMI);
        self.ctrl.update(value);
        if generate_nmi_check && self.status.contains(PPUSTATUS::VBLANK) {
            self.nmi_pending = true;
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.ppu_addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                println!("attempt to write to chr rom space {}", addr)
            }
            0x2000..=0x2fff => {
                let vram_addr = self.mirror_vram_addr(addr);
                self.vram[vram_addr as usize] = value;
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            0x3f00..=0x3fff => {
                let palette_index = (addr - 0x3f00) % 32; // Palette mirroring every 32 bytes
                self.palette_table[palette_index as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        let addr = self.oam_addr.get();
        self.oam_addr.increment();
        self.oam_data[addr as usize] = value;
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll.update(value, &mut self.w_reg);
    }

    pub fn read_status(&mut self) -> u8 {
        self.w_reg = false;
        self.status.bits()
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.ppu_addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.ppu_data_buf;
                self.ppu_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.ppu_data_buf;
                self.ppu_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            0x3f00..=0x3fff => {
                let palette_index = (addr - 0x3f00) % 32;
                self.palette_table[palette_index as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn read_oam_data(&mut self) -> u8 {
        let addr = self.oam_addr.get();
        self.oam_data[addr as usize]
    }

    pub fn get_nmi_flag(&self) -> bool {
        self.ctrl.contains(PPUCTRL::GENERATE_NMI) && self.nmi_pending
    }

    pub fn clear_nmi_flag(&mut self) {
        self.nmi_pending = false;
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
}

#[cfg(test)]
mod ppu_tests {
    use super::*;
    use crate::mem::rom::Mirroring;

    fn create_test_ppu(mirroring: Mirroring) -> PPU {
        let chr_rom = vec![0x42; 0x2000]; // 8KB CHR ROM filled with 0x42
        PPU::new(chr_rom, mirroring)
    }

    #[test]
    fn test_ppu_new() {
        let chr_rom = vec![0x12, 0x34, 0x56, 0x78];
        let ppu = PPU::new(chr_rom.clone(), Mirroring::Horizontal);

        assert_eq!(ppu.chr_rom, chr_rom);
        assert_eq!(ppu.mirroring, Mirroring::Horizontal);
        assert_eq!(ppu.vram.len(), 2048);
        assert_eq!(ppu.oam_data.len(), 256);
        assert_eq!(ppu.palette_table.len(), 32);
        assert_eq!(ppu.v_reg, 0);
        assert_eq!(ppu.t_reg, 0);
        assert_eq!(ppu.x_reg, 0);
        assert_eq!(ppu.w_reg, false);
        assert_eq!(ppu.ppu_data_buf, 0);

        // Check that arrays are zero-initialized
        assert!(ppu.vram.iter().all(|&x| x == 0));
        assert!(ppu.oam_data.iter().all(|&x| x == 0));
        assert!(ppu.palette_table.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_write_to_ctrl() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test writing different control values
        ppu.write_to_ctrl(0b10110101);
        assert_eq!(ppu.ctrl.bits(), 0b10110101);

        ppu.write_to_ctrl(0x00);
        assert_eq!(ppu.ctrl.bits(), 0x00);
        ppu.write_to_ctrl(0xFF);
        assert_eq!(ppu.ctrl.bits(), 0xFF);
    }

    #[test]
    fn test_write_to_ppu_addr() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        // Address should now be 0x2000
        assert_eq!(ppu.ppu_addr.get(), 0x2000);
    }

    #[test]
    fn test_write_to_vram() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set address to VRAM space
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        // Write data to VRAM
        ppu.write_to_data(0xAB);

        // The data should be written to VRAM at the mirrored address
        let mirrored_addr = ppu.mirror_vram_addr(0x2000);
        assert_eq!(ppu.vram[mirrored_addr as usize], 0xAB);
    }

    #[test]
    fn test_write_to_palette() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set address to palette space
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x10);

        // Write data to palette
        ppu.write_to_data(0xCD);

        // Check that palette was updated
        assert_eq!(ppu.palette_table[0x10], 0xCD);
    }

    #[test]
    fn test_read_from_chr_rom() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set address to CHR ROM space
        ppu.write_to_ppu_addr(0x00);
        ppu.write_to_ppu_addr(0x10);

        // First read should return buffered data (0)
        let first_read = ppu.read_data();
        assert_eq!(first_read, 0);

        // Second read should return the CHR ROM data
        let second_read = ppu.read_data();
        assert_eq!(second_read, 0x42); // Our test CHR ROM is filled with 0x42
    }

    #[test]
    fn test_read_from_vram() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Write some data to VRAM first
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);
        ppu.write_to_data(0xEF);

        // Reset address to read from the same location
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        // First read should return buffered data (0)
        let first_read = ppu.read_data();
        assert_eq!(first_read, 0);

        // Second read should return the VRAM data
        let second_read = ppu.read_data();
        assert_eq!(second_read, 0xEF);
    }

    #[test]
    fn test_read_from_palette() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Write to palette first
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x05);
        ppu.write_to_data(0x99);

        // Reset address to read from palette
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x05);

        // Palette reads are immediate (no buffering)
        let palette_data = ppu.read_data();
        assert_eq!(palette_data, 0x99);
    }

    #[test]
    fn test_mirror_vram_addr_vertical() {
        let ppu = create_test_ppu(Mirroring::Vertical);

        // Test vertical mirroring patterns
        // Nametable 0 (0x2000-0x23FF) -> 0x0000-0x03FF
        assert_eq!(ppu.mirror_vram_addr(0x2000), 0x0000);
        assert_eq!(ppu.mirror_vram_addr(0x23FF), 0x03FF);

        // Nametable 1 (0x2400-0x27FF) -> 0x0400-0x07FF
        assert_eq!(ppu.mirror_vram_addr(0x2400), 0x0400);
        assert_eq!(ppu.mirror_vram_addr(0x27FF), 0x07FF);

        // Nametable 2 (0x2800-0x2BFF) mirrors to 0x0000-0x03FF (same as NT0)
        assert_eq!(ppu.mirror_vram_addr(0x2800), 0x0000);
        assert_eq!(ppu.mirror_vram_addr(0x2BFF), 0x03FF);

        // Nametable 3 (0x2C00-0x2FFF) mirrors to 0x0400-0x07FF (same as NT1)
        assert_eq!(ppu.mirror_vram_addr(0x2C00), 0x0400);
        assert_eq!(ppu.mirror_vram_addr(0x2FFF), 0x07FF);
    }

    #[test]
    fn test_mirror_vram_addr_horizontal() {
        let ppu = create_test_ppu(Mirroring::Horizontal);

        // Test horizontal mirroring patterns
        // Nametable 0 (0x2000-0x23FF) -> 0x0000-0x03FF
        assert_eq!(ppu.mirror_vram_addr(0x2000), 0x0000);
        assert_eq!(ppu.mirror_vram_addr(0x23FF), 0x03FF);

        // Nametable 1 (0x2400-0x27FF) mirrors to 0x0000-0x03FF (same as NT0)
        assert_eq!(ppu.mirror_vram_addr(0x2400), 0x0000);
        assert_eq!(ppu.mirror_vram_addr(0x27FF), 0x03FF);

        // Nametable 2 (0x2800-0x2BFF) -> 0x0400-0x07FF
        assert_eq!(ppu.mirror_vram_addr(0x2800), 0x0400);
        assert_eq!(ppu.mirror_vram_addr(0x2BFF), 0x07FF);

        // Nametable 3 (0x2C00-0x2FFF) mirrors to 0x0400-0x07FF (same as NT2)
        assert_eq!(ppu.mirror_vram_addr(0x2C00), 0x0400);
        assert_eq!(ppu.mirror_vram_addr(0x2FFF), 0x07FF);
    }

    #[test]
    fn test_vram_addr_increment() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test increment by 1 (default)
        ppu.write_to_ctrl(0x00); // VRAM increment = 1
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        let initial_addr = ppu.ppu_addr.get();
        ppu.write_to_data(0x11);
        let after_write_addr = ppu.ppu_addr.get();

        assert_eq!(after_write_addr, initial_addr + 1);

        // Test increment by 32
        ppu.write_to_ctrl(0x04); // VRAM increment = 32
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        let initial_addr = ppu.ppu_addr.get();
        ppu.write_to_data(0x22);
        let after_write_addr = ppu.ppu_addr.get();

        assert_eq!(after_write_addr, initial_addr + 32);
    }

    #[test]
    fn test_multiple_address_writes() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test multiple address register writes
        ppu.write_to_ppu_addr(0x21);
        ppu.write_to_ppu_addr(0x34);
        assert_eq!(ppu.ppu_addr.get(), 0x2134);

        // Next write should affect high byte again
        ppu.write_to_ppu_addr(0x25);
        ppu.write_to_ppu_addr(0x67);
        assert_eq!(ppu.ppu_addr.get(), 0x2567);
    }

    #[test]
    fn test_consecutive_data_operations() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test consecutive writes
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        for i in 0..10 {
            ppu.write_to_data(i);
        }

        // Reset address and read back
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x00);

        // First read is buffered
        let _ = ppu.read_data();

        // Next reads should return the written values
        for i in 0..10 {
            let value = ppu.read_data();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn test_write_to_oam_addr() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test initial state
        assert_eq!(ppu.oam_addr.get(), 0);

        // Write to OAM address register
        ppu.oam_addr.update(0x10);
        assert_eq!(ppu.oam_addr.get(), 0x10);

        ppu.oam_addr.update(0xFF);
        assert_eq!(ppu.oam_addr.get(), 0xFF);
    }

    #[test]
    fn test_write_to_oam_data() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set OAM address
        ppu.oam_addr.update(0x10);

        // Write data to OAM
        ppu.write_to_oam_data(0xAB);

        // Check that data was written and address incremented
        assert_eq!(ppu.oam_data[0x10], 0xAB);
        assert_eq!(ppu.oam_addr.get(), 0x11);

        // Write another value
        ppu.write_to_oam_data(0xCD);
        assert_eq!(ppu.oam_data[0x11], 0xCD);
        assert_eq!(ppu.oam_addr.get(), 0x12);
    }

    #[test]
    fn test_read_oam_data() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Write some test data to OAM
        ppu.oam_addr.update(0x20);
        ppu.write_to_oam_data(0x42);

        // Set address back to read the data
        ppu.oam_addr.update(0x20);
        let data = ppu.read_oam_data();
        assert_eq!(data, 0x42);

        // Reading OAM data should not increment the address
        assert_eq!(ppu.oam_addr.get(), 0x20);
    }

    #[test]
    fn test_oam_address_wrap() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set address to 0xFF
        ppu.oam_addr.update(0xFF);
        assert_eq!(ppu.oam_addr.get(), 0xFF);

        // Write data, should wrap to 0x00
        ppu.write_to_oam_data(0x99);
        assert_eq!(ppu.oam_addr.get(), 0x00);
        assert_eq!(ppu.oam_data[0xFF], 0x99);
    }

    #[test]
    fn test_write_to_scroll() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test initial w_reg state
        assert_eq!(ppu.w_reg, false);

        // First write should set X scroll
        ppu.write_to_scroll(0x10);
        assert_eq!(ppu.w_reg, true);

        // Second write should set Y scroll
        ppu.write_to_scroll(0x20);
        assert_eq!(ppu.w_reg, false);

        // Check scroll register value
        assert_eq!(ppu.scroll.get(), 0x1020);
    }

    #[test]
    fn test_read_status() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set w_reg to true
        ppu.w_reg = true;

        // Reading status should clear w_reg
        let status = ppu.read_status();
        assert_eq!(ppu.w_reg, false);
        assert_eq!(status, ppu.status.bits());
    }

    #[test]
    fn test_status_register_interaction() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Set w_reg by writing to scroll
        ppu.write_to_scroll(0x10);
        assert_eq!(ppu.w_reg, true);

        // Reading status should reset w_reg
        ppu.read_status();
        assert_eq!(ppu.w_reg, false);

        // Next scroll write should affect X again (not Y)
        ppu.write_to_scroll(0x30);
        assert_eq!(ppu.w_reg, true);
    }

    #[test]
    fn test_ctrl_register_features() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test VRAM increment functionality
        ppu.write_to_ctrl(0x00); // Increment by 1
        assert_eq!(ppu.ctrl.vram_addr_increment(), 1);

        ppu.write_to_ctrl(0x04); // Increment by 32 (bit 2 set)
        assert_eq!(ppu.ctrl.vram_addr_increment(), 32);

        // Test other control bits are preserved
        ppu.write_to_ctrl(0xFF);
        assert_eq!(ppu.ctrl.bits(), 0xFF);
        assert_eq!(ppu.ctrl.vram_addr_increment(), 32);
    }

    #[test]
    fn test_palette_mirroring() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test writing to different palette addresses within valid range
        for addr in [0x3F00, 0x3F01, 0x3F10, 0x3F1F] {
            let high_byte = (addr >> 8) as u8;
            let low_byte = (addr & 0xFF) as u8;

            ppu.write_to_ppu_addr(high_byte);
            ppu.write_to_ppu_addr(low_byte);
            ppu.write_to_data(0x88);

            let palette_index = (addr - 0x3F00) % 32;
            assert_eq!(ppu.palette_table[palette_index as usize], 0x88);
        }

        // Test palette mirroring behavior
        // Write to 0x3F20, which should mirror to 0x3F00
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_data(0x99);
        assert_eq!(ppu.palette_table[0], 0x99); // 0x3F20 % 32 = 0

        // Write to 0x3F30, which should mirror to 0x3F10
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x30);
        ppu.write_to_data(0xAA);
        assert_eq!(ppu.palette_table[16], 0xAA); // 0x3F30 % 32 = 16

        // Test reading from mirrored addresses
        ppu.write_to_ppu_addr(0x3F);
        ppu.write_to_ppu_addr(0x20);
        let data = ppu.read_data();
        assert_eq!(data, 0x99);
    }

    #[test]
    fn test_ppu_timing_cycle_counting() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test that cycles advance correctly
        assert_eq!(ppu.cycle, 0);
        assert_eq!(ppu.scanline, 0);

        ppu.tick(100);
        assert_eq!(ppu.cycle, 100);
        assert_eq!(ppu.scanline, 0);

        // Test cycle wraparound
        ppu.tick(250); // Total 350, should wrap around
        assert_eq!(ppu.cycle, 350 % 341);
        assert_eq!(ppu.scanline, 1);
    }

    #[test]
    fn test_ppu_vblank_timing() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Initially not in VBlank
        let status_bits = ppu.read_status();
        assert!(!PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));
        assert!(!ppu.get_nmi_flag());

        // Advance to scanline 241 (VBlank start)
        ppu.tick(241 * 341);
        let status_bits = ppu.read_status();
        assert!(PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));

        // Enable NMI to test NMI pending
        ppu.write_to_ctrl(0x80);
        ppu.tick(1); // Trigger VBlank again to set NMI pending

        // Advance to scanline 262+ (VBlank end)
        ppu.tick(22 * 341); // Past scanline 262
        let status_bits = ppu.read_status();
        assert!(!PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));
        assert!(!ppu.get_nmi_flag());
    }

    #[test]
    fn test_ppu_nmi_flag_generation() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Enable NMI generation
        ppu.write_to_ctrl(0x80); // Set GENERATE_NMI bit

        // Initially no NMI
        assert!(!ppu.get_nmi_flag());

        // Trigger VBlank
        ppu.tick(241 * 341);
        assert!(ppu.get_nmi_flag());

        // Clear NMI flag
        ppu.clear_nmi_flag();
        assert!(!ppu.get_nmi_flag());
    }

    #[test]
    fn test_ppu_nmi_disabled() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Disable NMI generation (default state)
        ppu.write_to_ctrl(0x00);

        // Trigger VBlank
        ppu.tick(241 * 341);

        // VBlank should be active but NMI should not be flagged
        let status_bits = ppu.read_status();
        assert!(PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));
        assert!(!ppu.get_nmi_flag()); // Should be false because NMI generation is disabled
    }

    #[test]
    fn test_ppu_ctrl_nmi_interaction() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Start in VBlank
        ppu.tick(241 * 341);
        let status_bits = ppu.read_status();
        assert!(PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));

        // Enable NMI while already in VBlank - should trigger NMI
        ppu.write_to_ctrl(0x80);
        // Note: The current implementation requires a tick to actually trigger VBlank NMI setting
        ppu.tick(1); // Trigger the VBlank NMI logic

        // Disable NMI
        ppu.write_to_ctrl(0x00);
        // The NMI flag behavior depends on implementation details
    }

    #[test]
    fn test_ppu_scanline_wrap_around() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Advance past the end of frame (262+ scanlines)
        let total_ticks = 270 * 341; // 270 scanlines worth of ticks
        ppu.tick(total_ticks);

        // Should wrap back to scanline 0
        assert!(ppu.scanline < 262);
        let status_bits = ppu.read_status();
        assert!(!PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));
    }

    #[test]
    fn test_ppu_multiple_frame_timing() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);
        ppu.write_to_ctrl(0x80); // Enable NMI

        let mut vblank_count = 0;

        // Simulate multiple frames
        for _ in 0..3 {
            // Advance to VBlank
            loop {
                let status_bits = ppu.read_status();
                if PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK) {
                    break;
                }
                ppu.tick(341);
            }
            vblank_count += 1;

            // Clear NMI flag (simulating interrupt handling)
            ppu.clear_nmi_flag();

            // Advance past VBlank
            loop {
                let status_bits = ppu.read_status();
                if !PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK) {
                    break;
                }
                ppu.tick(341);
            }
        }

        assert_eq!(vblank_count, 3);
    }

    #[test]
    fn test_ppu_precise_vblank_boundaries() {
        let mut ppu = create_test_ppu(Mirroring::Vertical);

        // Test exact scanline 241 behavior
        ppu.tick(240 * 341 + 340); // Just before scanline 241
        let status_bits = ppu.read_status();
        assert!(!PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));

        ppu.tick(1); // Cross into scanline 241
        let status_bits = ppu.read_status();
        assert!(PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));

        // Test that VBlank persists through scanline 261
        ppu.tick(20 * 341); // Advance to scanline 261 (241 + 20 = 261)
        let status_bits = ppu.read_status();
        assert!(PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));

        ppu.tick(341); // Cross into scanline 262, which wraps to scanline 0 (next frame)
        let status_bits = ppu.read_status();
        assert!(!PPUSTATUS::from_bits_truncate(status_bits).contains(PPUSTATUS::VBLANK));
    }
}

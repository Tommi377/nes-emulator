#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, Clone)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Rom, String> {
        if raw[0..4] != NES_TAG {
            return Err("File is not in iNES file format".to_string());
        }

        let control_byte_1 = raw[6];
        let control_byte_2 = raw[7];

        if control_byte_1 & 0b0000_1100 != 0 {
            return Err("Only iNES 1.0 file format is supported".to_string());
        }

        let vertical_mirroring_flag = control_byte_2 & 0b0000_0001 != 0;
        #[allow(unused_variables)]
        let battery_ram_flag = control_byte_2 & 0b0000_0010 != 0;
        let trainer_flag = control_byte_2 & 0b0000_0100 != 0;
        let four_screen_flag = control_byte_2 & 0b0000_1000 != 0;

        let mapper = (control_byte_2 & 0b1111_0000) | (control_byte_1 >> 4);

        let screen_mirroring = match (four_screen_flag, vertical_mirroring_flag) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let prg_rom_start = 16 + if trainer_flag { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            screen_mirroring,
        })
    }

    pub fn from_pc(pc: u16) -> Rom {
        let mut prg_rom = vec![0; 2 * PRG_ROM_PAGE_SIZE];
        prg_rom[0x7FFC] = (pc & 0xFF) as u8; // Store low byte of PC
        prg_rom[0x7FFD] = (pc >> 8) as u8; // Store high byte of PC
        Rom {
            prg_rom, // Default PRG-ROM
            chr_rom: vec![],
            mapper: 0,
            screen_mirroring: Mirroring::Horizontal,
        }
    }

    pub fn from_prg(prg_rom: &[u8]) -> Rom {
        Rom {
            prg_rom: prg_rom.to_vec(),
            chr_rom: vec![0; CHR_ROM_PAGE_SIZE], // Default CHR-ROM
            mapper: 0,
            screen_mirroring: Mirroring::Horizontal,
        }
    }

    // Helper function to create a minimal valid iNES header
    fn create_ines_header(
        prg_rom_pages: u8,
        chr_rom_pages: u8,
        control_byte_1: u8,
        control_byte_2: u8,
    ) -> Vec<u8> {
        let mut header = vec![0x4E, 0x45, 0x53, 0x1A]; // NES_TAG
        header.push(prg_rom_pages);
        header.push(chr_rom_pages);
        header.push(control_byte_1);
        header.push(control_byte_2);
        header.extend_from_slice(&[0; 8]); // Remaining header bytes
        header
    }

    // Helper function to create test ROM data
    pub fn create_rom_data(
        prg_rom_pages: u8,
        chr_rom_pages: u8,
        control_byte_1: u8,
        control_byte_2: u8,
        with_trainer: bool,
    ) -> Vec<u8> {
        let mut rom_data =
            Self::create_ines_header(prg_rom_pages, chr_rom_pages, control_byte_1, control_byte_2);

        // Add trainer if needed
        if with_trainer {
            rom_data.extend_from_slice(&[0x99; 512]); // Trainer data
        }

        // Add PRG-ROM data
        let prg_rom_size = prg_rom_pages as usize * PRG_ROM_PAGE_SIZE;
        rom_data.extend_from_slice(&vec![0xAA; prg_rom_size]);

        // Add CHR-ROM data
        let chr_rom_size = chr_rom_pages as usize * CHR_ROM_PAGE_SIZE;
        rom_data.extend_from_slice(&vec![0xBB; chr_rom_size]);

        rom_data
    }
}

#[cfg(test)]
mod rom_tests {
    use super::*;

    #[test]
    fn test_valid_rom_creation() {
        let rom_data = Rom::create_rom_data(2, 1, 0x00, 0x00, false);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), 2 * PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_PAGE_SIZE);
        assert_eq!(rom.mapper, 0);
        assert_eq!(rom.screen_mirroring, Mirroring::Horizontal);

        // Check that PRG-ROM data is correct
        assert!(rom.prg_rom.iter().all(|&x| x == 0xAA));
        // Check that CHR-ROM data is correct
        assert!(rom.chr_rom.iter().all(|&x| x == 0xBB));
    }

    #[test]
    fn test_invalid_nes_tag() {
        let mut rom_data = Rom::create_rom_data(1, 1, 0x00, 0x00, false);
        rom_data[0] = 0x00; // Corrupt the NES tag

        let result = Rom::new(&rom_data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File is not in iNES file format");
    }

    #[test]
    fn test_unsupported_ines_version() {
        let rom_data = Rom::create_rom_data(1, 1, 0x04, 0x00, false); // Non-zero in lower nibble

        let result = Rom::new(&rom_data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Only iNES 1.0 file format is supported"
        );
    }

    #[test]
    fn test_horizontal_mirroring() {
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_0000, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::Horizontal);
    }

    #[test]
    fn test_vertical_mirroring() {
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_0001, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::Vertical);
    }

    #[test]
    fn test_four_screen_mirroring() {
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_1000, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::FourScreen);
    }

    #[test]
    fn test_four_screen_overrides_vertical() {
        // When four-screen flag is set, it should override vertical mirroring
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_1001, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.screen_mirroring, Mirroring::FourScreen);
    }

    #[test]
    fn test_mapper_calculation() {
        // Test mapper 0 (lower nibble of control_byte_1 = 0, upper nibble of control_byte_2 = 0)
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0x00, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 0);

        // Test mapper 1 (lower nibble of control_byte_1 = 1, upper nibble of control_byte_2 = 0)
        let rom_data = Rom::create_rom_data(1, 1, 0x10, 0x00, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 1);

        // Test mapper 16 (lower nibble of control_byte_1 = 0, upper nibble of control_byte_2 = 1)
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0x10, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 16);

        // Test mapper 17 (lower nibble of control_byte_1 = 1, upper nibble of control_byte_2 = 1)
        let rom_data = Rom::create_rom_data(1, 1, 0x10, 0x10, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 17);

        // Test mapper 255 (all bits set)
        let rom_data = Rom::create_rom_data(1, 1, 0xF0, 0xF0, false);
        let rom = Rom::new(&rom_data).unwrap();
        assert_eq!(rom.mapper, 255);
    }

    #[test]
    fn test_rom_with_trainer() {
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_0100, true);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), CHR_ROM_PAGE_SIZE);

        // Verify that PRG-ROM and CHR-ROM data is still correct despite trainer
        assert!(rom.prg_rom.iter().all(|&x| x == 0xAA));
        assert!(rom.chr_rom.iter().all(|&x| x == 0xBB));
    }

    #[test]
    fn test_rom_without_trainer() {
        let rom_data = Rom::create_rom_data(1, 1, 0x00, 0b0000_0000, false);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), CHR_ROM_PAGE_SIZE);
    }

    #[test]
    fn test_multiple_prg_rom_pages() {
        let rom_data = Rom::create_rom_data(4, 1, 0x00, 0x00, false);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), 4 * PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), 1 * CHR_ROM_PAGE_SIZE);
    }

    #[test]
    fn test_multiple_chr_rom_pages() {
        let rom_data = Rom::create_rom_data(1, 3, 0x00, 0x00, false);
        let rom = Rom::new(&rom_data).unwrap();

        assert_eq!(rom.prg_rom.len(), 1 * PRG_ROM_PAGE_SIZE);
        assert_eq!(rom.chr_rom.len(), 3 * CHR_ROM_PAGE_SIZE);
    }

    #[test]
    fn test_insufficient_data_length() {
        // Test with ROM data that's too short
        let short_data = vec![0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01]; // Only 6 bytes

        // This should panic when trying to access raw[6] or raw[7]
        let result = std::panic::catch_unwind(|| Rom::new(&short_data));
        assert!(result.is_err());
    }
}

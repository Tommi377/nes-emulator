#[derive(Debug, PartialEq)]
pub enum Mirroring {
  Vertical,
  Horizontal,
  FourScreen,
}

pub struct Rom {
  pub prg_rom: Vec<u8>,
  pub chr_rom: Vec<u8>,
  pub mapper: u8,
  pub screen_mirroring: Mirroring,
}

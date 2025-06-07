pub struct CPU {
  pub pc: u8,
  pub status: u8,
  pub reg_a: u8,
  pub reg_x: u8,
}

impl CPU {
  pub fn new() -> Self {
      CPU {
          pc: 0,
          status: 0,
          reg_a: 0,
          reg_x: 0,
      }
  }
  
  pub fn interpret(&mut self, program: Vec<u8>) {
      self.pc = 0;
      
      loop {
          let opscode = self.get_and_increment_pc(&program);

          match opscode {
              0xA9 => {
                  let param = self.get_and_increment_pc(&program);
                  self.reg_a = param;

                  if self.reg_a == 0 {
                      self.status = self.status | 0b0000_0010;
                  } else {
                      self.status = self.status & 0b1111_1101;
                  }

                  if self.reg_a & 0b1000_0000 != 0 {
                      self.status = self.status | 0b1000_0000;
                  } else {
                      self.status = self.status & 0b0111_1111;
                  }
              }
              0xAA => {
                self.reg_x = self.reg_a;

                if self.reg_x == 0 {
                  self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }

                if self.reg_x & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }
              }
              0x00 => {
                  return;
              }
              _ => todo!()
          }
      }
  }

  fn get_and_increment_pc(&mut self, program: &Vec<u8>) -> u8 {
      let value = program[self.pc as usize];
      self.pc += 1;
      value
  }
}

#[cfg(test)]
mod test {
 use super::*;

 #[test]
  fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.reg_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
  }

  #[test]
  fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
  }

  #[test]
  fn test_0xa9_lda_neg_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0xFF, 0x00]);
    assert!(cpu.status & 0b1000_0000 != 0);
  }

  #[test]
   fn test_0xaa_tax_immediate_load_data() {
     let mut cpu = CPU::new();
     cpu.reg_a = 5;
     cpu.interpret(vec![0xaa, 0x00]);
     assert_eq!(cpu.reg_x, 0x05);
     assert!(cpu.status & 0b0000_0010 == 0b00);
     assert!(cpu.status & 0b1000_0000 == 0);
   }
 
   #[test]
   fn test_0xaa_tax_zero_flag() {
     let mut cpu = CPU::new();
     cpu.reg_a = 0;
     cpu.interpret(vec![0xaa, 0x00]);
     assert!(cpu.status & 0b0000_0010 == 0b10);
   }
   #[test]
   fn test_0xaa_tax_neg_flag() {
     let mut cpu = CPU::new();
     cpu.reg_a = 255;
     cpu.interpret(vec![0xaa, 0x00]);
     assert!(cpu.status & 0b1000_0000 != 0);
   }
}
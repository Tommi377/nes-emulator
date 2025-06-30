pub struct OAMADDRESS(u8);

impl OAMADDRESS {
    pub fn new() -> Self {
        OAMADDRESS(0)
    }

    pub fn update(&mut self, data: u8) {
        self.0 = data;
    }

    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Default for OAMADDRESS {
    fn default() -> Self {
        OAMADDRESS::new()
    }
}

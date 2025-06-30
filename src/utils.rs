pub fn set_bit(byte: u8, mask: u8, value: bool) -> u8 {
    if value {
        byte | mask
    } else {
        byte & (mask ^ 0b1111_1111)
    }
}

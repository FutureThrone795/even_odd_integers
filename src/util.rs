pub fn u64_to_char_unchecked(n: u64) -> char {
    return (n as u8 + '0' as u8) as char;
}
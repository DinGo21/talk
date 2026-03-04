pub fn strtobin(str: String) -> String {
    let mut bin = String::new();

    for byte in str.bytes() {
        let mut byte_idx = 7;
        while byte_idx >= 0 {
            bin.push(((byte >> byte_idx & 1) + 48) as char);
            byte_idx -= 1;
        }
    }
    bin
}

pub fn bintochar(bin: String) -> Option<char> {
    let mut byte: u8 = 0;
    let mut base: u8 = 1;

    for bit in bin.chars().rev() {
        if bit == '1' {
            byte += base;
        }
        if base < 128 {
            base *= 2;
        }
    }
    let c = byte as char;
    if c == '\0' {
        return None;
    }
    Some(c)
}

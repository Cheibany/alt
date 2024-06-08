pub fn rot21(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + 21) % 26;
            (base + offset) as char
        } else {
            c
        }
    }).collect()
}
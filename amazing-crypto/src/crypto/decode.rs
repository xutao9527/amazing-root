use crate::dict::{char_to_index, index_to_char};

pub fn decode(text: &str) -> Vec<u8> {
    let chars: Vec<char> = text.chars().collect();
    let has_padding = index_to_char(65536).unwrap() == chars.last().unwrap().clone();
    let effective_len = if has_padding {
        chars.len() - 1
    } else {
        chars.len()
    };
    let mut bytes = Vec::new();
    let mut i = 0;
    while i < effective_len {
        if let Some(idx) = char_to_index(chars[i]) {
            let hi = (idx >> 8) as u8;
            let lo = (idx & 0xFF) as u8;
            if i == (effective_len - 1) && has_padding {
                bytes.push(hi);
            } else {
                bytes.push(hi);
                bytes.push(lo);
            }
        }
        i += 1;
    }
    bytes
}

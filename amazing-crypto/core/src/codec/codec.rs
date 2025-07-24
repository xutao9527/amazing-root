use crate::codec::codec_dict::RANGES_HARD_CODED;

/// 字符转索引
pub fn char_to_index(c: char) -> Option<u32> {
    let code = c as u32;
    for section in RANGES_HARD_CODED.iter() {
        if let Some(idx) = section.char_to_index(code) {
            return Some(idx);
        }
    }
    None
}

/// 索引转字符
pub fn index_to_char(index: u32) -> Option<char> {
    for section in RANGES_HARD_CODED.iter() {
        if let Some(c) = section.index_to_char(index) {
            return Some(c);
        }
    }
    None
}

/// 编码
pub fn encode(data: &[u8]) -> String {
    let data_len = data.len();
    let data_pairs = data_len / 2;
    let data_has_odd = data_len % 2 == 1;

    let mut encoded = String::with_capacity(data_pairs + 2);

    for i in 0..data_pairs {
        let hi = data[2 * i];
        let lo = data[2 * i + 1];
        let index = ((hi as usize) << 8) | (lo as usize);
        if let Some(c) = index_to_char(index as u32) {
            encoded.push(c);
        }
    }
    // 处理奇数字节
    if data_has_odd {
        let hi = data[data_len - 1];
        let index = (hi as usize) << 8;
        if let Some(c) = index_to_char(index as u32) {
            encoded.push(c);
            encoded.push(index_to_char(65536).unwrap());
        }
    }
    encoded
}

/// 解码
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

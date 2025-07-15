use crate::dict::unicode_crypto_dict::{CRYPTO_CHAR_TO_INDEX, CRYPTO_CHARS};

pub fn decode(text: &str) -> Vec<u8> {
    let chars: Vec<char> = text.chars().collect();
    let mut bytes = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == CRYPTO_CHARS[65536] {
            // 补位标记，不产生字节，跳过
            i += 1;
            continue;
        }
        // 查找当前字符在 UNICODE_MAP 中的索引
        if let Some(idx) = CRYPTO_CHAR_TO_INDEX.get(&c) {
            let hi = (idx >> 8) as u8;
            let lo = (idx & 0xFF) as u8;
            // 判断下一个字符是否是补位标记
            let next_is_padding = (i + 1 < chars.len()) && (chars[i + 1] == CRYPTO_CHARS[65536]);
            if next_is_padding {
                // 奇数字节，只输出高字节
                bytes.push(hi);
                i += 2; // 跳过当前字符和补位标记
            } else {
                // 正常两个字节
                bytes.push(hi);
                bytes.push(lo);
                i += 1; // 只跳过当前字符
            }
        }
    }
    bytes
}

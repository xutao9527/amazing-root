use std::fs::{read, read_to_string, write};
use crate::crypto::unicode_crypto_dict::{CRYPTO_CHARS, CRYPTO_CHAR_TO_INDEX};

// 
// 编码
pub fn encode_file_to_unicode(input_path: &str, output_path: &str) -> std::io::Result<()> {
    // 1. 读取文件内容（u8 数据）
    let data = read(input_path)?;
    let data_len = data.len();
    let data_pairs = data_len / 2;
    let data_has_odd = data_len % 2 == 1;

    // 处理完整字节对
    let mut encoded = String::new();
    for i in 0..data_pairs {
        let hi = data[2 * i];
        let lo = data[2 * i + 1];
        let index = ((hi as usize) << 8) | (lo as usize);
        let c = CRYPTO_CHARS[index];
        encoded.push(c);
    }
    // 处理奇数字节
    if data_has_odd {
        let hi = data[data_len  - 1];
        let index = (hi as usize) << 8;
        let c = CRYPTO_CHARS[index];
        encoded.push(c);
        encoded.push(CRYPTO_CHARS[65536]);
    }

    // 3. 写入为 UTF-8 文本文件
    write(output_path, encoded)?;
    Ok(())
}

// 解码
pub fn decode_unicode_to_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let content = read_to_string(input_path)?;
    let chars: Vec<char> = content.chars().collect();

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
    write(output_path, bytes)?;
    Ok(())
}
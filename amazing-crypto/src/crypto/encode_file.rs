use std::fs::{read, write};
use crate::dict::unicode_crypto_dict::CRYPTO_CHARS;


pub fn encode_file(input_path: &str, output_path: &str){
    // 1. 读取文件内容（u8 数据）
    let data = read(input_path).unwrap();
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
    write(output_path, encoded).unwrap();
 
}
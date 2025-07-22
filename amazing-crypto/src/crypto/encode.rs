use crate::dict::index_to_char;

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

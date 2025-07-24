

pub mod crypto;
pub mod codec;
pub mod api;



#[cfg(test)]
mod tests {
    use crate::api::{base64_to_key_nonce, key_nonce_to_base64};
    use crate::codec::codec::{char_to_index, index_to_char};
    use crate::codec::generate_hardcoded;
    use crate::crypto::chacha_cipher::encrypt_decrypt_binary;

    // 生成硬编码
    #[test]
    pub fn test_generate_hardcoded(){
        generate_hardcoded();
    }
    
    // 生成密钥
    #[test]
    pub fn test_generate_base64_key(){
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let key_base64 = key_nonce_to_base64(&key, &nonce);
        println!("key_base64: {:?}", key_base64);
        let kn = base64_to_key_nonce(&key_base64);
        println!("kn: {:?}", kn);
    }

    // 加解密测试
    #[test]
    pub fn test_chacha_cipher(){
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let mut buffer = b"hello chacha20 pure stream cipher".to_vec();
        println!("Plaintext: {:?}", String::from_utf8_lossy(&buffer));
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        println!("Ciphertext: {:?}", buffer);
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        println!("Decrypted: {:?}", String::from_utf8_lossy(&buffer));
        println!("Encryption and decryption successful!");
    }

    // 编解码
    #[test]
    pub fn test_codec(){
        let c = index_to_char(65534);
        let index = char_to_index(c.unwrap());
        println!("c: {}, index: {}", c.unwrap(), index.unwrap());
    }
}

pub mod crypto;
pub mod dict;


#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::crypto::chacha_cipher::encrypt_decrypt_binary;
    use crate::crypto::file_decode::decode_file;
    use crate::crypto::file_encode::encode_file;
    use crate::dict::{char_to_index, generate_hardcoded, index_to_char};

    // 加解密测试
    #[test]
    pub fn chacha_cipher(){
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let mut buffer = b"hello chacha20 pure stream cipher".to_vec();
        println!("Plaintext: {:?}", String::from_utf8_lossy(&buffer));
        // 加密（原地修改 buffer）
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        println!("Ciphertext: {:?}", buffer);
        // 解密（原地再修改回明文）
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        println!("Decrypted: {:?}", String::from_utf8_lossy(&buffer));
        println!("Encryption and decryption successful!");
    }
    
    // 生成硬编码
    #[test]
    pub fn test_generate_hardcoded(){
        generate_hardcoded();
    }

    // 编解码
    #[test]
    pub fn test_crypto(){
        let c = index_to_char(65534);
        let index = char_to_index(c.unwrap());
        println!("c: {}, index: {}", c.unwrap(), index.unwrap());
    }
    
    // 图片编码
    #[test]
    pub fn test_encode_img() {
        encode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test.jpg",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test.txt",
        );
    }

    // 图片解码
    #[test]
    pub fn test_decode_img() {
        decode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test.txt",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test_decode.jpg",
        );
    }

    // 视频编码
    #[test]
    pub fn test_decode_video() {
        let video_vec = vec![
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dashinit.mp4",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash1.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash2.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash3.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash4.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash5.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash6.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash7.m4s",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\test\test_dash8.m4s",
        ];

        for input_path  in video_vec {
            let input_path = Path::new(input_path);
            let output_path = input_path.with_extension("txt");
            encode_file(
                input_path.to_str().unwrap(),
                output_path.to_str().unwrap(),
            );
        }
    }
}

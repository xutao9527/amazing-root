use std::io::{Read, Write};
use base64::Engine;
use base64::engine::general_purpose;
use zstd::{Decoder, Encoder};


use crate::codec::codec::{decode, encode};
use crate::crypto::chacha_cipher::encrypt_decrypt_binary;

pub fn base64_to_key_nonce(secret_key: &str) -> Option<([u8; 32], [u8; 12])> {
    let bytes = general_purpose::STANDARD.decode(secret_key).ok()?;
    if bytes.len() != 44 {
        return None;
    }
    let key = <[u8; 32]>::try_from(&bytes[0..32]).ok()?;
    let nonce = <[u8; 12]>::try_from(&bytes[32..44]).ok()?;
    Some((key, nonce))
}

pub fn key_nonce_to_base64(key: &[u8; 32], nonce: &[u8; 12]) -> String {
    let mut combined = Vec::with_capacity(32 + 12);
    combined.extend_from_slice(key);
    combined.extend_from_slice(nonce);
    general_purpose::STANDARD.encode(&combined)
}

pub fn encrypt(data: &mut [u8], secret_key: &str) -> String {
    // let mut data = data.to_vec();
    if let Some((key, nonce)) = base64_to_key_nonce(secret_key) {
        // let start = Instant::now();
        // 1,压缩
        let mut compressed = Vec::new();
        {
            let mut encoder = Encoder::new(&mut compressed, 10).expect("zstd encoder failed");
            encoder.write_all(data).expect("zstd write failed");
            encoder.finish().expect("zstd finish failed");
        }
        // println!("Compression took: {:?}", start.elapsed());
        // println!("data: {:?}", data.len());
        // println!("compressed: {:?}", compressed.len());
        // 2,加密
        encrypt_decrypt_binary(&key, &nonce, &mut compressed);
        // 3,编码
        return encode(&compressed);
    }
    String::new()
}

pub fn decrypt(data: &str,secret_key: &str) -> Vec<u8>{
    if let Some((key, nonce)) = base64_to_key_nonce(secret_key) {
        // 1,解码
        let mut buffer = decode(data);
        // 2,解密
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        // 3,解压
        let mut decoder = Decoder::new(&buffer[..]).expect("zstd decoder failed");
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).expect("zstd decompression failed");
        return decompressed;
    }
    Vec::new()
}




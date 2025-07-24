use base64::Engine;
use base64::engine::general_purpose;
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
        encrypt_decrypt_binary(&key, &nonce, data);
        return encode(&data);
    }
    String::new()
}

pub fn decrypt(data: &str,secret_key: &str) -> Vec<u8>{
    if let Some((key, nonce)) = base64_to_key_nonce(secret_key) {
        let mut buffer = decode(data);
        encrypt_decrypt_binary(&key, &nonce, &mut buffer);
        return buffer;
    }
    Vec::new()
}




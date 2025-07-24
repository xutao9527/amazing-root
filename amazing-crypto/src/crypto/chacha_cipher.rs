use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};


#[allow(dead_code)]
pub fn generate_key_nonce() -> ([u8; 32], [u8; 12]) {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];

    getrandom::fill(&mut key).expect("Failed to fill key");
    getrandom::fill(&mut nonce).expect("Failed to fill nonce");

    (key, nonce)
}

pub fn encrypt_decrypt_binary(key: &[u8; 32], nonce: &[u8; 12], data: &mut [u8]){
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(data);
}

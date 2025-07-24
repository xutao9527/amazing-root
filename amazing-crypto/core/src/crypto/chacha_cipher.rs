use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

pub fn encrypt_decrypt_binary(key: &[u8; 32], nonce: &[u8; 12], data: &mut [u8]){
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(data);
}

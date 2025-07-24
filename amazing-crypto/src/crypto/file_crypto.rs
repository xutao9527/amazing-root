use std::fs::{read, read_to_string, write};
use amazing_crypto_core::api::{decrypt, encrypt, key_nonce_to_base64};

pub fn encrypt_file(input_path: &str, output_path: &str) {
    let key = [0u8; 32];
    let nonce = [0u8; 12];
    let secret_key = key_nonce_to_base64(&key, &nonce);
    let mut data = read(input_path).expect("failed to read input file");
    // println!("{}", String::from_utf8_lossy(&data));
    let encoded = encrypt(&mut data, &secret_key);
    write(output_path, encoded).expect("failed to write output file");
}

pub fn decrypt_file(input_path: &str, output_path: &str) {
    let key = [0u8; 32];
    let nonce = [0u8; 12];
    let secret_key = key_nonce_to_base64(&key, &nonce);
    let content = read_to_string(input_path).expect("failed to read input file");
    let bytes = decrypt(&content, &secret_key);
    // println!("{}", String::from_utf8_lossy(&bytes));
    write(output_path, &bytes).expect("failed to write output file");
}


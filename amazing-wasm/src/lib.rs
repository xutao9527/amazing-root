use wasm_bindgen::prelude::wasm_bindgen;
use amazing_crypto_core::api::{decrypt, key_nonce_to_base64};
/*
    打包说明
    cargo install wasm-pack
    wasm-pack build --target web --out-dir ../amazing-web/wasm
    切片说明
    MP4Box -dash 5000 -profile live test_h264.mp4
 */
#[wasm_bindgen]
pub fn decode_unicode(text:& str) -> Vec<u8> {
    let key = [0u8; 32];
    let nonce = [0u8; 12];
    let secret_key = key_nonce_to_base64(&key, &nonce);
    decrypt(text,&secret_key)
}



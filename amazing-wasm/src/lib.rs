use wasm_bindgen::prelude::wasm_bindgen;
use amazing_crypto::crypto::decode::decode;
/*
    打包说明
    cargo install wasm-pack
    wasm-pack build --target web --out-dir ../amazing-web/wasm
 */
#[wasm_bindgen]
pub fn decode_unicode(text:& str) -> Vec<u8> {
    decode(text)
}
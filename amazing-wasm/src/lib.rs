use wasm_bindgen::prelude::wasm_bindgen;
use amazing_crypto::crypto::decode::decode;
/*
    打包说明
    cargo install wasm-pack
    wasm-pack build --target web --out-dir ../amazing-web/wasm
    切片说明
    MP4Box -dash 5000 -profile live test_h264.mp4
 */
#[wasm_bindgen]
pub fn decode_unicode(text:& str) -> Vec<u8> {
    decode(text)
}
pub mod crypto;
pub mod dict;
mod generator;

#[cfg(test)]
mod tests {
    use crate::crypto::file_decode::decode_file;
    use crate::crypto::file_encode::encode_file;
    use crate::generator::unicode_crypto_generator::UnicodeCryptoGenerator;

    // 生成加密字典
    #[test]
    pub fn test_generator() {
        let generator = UnicodeCryptoGenerator::new();
        generator.run()
    }

    // 生成加密图片
    #[test]
    pub fn test_encode_img() {
        encode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.jpg",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.txt",
        );
    }

    // 生成加密图片
    #[test]
    pub fn test_decode_img() {
        decode_file(
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179.txt",
            r"D:\codebase\rustProjects\amazing-root\amazing-web\static\12179_decode.jpg",
        );
    }
}

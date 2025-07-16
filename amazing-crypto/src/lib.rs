pub mod crypto;
pub mod dict;
mod generator;


#[cfg(test)]
mod tests {
    use crate::generator::unicode_crypto_generator::UnicodeCryptoGenerator;

    // 生成加密字典
    #[test]
    pub fn test_generator() {
        let generator = UnicodeCryptoGenerator::new();
        generator.run()
    }
}

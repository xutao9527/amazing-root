



#[cfg(test)]
mod tests {
    use crate::generator::unicode_crypto_generator::UnicodeCryptoGenerator;

    #[test]
    pub fn test_generator() {
        let generator = UnicodeCryptoGenerator::new();
        generator.run()
    }
}

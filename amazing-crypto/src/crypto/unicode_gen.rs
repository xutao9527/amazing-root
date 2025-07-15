use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use rand::seq::SliceRandom;
use rand::rng;

struct UnicodeCharRange {
    char_begin:  u32,
    char_end: u32
}

impl UnicodeCharRange {
    fn get_chars(self) -> Vec<char> {
        (self.char_begin..=self.char_end)
            .filter_map(|codepoint| std::char::from_u32(codepoint))
            .collect()
    }

    pub fn new(char_begin: u32, char_end: u32) -> Self {
        UnicodeCharRange { char_begin, char_end }
    }
}

pub fn generate_chars() -> Vec<char> {
    let ranges = vec![
        UnicodeCharRange::new(0x0020 + 1, 0x0080 - 1 - 1),              //  一字节字符
        UnicodeCharRange::new(0x31F0, 0x3200),                          //  片假名语音扩展
        UnicodeCharRange::new(0x3300, 0x3400 - 1),                      //  CJK兼容性
        UnicodeCharRange::new(0x3400, 0x4DC0 - 1),                      //  中日韩统一表意文字扩展A
        UnicodeCharRange::new(0x4DC0, 0x4E00 - 1),                      //  易经卦象
        UnicodeCharRange::new(0x4E00, 0xA000 - 1),                      //  中日韩统一表意文字
        UnicodeCharRange::new(0xA000, 0xA490 - 1 - 3),                  //  彝语音节
        UnicodeCharRange::new(0xAC00, 0xD7B0 - 1 - 12),                 //  易经卦象
        UnicodeCharRange::new(0x20000, 0x26260),                        //  中日韩统一表意文字
    ];
    let mut all_chars = Vec::new();
    for range in ranges {
        let chars = range.get_chars();
       
        all_chars.extend(chars);
    }

    let mut rng = rng();
    all_chars.shuffle(&mut rng);
    all_chars
}

pub(crate) fn generate_unicode_crypto_dict() -> std::io::Result<()> {
    let all_chars = generate_chars();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&manifest_dir).join("src/crypto/unicode_crypto_dict.rs");
    let mut file = BufWriter::new(File::create(out_path)?);

    writeln!(file, "// AUTO-GENERATED FILE. DO NOT EDIT.")?;
    writeln!(file, "")?;

    // Forward map
    writeln!(
        file,
        "pub const CRYPTO_CHARS: [char; {}] = [",
        all_chars.len()
    )?;
    for c in &all_chars {
        writeln!(file, "    '\\u{{{:04X}}}',", *c as u32)?;
    }
    writeln!(file, "];")?;
    writeln!(file)?;

    writeln!(file, "use phf::phf_map;\n")?;
    writeln!(file, "pub static CRYPTO_CHAR_TO_INDEX: phf::Map<char, usize> = phf_map! {{")?;
    for (i, c) in all_chars.iter().enumerate() {
        writeln!(file, "    '\\u{{{:04X}}}' => {},", *c as u32, i)?;
    }
    writeln!(file, "}};")?;
    
    // Reverse lookup function
    // writeln!(file, "pub const fn reverse_lookup(c: char) -> isize {{")?;
    // writeln!(file, "    match c {{")?;
    // for (i, c) in all_chars.iter().enumerate() {
    //     writeln!(file, "        '\\u{{{:X}}}' => {},", *c as u32, i)?;
    // }
    // writeln!(file, "        _ => -1,")?;
    // writeln!(file, "    }}")?;
    // writeln!(file, "}}")?;

    Ok(())
}



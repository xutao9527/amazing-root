use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::dict::definition::{UnicodeCharSection, RAW_RANGES};
use crate::dict::unicode_dict::RANGES_HARD_CODED;

mod unicode_dict;
mod definition;



// 生成硬编码
pub fn generate_hardcoded(){
    let mut index_offset = 0;
    let mut sections = Vec::new();
    for &(char_begin, char_end) in RAW_RANGES.iter() {
        let mut section = UnicodeCharSection::new(char_begin, char_end);
        index_offset += section.update(index_offset);
        sections.push(section);
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&manifest_dir).join("src/dict/unicode_dict.rs");
    let mut file = BufWriter::new(File::create(out_path).unwrap());
    writeln!(file, "// AUTO-GENERATED FILE. DO NOT EDIT.").unwrap();

    writeln!(file, "use crate::dict::UnicodeCharSection;").unwrap();
    writeln!(file).unwrap();

    writeln!(file, "pub const RANGES_HARD_CODED: &[UnicodeCharSection] = &[").unwrap();

    for section in sections {
        writeln!(file, "    UnicodeCharSection {{").unwrap();
        writeln!(file, "        char_begin: {},",section.char_begin).unwrap();
        writeln!(file, "        char_end: {},",section.char_end).unwrap();
        writeln!(file, "        char_len: {},",section.char_len).unwrap();
        writeln!(file, "        char_begin_index: {},",section.char_begin_index).unwrap();
        writeln!(file, "        char_end_index: {},",section.char_end_index).unwrap();
        writeln!(file, "    }},").unwrap();
    }
    writeln!(file, "];").unwrap();
}

// 字符转索引
pub fn char_to_index(c: char) -> Option<u32> {
    let code = c as u32;
    for section in RANGES_HARD_CODED.iter() {
        if let Some(idx) = section.char_to_index(code) {
            return Some(idx);
        }
    }
    None
}

// 索引转字符
pub fn index_to_char(index: u32) -> Option<char> {
    for section in RANGES_HARD_CODED.iter() {
        if let Some(c) = section.index_to_char(index) {
            return Some(c);
        }
    }
    None
}
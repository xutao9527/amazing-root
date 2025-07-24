use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::codec::define::{UnicodeCharSection, RAW_RANGES};

pub mod define;
pub mod codec;
pub mod codec_dict;


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
    println!("manifest_dir: {}", manifest_dir);
    let out_path = Path::new(&manifest_dir).join("src/codec/codec_dict.rs");
    let mut file = BufWriter::new(File::create(out_path).unwrap());
    writeln!(file, "// AUTO-GENERATED FILE. DO NOT EDIT.").unwrap();

    writeln!(file, "use crate::codec::define::UnicodeCharSection;").unwrap();
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


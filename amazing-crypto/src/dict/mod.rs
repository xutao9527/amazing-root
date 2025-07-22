use once_cell::sync::Lazy;

// 字符区间
#[derive(Debug)]
struct UnicodeCharSection {
    char_begin: u32,
    char_end: u32,
    char_begin_index: u32,
    char_end_index: u32,
    char_len:u32,
}

impl UnicodeCharSection {
    fn new(char_begin: u32, char_end: u32) -> Self {
        UnicodeCharSection {
            char_begin,
            char_end,
            char_begin_index:0,
            char_end_index:0,
            char_len:char_end - char_begin + 1,
        }
    }

    fn update(&mut self, index_offset: u32) ->u32 {
        self.char_begin_index = index_offset;
        self.char_end_index = index_offset + self.char_len;
        self.char_len
    }

    fn contains_char(&self, c: u32) -> bool {
        c >= self.char_begin && c <= self.char_end
    }

    fn contains_index(&self, index: u32) -> bool {
        index >= self.char_begin_index && index < self.char_end_index
    }

    fn char_to_index(&self, c: u32) -> Option<u32> {
        if self.contains_char(c) {
            Some(self.char_begin_index + (c - self.char_begin))
        } else {
            None
        }
    }
    
    fn index_to_char(&self, index: u32) -> Option<char> {
        if self.contains_index(index) {
            let cp = self.char_begin + (index - self.char_begin_index);
            std::char::from_u32(cp)
        } else {
            None
        }
    }
    
}

const RAW_RANGES: &[(u32, u32)] = &[
    (0x0020 + 1, 0x0080 - 1 - 1), //  一字节字符
    (0x31F0, 0x3200),             //  片假名语音扩展
    (0x3300, 0x3400 - 1),         //  CJK兼容性
    (0x3400, 0x4DC0 - 1),         //  中日韩统一表意文字扩展A
    (0x4DC0, 0x4E00 - 1),         //  易经卦象
    (0x4E00, 0xA000 - 1),         //  中日韩统一表意文字
    (0xA000, 0xA490 - 1 - 3),     //  彝语音节
    (0xAC00, 0xD7B0 - 1 - 12),    //  易经卦象
    (0x20000, 0x26260),           //  中日韩统一表意文字
];

static RANGES: Lazy<Vec<UnicodeCharSection>> = Lazy::new(|| {
    let mut index_offset = 0;
    let r = RAW_RANGES.iter().map(|&(char_begin, char_end)| {
        let mut section = UnicodeCharSection::new(char_begin, char_end);
        index_offset += section.update(index_offset);
        section
    }).collect();
    r
});


/// 字符转索引
pub fn char_to_index(c: char) -> Option<u32> {
    let code = c as u32;
    for section in RANGES.iter() {
        if let Some(idx) = section.char_to_index(code) {
            return Some(idx);
        }
    }
    None
}

/// 索引转字符
pub fn index_to_char(index: u32) -> Option<char> {
    for section in RANGES.iter() {
        if let Some(c) = section.index_to_char(index) {
            return Some(c);
        }
    }
    None
}
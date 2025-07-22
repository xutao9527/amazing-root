#[derive(Debug)]
pub struct UnicodeCharSection {
    pub char_begin: u32,
    pub char_end: u32,
    pub char_begin_index: u32,
    pub char_end_index: u32,
    pub char_len:u32,
}

impl UnicodeCharSection {
    pub fn new(char_begin: u32, char_end: u32) -> Self {
        UnicodeCharSection {
            char_begin,
            char_end,
            char_begin_index:0,
            char_end_index:0,
            char_len:char_end - char_begin + 1,
        }
    }

    pub fn update(&mut self, index_offset: u32) ->u32 {
        self.char_begin_index = index_offset;
        self.char_end_index = index_offset + self.char_len;
        self.char_len
    }

    pub fn contains_char(&self, c: u32) -> bool {
        c >= self.char_begin && c <= self.char_end
    }

    pub fn contains_index(&self, index: u32) -> bool {
        index >= self.char_begin_index && index < self.char_end_index
    }

    pub fn char_to_index(&self, c: u32) -> Option<u32> {
        if self.contains_char(c) {
            Some(self.char_begin_index + (c - self.char_begin))
        } else {
            None
        }
    }

    pub fn index_to_char(&self, index: u32) -> Option<char> {
        if self.contains_index(index) {
            let cp = self.char_begin + (index - self.char_begin_index);
            std::char::from_u32(cp)
        } else {
            None
        }
    }

}

// const RAW_RANGES: &[(u32, u32)] = &[
//     (0x0020 + 1, 0x0080 - 1 - 1), //  一字节字符
//     (0x31F0, 0x3200),             //  片假名语音扩展
//     (0x3300, 0x3400 - 1),         //  CJK兼容性
//     (0x3400, 0x4DC0 - 1),         //  中日韩统一表意文字扩展A
//     (0x4DC0, 0x4E00 - 1),         //  易经卦象
//     (0x4E00, 0xA000 - 1),         //  中日韩统一表意文字
//     (0xA000, 0xA490 - 1 - 3),     //  彝语音节
//     (0xAC00, 0xD7B0 - 1 - 12),    //  易经卦象
//     (0x20000, 0x26260),           //  中日韩统一表意文字
// ];

pub const RAW_RANGES: &[(u32, u32)] = &[
    (0x3400, 0x4DC0 - 1),         //  中日韩统一表意文字扩展A
    (0x4DC0, 0x4E00 - 1),         //  易经卦象
    (0x0020 + 1, 0x0080 - 1 - 1), //  一字节字符
    (0x20000, 0x26260),           //  中日韩统一表意文字
    (0xA000, 0xA490 - 1 - 3),     //  彝语音节
    (0x4E00, 0xA000 - 1),         //  中日韩统一表意文字
    (0x3300, 0x3400 - 1),         //  CJK兼容性
    (0xAC00, 0xD7B0 - 1 - 12),    //  易经卦象
];
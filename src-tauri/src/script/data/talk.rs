use core::fmt;

use crate::script::file::dat::{code_map, reverse_code_map};

pub fn read_u16(datum1: u8, datum2: u8) -> u16 {
    (datum1 - 1) as u16 * 0x100 + datum2 as u16
}
pub fn write_u16(flag: u16) -> (u8, u8) {
    ((flag / 0x100) as u8 + 1, (flag % 0x100) as u8)
}

#[derive(Clone)]
pub struct Talk(Vec<u8>);

impl Talk {
    pub fn from_text(text: &str) -> Self {
        let char_to_code = reverse_code_map();
        Self(text.chars().map(|c| char_to_code[&c]).collect())
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        debug_assert_eq!(data.len(), 7 * 3);
        Self(data)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl fmt::Display for Talk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_map = code_map();
        self.0
            .iter()
            .try_for_each(|&x| write!(f, "{}", code_map[x as usize]))
    }
}

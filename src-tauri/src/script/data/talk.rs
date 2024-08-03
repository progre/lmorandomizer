use core::fmt;
use std::ops::Range;

use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use crate::script::{
    consts::UNUSED_PR3_FLAG_NO,
    enums::{self, TalkItem},
    file::dat::{code_map, reverse_code_map},
};

pub fn read_u16(datum1: u8, datum2: u8) -> u16 {
    (datum1 - 1) as u16 * 0x100 + datum2 as u16
}
pub fn write_u16(flag: u16) -> (u8, u8) {
    ((flag / 0x100) as u8 + 1, (flag % 0x100) as u8)
}

#[derive(Clone, Debug)]
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

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }

    pub fn control_talk_command_ranges(&self) -> Vec<Range<usize>> {
        let mut vec = Vec::new();
        let mut i = 0;
        while i < self.0.len() {
            let range = match self.0[i] {
                1 => i..i + 1,
                2 => i..i + 3,
                3 => i..i + 3,
                4 => i..i + 2,
                5 => i..i + 2,
                6 => i..i + 2,
                7 => i..i + 1,
                8 => i..i + 1,
                10 => i..i + 1,
                _ => {
                    i += 1;
                    continue;
                }
            };
            i += range.len();
            vec.push(range);
        }
        vec
    }

    pub fn item(&self) -> Result<Option<(TalkItem, u16)>> {
        let mut set_flag = None;
        let mut item = None;
        for range in self.control_talk_command_ranges() {
            let cmd = &self.0[range];
            match cmd[0] {
                2 => {
                    let flag = read_u16(cmd[1], cmd[2]);
                    if flag != UNUSED_PR3_FLAG_NO {
                        set_flag = Some(flag);
                    }
                }
                4 => {
                    let equipment = enums::Equipment::from_u8(cmd[1] - 1)
                        .ok_or_else(|| anyhow!("Invalid equipment code: {}", cmd[1]))?;
                    item = Some(enums::TalkItem::Equipment(equipment));
                }
                5 => {
                    let rom = enums::Rom::from_u8(cmd[1] - 1)
                        .ok_or_else(|| anyhow!("Invalid rom code: {}", cmd[1]))?;
                    item = Some(enums::TalkItem::Rom(rom));
                }
                _ => {}
            }
        }
        let (Some(item), Some(set_flag)) = (item, set_flag) else {
            if item.is_some() && set_flag.is_none() {
                bail!("Invalid talk command: {:?}", self.to_string())
            }
            return Ok(None);
        };
        Ok(Some((item, set_flag)))
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

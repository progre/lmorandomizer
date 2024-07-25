use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

const U16_MAX: i32 = u16::MAX as i32;

use super::{
    super::item::{ChestItem, Equipment, Rom, Seal},
    items, Start,
};

#[derive(Clone, Copy, PartialEq)]
pub enum ChestContent {
    Equipment(items::Equipment),
    Rom(items::Rom),
}

#[derive(Clone)]
pub struct ChestObject {
    x: i32,
    y: i32,
    open_flag: i32,
    content: Option<ChestContent>,
    set_flag: i32,
    unused: i32,
    starts: Vec<Start>,
}

impl ChestObject {
    pub fn new(
        x: i32,
        y: i32,
        open_flag: i32,
        content: i32,
        set_flag: i32,
        unused: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if ![-1, 0].contains(&unused) {
            bail!("invalid parameter: op4={}", unused);
        }
        if content >= 0 && !matches!(set_flag, 0..=U16_MAX) {
            bail!("invalid parameters: op2={}, op3={}", content, set_flag);
        }
        Ok(Self {
            x,
            y,
            open_flag,
            content: match content {
                -1 => None,
                0..=99 => Some(ChestContent::Equipment(
                    items::Equipment::from_i32(content)
                        .ok_or_else(|| anyhow!("invalid parameter: op2={}", content))?,
                )),
                _ => Some(ChestContent::Rom(items::Rom(u8::try_from(content - 100)?))),
            },
            set_flag,
            unused,
            starts,
        })
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn open_flag(&self) -> i32 {
        self.open_flag
    }
    pub fn content(&self) -> Option<&ChestContent> {
        self.content.as_ref()
    }
    pub fn op2(&self) -> i32 {
        match self.content {
            Some(ChestContent::Equipment(content)) => content as i32,
            Some(ChestContent::Rom(content)) => content.0 as i32 + 100,
            None => -1,
        }
    }
    pub fn set_flag(&self) -> i32 {
        self.set_flag
    }
    pub fn op4(&self) -> i32 {
        self.unused
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }

    pub fn to_chest_item(&self) -> Option<ChestItem> {
        match self.content {
            Some(ChestContent::Equipment(content)) => Some(ChestItem::Equipment(Equipment {
                content,
                price: None,
                flag: self.set_flag as u16,
            })),
            Some(ChestContent::Rom(rom)) => Some(ChestItem::Rom(Rom {
                content: rom,
                price: None,
                flag: self.set_flag as u16,
            })),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct SealObject {
    x: i32,
    y: i32,
    content: items::Seal,
    set_flag: u16,
    starts: Vec<Start>,
}

impl SealObject {
    pub fn new(
        x: i32,
        y: i32,
        content: i32,
        set_flag: i32,
        op3: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if op3 != -1 || op4 != -1 {
            bail!("invalid parameters: op3={}, op4={}", op3, op4);
        }
        Ok(Self {
            x,
            y,
            content: items::Seal::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: content={}", content))?,
            set_flag: u16::try_from(set_flag)?,
            starts,
        })
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn op1(&self) -> i32 {
        self.content as i32
    }
    pub fn set_flag(&self) -> u16 {
        self.set_flag
    }
    pub fn op2(&self) -> i32 {
        self.set_flag as i32
    }
    pub const fn op3() -> i32 {
        -1
    }
    pub const fn op4() -> i32 {
        -1
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }

    pub fn to_seal(&self) -> Seal {
        Seal {
            content: self.content,
            flag: self.set_flag,
        }
    }
}

#[derive(Clone)]
pub struct UnknownObject {
    pub number: u16,
    pub x: i32,
    pub y: i32,
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    pub starts: Vec<Start>,
}

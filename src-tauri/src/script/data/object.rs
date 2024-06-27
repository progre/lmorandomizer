use std::num::NonZero;

use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use super::{
    items,
    shop_items_data::{self, ShopItem},
};

pub struct MainWeapon {
    pub content: items::MainWeapon,
    pub flag: u16,
}

pub struct SubWeapon {
    pub content: items::SubWeapon,
    pub count: u16,
    pub flag: u16,
}

#[derive(PartialEq)]
pub enum ChestContent {
    Equipment(items::Equipment),
    Rom(items::Rom),
}

pub struct ChestItem {
    pub content: Option<ChestContent>,
    pub open_flag: i32,
    /// < 0 means not set
    pub flag: i32,
}

pub struct Seal {
    pub content: items::Seal,
    pub flag: u16,
}

pub struct Shop {
    pub talk_number: u16,
    pub items: (ShopItem, ShopItem, ShopItem),
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Start {
    /// max 99999
    pub flag: u32,
    pub run_when_unset: bool,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub number: u16,
    pub x: i32,
    pub y: i32,
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    pub starts: Vec<Start>,
}

impl Object {
    pub fn chest(pos: &Object, op1: i32, number: i16, set_flag: u16, starts: Vec<Start>) -> Self {
        Object {
            number: 1,
            x: pos.x,
            y: pos.y,
            op1,
            op2: number as i32,
            op3: set_flag as i32,
            op4: -1,
            starts,
        }
    }

    pub fn main_weapon(
        pos: &Object,
        content: items::MainWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Self {
        Object {
            number: 77,
            x: pos.x,
            y: pos.y,
            op1: content as i32,
            op2: set_flag as i32,
            op3: -1,
            op4: -1,
            starts,
        }
    }

    pub fn sub_weapon_body(
        pos: &Object,
        content: items::SubWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Self {
        debug_assert!(content != items::SubWeapon::AnkhJewel);
        Object {
            number: 13,
            x: pos.x,
            y: pos.y,
            op1: content as i32,
            op2: 0,
            op3: set_flag as i32,
            op4: -1,
            starts,
        }
    }

    pub fn sub_weapon_ammo(
        pos: &Object,
        content: items::SubWeapon,
        count: NonZero<u8>,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Self {
        debug_assert!(content != items::SubWeapon::AnkhJewel || count.get() == 1);
        Object {
            number: 13,
            x: pos.x,
            y: pos.y,
            op1: content as i32,
            op2: count.get() as i32,
            op3: set_flag as i32,
            op4: -1,
            starts,
        }
    }

    pub fn seal(pos: &Object, content: items::Seal, set_flag: u16, starts: Vec<Start>) -> Self {
        Object {
            number: 71,
            x: pos.x,
            y: pos.y,
            op1: content as i32,
            op2: set_flag as i32,
            op3: -1,
            op4: -1,
            starts,
        }
    }

    pub fn to_main_weapon(&self) -> Result<Option<MainWeapon>> {
        if self.number != 77 {
            return Ok(None);
        }
        Ok(Some(MainWeapon {
            content: items::MainWeapon::from_i32(self.op1)
                .ok_or_else(|| anyhow!("invalid main weapon number: {}", self.op1))?,
            flag: u16::try_from(self.op2)?,
        }))
    }

    pub fn to_sub_weapon(&self) -> Result<Option<SubWeapon>> {
        if self.number != 13 {
            return Ok(None);
        }
        Ok(Some(SubWeapon {
            content: items::SubWeapon::from_i32(self.op1)
                .ok_or_else(|| anyhow!("invalid sub weapon number: {}", self.op1))?,
            count: u16::try_from(self.op2)?,
            flag: u16::try_from(self.op3)?,
        }))
    }

    pub fn to_chest_item(&self) -> Result<Option<ChestItem>> {
        if self.number != 1 {
            return Ok(None);
        }
        Ok(Some(ChestItem {
            content: if self.op2 == -1 {
                None
            } else if self.op2 < 100 {
                Some(ChestContent::Equipment(
                    items::Equipment::from_i32(self.op2)
                        .ok_or_else(|| anyhow!("invalid equipment number: {}", self.op2))?,
                ))
            } else {
                Some(ChestContent::Rom(items::Rom(u8::try_from(self.op2 - 100)?)))
            },
            open_flag: self.op1,
            flag: self.op3,
        }))
    }

    pub fn to_seal(&self) -> Result<Option<Seal>> {
        if self.number != 71 {
            return Ok(None);
        }
        Ok(Some(Seal {
            content: items::Seal::from_i32(self.op1)
                .ok_or_else(|| anyhow!("invalid seal number: {}", self.op1))?,
            flag: u16::try_from(self.op2)?,
        }))
    }

    pub fn to_shop(&self, talks: &[String]) -> Result<Option<Shop>> {
        if !(self.number == 14 && self.op1 <= 99) {
            return Ok(None);
        }
        let talk_number = u16::try_from(self.op4)?;
        Ok(Some(Shop {
            talk_number,
            items: shop_items_data::parse(&talks[talk_number as usize])?,
        }))
    }

    pub fn get_item_flag(&self) -> Result<u16> {
        Ok(match self.number {
            77 => self.to_main_weapon()?.unwrap().flag,
            13 => self.to_sub_weapon()?.unwrap().flag,
            1 => u16::try_from(self.to_chest_item()?.unwrap().flag)?,
            71 => self.to_seal()?.unwrap().flag,
            _ => bail!("invalid number: {}", self.number),
        })
    }
}

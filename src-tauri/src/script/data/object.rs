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

#[derive(Clone)]
pub struct Start {
    /// max 99999
    pub flag: u32,
    pub run_when_unset: bool,
}

#[derive(Clone)]
pub struct ChestObject {
    number: u16,
    x: i32,
    y: i32,
    pub op1: i32,
    pub op2: i32,
    op3: i32,
    op4: i32,
    pub starts: Vec<Start>,
}

impl ChestObject {
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
}

#[derive(Clone)]
pub struct SubWeaponObject {
    number: u16,
    x: i32,
    y: i32,
    pub op1: i32,
    op2: i32,
    pub op3: i32,
    op4: i32,
    starts: Vec<Start>,
}

impl SubWeaponObject {
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
}

#[derive(Clone)]
pub struct ShopObject {
    number: u16,
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
}

impl ShopObject {
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
}

#[derive(Clone)]
pub struct SealObject {
    number: u16,
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
}

impl SealObject {
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
}

#[derive(Clone)]
pub struct MainWeaponObject {
    number: u16,
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
}

impl MainWeaponObject {
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

#[derive(Clone)]
pub enum Object {
    Chest(ChestObject),
    SubWeapon(SubWeaponObject),
    Shop(ShopObject),
    Seal(SealObject),
    MainWeapon(MainWeaponObject),
    Unknown(UnknownObject),
}

impl Object {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        number: u16,
        x: i32,
        y: i32,
        op1: i32,
        op2: i32,
        op3: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Object {
        match number {
            1 => Object::Chest(ChestObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
            13 => Object::SubWeapon(SubWeaponObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
            14 => Object::Shop(ShopObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
            71 => Object::Seal(SealObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
            77 => Object::MainWeapon(MainWeaponObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
            _ => Object::Unknown(UnknownObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
        }
    }

    pub fn chest(pos: &Object, op1: i32, number: i16, set_flag: u16, starts: Vec<Start>) -> Object {
        Object::Chest(ChestObject {
            number: 1,
            x: pos.x(),
            y: pos.y(),
            op1,
            op2: number as i32,
            op3: set_flag as i32,
            op4: -1,
            starts,
        })
    }

    pub fn sub_weapon_body(
        pos: &Object,
        content: items::SubWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Object {
        debug_assert!(content != items::SubWeapon::AnkhJewel);
        Object::SubWeapon(SubWeaponObject {
            number: 13,
            x: pos.x(),
            y: pos.y(),
            op1: content as i32,
            op2: 0,
            op3: set_flag as i32,
            op4: -1,
            starts,
        })
    }

    pub fn sub_weapon_ammo(
        pos: &Object,
        content: items::SubWeapon,
        count: NonZero<u8>,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Object {
        debug_assert!(content != items::SubWeapon::AnkhJewel || count.get() == 1);
        Object::SubWeapon(SubWeaponObject {
            number: 13,
            x: pos.x(),
            y: pos.y(),
            op1: content as i32,
            op2: count.get() as i32,
            op3: set_flag as i32,
            op4: -1,
            starts,
        })
    }

    pub fn seal(pos: &Object, content: items::Seal, set_flag: u16, starts: Vec<Start>) -> Object {
        Object::Seal(SealObject {
            number: 71,
            x: pos.x(),
            y: pos.y(),
            op1: content as i32,
            op2: set_flag as i32,
            op3: -1,
            op4: -1,
            starts,
        })
    }

    pub fn main_weapon(
        pos: &Object,
        content: items::MainWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Object {
        Object::MainWeapon(MainWeaponObject {
            number: 77,
            x: pos.x(),
            y: pos.y(),
            op1: content as i32,
            op2: set_flag as i32,
            op3: -1,
            op4: -1,
            starts,
        })
    }

    pub fn number(&self) -> u16 {
        match self {
            Self::Chest(obj) => obj.number,
            Self::SubWeapon(obj) => obj.number,
            Self::Shop(obj) => obj.number,
            Self::Seal(obj) => obj.number,
            Self::MainWeapon(obj) => obj.number,
            Self::Unknown(obj) => obj.number,
        }
    }
    pub fn x(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.x,
            Self::SubWeapon(obj) => obj.x,
            Self::Shop(obj) => obj.x,
            Self::Seal(obj) => obj.x,
            Self::MainWeapon(obj) => obj.x,
            Self::Unknown(obj) => obj.x,
        }
    }
    pub fn y(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.y,
            Self::SubWeapon(obj) => obj.y,
            Self::Shop(obj) => obj.y,
            Self::Seal(obj) => obj.y,
            Self::MainWeapon(obj) => obj.y,
            Self::Unknown(obj) => obj.y,
        }
    }
    pub fn op1(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op1,
            Self::SubWeapon(obj) => obj.op1,
            Self::Shop(obj) => obj.op1,
            Self::Seal(obj) => obj.op1,
            Self::MainWeapon(obj) => obj.op1,
            Self::Unknown(obj) => obj.op1,
        }
    }
    pub fn op2(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op2,
            Self::SubWeapon(obj) => obj.op2,
            Self::Shop(obj) => obj.op2,
            Self::Seal(obj) => obj.op2,
            Self::MainWeapon(obj) => obj.op2,
            Self::Unknown(obj) => obj.op2,
        }
    }
    pub fn op3(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op3,
            Self::SubWeapon(obj) => obj.op3,
            Self::Shop(obj) => obj.op3,
            Self::Seal(obj) => obj.op3,
            Self::MainWeapon(obj) => obj.op3,
            Self::Unknown(obj) => obj.op3,
        }
    }
    pub fn op4(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op4,
            Self::SubWeapon(obj) => obj.op4,
            Self::Shop(obj) => obj.op4,
            Self::Seal(obj) => obj.op4,
            Self::MainWeapon(obj) => obj.op4,
            Self::Unknown(obj) => obj.op4,
        }
    }
    pub fn starts(&self) -> &[Start] {
        match self {
            Self::Chest(obj) => &obj.starts,
            Self::SubWeapon(obj) => &obj.starts,
            Self::Shop(obj) => &obj.starts,
            Self::Seal(obj) => &obj.starts,
            Self::MainWeapon(obj) => &obj.starts,
            Self::Unknown(obj) => &obj.starts,
        }
    }

    pub fn get_item_flag(&self) -> Result<u16> {
        Ok(match self {
            Self::Chest(obj) => u16::try_from(obj.to_chest_item()?.unwrap().flag)?,
            Self::SubWeapon(obj) => obj.to_sub_weapon()?.unwrap().flag,
            Self::Seal(obj) => obj.to_seal()?.unwrap().flag,
            Self::MainWeapon(obj) => obj.to_main_weapon()?.unwrap().flag,
            Self::Shop(ShopObject { number, .. }) | Self::Unknown(UnknownObject { number, .. }) => {
                bail!("invalid number: {}", number)
            }
        })
    }
}

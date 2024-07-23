mod field_objects;
mod shop_object;
mod weapon_objects;

use anyhow::{bail, Result};

use super::{items, shop_items_data};

pub use field_objects::{ChestContent, ChestObject, SealObject, UnknownObject};
pub use shop_object::{Shop, ShopObject};
pub use weapon_objects::{MainWeaponObject, SubWeaponObject};

#[derive(Clone)]
pub struct Start {
    /// max 99999
    pub flag: u32,
    pub run_when_unset: bool,
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
    ) -> Result<Object> {
        Ok(match number {
            1 => Object::Chest(ChestObject::new(x, y, op1, op2, op3, op4, starts)?),
            13 => Object::SubWeapon(SubWeaponObject::new(x, y, op1, op2, op3, op4, starts)?),
            14 => Object::Shop(ShopObject::new(x, y, op1, op2, op3, op4, starts)?),
            71 => Object::Seal(SealObject::new(x, y, op1, op2, op3, op4, starts)?),
            77 => Object::MainWeapon(MainWeaponObject::new(x, y, op1, op2, op3, op4, starts)?),
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
        })
    }

    pub fn chest(
        pos: &Object,
        op1: i32,
        number: i16,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::Chest(ChestObject::new(
            pos.x(),
            pos.y(),
            op1,
            number as i32,
            set_flag as i32,
            -1,
            starts,
        )?))
    }

    pub fn sub_weapon(
        pos: &Object,
        content: items::SubWeapon,
        count: u8,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        if cfg!(debug_assertions) {
            if count == 0 {
                debug_assert!(content != items::SubWeapon::AnkhJewel);
            } else {
                debug_assert!(content != items::SubWeapon::AnkhJewel || count == 1);
            }
        }
        Ok(Object::SubWeapon(SubWeaponObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            count as i32,
            set_flag as i32,
            -1,
            starts,
        )?))
    }

    pub fn seal(
        pos: &Object,
        content: items::Seal,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::Seal(SealObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            set_flag as i32,
            -1,
            -1,
            starts,
        )?))
    }

    pub fn main_weapon(
        pos: &Object,
        content: items::MainWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::MainWeapon(MainWeaponObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            set_flag as i32,
            -1,
            -1,
            starts,
        )?))
    }

    pub fn number(&self) -> u16 {
        match self {
            Self::Chest(_) => 1,
            Self::SubWeapon(_) => 13,
            Self::Shop(_) => 14,
            Self::Seal(_) => 71,
            Self::MainWeapon(_) => 77,
            Self::Unknown(obj) => obj.number,
        }
    }
    pub fn x(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.x(),
            Self::SubWeapon(obj) => obj.x(),
            Self::Shop(obj) => obj.x(),
            Self::Seal(obj) => obj.x(),
            Self::MainWeapon(obj) => obj.x(),
            Self::Unknown(obj) => obj.x,
        }
    }
    pub fn y(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.y(),
            Self::SubWeapon(obj) => obj.y(),
            Self::Shop(obj) => obj.y(),
            Self::Seal(obj) => obj.y(),
            Self::MainWeapon(obj) => obj.y(),
            Self::Unknown(obj) => obj.y,
        }
    }
    pub fn op1(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.open_flag(),
            Self::SubWeapon(obj) => obj.op1(),
            Self::Shop(obj) => obj.form(),
            Self::Seal(obj) => obj.op1(),
            Self::MainWeapon(obj) => obj.op1(),
            Self::Unknown(obj) => obj.op1,
        }
    }
    pub fn op2(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op2(),
            Self::SubWeapon(obj) => obj.op2(),
            Self::Shop(obj) => obj.music(),
            Self::Seal(obj) => obj.op2(),
            Self::MainWeapon(obj) => obj.op2(),
            Self::Unknown(obj) => obj.op2,
        }
    }
    pub fn op3(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.set_flag(),
            Self::SubWeapon(obj) => obj.op3(),
            Self::Shop(obj) => obj.op3(),
            Self::Seal(_) => SealObject::op3(),
            Self::MainWeapon(_) => MainWeaponObject::op3(),
            Self::Unknown(obj) => obj.op3,
        }
    }
    pub fn op4(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op4(),
            Self::SubWeapon(_) => SubWeaponObject::op4(),
            Self::Shop(obj) => obj.op4(),
            Self::Seal(_) => SealObject::op4(),
            Self::MainWeapon(_) => MainWeaponObject::op4(),
            Self::Unknown(obj) => obj.op4,
        }
    }
    pub fn starts(&self) -> &[Start] {
        match self {
            Self::Chest(obj) => obj.starts(),
            Self::SubWeapon(obj) => obj.starts(),
            Self::Shop(obj) => obj.starts(),
            Self::Seal(obj) => obj.starts(),
            Self::MainWeapon(obj) => obj.starts(),
            Self::Unknown(obj) => &obj.starts,
        }
    }

    pub fn get_item_flag(&self) -> Result<u16> {
        Ok(match self {
            Self::Chest(obj) => u16::try_from(obj.set_flag())?,
            Self::SubWeapon(obj) => obj.set_flag(),
            Self::Seal(obj) => obj.set_flag(),
            Self::MainWeapon(obj) => obj.set_flag(),
            Self::Shop(_) => bail!("shop has no item flag"),
            Self::Unknown(UnknownObject { number, .. }) => bail!("invalid number: {}", number),
        })
    }
}

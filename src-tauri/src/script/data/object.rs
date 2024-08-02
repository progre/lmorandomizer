mod field_objects;
mod shop_object;
pub mod starts;
mod weapon_objects;

use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use crate::script::enums;

use super::{
    item::{ChestItem, Equipment, MainWeapon, Rom, Seal, SubWeapon},
    shop_items_data,
};

pub use field_objects::{ChestObject, RomObject, SealObject, UnknownObject};
pub use shop_object::{Shop, ShopObject};
pub use weapon_objects::{MainWeaponObject, SubWeaponObject};

#[derive(Clone, PartialEq)]
pub struct Start {
    /// max 99999
    pub flag: u32,
    pub run_when: bool,
}

fn create_chest_object(
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Result<ChestObject> {
    let open_flag = u16::try_from(op1)?;
    let item = match op2 {
        -1 => ChestItem::None(op3),
        0..=99 => ChestItem::Equipment(Equipment {
            content: enums::Equipment::from_i32(op2)
                .ok_or_else(|| anyhow!("invalid parameter: op2={}", op2))?,
            price: None,
            flag: u16::try_from(op3)?,
        }),
        _ => ChestItem::Rom(Rom {
            content: enums::Rom::from_i32(op2 - 100)
                .ok_or_else(|| anyhow!("invalid parameter: op2={}", op2))?,
            price: None,
            flag: u16::try_from(op3)?,
        }),
    };
    if ![-1, 0].contains(&op4) {
        bail!("invalid parameter: op4={}", op4);
    }
    Ok(ChestObject::new(x, y, open_flag, item, op4, starts))
}

fn create_sub_weapon_object(
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Result<SubWeaponObject> {
    if op4 != -1 {
        bail!("invalid parameter: op4={}", op4);
    }
    let sub_weapon = SubWeapon {
        content: enums::SubWeapon::from_i32(op1)
            .ok_or_else(|| anyhow!("invalid parameter: op1={}", op1))?,
        amount: u8::try_from(op2)?,
        price: None,
        flag: u16::try_from(op3)?,
    };
    Ok(SubWeaponObject::new(x, y, sub_weapon, starts))
}

fn create_rom_object(
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Result<RomObject> {
    if op3 != -1 || op4 != -1 {
        bail!("invalid parameters: op3={}, op4={}", op3, op4);
    }
    let rom = Rom {
        content: enums::Rom::from_i32(op1)
            .ok_or_else(|| anyhow!("invalid parameter: op1={}", op1))?,
        price: None,
        flag: u16::try_from(op2)?,
    };
    Ok(RomObject::new(x, y, rom, starts))
}

fn create_seal_object(
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Result<SealObject> {
    if op3 != -1 || op4 != -1 {
        bail!("invalid parameters: op3={}, op4={}", op3, op4);
    }
    let seal = Seal {
        content: enums::Seal::from_i32(op1)
            .ok_or_else(|| anyhow!("invalid parameter: content={}", op1))?,
        flag: u16::try_from(op2)?,
    };
    Ok(SealObject::new(x, y, seal, starts))
}

fn create_main_weapon_object(
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Result<MainWeaponObject> {
    if op3 != -1 || op4 != -1 {
        bail!("invalid parameters: op3={}, op4={}", op3, op4);
    }
    let main_weapon = MainWeapon {
        content: enums::MainWeapon::from_i32(op1)
            .ok_or_else(|| anyhow!("invalid parameter: op1={}", op1))?,
        flag: u16::try_from(op2)?,
    };
    Ok(MainWeaponObject::new(x, y, main_weapon, starts))
}

#[derive(Clone)]
pub enum Object {
    Chest(ChestObject),
    SubWeapon(SubWeaponObject),
    Shop(ShopObject),
    Rom(RomObject),
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
            1 => Object::Chest(create_chest_object(x, y, op1, op2, op3, op4, starts)?),
            13 => Object::SubWeapon(create_sub_weapon_object(x, y, op1, op2, op3, op4, starts)?),
            14 => Object::Shop(ShopObject::new(x, y, op1, op2, op3, op4, starts)?),
            32 => Object::Rom(create_rom_object(x, y, op1, op2, op3, op4, starts)?),
            71 => Object::Seal(create_seal_object(x, y, op1, op2, op3, op4, starts)?),
            77 => Object::MainWeapon(create_main_weapon_object(x, y, op1, op2, op3, op4, starts)?),
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

    pub fn number(&self) -> u16 {
        match self {
            Self::Chest(_) => 1,
            Self::SubWeapon(_) => 13,
            Self::Shop(_) => 14,
            Self::Rom(_) => 32,
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
            Self::Rom(obj) => obj.x(),
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
            Self::Rom(obj) => obj.y(),
            Self::Seal(obj) => obj.y(),
            Self::MainWeapon(obj) => obj.y(),
            Self::Unknown(obj) => obj.y,
        }
    }
    pub fn op1(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.open_flag() as i32,
            Self::SubWeapon(obj) => obj.sub_weapon().content as i32,
            Self::Shop(obj) => obj.form(),
            Self::Rom(obj) => obj.rom().content as i32,
            Self::Seal(obj) => obj.seal().content as i32,
            Self::MainWeapon(obj) => obj.main_weapon().content as i32,
            Self::Unknown(obj) => obj.op1,
        }
    }
    pub fn op2(&self) -> i32 {
        match self {
            Self::Chest(obj) => match obj.item() {
                ChestItem::Equipment(item) => item.content as i32,
                ChestItem::Rom(item) => item.content as i32 + 100,
                ChestItem::None(_) => -1,
            },
            Self::SubWeapon(obj) => obj.sub_weapon().amount as i32,
            Self::Shop(obj) => obj.music(),
            Self::Rom(obj) => obj.rom().flag as i32,
            Self::Seal(obj) => obj.seal().flag as i32,
            Self::MainWeapon(obj) => obj.main_weapon().flag as i32,
            Self::Unknown(obj) => obj.op2,
        }
    }
    pub fn op3(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.item().flag(),
            Self::SubWeapon(obj) => obj.sub_weapon().flag as i32,
            Self::Shop(obj) => obj.op3(),
            Self::Rom(_) => -1,
            Self::Seal(_) => -1,
            Self::MainWeapon(_) => -1,
            Self::Unknown(obj) => obj.op3,
        }
    }
    pub fn op4(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op4(),
            Self::SubWeapon(_) => -1,
            Self::Shop(obj) => obj.op4(),
            Self::Rom(_) => -1,
            Self::Seal(_) => -1,
            Self::MainWeapon(_) => -1,
            Self::Unknown(obj) => obj.op4,
        }
    }
    pub fn starts(&self) -> &[Start] {
        match self {
            Self::Chest(obj) => obj.starts(),
            Self::SubWeapon(obj) => obj.starts(),
            Self::Shop(obj) => obj.starts(),
            Self::Rom(obj) => obj.starts(),
            Self::Seal(obj) => obj.starts(),
            Self::MainWeapon(obj) => obj.starts(),
            Self::Unknown(obj) => &obj.starts,
        }
    }

    pub fn set_flag(&self) -> Result<u16> {
        Ok(match self {
            Self::Chest(obj) => u16::try_from(obj.item().flag())?,
            Self::SubWeapon(obj) => obj.sub_weapon().flag,
            Self::Shop(_) => bail!("shop has no item flag"),
            Self::Rom(obj) => obj.rom().flag,
            Self::Seal(obj) => obj.seal().flag,
            Self::MainWeapon(obj) => obj.main_weapon().flag,
            Self::Unknown(UnknownObject { number, .. }) => bail!("invalid number: {}", number),
        })
    }
}

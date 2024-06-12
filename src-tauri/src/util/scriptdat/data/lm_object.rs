use anyhow::{bail, Result};

use super::script::{ChestItem, MainWeapon, Seal, SubWeapon};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct LMStart {
    pub number: i32,
    pub value: bool,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct LMObject {
    pub number: u16,
    pub x: i32,
    pub y: i32,
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    // elements
    pub starts: Vec<LMStart>,
}

impl LMObject {
    pub fn as_main_weapon(&self) -> Result<MainWeapon> {
        Ok(MainWeapon {
            main_weapon_number: u8::try_from(self.op1)?,
            flag: self.op2,
        })
    }

    pub fn as_sub_weapon(&self) -> Result<SubWeapon> {
        Ok(SubWeapon {
            sub_weapon_number: u8::try_from(self.op1)?,
            count: u16::try_from(self.op2)?,
            flag: self.op3,
        })
    }

    pub fn as_chest_item(&self) -> Result<ChestItem> {
        debug_assert_eq!(self.number, 1);
        Ok(ChestItem {
            chest_item_number: i16::try_from(self.op2)?,
            open_flag: self.op1,
            flag: self.op3,
        })
    }

    pub fn as_seal(&self) -> Result<Seal> {
        Ok(Seal {
            seal_number: u8::try_from(self.op1)?,
            flag: self.op2,
        })
    }

    pub fn get_item_flag(&self) -> Result<i32> {
        Ok(match self.number {
            77 => self.as_main_weapon()?.flag,
            13 => self.as_sub_weapon()?.flag,
            1 => self.as_chest_item()?.flag,
            71 => self.as_seal()?.flag,
            _ => bail!("invalid number: {}", self.number),
        })
    }
}

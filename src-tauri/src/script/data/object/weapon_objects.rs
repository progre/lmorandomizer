use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use super::{
    super::item::{MainWeapon, SubWeapon},
    items, Start,
};

#[derive(Clone)]
pub struct SubWeaponObject {
    x: i32,
    y: i32,
    content: items::SubWeapon,
    amount: u8,
    set_flag: u16,
    starts: Vec<Start>,
}

impl SubWeaponObject {
    pub fn new(
        x: i32,
        y: i32,
        content: i32,
        amount: i32,
        set_flag: i32,
        unused: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if unused != -1 {
            bail!("invalid parameter: op4={}", unused);
        }
        Ok(Self {
            x,
            y,
            content: items::SubWeapon::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: op1={}", content))?,
            amount: u8::try_from(amount)?,
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
    pub fn content(&self) -> items::SubWeapon {
        self.content
    }
    pub fn op1(&self) -> i32 {
        self.content as i32
    }
    pub fn op2(&self) -> i32 {
        self.amount as i32
    }
    pub fn set_flag(&self) -> u16 {
        self.set_flag
    }
    pub fn op3(&self) -> i32 {
        self.set_flag as i32
    }
    pub const fn op4() -> i32 {
        -1
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }

    pub fn to_sub_weapon(&self) -> SubWeapon {
        SubWeapon {
            content: self.content,
            amount: self.amount,
            price: None,
            set_flag: self.set_flag,
        }
    }
}

#[derive(Clone)]
pub struct MainWeaponObject {
    x: i32,
    y: i32,
    content: items::MainWeapon,
    set_flag: u16,
    starts: Vec<Start>,
}

impl MainWeaponObject {
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
            content: items::MainWeapon::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: op1={}", content))?,
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

    pub fn to_main_weapon(&self) -> MainWeapon {
        MainWeapon {
            content: self.content,
            set_flag: self.set_flag,
        }
    }
}

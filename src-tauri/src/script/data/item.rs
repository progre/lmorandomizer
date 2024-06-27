use std::num::NonZero;

use crate::dataset;

use super::items;

pub struct MainWeapon {
    pub content: items::MainWeapon,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct SubWeaponBody {
    pub content: items::SubWeapon,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct SubWeaponAmmo {
    pub content: items::SubWeapon,
    pub amount: NonZero<u8>,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct Equipment {
    pub content: items::Equipment,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct Rom {
    pub content: items::Rom,
    pub set_flag: u16,
}

pub struct Seal {
    pub content: items::Seal,
    pub set_flag: u16,
}

pub enum Item {
    MainWeapon(MainWeapon),
    SubWeaponBody(SubWeaponBody),
    SubWeaponAmmo(SubWeaponAmmo),
    Equipment(Equipment),
    Rom(Rom),
    Seal(Seal),
}
impl Item {
    pub fn from_dataset(item: &dataset::item::Item) -> Self {
        match item {
            dataset::item::Item::MainWeapon(item) => Self::MainWeapon(MainWeapon {
                content: item.content,
                set_flag: item.flag,
            }),
            dataset::item::Item::SubWeaponBody(item) => Self::SubWeaponBody(SubWeaponBody {
                content: item.content,
                set_flag: item.flag,
            }),
            dataset::item::Item::SubWeaponAmmo(item) => Self::SubWeaponAmmo(SubWeaponAmmo {
                content: item.content,
                amount: item.count,
                set_flag: item.flag,
            }),
            dataset::item::Item::Equipment(item) => Self::Equipment(Equipment {
                content: item.content,
                set_flag: item.flag,
            }),
            dataset::item::Item::Rom(item) => Self::Rom(Rom {
                content: item.content,
                set_flag: item.flag,
            }),
            dataset::item::Item::Seal(item) => Self::Seal(Seal {
                content: item.content,
                set_flag: item.flag,
            }),
        }
    }

    pub fn set_flag(&self) -> u16 {
        match self {
            Self::MainWeapon(item) => item.set_flag,
            Self::SubWeaponBody(item) => item.set_flag,
            Self::SubWeaponAmmo(item) => item.set_flag,
            Self::Equipment(item) => item.set_flag,
            Self::Rom(item) => item.set_flag,
            Self::Seal(item) => item.set_flag,
        }
    }
}

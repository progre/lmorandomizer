use std::num::NonZero;

use anyhow::{anyhow, bail, Result};

use crate::dataset;

use super::{
    items,
    object::{ChestContent, Shop},
    script::Script,
    shop_items_data::ShopItem,
};

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

fn shop_item(shops: &[Shop], shop_idx: usize, item_idx: usize) -> Result<&ShopItem> {
    let shop = shops
        .get(shop_idx)
        .ok_or_else(|| anyhow!("invalid shop index: {}", shop_idx))?;
    Ok(match item_idx {
        0 => &shop.items.0,
        1 => &shop.items.1,
        2 => &shop.items.2,
        _ => bail!("invalid shop item index: {}", item_idx),
    })
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
    #[inline]
    fn initial_assert(number: i8, set_flag: u16, is_sub_weapon: bool) {
        debug_assert!(
            [494, 524].contains(&set_flag)
                || (684..=883).contains(&set_flag)
                || is_sub_weapon && set_flag == 65279,
            "invalid value: {set_flag} ({number})"
        );
    }

    pub fn from_dataset(item: &dataset::item::Item, script: &Script) -> Result<Self> {
        match item {
            dataset::item::Item::MainWeapon(item) => {
                let main_weapons = script.main_weapons()?;
                let item = main_weapons
                    .get(item.src_idx)
                    .ok_or_else(|| anyhow!("invalid main weapon index: {}", item.src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, false);
                Ok(Self::MainWeapon(MainWeapon { content, set_flag }))
            }
            dataset::item::Item::SubWeaponBody(item) => {
                let sub_weapons = script.sub_weapons()?;
                let item = sub_weapons
                    .get(item.src_idx)
                    .ok_or_else(|| anyhow!("invalid sub weapon index: {}", item.src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, true);
                Ok(Self::SubWeaponBody(SubWeaponBody { content, set_flag }))
            }
            dataset::item::Item::SubWeaponAmmo(item) => {
                let sub_weapons = script.sub_weapons()?;
                let item = sub_weapons
                    .get(item.src_idx)
                    .ok_or_else(|| anyhow!("invalid sub weapon index: {}", item.src_idx))?;
                Self::initial_assert(item.content as i8, item.flag, true);
                Ok(Self::SubWeaponAmmo(SubWeaponAmmo {
                    content: item.content,
                    amount: NonZero::new(u8::try_from(item.count)?)
                        .ok_or_else(|| anyhow!("invalid sub weapon ammo count: {}", item.count))?,
                    set_flag: item.flag,
                }))
            }
            dataset::item::Item::ChestItem(item) => {
                let chests = script.chests()?;
                let item = chests
                    .get(item.src_idx)
                    .ok_or_else(|| anyhow!("invalid chest index: {}", item.src_idx))?;
                match item.content {
                    Some(ChestContent::Equipment(content)) => {
                        let set_flag = u16::try_from(item.flag)?;
                        Self::initial_assert(content as i8, set_flag, false);
                        Ok(Self::Equipment(Equipment { content, set_flag }))
                    }
                    Some(ChestContent::Rom(content)) => {
                        let set_flag = u16::try_from(item.flag)?;
                        Self::initial_assert(content.0 as i8, set_flag, false);
                        Ok(Self::Rom(Rom { content, set_flag }))
                    }
                    None => bail!("chest item type mismatch"),
                }
            }
            dataset::item::Item::Seal(item) => {
                let seals = script.seals()?;
                let item = seals
                    .get(item.src_idx)
                    .ok_or_else(|| anyhow!("invalid seal index: {}", item.src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, false);
                Ok(Self::Seal(Seal { content, set_flag }))
            }
            dataset::item::Item::ShopItem(item) => {
                let shops = script.shops()?;
                let item = shop_item(&shops, item.src_idx.0, item.src_idx.1)?;
                match item {
                    ShopItem::SubWeaponBody(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, true);
                        Ok(Self::SubWeaponBody(SubWeaponBody {
                            content: item.item.content,
                            set_flag: item.item.set_flag,
                        }))
                    }
                    ShopItem::SubWeaponAmmo(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, true);
                        Ok(Self::SubWeaponAmmo(SubWeaponAmmo {
                            content: item.item.content,
                            amount: item.item.amount,
                            set_flag: item.item.set_flag,
                        }))
                    }
                    ShopItem::Equipment(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, false);
                        Ok(Self::Equipment(Equipment {
                            content: item.item.content,
                            set_flag: item.item.set_flag,
                        }))
                    }
                    ShopItem::Rom(item) => {
                        Self::initial_assert(item.item.content.0 as i8, item.item.set_flag, false);
                        Ok(Self::Rom(Rom {
                            content: item.item.content,
                            set_flag: item.item.set_flag,
                        }))
                    }
                }
            }
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

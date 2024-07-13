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
    pub price: Option<u16>,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct Equipment {
    pub content: items::Equipment,
    pub price: Option<u16>,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct Rom {
    pub content: items::Rom,
    pub price: Option<u16>,
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
        match &item.src {
            dataset::item::ItemSource::MainWeapon(src_idx) => {
                let main_weapons = script.main_weapons()?;
                let item = main_weapons
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid main weapon index: {}", src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, false);
                Ok(Self::MainWeapon(MainWeapon { content, set_flag }))
            }
            dataset::item::ItemSource::SubWeapon(src_idx) => {
                let sub_weapons = script.sub_weapons()?;
                let item = sub_weapons
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid sub weapon index: {}", src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, true);
                if item.count == 0 {
                    Ok(Self::SubWeaponBody(SubWeaponBody { content, set_flag }))
                } else {
                    Ok(Self::SubWeaponAmmo(SubWeaponAmmo {
                        content,
                        amount: NonZero::new(u8::try_from(item.count)?).ok_or_else(|| {
                            anyhow!("invalid sub weapon ammo count: {}", item.count)
                        })?,
                        price: None,
                        set_flag,
                    }))
                }
            }
            dataset::item::ItemSource::Chest(src_idx) => {
                let chests = script.chests()?;
                let item = chests
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid chest index: {}", src_idx))?;
                match item.content {
                    Some(ChestContent::Equipment(content)) => {
                        let set_flag = u16::try_from(item.flag)?;
                        Self::initial_assert(content as i8, set_flag, false);
                        Ok(Self::Equipment(Equipment {
                            content,
                            price: None,
                            set_flag,
                        }))
                    }
                    Some(ChestContent::Rom(content)) => {
                        let set_flag = u16::try_from(item.flag)?;
                        Self::initial_assert(content.0 as i8, set_flag, false);
                        Ok(Self::Rom(Rom {
                            content,
                            price: None,
                            set_flag,
                        }))
                    }
                    None => bail!("chest item type mismatch"),
                }
            }
            dataset::item::ItemSource::Seal(src_idx) => {
                let seals = script.seals()?;
                let item = seals
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid seal index: {}", src_idx))?;
                let content = item.content;
                let set_flag = item.flag;
                Self::initial_assert(content as i8, set_flag, false);
                Ok(Self::Seal(Seal { content, set_flag }))
            }
            dataset::item::ItemSource::Shop(shop_idx, item_idx) => {
                let shops = script.shops()?;
                let item = shop_item(&shops, *shop_idx, *item_idx)?;
                match item {
                    ShopItem::SubWeaponBody(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, true);
                        Ok(Self::SubWeaponBody(item.item.clone()))
                    }
                    ShopItem::SubWeaponAmmo(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, true);
                        Ok(Self::SubWeaponAmmo(item.item.clone()))
                    }
                    ShopItem::Equipment(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, false);
                        Ok(Self::Equipment(item.item.clone()))
                    }
                    ShopItem::Rom(item) => {
                        Self::initial_assert(item.item.content.0 as i8, item.item.set_flag, false);
                        Ok(Self::Rom(item.item.clone()))
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

    pub fn price(&self) -> Option<u16> {
        match self {
            Self::SubWeaponAmmo(item) => item.price,
            Self::Equipment(item) => item.price,
            Self::Rom(item) => item.price,
            _ => None,
        }
    }
}

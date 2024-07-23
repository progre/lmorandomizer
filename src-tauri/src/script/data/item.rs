use anyhow::{anyhow, bail, Result};

use crate::dataset;

use super::{items, object::Shop, script::Script, shop_items_data::ShopItem};

#[derive(Clone)]
pub struct MainWeapon {
    pub content: items::MainWeapon,
    pub set_flag: u16,
}

#[derive(Clone)]
pub struct SubWeapon {
    pub content: items::SubWeapon,
    pub amount: u8,
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

pub enum ChestItem {
    Equipment(Equipment),
    Rom(Rom),
}

#[derive(Clone)]
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
    SubWeapon(SubWeapon),
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
                let main_weapons = script.main_weapons();
                let item = main_weapons
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid main weapon index: {}", src_idx))?;
                Ok(Self::MainWeapon(item.clone()))
            }
            dataset::item::ItemSource::SubWeapon(src_idx) => {
                let sub_weapons = script.sub_weapons();
                let item = sub_weapons
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid sub weapon index: {}", src_idx))?;
                Ok(Self::SubWeapon(item.clone()))
            }
            dataset::item::ItemSource::Chest(src_idx) => {
                let chests = script.chests();
                let item = chests
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid chest index: {}", src_idx))?;
                match item {
                    ChestItem::Equipment(content) => Ok(Self::Equipment(content.clone())),
                    ChestItem::Rom(content) => Ok(Self::Rom(content.clone())),
                }
            }
            dataset::item::ItemSource::Seal(src_idx) => {
                let seals = script.seals();
                let item = seals
                    .get(*src_idx)
                    .ok_or_else(|| anyhow!("invalid seal index: {}", src_idx))?;
                Ok(Self::Seal(item.clone()))
            }
            dataset::item::ItemSource::Shop(shop_idx, item_idx) => {
                let shops = script.shops()?;
                let item = shop_item(&shops, *shop_idx, *item_idx)?;
                match item {
                    ShopItem::SubWeapon(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.set_flag, true);
                        Ok(Self::SubWeapon(item.item.clone()))
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
            Self::SubWeapon(item) => item.set_flag,
            Self::Equipment(item) => item.set_flag,
            Self::Rom(item) => item.set_flag,
            Self::Seal(item) => item.set_flag,
        }
    }

    pub fn price(&self) -> Option<u16> {
        match self {
            Self::SubWeapon(item) => item.price,
            Self::Equipment(item) => item.price,
            Self::Rom(item) => item.price,
            _ => None,
        }
    }
}

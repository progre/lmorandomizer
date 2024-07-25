use anyhow::{bail, Result};

use crate::dataset;

use super::{items, script::Script, shop_items_data::ShopItem};

#[derive(Clone)]
pub struct MainWeapon {
    pub content: items::MainWeapon,
    pub flag: u16,
}

#[derive(Clone)]
pub struct SubWeapon {
    pub content: items::SubWeapon,
    pub amount: u8,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone)]
pub struct Equipment {
    pub content: items::Equipment,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone)]
pub struct Rom {
    pub content: items::Rom,
    pub price: Option<u16>,
    pub flag: u16,
}

pub enum ChestItem {
    Equipment(Equipment),
    Rom(Rom),
}

#[derive(Clone)]
pub struct Seal {
    pub content: items::Seal,
    pub flag: u16,
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
    fn initial_assert(number: i8, flag: u16, is_sub_weapon: bool) {
        debug_assert!(
            [494, 524].contains(&flag)
                || (684..=883).contains(&flag)
                || is_sub_weapon && flag == 65279,
            "invalid value: {flag} ({number})"
        );
    }

    pub fn from_dataset(item: &dataset::item::Item, script: &Script) -> Result<Self> {
        Ok(match &item.src {
            dataset::item::ItemSource::MainWeapon(src_idx) => {
                let Some(item) = script.main_weapons().nth(*src_idx) else {
                    bail!("invalid main weapon index: {}", src_idx)
                };
                Self::MainWeapon(item.to_main_weapon())
            }
            dataset::item::ItemSource::SubWeapon(src_idx) => {
                let Some(item) = script.sub_weapons().nth(*src_idx) else {
                    bail!("invalid sub weapon index: {}", src_idx)
                };
                Self::SubWeapon(item.to_sub_weapon())
            }
            dataset::item::ItemSource::Chest(src_idx) => {
                let Some(item) = script.chests().nth(*src_idx) else {
                    bail!("invalid chest index: {}", src_idx)
                };
                match item.to_chest_item().unwrap() {
                    ChestItem::Equipment(content) => Self::Equipment(content),
                    ChestItem::Rom(content) => Self::Rom(content),
                }
            }
            dataset::item::ItemSource::Seal(src_idx) => {
                let Some(item) = script.seals().nth(*src_idx) else {
                    bail!("invalid seal index: {}", src_idx)
                };
                Self::Seal(item.to_seal())
            }
            dataset::item::ItemSource::Shop(shop_idx, item_idx) => {
                let Some(shop) = script
                    .shops()
                    .filter_map(|x| x.to_shop(&script.talks).transpose())
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .nth(*shop_idx)
                else {
                    bail!("invalid shop index: {}", shop_idx)
                };
                let item = match *item_idx {
                    0 => shop.items.0,
                    1 => shop.items.1,
                    2 => shop.items.2,
                    _ => bail!("invalid shop item index: {}", item_idx),
                };
                match item {
                    ShopItem::SubWeapon(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.flag, true);
                        Self::SubWeapon(item.item)
                    }
                    ShopItem::Equipment(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.flag, false);
                        Self::Equipment(item.item)
                    }
                    ShopItem::Rom(item) => {
                        Self::initial_assert(item.item.content.0 as i8, item.item.flag, false);
                        Self::Rom(item.item)
                    }
                }
            }
        })
    }

    pub fn flag(&self) -> u16 {
        match self {
            Self::MainWeapon(item) => item.flag,
            Self::SubWeapon(item) => item.flag,
            Self::Equipment(item) => item.flag,
            Self::Rom(item) => item.flag,
            Self::Seal(item) => item.flag,
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

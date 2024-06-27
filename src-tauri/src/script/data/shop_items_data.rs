use std::num::NonZero;

use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use crate::script::file::dat::{byte_code_to_text, text_to_byte_code};

use super::{
    item::{self, Equipment, Item, Rom, SubWeaponAmmo, SubWeaponBody},
    items,
};

pub fn parse(text: &str) -> Result<(ShopItem, ShopItem, ShopItem)> {
    debug_assert_eq!(text.chars().count(), 7 * 3);
    let data = text_to_byte_code(text);
    debug_assert_eq!(data.len(), 7 * 3);
    let mut iter = (0..3)
        .map(|i| i * 7)
        .map(|x| ShopItem::from_bytes(&data[x..x + 7]));
    Ok((
        iter.next().unwrap()?,
        iter.next().unwrap()?,
        iter.next().unwrap()?,
    ))
}

pub fn stringify(items: (ShopItem, ShopItem, ShopItem)) -> Result<String> {
    let data: Vec<_> = [items.0, items.1, items.2]
        .iter()
        .flat_map(|x| x.to_bytes())
        .collect();
    Ok(byte_code_to_text(&data))
}

#[derive(Clone)]
pub struct ShopSubWeaponBody {
    pub item: SubWeaponBody,
    price: u16,
}

#[derive(Clone)]
pub struct ShopSubWeaponAmmo {
    pub item: SubWeaponAmmo,
    price: u16,
}

#[derive(Clone)]
pub struct ShopEquipment {
    pub item: Equipment,
    price: u16,
}

#[derive(Clone)]
pub struct ShopRom {
    pub item: Rom,
    price: u16,
}

#[derive(Clone)]
pub enum ShopItem {
    SubWeaponBody(ShopSubWeaponBody),
    SubWeaponAmmo(ShopSubWeaponAmmo),
    Equipment(ShopEquipment),
    Rom(ShopRom),
}

impl ShopItem {
    pub fn from_item(item: Item, price: u16) -> Self {
        match item {
            Item::MainWeapon(_) => unreachable!(),
            Item::SubWeaponBody(item) => Self::SubWeaponBody(ShopSubWeaponBody { item, price }),
            Item::SubWeaponAmmo(item) => Self::SubWeaponAmmo(ShopSubWeaponAmmo { item, price }),
            Item::Equipment(item) => Self::Equipment(ShopEquipment { item, price }),
            Item::Rom(item) => Self::Rom(ShopRom { item, price }),
            Item::Seal(_) => unreachable!(),
        }
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        let shop_item_type = data[0] - 1;
        let number = data[1] - 1;
        let price = (((data[2] - 1) as u16) << 8) + data[3] as u16;
        let set_flag = (((data[5] - 1) as u16) << 8) + data[6] as u16; // 254 * 256 + 255 is no set flag
        match shop_item_type {
            0 => NonZero::new(data[4] - 1).map_or_else(
                || {
                    let item = item::SubWeaponBody {
                        content: items::SubWeapon::from_u8(number)
                            .ok_or_else(|| anyhow!("Invalid subweapon number: {}", number))?,
                        set_flag,
                    };
                    Ok(Self::SubWeaponBody(ShopSubWeaponBody { item, price }))
                },
                |amount| {
                    let item = item::SubWeaponAmmo {
                        content: items::SubWeapon::from_u8(number)
                            .ok_or_else(|| anyhow!("Invalid subweapon number: {}", number))?,
                        set_flag,
                        amount,
                    };
                    Ok(Self::SubWeaponAmmo(ShopSubWeaponAmmo { item, price }))
                },
            ),
            1 => {
                let item = item::Equipment {
                    content: items::Equipment::from_u8(number)
                        .ok_or_else(|| anyhow!("Invalid equipment number: {}", number))?,
                    set_flag,
                };
                Ok(Self::Equipment(ShopEquipment { item, price }))
            }
            // NOTE: 占いセンセーション(72) count is not as specified. It has 1 in it, not 0.
            2 => {
                let content = items::Rom(number);
                let item = item::Rom { content, set_flag };
                Ok(Self::Rom(ShopRom { item, price }))
            }
            _ => bail!("Invalid shop item type: {}", shop_item_type),
        }
    }

    fn to_bytes(&self) -> [u8; 7] {
        [
            match self {
                Self::SubWeaponBody(_) | Self::SubWeaponAmmo(_) => 0,
                Self::Equipment(_) => 1,
                Self::Rom(_) => 2,
            } + 1,
            self.number() + 1,
            ((self.price() >> 8) + 1) as u8,
            (self.price() % 0x100) as u8,
            self.count().map_or(0, |x| x.get()) + 1,
            ((self.set_flag() >> 8) + 1) as u8,
            (self.set_flag() % 0x100) as u8,
        ]
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::SubWeaponBody(x) => x.item.content as u8,
            Self::SubWeaponAmmo(x) => x.item.content as u8,
            Self::Equipment(x) => x.item.content as u8,
            Self::Rom(x) => x.item.content.0,
        }
    }
    pub fn price(&self) -> u16 {
        match self {
            Self::SubWeaponBody(x) => x.price,
            Self::SubWeaponAmmo(x) => x.price,
            Self::Equipment(x) => x.price,
            Self::Rom(x) => x.price,
        }
    }
    pub fn count(&self) -> Option<NonZero<u8>> {
        match self {
            Self::SubWeaponBody(_) => None,
            Self::SubWeaponAmmo(x) => Some(x.item.amount),
            Self::Equipment(_) => None,
            Self::Rom(_) => None,
        }
    }
    pub fn set_flag(&self) -> u16 {
        match self {
            Self::SubWeaponBody(x) => x.item.set_flag,
            Self::SubWeaponAmmo(x) => x.item.set_flag,
            Self::Equipment(x) => x.item.set_flag,
            Self::Rom(x) => x.item.set_flag,
        }
    }
}

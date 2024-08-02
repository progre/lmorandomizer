use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use crate::script::enums;

use super::{
    item::{Equipment, Item, Rom, SubWeapon},
    script::Talk,
};

pub fn parse(talk: &Talk) -> Result<(ShopItem, ShopItem, ShopItem)> {
    let data = talk.as_bytes();
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

pub fn stringify(items: (ShopItem, ShopItem, ShopItem)) -> Result<Talk> {
    let data: Vec<_> = [items.0, items.1, items.2]
        .iter()
        .flat_map(|x| x.to_bytes())
        .collect();
    Ok(Talk::from_bytes(data))
}

#[derive(Clone)]
pub struct ShopSubWeapon {
    pub item: SubWeapon,
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
    SubWeapon(ShopSubWeapon),
    Equipment(ShopEquipment),
    Rom(ShopRom),
}

impl ShopItem {
    pub fn to_spot_shop_items(
        selfs: &(ShopItem, ShopItem, ShopItem),
    ) -> (enums::ShopItem, enums::ShopItem, enums::ShopItem) {
        (
            selfs.0.to_spot_shop_item(),
            selfs.1.to_spot_shop_item(),
            selfs.2.to_spot_shop_item(),
        )
    }

    pub fn from_item(item: Item, price: u16) -> Self {
        match item {
            Item::MainWeapon(_) => unreachable!(),
            Item::SubWeapon(item) => Self::SubWeapon(ShopSubWeapon { item, price }),
            Item::Equipment(item) => Self::Equipment(ShopEquipment { item, price }),
            Item::Rom(item) => Self::Rom(ShopRom { item, price }),
            Item::Seal(_) => unreachable!(),
        }
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        let shop_item_type = data[0] - 1;
        let number = data[1] - 1;
        let price = (((data[2] - 1) as u16) << 8) + data[3] as u16;
        let mut flag = (((data[5] - 1) as u16) << 8) + data[6] as u16; // 254 * 256 + 255 is no set flag
        match shop_item_type {
            0 => {
                let content = enums::SubWeapon::from_u8(number)
                    .ok_or_else(|| anyhow!("Invalid subweapon number: {}", number))?;
                if content == enums::SubWeapon::HandScanner && flag == 65279 {
                    flag = 696;
                }
                let item = SubWeapon {
                    content,
                    amount: data[4] - 1,
                    price: Some(price),
                    flag,
                };
                Ok(Self::SubWeapon(ShopSubWeapon { item, price }))
            }
            1 => {
                let item = Equipment {
                    content: enums::Equipment::from_u8(number)
                        .ok_or_else(|| anyhow!("Invalid equipment number: {}", number))?,
                    price: Some(price),
                    flag,
                };
                Ok(Self::Equipment(ShopEquipment { item, price }))
            }
            // NOTE: 占いセンセーション(72) count is not as specified. It has 1 in it, not 0.
            2 => {
                let content = enums::Rom::from_u8(number)
                    .ok_or_else(|| anyhow!("Invalid rom number: {}", number))?;
                let item = Rom {
                    content,
                    price: Some(price),
                    flag,
                };
                Ok(Self::Rom(ShopRom { item, price }))
            }
            _ => bail!("Invalid shop item type: {}", shop_item_type),
        }
    }

    pub fn to_spot_shop_item(&self) -> enums::ShopItem {
        match self {
            ShopItem::Equipment(script) => enums::ShopItem::Equipment(script.item.content),
            ShopItem::Rom(script) => enums::ShopItem::Rom(script.item.content),
            ShopItem::SubWeapon(script) => enums::ShopItem::SubWeapon(script.item.content),
        }
    }
    fn to_bytes(&self) -> [u8; 7] {
        [
            match self {
                Self::SubWeapon(_) => 0,
                Self::Equipment(_) => 1,
                Self::Rom(_) => 2,
            } + 1,
            self.number() + 1,
            ((self.price() >> 8) + 1) as u8,
            (self.price() % 0x100) as u8,
            self.count().unwrap_or(0) + 1,
            ((self.flag() >> 8) + 1) as u8,
            (self.flag() % 0x100) as u8,
        ]
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::SubWeapon(x) => x.item.content as u8,
            Self::Equipment(x) => x.item.content as u8,
            Self::Rom(x) => x.item.content as u8,
        }
    }
    pub fn price(&self) -> u16 {
        match self {
            Self::SubWeapon(x) => x.price,
            Self::Equipment(x) => x.price,
            Self::Rom(x) => x.price,
        }
    }
    pub fn count(&self) -> Option<u8> {
        match self {
            Self::SubWeapon(x) => Some(x.item.amount),
            Self::Equipment(_) => None,
            Self::Rom(_) => None,
        }
    }
    pub fn flag(&self) -> u16 {
        match self {
            Self::SubWeapon(x) => x.item.flag,
            Self::Equipment(x) => x.item.flag,
            Self::Rom(x) => x.item.flag,
        }
    }
}

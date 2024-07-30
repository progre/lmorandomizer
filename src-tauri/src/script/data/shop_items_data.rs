use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

use crate::{
    dataset::spot,
    script::file::dat::{byte_code_to_text, text_to_byte_code},
};

use super::{
    item::{self, Equipment, Item, Rom, SubWeapon},
    items,
    script::Talk,
};

pub fn parse(text: &Talk) -> Result<(ShopItem, ShopItem, ShopItem)> {
    debug_assert_eq!(text.as_str().chars().count(), 7 * 3);
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

pub fn stringify(items: (ShopItem, ShopItem, ShopItem)) -> Result<Talk> {
    let data: Vec<_> = [items.0, items.1, items.2]
        .iter()
        .flat_map(|x| x.to_bytes())
        .collect();
    Ok(Talk::new(byte_code_to_text(&data)))
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
    ) -> (spot::ShopItem, spot::ShopItem, spot::ShopItem) {
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
        let flag = (((data[5] - 1) as u16) << 8) + data[6] as u16; // 254 * 256 + 255 is no set flag
        match shop_item_type {
            0 => {
                let item = item::SubWeapon {
                    content: items::SubWeapon::from_u8(number)
                        .ok_or_else(|| anyhow!("Invalid subweapon number: {}", number))?,
                    amount: data[4] - 1,
                    price: Some(price),
                    flag,
                };
                Ok(Self::SubWeapon(ShopSubWeapon { item, price }))
            }
            1 => {
                let item = item::Equipment {
                    content: items::Equipment::from_u8(number)
                        .ok_or_else(|| anyhow!("Invalid equipment number: {}", number))?,
                    price: Some(price),
                    flag,
                };
                Ok(Self::Equipment(ShopEquipment { item, price }))
            }
            // NOTE: 占いセンセーション(72) count is not as specified. It has 1 in it, not 0.
            2 => {
                let content = items::Rom::from_u8(number)
                    .ok_or_else(|| anyhow!("Invalid rom number: {}", number))?;
                let item = item::Rom {
                    content,
                    price: Some(price),
                    flag,
                };
                Ok(Self::Rom(ShopRom { item, price }))
            }
            _ => bail!("Invalid shop item type: {}", shop_item_type),
        }
    }

    pub fn to_spot_shop_item(&self) -> spot::ShopItem {
        match self {
            ShopItem::Equipment(script) => spot::ShopItem::Equipment(script.item.content),
            ShopItem::Rom(script) => spot::ShopItem::Rom(script.item.content),
            ShopItem::SubWeapon(script) => spot::ShopItem::SubWeapon(script.item.content),
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

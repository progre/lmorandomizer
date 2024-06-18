use std::num::NonZero;

use anyhow::{anyhow, Result};

use crate::script::dat::{byte_code_to_text, text_to_byte_code};

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

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ShopItemType {
    SubWeapon = 0,
    Item,
    Rom,
}

pub struct ShopItem {
    pub shop_item_type: ShopItemType,
    number: u8,
    price: u16,
    /// None means subweapon body
    count: Option<NonZero<u8>>,
    set_flag: u16,
}

impl ShopItem {
    pub fn sub_weapon(number: u8, price: u16, count: Option<NonZero<u8>>, set_flag: u16) -> Self {
        Self {
            shop_item_type: ShopItemType::SubWeapon,
            number,
            price,
            count,
            set_flag,
        }
    }
    pub fn equipment(number: u8, price: u16, set_flag: u16) -> Self {
        Self {
            shop_item_type: ShopItemType::Item,
            number,
            price,
            count: None,
            set_flag,
        }
    }
    pub fn rom(number: u8, price: u16, set_flag: u16) -> Self {
        Self {
            shop_item_type: ShopItemType::Rom,
            number,
            price,
            count: None,
            set_flag,
        }
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        let shop_item_type = match data[0] - 1 {
            0 => ShopItemType::SubWeapon,
            1 => ShopItemType::Item,
            2 => ShopItemType::Rom,
            _ => return Err(anyhow!("Invalid shop item type: {}", data[0] - 1)),
        };
        let number = data[1] - 1;
        // Only 占いセンセーション(72) is not in accordance with the specifications
        let count = if shop_item_type == ShopItemType::Rom && number == 72 {
            None
        } else {
            NonZero::new(data[4] - 1)
        };
        Ok(Self {
            shop_item_type,
            number,
            price: (((data[2] - 1) as u16) << 8) + data[3] as u16,
            count,
            set_flag: (((data[5] - 1) as u16) << 8) + data[6] as u16, // 254 * 256 + 255 is no set flag
        })
    }

    fn to_bytes(&self) -> [u8; 7] {
        [
            self.shop_item_type as u8 + 1,
            self.number + 1,
            ((self.price >> 8) + 1) as u8,
            (self.price % 0x100) as u8,
            self.count.map_or(0, |x| x.get()) + 1,
            ((self.set_flag >> 8) + 1) as u8,
            (self.set_flag % 0x100) as u8,
        ]
    }

    pub fn number(&self) -> u8 {
        self.number
    }
    pub fn price(&self) -> u16 {
        self.price
    }
    pub fn count(&self) -> Option<NonZero<u8>> {
        self.count
    }
    pub fn set_flag(&self) -> u16 {
        self.set_flag
    }
}

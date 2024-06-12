use anyhow::Result;

use crate::util::scriptdat::format::codec::text_to_shop_data;

use super::codec::shop_item_data_to_text;

pub fn parse(text: &str) -> (ShopItemData, ShopItemData, ShopItemData) {
    debug_assert_eq!(text.chars().count(), 7 * 3);
    let data = text_to_shop_data(text);
    debug_assert_eq!(data.len(), 7 * 3);
    let mut iter = (0..3).map(|x| x * 7).map(|x| ShopItemData {
        r#type: (data[x] - 1),
        number: data[x + 1] as i8 - 1,
        price: ((data[x + 2] as u16 - 1) << 8) + data[x + 3] as u16,
        count: data[x + 4] - 1,
        flag: ((data[x + 5] as i32 - 1) << 8) + data[x + 6] as i32, // 254 * 256 + 255 is no set flag
    });
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn stringify(items: (ShopItemData, ShopItemData, ShopItemData)) -> Result<String> {
    let data: Vec<_> = [items.0, items.1, items.2]
        .iter()
        .map(|x| {
            Ok([
                x.r#type + 1,
                u8::try_from(x.number + 1)?,
                u8::try_from(x.price >> 8)? + 1,
                u8::try_from(x.price % 256)?,
                x.count + 1,
                u8::try_from(x.flag >> 8)? + 1,
                u8::try_from(x.flag % 256)?,
            ])
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(shop_item_data_to_text(&data))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ShopItemData {
    pub r#type: u8,
    pub number: i8,
    pub price: u16,
    pub count: u8,
    pub flag: i32,
}

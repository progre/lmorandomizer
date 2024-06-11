use crate::util::scriptdat::format::codec::text_to_shop_data;

use super::codec::shop_item_data_to_text;

pub fn parse(text: &str) -> (ShopItemData, ShopItemData, ShopItemData) {
    debug_assert_eq!(text.chars().count(), 7 * 3);
    let data = text_to_shop_data(text);
    debug_assert_eq!(data.len(), 7 * 3);
    let mut iter = (0..3).map(|x| x * 7).map(|x| ShopItemData {
        r#type: (data[x] - 1),
        number: data[x + 1] - 1,
        price: (((data[x + 2] - 1) as u16) << 8) + data[x + 3] as u16,
        count: data[x + 4] - 1,
        flag: (((data[x + 5] - 1) as u16) << 8) + data[x + 6] as u16, // 254 * 256 + 255 is no set flag
    });
    (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

pub fn stringify(items: (ShopItemData, ShopItemData, ShopItemData)) -> String {
    let data = [items.0, items.1, items.2]
        .iter()
        .flat_map(|x| {
            [
                x.r#type + 1,
                x.number + 1,
                (x.price >> 8) as u8 + 1,
                (x.price % 256) as u8,
                x.count + 1,
                (x.flag >> 8) as u8 + 1,
                (x.flag % 256) as u8,
            ]
        })
        .collect::<Vec<_>>();
    shop_item_data_to_text(&data)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ShopItemData {
    pub r#type: u8,
    pub number: u8,
    pub price: u16,
    pub count: u8,
    pub flag: u16,
}

use crate::util::scriptdat::format::codec::text_to_shop_data;

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

#[derive(serde::Serialize)]
pub struct ShopItemData {
    pub r#type: u8,
    pub number: u8,
    pub price: u16,
    pub count: u8,
    pub flag: u16,
}

use anyhow::{bail, Result};

use crate::dataset::{
    spot::{self},
    storage::{self},
    WARE_NO_MISE_COUNT,
};

use super::{
    item::Item,
    object::Shop,
    script::Script,
    shop_items_data::{self, ShopItem},
};

fn to_dataset_shop(
    script_shop_items: &(ShopItem, ShopItem, ShopItem),
) -> (spot::ShopItem, spot::ShopItem, spot::ShopItem) {
    let items = [
        &script_shop_items.0,
        &script_shop_items.1,
        &script_shop_items.2,
    ]
    .map(|y| match y {
        ShopItem::Equipment(script) => spot::ShopItem::Equipment(script.item.content),
        ShopItem::Rom(script) => spot::ShopItem::Rom(script.item.content),
        ShopItem::SubWeapon(script) => spot::ShopItem::SubWeapon(script.item.content),
    });
    (items[0], items[1], items[2])
}

pub fn replace_shops(
    talks: &mut [String],
    script: &Script,
    script_shops: &[Shop],
    dataset_shops: &[storage::Shop],
) -> Result<()> {
    assert_eq!(script_shops.len(), dataset_shops.len() + WARE_NO_MISE_COUNT);
    for dataset_shop in dataset_shops {
        let Some(script_shop) = script_shops
            .iter()
            .find(|script_shop| to_dataset_shop(&script_shop.items) == dataset_shop.spot.items())
        else {
            bail!("shop not found: {:?}", dataset_shop.spot.items())
        };
        let Some(talk) = talks.get_mut(script_shop.talk_number as usize) else {
            bail!("script broken: talk_number={}", script_shop.talk_number)
        };
        let old = shop_items_data::parse(talk)?;
        let mut replaced = [old.0, old.1, old.2]
            .into_iter()
            .enumerate()
            .map(|(j, old_item)| {
                let items = &dataset_shop.items;
                let new_item = [&items.0, &items.1, &items.2][j];
                let is_consumable = new_item.name.is_consumable();
                let new_item = Item::from_dataset(new_item, script)?;
                let price = if is_consumable {
                    new_item.price().unwrap()
                } else {
                    old_item.price()
                };
                Ok(ShopItem::from_item(new_item, price))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter();
        *talk = shop_items_data::stringify((
            replaced.next().unwrap(),
            replaced.next().unwrap(),
            replaced.next().unwrap(),
        ))?;
    }
    Ok(())
}

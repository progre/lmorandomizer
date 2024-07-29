use anyhow::{bail, Result};
use log::warn;
use regex::Regex;

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

fn hide_overflow(kana: &str) -> String {
    let mut count = 0;
    kana.chars()
        .filter_map(|c| {
            match count {
                ..=21 => {
                    if !matches!(c, 'ﾞ' | 'ﾟ') {
                        count += 1;
                    }
                    Some(c)
                }
                // Overflow is represented by whitespace
                22 => {
                    if !matches!(c, 'ﾞ' | 'ﾟ') {
                        Some(' ')
                    } else {
                        Some(c)
                    }
                }
                _ => {
                    if !matches!(c, 'ﾞ' | 'ﾟ') {
                        Some(' ')
                    } else {
                        None
                    }
                }
            }
        })
        .collect()
}

fn to_name_talk_number(item: spot::ShopItem) -> usize {
    match item {
        spot::ShopItem::Rom(rom) => rom as usize,
        spot::ShopItem::Equipment(equipment) => 500 + equipment as usize,
        spot::ShopItem::SubWeapon(sub_weapon) => 645 + sub_weapon as usize,
    }
}

fn replace_shop_item_talk(
    talks: &[String],
    talk_number: usize,
    old: spot::ShopItem,
    new: spot::ShopItem,
) -> Result<String> {
    let old_item_name_talk_number = to_name_talk_number(old);
    let Some(old_item_name) = talks.get(old_item_name_talk_number).cloned() else {
        bail!("script broken: talk_number={}", old_item_name_talk_number)
    };
    let new_item_name_talk_number = to_name_talk_number(new);
    let Some(new_item_name) = talks.get(new_item_name_talk_number).cloned() else {
        bail!("script broken: talk_number={}", new_item_name_talk_number)
    };
    let Some(talk) = talks.get(talk_number) else {
        bail!("script broken: talk_number={}", talk_number)
    };
    const ITEM_NAME_NORMALIZE_MAP: [(&str, &str); 19] = [
        (r"^ﾊﾂﾀﾞﾝﾄｳ", "はつたﾞんとう"),       // 3
        (r"^ﾏｼﾞｮｳﾃﾞﾝｾﾂ", "魔しﾞょうてﾞんせつ"), // 42
        (r"あくま", "ｱｸﾏ"),                 // 47
        (r"ふういん", "ﾌｳ印"),              // 62
        (r"ましﾞゅつ", "魔しﾞゅつ"),          // 66
        (r"^うらない", "ｺﾅﾐのうらない"),    // 72
        (r"^F1SP", "F1ｽﾋﾟﾘｯﾄ"),              // 74
        (r"^ﾊﾞｸﾀﾞﾝ", "はﾞくたﾞん"),             // 4
        (r"^しﾞゅうたﾞん", "たﾞんかﾞん"),       // 12
        (r"^ｶﾌﾞﾄ", "かふﾞと"),                // 32
        (r"^時をとめる", "時の"),           // 43
        (r"^ｽｷｬﾅ", "にせｽｷｬﾅｰ"),            // 45
        // NOTE: English doesn't work because of the complexity of "a" etc.
        //       Moreover, there is an error in the English patch
        //       (there is a mistake where a bomb is mistaken for a flare gun).
        //       I'm not a native speaker and cannot work on this problem.
        //       If anyone cares to help, please contribute.
        (r"MSX 2", "MSX2"),                     // 40
        (r"lamp", "Lamp of Time"),              // 43
        (r"a Scanner", "a Fake Scanner"),       // 45
        (r"Throwing Knives", "Throwing Knife"), // 1
        (r"(?i)Flares", "Flare Gun"),           // 3
        (r"Shield", "Silver Shield"),           // 10
        (r"ammo", "Ammunition"),                // 12
    ];
    let normalized_talk =
        &ITEM_NAME_NORMALIZE_MAP
            .iter()
            .fold(talk.to_owned(), |name, &(from, to)| {
                Regex::new(from)
                    .unwrap()
                    .replace(name.as_ref(), to)
                    .to_string()
            });
    let result = Regex::new(&format!("(?i){old_item_name}"))
        .unwrap()
        .replace(normalized_talk, &new_item_name);
    if &result == normalized_talk {
        warn!(
            "failed to replace shop item name: talk={}, old={}, new={}",
            talk, old_item_name, new_item_name,
        );
    }
    Ok(hide_overflow(&result))
}

fn to_dataset_shop_items_from_shop_items(
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

fn to_dataset_shop_items_from_items(
    items: &(Item, Item, Item),
) -> (spot::ShopItem, spot::ShopItem, spot::ShopItem) {
    let items = [&items.0, &items.1, &items.2].map(|y| match y {
        Item::Equipment(dataset) => spot::ShopItem::Equipment(dataset.content),
        Item::Rom(dataset) => spot::ShopItem::Rom(dataset.content),
        Item::SubWeapon(dataset) => spot::ShopItem::SubWeapon(dataset.content),
        Item::Seal(_) | Item::MainWeapon(_) => unreachable!(),
    });
    (items[0], items[1], items[2])
}

fn is_consumable(item: &Item) -> bool {
    match item {
        Item::SubWeapon(sub_weapon) => sub_weapon.amount > 0,
        Item::Rom(_) | Item::Equipment(_) | Item::Seal(_) | Item::MainWeapon(_) => false,
    }
}

fn replace_items(
    old: (ShopItem, ShopItem, ShopItem),
    new: (Item, Item, Item),
) -> (ShopItem, ShopItem, ShopItem) {
    let mut items = [(&old.0, new.0), (&old.1, new.1), (&old.2, new.2)]
        .into_iter()
        .map(|(old_item, new_item)| {
            let price = if is_consumable(&new_item) {
                new_item.price().unwrap()
            } else {
                old_item.price()
            };
            ShopItem::from_item(new_item, price)
        });
    (
        items.next().unwrap(),
        items.next().unwrap(),
        items.next().unwrap(),
    )
}

fn create_shop_item_talks(
    talks: &[String],
    base_talk_number: u16,
    old: (spot::ShopItem, spot::ShopItem, spot::ShopItem),
    new: (spot::ShopItem, spot::ShopItem, spot::ShopItem),
) -> Result<Vec<(usize, String)>> {
    [(1, old.0, new.0), (2, old.1, new.1), (3, old.2, new.2)]
        .into_iter()
        .filter(|(_, old, new)| old != new)
        .map(|(idx, old, new)| {
            let talk_number = base_talk_number as usize + idx;
            let new_talk = replace_shop_item_talk(talks, talk_number, old, new)?;
            Ok((talk_number, new_talk))
        })
        .collect()
}

pub fn replace_shops(
    talks: &mut [String],
    script: &Script,
    script_shops: &[Shop],
    dataset_shops: &[storage::Shop],
) -> Result<()> {
    assert_eq!(script_shops.len(), dataset_shops.len() + WARE_NO_MISE_COUNT);
    for dataset_shop in dataset_shops {
        let new_items = (
            Item::from_dataset(&dataset_shop.items.0, script)?,
            Item::from_dataset(&dataset_shop.items.1, script)?,
            Item::from_dataset(&dataset_shop.items.2, script)?,
        );
        let new_dataset_shop_items = to_dataset_shop_items_from_items(&new_items);

        let Some(script_shop) = script_shops.iter().find(|script_shop| {
            to_dataset_shop_items_from_shop_items(&script_shop.items) == dataset_shop.spot.items()
        }) else {
            bail!("shop not found: {:?}", dataset_shop.spot.items())
        };

        let old = {
            let Some(talk) = talks.get(script_shop.talk_number as usize) else {
                bail!("script broken: talk_number={}", script_shop.talk_number)
            };
            shop_items_data::parse(talk)?
        };
        let new = new_items;
        let new_shop_talk = shop_items_data::stringify(replace_items(old, new))?;

        let old = to_dataset_shop_items_from_shop_items(&script_shop.items);
        let new = new_dataset_shop_items;
        let new_shop_item_talks = create_shop_item_talks(talks, script_shop.talk_number, old, new)?;

        let Some(talk) = talks.get_mut(script_shop.talk_number as usize) else {
            bail!("script broken: talk_number={}", script_shop.talk_number)
        };
        *talk = new_shop_talk;
        for (talk_number, new_talk) in new_shop_item_talks {
            let Some(talk) = talks.get_mut(talk_number) else {
                bail!("script broken: talk_number={}", talk_number)
            };
            *talk = new_talk;
        }
    }
    Ok(())
}

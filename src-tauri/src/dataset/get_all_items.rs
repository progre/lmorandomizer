use std::num::NonZero;

use anyhow::{bail, Result};

use crate::{
    dataset::{
        item::Item,
        supplements::{
            Supplements, NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT,
            NIGHT_SURFACE_SUB_WEAPON_COUNT, TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT,
            WARE_NO_MISE_COUNT,
        },
    },
    script::data::{object::ChestContent, script::Script},
};

pub struct AllItems {
    pub main_weapons: Vec<Item>,
    pub sub_weapons: Vec<Item>,
    pub chests: Vec<Item>,
    pub seals: Vec<Item>,
    pub shops: Vec<(Item, Item, Item)>,
}

pub fn get_all_items(script: &Script, supplements: &Supplements) -> Result<AllItems> {
    Ok(AllItems {
        main_weapons: main_weapons(script, supplements)?,
        sub_weapons: sub_weapons(script, supplements)?,
        chests: chests(script, supplements)?,
        seals: seals(script, supplements)?,
        shops: shops(script, supplements)?,
    })
}

fn main_weapons(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let main_weapons_data_list = script.main_weapons()?;
    debug_assert_eq!(main_weapons_data_list.len(), supplements.main_weapons.len());
    supplements
        .main_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &main_weapons_data_list[i];
            Ok(Item::main_weapon(
                supplement.name.clone(),
                data.content,
                data.flag,
            ))
        })
        .collect()
}

fn sub_weapons(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let sub_weapons_data_list = script.sub_weapons()?;
    debug_assert_eq!(
        sub_weapons_data_list.len(),
        supplements.sub_weapons.len() + NIGHT_SURFACE_SUB_WEAPON_COUNT
    );
    supplements
        .sub_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &sub_weapons_data_list[i];
            Ok(NonZero::new(u8::try_from(data.count)?).map_or_else(
                || Item::sub_weapon_body(supplement.name.clone(), data.content, data.flag),
                |count| {
                    Item::sub_weapon_ammo(supplement.name.clone(), data.content, count, data.flag)
                },
            ))
        })
        .collect()
}

fn chests(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let chests_data_list = script.chests()?;
    debug_assert_eq!(
        chests_data_list.len(),
        supplements.chests.len() + NIGHT_SURFACE_CHEST_COUNT
    );
    supplements
        .chests
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &chests_data_list[i];
            Ok(match data.content {
                None => bail!("invalid chest item number: None"),
                Some(ChestContent::Equipment(equipment)) => Item::equipment(
                    supplement.name.clone(),
                    equipment,
                    u16::try_from(data.flag)?,
                ),
                Some(ChestContent::Rom(rom)) => {
                    Item::rom(supplement.name.clone(), rom, u16::try_from(data.flag)?)
                }
            })
        })
        .collect()
}

fn seals(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let seals_data_list = script.seals()?;
    debug_assert_eq!(
        seals_data_list.len(),
        supplements.seals.len() + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT + NIGHT_SURFACE_SEAL_COUNT
    );
    Ok(supplements
        .seals
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &seals_data_list[i];
            Item::seal(supplement.name.clone(), data.content, data.flag)
        })
        .collect())
}

fn shops(script: &Script, supplements: &Supplements) -> Result<Vec<(Item, Item, Item)>> {
    let shops_data_list = script.shops()?;
    debug_assert_eq!(
        shops_data_list.len(),
        supplements.shops.len() + WARE_NO_MISE_COUNT
    );
    supplements
        .shops
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let shop = &shops_data_list[i];
            Ok((
                Item::from_shop_item(shop.items.0.clone(), supplement.names.0.clone())?,
                Item::from_shop_item(shop.items.1.clone(), supplement.names.1.clone())?,
                Item::from_shop_item(shop.items.2.clone(), supplement.names.2.clone())?,
            ))
        })
        .collect::<Result<_>>()
}

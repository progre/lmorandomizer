use std::num::NonZero;

use anyhow::{anyhow, Result};
use num_traits::FromPrimitive;

use crate::{
    dataset::{
        item::Item,
        supplements::{
            Supplements, NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT,
            NIGHT_SURFACE_SUB_WEAPON_COUNT, TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT,
            WARE_NO_MISE_COUNT,
        },
    },
    script::data::{
        items::{Equipment, MainWeapon, Rom, SubWeapon},
        script::Script,
        shop_items_data::ShopItem,
    },
};

use super::supplements::StrategyFlag;

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
                MainWeapon::from_u8(data.main_weapon_number).ok_or_else(|| {
                    anyhow!("Invalid main weapon number: {}", data.main_weapon_number)
                })?,
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
            Ok(Item::sub_weapon(
                supplement.name.clone(),
                SubWeapon::from_u8(data.sub_weapon_number).ok_or_else(|| {
                    anyhow!("Invalid sub weapon number: {}", data.sub_weapon_number)
                })?,
                NonZero::new(u8::try_from(data.count)?),
                data.flag,
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
            Ok(if data.chest_item_number < 100 {
                Item::equipment(
                    supplement.name.clone(),
                    Equipment::from_i16(data.chest_item_number).ok_or_else(|| {
                        anyhow!("Invalid equipment number: {}", data.chest_item_number)
                    })?,
                    u16::try_from(data.flag)?,
                )
            } else {
                Item::rom(
                    supplement.name.clone(),
                    Rom(u8::try_from(data.chest_item_number - 100)?),
                    u16::try_from(data.flag)?,
                )
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
            Item::seal(supplement.name.clone(), data.seal_number, data.flag)
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
                create_item_from_shop(supplement.names.0.clone(), &shop.items.0)?,
                create_item_from_shop(supplement.names.1.clone(), &shop.items.1)?,
                create_item_from_shop(supplement.names.2.clone(), &shop.items.2)?,
            ))
        })
        .collect::<Result<_>>()
}

fn create_item_from_shop(name: StrategyFlag, data: &ShopItem) -> Result<Item> {
    Ok(match data {
        ShopItem::SubWeapon(data) => {
            Item::sub_weapon(name, data.sub_weapon, data.count, data.set_flag)
        }
        ShopItem::Equipment(data) => Item::equipment(name, data.equipment, data.set_flag),
        ShopItem::Rom(data) => Item::rom(name, data.rom, data.set_flag),
    })
}

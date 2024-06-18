use anyhow::Result;

use crate::{
    dataset::{
        item::Item,
        supplements::{
            Supplements, NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT,
            NIGHT_SURFACE_SUB_WEAPON_COUNT, TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT,
            WARE_NO_MISE_COUNT,
        },
    },
    script::{data::script::Script, format::shop_items_data::ShopItemData},
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
    Ok(supplements
        .main_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &main_weapons_data_list[i];
            Item::new(
                supplement.name.clone(),
                "mainWeapon".to_owned(),
                data.main_weapon_number as i8,
                1, // Count of main weapon is always 1.
                data.flag,
            )
        })
        .collect())
}

fn sub_weapons(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let sub_weapons_data_list = script.sub_weapons()?;
    debug_assert_eq!(
        sub_weapons_data_list.len(),
        supplements.sub_weapons.len() + NIGHT_SURFACE_SUB_WEAPON_COUNT
    );
    Ok(supplements
        .sub_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &sub_weapons_data_list[i];
            Item::new(
                supplement.name.clone(),
                "subWeapon".to_owned(),
                data.sub_weapon_number as i8,
                data.count as u8,
                data.flag,
            )
        })
        .collect())
}

fn chests(script: &Script, supplements: &Supplements) -> Result<Vec<Item>> {
    let chests_data_list = script.chests()?;
    debug_assert_eq!(
        chests_data_list.len(),
        supplements.chests.len() + NIGHT_SURFACE_CHEST_COUNT
    );
    Ok(supplements
        .chests
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let data = &chests_data_list[i];
            Item::new(
                supplement.name.clone(),
                if data.chest_item_number < 100 {
                    "equipment".to_owned()
                } else {
                    "rom".to_owned()
                },
                if data.chest_item_number < 100 {
                    data.chest_item_number as i8
                } else {
                    (data.chest_item_number - 100) as i8
                },
                1, // Count of chest item is always 1.
                data.flag,
            )
        })
        .collect())
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
            Item::new(
                supplement.name.clone(),
                "seal".to_owned(),
                data.seal_number as i8,
                1, // Count of seal is always 1.
                data.flag,
            )
        })
        .collect())
}

fn shops(script: &Script, supplements: &Supplements) -> Result<Vec<(Item, Item, Item)>> {
    let shops_data_list = script.shops()?;
    debug_assert_eq!(
        shops_data_list.len(),
        supplements.shops.len() + WARE_NO_MISE_COUNT
    );
    Ok(supplements
        .shops
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            let shop = &shops_data_list[i];
            let names: Vec<_> = supplement.names.split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
            (
                create_item_from_shop(names[0].to_owned(), &shop.items.0),
                create_item_from_shop(names[1].to_owned(), &shop.items.1),
                create_item_from_shop(names[2].to_owned(), &shop.items.2),
            )
        })
        .collect())
}

fn create_item_from_shop(name: String, data: &ShopItemData) -> Item {
    Item::new(
        name,
        if data.r#type == 0 {
            "subWeapon".to_owned()
        } else if data.r#type == 1 {
            "equipment".to_owned()
        } else if data.r#type == 2 {
            "rom".to_owned()
        } else {
            panic!("invalid value: {}", data.r#type)
        },
        data.number,
        data.count,
        data.flag,
    )
}

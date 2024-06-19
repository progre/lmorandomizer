use std::collections::HashSet;

use anyhow::Result;
use log::warn;

use crate::{
    dataset::{
        item::Item,
        spot::Spot,
        storage::Storage,
        storage::{ItemSpot, Shop},
        supplements::NIGHT_SURFACE_CHEST_COUNT,
        supplements::{self, Requirement, SupplementFiles, Supplements},
    },
    script::data::script::Script,
};

use super::get_all_items::{get_all_items, AllItems};

pub fn create_source(script: &Script, supplement_files: &SupplementFiles) -> Result<Storage> {
    let supplements = Supplements::new(supplement_files);
    let all_items = get_all_items(script, &supplements)?;
    let enumerate_items: Vec<_> = all_items
        .main_weapons
        .iter()
        .chain(all_items.sub_weapons.iter())
        .chain(all_items.chests.iter())
        .chain(all_items.seals.iter())
        .chain(all_items.shops.iter().flat_map(|(a, b, c)| [a, b, c]))
        .cloned()
        .collect();
    ware_missing_requirements(&supplements, &enumerate_items);
    let chest_data_list = script.chests()?;
    debug_assert_eq!(
        chest_data_list.len(),
        all_items.chests.len() + NIGHT_SURFACE_CHEST_COUNT
    );
    let shops = script.shops()?;
    let AllItems {
        main_weapons: all_items_main_weapons,
        sub_weapons: all_items_sub_weapons,
        chests: all_items_chests,
        seals: all_items_seals,
        shops: all_items_shops,
    } = all_items;
    Ok(Storage::new(
        all_items_main_weapons
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let supplement = &supplements.main_weapons[i];
                let spot = Spot::new(parse_requirements(
                    &supplement.requirements,
                    &enumerate_items,
                )?);
                Ok(ItemSpot { spot, item })
            })
            .collect::<Result<_>>()?,
        all_items_sub_weapons
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let supplement = &supplements.sub_weapons[i];
                let spot = Spot::new(parse_requirements(
                    &supplement.requirements,
                    &enumerate_items,
                )?);
                Ok(ItemSpot { spot, item })
            })
            .collect::<Result<_>>()?,
        all_items_chests
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let supplement = &supplements.chests[i];
                let spot = Spot::new(parse_requirements(
                    &supplement.requirements,
                    &enumerate_items,
                )?);
                Ok(ItemSpot { spot, item })
            })
            .collect::<Result<_>>()?,
        all_items_seals
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let supplement = &supplements.seals[i];
                let spot = Spot::new(parse_requirements(
                    &supplement.requirements,
                    &enumerate_items,
                )?);
                Ok(ItemSpot { spot, item })
            })
            .collect::<Result<_>>()?,
        all_items_shops
            .into_iter()
            .enumerate()
            .map(|(i, items)| {
                let supplement = &supplements.shops[i];
                let spot = Spot::new(parse_requirements(
                    &supplement.requirements,
                    &enumerate_items,
                )?);
                Ok(Shop {
                    talk_number: u16::try_from(shops[i].talk_number)?,
                    spot,
                    items,
                })
            })
            .collect::<Result<_>>()?,
    ))
}

fn parse_requirements(
    requirements: &Option<Vec<Vec<Requirement>>>,
    all_items: &[Item],
) -> Result<Option<Vec<Vec<Item>>>> {
    let Some(requirements) = requirements else {
        return Ok(None);
    };
    Ok(Some(
        requirements
            .iter()
            .map(|y| {
                y.iter()
                    .map(|z| {
                        if let Some(&v) = all_items.iter().find(|w| w.name == z.0).as_ref() {
                            Ok(v.clone())
                        } else if z.0.starts_with("sacredOrb:") {
                            Ok(Item::new(
                                "sacredOrb".to_owned(),
                                "equipment".to_owned(),
                                -1,
                                z.0.split(':').nth(1).unwrap().parse()?,
                                -1,
                            ))
                        } else {
                            panic!("Unknown item name: {:?}", z);
                        }
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?,
    ))
}

fn ware_missing_requirements(supplements: &Supplements, all_items: &[Item]) {
    let mut set = HashSet::new();
    add_spot_supplement_to(&mut set, &supplements.main_weapons);
    add_spot_supplement_to(&mut set, &supplements.sub_weapons);
    add_spot_supplement_to(&mut set, &supplements.chests);
    add_spot_supplement_to(&mut set, &supplements.seals);
    add_shop_supplement_to(&mut set, &supplements.shops);
    if cfg!(debug_assertions) {
        let mut vec: Vec<_> = set
            .iter()
            .filter(|&x| all_items.iter().all(|y| &y.name != x))
            .filter(|x| !x.starts_with("sacredOrb:"))
            .collect();
        vec.sort();
        for x in vec {
            warn!("Missing item: {:?}", x);
        }
    }
}

fn add_spot_supplement_to(set: &mut HashSet<String>, supplements: &[supplements::Spot]) {
    supplements
        .iter()
        .map(|x| &x.requirements)
        .for_each(|item| {
            item.as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|group| {
                    group.iter().for_each(|x| {
                        set.insert(x.0.clone());
                    });
                });
        });
}

fn add_shop_supplement_to(set: &mut HashSet<String>, supplements: &[supplements::Shop]) {
    supplements
        .iter()
        .map(|x| &x.requirements)
        .for_each(|item| {
            item.as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .for_each(|group| {
                    group.iter().for_each(|x| {
                        set.insert(x.0.clone());
                    });
                });
        });
}

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
        supplements::{self, SupplementFiles, Supplements},
    },
    script::data::script::Script,
};

use super::{
    get_all_items::{get_all_items, AllItems},
    spot::AnyOfAllRequirements,
};

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
    let AllItems {
        main_weapons: all_items_main_weapons,
        sub_weapons: all_items_sub_weapons,
        chests: all_items_chests,
        seals: all_items_seals,
        shops: all_items_shops,
    } = all_items;
    let Supplements {
        main_weapons: supplements_main_weapons,
        sub_weapons: supplements_sub_weapons,
        chests: supplements_chests,
        seals: supplements_seals,
        shops: supplements_shops,
    } = supplements;
    Ok(Storage::new(
        all_items_main_weapons
            .into_iter()
            .zip(supplements_main_weapons)
            .map(|(item, sup_spot)| {
                let spot = Spot::new(sup_spot.requirements);
                ItemSpot { spot, item }
            })
            .collect(),
        all_items_sub_weapons
            .into_iter()
            .zip(supplements_sub_weapons)
            .map(|(item, sup_spot)| {
                let spot = Spot::new(sup_spot.requirements);
                ItemSpot { spot, item }
            })
            .collect(),
        all_items_chests
            .into_iter()
            .zip(supplements_chests)
            .map(|(item, sup_spot)| {
                let spot = Spot::new(sup_spot.requirements);
                ItemSpot { spot, item }
            })
            .collect(),
        all_items_seals
            .into_iter()
            .zip(supplements_seals)
            .map(|(item, sup_spot)| {
                let spot = Spot::new(sup_spot.requirements);
                ItemSpot { spot, item }
            })
            .collect(),
        all_items_shops
            .into_iter()
            .zip(supplements_shops)
            .map(|(items, sup_shop)| Shop {
                spot: Spot::new(sup_shop.requirements),
                items,
            })
            .collect(),
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
            .filter(|&x| all_items.iter().all(|y| y.name() != x))
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
                .unwrap_or(&AnyOfAllRequirements(Vec::new()))
                .0
                .iter()
                .for_each(|group| {
                    group.0.iter().for_each(|x| {
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
                .unwrap_or(&AnyOfAllRequirements(Vec::new()))
                .0
                .iter()
                .for_each(|group| {
                    group.0.iter().for_each(|x| {
                        set.insert(x.0.clone());
                    });
                });
        });
}

use std::collections::HashSet;

use log::warn;

use crate::{
    dataset::{
        item::Item,
        spot::Spot,
        storage::{ItemSpot, Shop, Storage},
        supplements::{
            SupplementFiles, Supplements, NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT,
            NIGHT_SURFACE_SUB_WEAPON_COUNT, TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT,
            WARE_NO_MISE_COUNT,
        },
    },
    script::data::script::Script,
};

use super::{
    get_all_items::{get_all_items, AllItems},
    supplements::{AnyOfAllRequirements, RequirementFlag},
};

pub fn create_source(supplement_files: &SupplementFiles, script: &Script) -> Storage {
    let supplements = Supplements::new(supplement_files);

    debug_assert_eq!(
        script.main_weapons().unwrap().len(),
        supplements.main_weapons.len()
    );
    debug_assert_eq!(
        script.sub_weapons().unwrap().len(),
        supplements.sub_weapons.len() + NIGHT_SURFACE_SUB_WEAPON_COUNT
    );
    debug_assert_eq!(
        script.chests().unwrap().len(),
        supplements.chests.len() + NIGHT_SURFACE_CHEST_COUNT
    );
    debug_assert_eq!(
        script.seals().unwrap().len(),
        supplements.seals.len() + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT + NIGHT_SURFACE_SEAL_COUNT
    );
    debug_assert_eq!(
        script.shops().unwrap().len(),
        supplements.shops.len() + WARE_NO_MISE_COUNT
    );

    let all_items = get_all_items(&supplements);
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
    Storage::new(
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
    )
}

fn ware_missing_requirements(supplements: &Supplements, all_items: &[Item]) {
    let mut set = HashSet::new();
    let main_weapon_requirements = supplements.main_weapons.iter().map(|x| &x.requirements);
    append(&mut set, main_weapon_requirements);
    let sub_weapon_requirements = supplements.sub_weapons.iter().map(|x| &x.requirements);
    append(&mut set, sub_weapon_requirements);
    append(&mut set, supplements.chests.iter().map(|x| &x.requirements));
    append(&mut set, supplements.seals.iter().map(|x| &x.requirements));
    append(&mut set, supplements.shops.iter().map(|x| &x.requirements));
    if cfg!(debug_assertions) {
        let mut vec: Vec<_> = set
            .iter()
            .filter(|&x| all_items.iter().all(|y| y.name() != x))
            .filter(|x| !x.is_sacred_orb())
            .collect();
        vec.sort();
        for x in vec {
            warn!("Missing item: {:?}", x);
        }
    }
}

fn append<'a>(
    set: &mut HashSet<RequirementFlag>,
    any_of_requirements: impl Iterator<Item = &'a Option<AnyOfAllRequirements>>,
) {
    any_of_requirements
        .filter_map(|item| item.as_ref().map(|x| &x.0))
        .flatten()
        .flat_map(|group| &group.0)
        .for_each(|x| {
            set.insert(x.clone());
        });
}

use std::collections::HashMap;

use anyhow::Result;
use log::trace;

use crate::dataset::{
    item::Item,
    spot::{AllRequirements, Spot},
    storage::{ItemSpot, Storage},
};

use super::{
    assertions::{assert_chests, ware_missing_requirements},
    game_structure::GameStructureFiles,
    item::StrategyFlag,
    merge_events::{merge_events, Event},
    spot::{self, AnyOfAllRequirements, FieldId, RequirementFlag, SpotName},
    storage,
};

fn to_any_of_all_requirements(requirements: Vec<String>) -> Option<AnyOfAllRequirements> {
    if requirements.is_empty() {
        None
    } else {
        let requirements = requirements
            .into_iter()
            .map(|y| {
                y.split(',')
                    .map(|z| RequirementFlag::new(z.trim().to_owned()))
                    .collect()
            })
            .map(AllRequirements)
            .collect();
        Some(AnyOfAllRequirements(requirements))
    }
}

fn parse_item_spot_requirements(
    create_spot: impl Fn(FieldId, usize, SpotName, Option<AnyOfAllRequirements>) -> Spot,
    create_item: impl Fn(usize, StrategyFlag) -> Item,
    items: Vec<(FieldId, HashMap<String, Vec<String>>)>,
) -> Vec<ItemSpot> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, spot))| {
            let (name, requirements) = spot.into_iter().next().unwrap();
            let name = SpotName::new(name);
            let requirements = to_any_of_all_requirements(requirements);
            ItemSpot {
                spot: create_spot(field_id, src_idx, name.clone(), requirements),
                item: create_item(src_idx, name.into()),
            }
        })
        .collect()
}

fn parse_shop_requirements(
    items: Vec<(FieldId, HashMap<String, Vec<String>>)>,
) -> Vec<storage::Shop> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, shop))| {
            let (names, requirements) = shop.into_iter().next().unwrap();
            let name = SpotName::new(names);
            let requirements = to_any_of_all_requirements(requirements);
            let shop_spot = spot::ShopSpot::new(field_id, src_idx, name.clone(), requirements);
            let flags = shop_spot.to_strategy_flags();
            storage::Shop {
                spot: Spot::shop(shop_spot),
                items: (
                    Item::shop_item(src_idx, 0, flags.0),
                    Item::shop_item(src_idx, 1, flags.1),
                    Item::shop_item(src_idx, 2, flags.2),
                ),
            }
        })
        .collect()
}

fn parse_requirements_of_events(items: Vec<HashMap<String, Vec<String>>>) -> Vec<Event> {
    let mut current: Vec<Event> = items
        .into_iter()
        .map(|x| {
            let (name, requirements) = x.into_iter().next().unwrap();
            Event {
                name,
                requirements: to_any_of_all_requirements(requirements).unwrap(),
            }
        })
        .collect();
    for _ in 0..100 {
        let events: Vec<_> = current
            .iter()
            .filter(|x| {
                x.requirements
                    .0
                    .iter()
                    .all(|y| y.0.iter().all(|z| !z.get().starts_with("event:")))
            })
            .cloned()
            .collect();
        if events.len() == current.len() {
            return current;
        }
        current = current
            .into_iter()
            .map(|x| Event {
                name: x.name,
                requirements: merge_events(x.requirements, &events),
            })
            .collect();
    }
    unreachable!();
}

pub fn create_source(game_structure_files: GameStructureFiles) -> Result<Storage> {
    let start = std::time::Instant::now();

    let mut main_weapons = Vec::new();
    let mut sub_weapons = Vec::new();
    let mut chests = Vec::new();
    let mut seals = Vec::new();
    let mut shops = Vec::new();
    for (field_id, field_data) in game_structure_files.fields {
        for item in field_data.main_weapons {
            main_weapons.push((field_id, item));
        }
        for item in field_data.sub_weapons {
            sub_weapons.push((field_id, item));
        }
        for item in field_data.chests {
            chests.push((field_id, item));
        }
        for item in field_data.seals {
            seals.push((field_id, item));
        }
        for item in field_data.shops {
            shops.push((field_id, item));
        }
    }
    let events = parse_requirements_of_events(game_structure_files.events.0);

    let mut main_weapons =
        parse_item_spot_requirements(Spot::main_weapon, Item::main_weapon, main_weapons);
    main_weapons.iter_mut().for_each(|item_spot| {
        if let Some(requirements) = item_spot.spot.requirements_mut().take() {
            *item_spot.spot.requirements_mut() = Some(merge_events(requirements, &events));
        }
    });

    let mut sub_weapons =
        parse_item_spot_requirements(Spot::sub_weapon, Item::sub_weapon, sub_weapons);
    sub_weapons.iter_mut().for_each(|item_spot| {
        if let Some(requirements) = item_spot.spot.requirements_mut().take() {
            *item_spot.spot.requirements_mut() = Some(merge_events(requirements, &events));
        }
    });

    let mut chests = parse_item_spot_requirements(Spot::chest, Item::chest_item, chests);
    chests.iter_mut().for_each(|item_spot| {
        if let Some(requirements) = item_spot.spot.requirements_mut().take() {
            *item_spot.spot.requirements_mut() = Some(merge_events(requirements, &events));
        }
    });

    let mut seals = parse_item_spot_requirements(Spot::seal, Item::seal, seals);
    seals.iter_mut().for_each(|item_spot| {
        if let Some(requirements) = item_spot.spot.requirements_mut().take() {
            *item_spot.spot.requirements_mut() = Some(merge_events(requirements, &events));
        }
    });

    let mut shops = parse_shop_requirements(shops);
    shops.iter_mut().for_each(|shop| {
        if let Some(requirements) = shop.spot.requirements_mut().take() {
            *shop.spot.requirements_mut() = Some(merge_events(requirements, &events));
        }
    });
    trace!("create_source parse: {:?}", start.elapsed());

    if cfg!(debug_assertions) {
        let start = std::time::Instant::now();
        assert_chests(&chests);
        trace!("assert_chests: {:?}", start.elapsed());
    }
    let start = std::time::Instant::now();
    let storage = Storage::new(main_weapons, sub_weapons, chests, seals, shops);
    trace!("Storage::new: {:?}", start.elapsed());
    if cfg!(debug_assertions) {
        let start = std::time::Instant::now();
        ware_missing_requirements(&storage);
        trace!("ware_missing_requirements: {:?}", start.elapsed());
    }
    Ok(storage)
}

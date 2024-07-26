use std::collections::HashMap;

use anyhow::Result;
use log::trace;

use crate::{
    dataset::{
        game_structure::GameStructureFiles,
        item::{Item, StrategyFlag},
        spot::{
            AllRequirements, AnyOfAllRequirements, ChestSpot, FieldId, MainWeaponSpot,
            RequirementFlag, RomSpot, SealSpot, ShopSpot, SpotName, SubWeaponSpot,
        },
        storage::{Chest, Event, MainWeapon, Rom, Seal, Shop, Storage, SubWeapon},
    },
    script::data::items,
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

fn parse_game_structure<T>(
    list: Vec<(FieldId, HashMap<String, Vec<String>>)>,
    create: impl Fn(FieldId, usize, SpotName, Option<AnyOfAllRequirements>) -> T,
) -> Vec<T> {
    list.into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, spot))| {
            let (name, requirements) = spot.into_iter().next().unwrap();
            let name = SpotName::new(name);
            let requirements = to_any_of_all_requirements(requirements);
            create(field_id, src_idx, name, requirements)
        })
        .collect()
}

fn parse_shop_requirements(items: Vec<(FieldId, HashMap<String, Vec<String>>)>) -> Vec<Shop> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, shop))| {
            let (names, requirements) = shop.into_iter().next().unwrap();
            let name = SpotName::new(names);
            let requirements = to_any_of_all_requirements(requirements);
            let spot = ShopSpot::new(field_id, src_idx, name.clone(), requirements);
            let flags = spot.to_strategy_flags();
            Shop {
                spot,
                items: (
                    Item::shop_item(src_idx, 0, flags.0),
                    Item::shop_item(src_idx, 1, flags.1),
                    Item::shop_item(src_idx, 2, flags.2),
                ),
            }
        })
        .collect()
}

fn parse_event_requirements(items: Vec<HashMap<String, Vec<String>>>) -> Vec<Event> {
    items
        .into_iter()
        .map(|x| {
            let (name, requirements) = x.into_iter().next().unwrap();
            Event {
                name: StrategyFlag(name),
                requirements: to_any_of_all_requirements(requirements).unwrap(),
            }
        })
        .collect()
}

pub fn create_source(game_structure_files: GameStructureFiles) -> Result<Storage> {
    let start = std::time::Instant::now();

    let mut main_weapons = Vec::new();
    let mut sub_weapons = Vec::new();
    let mut chests = Vec::new();
    let mut seals = Vec::new();
    let mut shops = Vec::new();
    let mut roms = Vec::new();
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
        for (key, value) in field_data.roms {
            let name = StrategyFlag::new(key);
            let rom = items::Rom::try_from_camel_case(name.get()).unwrap();
            roms.push(Rom {
                spot: RomSpot::new(
                    field_id,
                    SpotName::new(name.get().to_owned()),
                    to_any_of_all_requirements(value),
                ),
                item: Item::rom(name, rom),
            });
        }
    }

    let main_weapons =
        parse_game_structure(main_weapons, |field_id, src_idx, name, requirements| {
            MainWeapon {
                spot: MainWeaponSpot::new(field_id, src_idx, name.clone(), requirements),
                item: Item::main_weapon(src_idx, name.into()),
            }
        });
    let sub_weapons = parse_game_structure(sub_weapons, |field_id, src_idx, name, requirements| {
        SubWeapon {
            spot: SubWeaponSpot::new(field_id, src_idx, name.clone(), requirements),
            item: Item::sub_weapon(src_idx, name.into()),
        }
    });
    let chests = parse_game_structure(chests, |field_id, src_idx, name, requirements| Chest {
        spot: ChestSpot::new(field_id, src_idx, name.clone(), requirements),
        item: Item::chest_item(src_idx, name.into()),
    });
    let seals = parse_game_structure(seals, |field_id, src_idx, name, requirements| Seal {
        spot: SealSpot::new(field_id, src_idx, name.clone(), requirements),
        item: Item::seal(src_idx, name.into()),
    });
    let shops = parse_shop_requirements(shops);
    let events = parse_event_requirements(game_structure_files.events.0);
    trace!("create_source parse: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let storage = Storage::new(
        main_weapons,
        sub_weapons,
        chests,
        seals,
        shops,
        roms,
        events,
    )?;
    trace!("Storage::new: {:?}", start.elapsed());

    Ok(storage)
}

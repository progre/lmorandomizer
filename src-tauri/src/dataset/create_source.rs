use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use anyhow::Result;
use log::trace;
use vec1::Vec1;

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

fn to_any_of_all_requirements(requirements: Vec<String>) -> Result<Option<AnyOfAllRequirements>> {
    if requirements.is_empty() {
        return Ok(None);
    }
    let requirements = requirements
        .into_iter()
        .map(|y| {
            y.split(',')
                .map(|z| RequirementFlag::new(z.trim().to_owned()))
                .collect::<Vec<_>>()
        })
        .map(|x| Ok(AllRequirements(x.try_into()?)))
        .collect::<Result<Vec<_>>>()?
        .try_into()?;
    Ok(Some(AnyOfAllRequirements(requirements)))
}

fn parse_game_structure<T>(
    list: Vec<(FieldId, HashMap<String, Vec<String>>)>,
    create: impl Fn(FieldId, usize, SpotName, Option<AnyOfAllRequirements>) -> T,
) -> Result<Vec<T>> {
    list.into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, spot))| {
            let (name, requirements) = spot.into_iter().next().unwrap();
            let name = SpotName::new(name);
            let requirements = to_any_of_all_requirements(requirements)?;
            Ok(create(field_id, src_idx, name, requirements))
        })
        .collect::<Result<_>>()
}

fn parse_shop_requirements(
    items: Vec<(FieldId, HashMap<String, Vec<String>>)>,
) -> Result<Vec<Shop>> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, (field_id, shop))| {
            let (names, requirements) = shop.into_iter().next().unwrap();
            let name = SpotName::new(names);
            let requirements = to_any_of_all_requirements(requirements)?;
            let spot = ShopSpot::new(field_id, src_idx, name.clone(), requirements);
            let flags = spot.to_strategy_flags();
            Ok(Shop {
                spot,
                items: (
                    Item::shop_item(src_idx, 0, flags.0),
                    Item::shop_item(src_idx, 1, flags.1),
                    Item::shop_item(src_idx, 2, flags.2),
                ),
            })
        })
        .collect()
}

fn parse_event_requirements(items: Vec<HashMap<String, Vec<String>>>) -> Result<Vec<Event>> {
    items
        .into_iter()
        .map(|x| {
            let (name, requirements) = x.into_iter().next().unwrap();
            Ok(Event {
                name: StrategyFlag(name),
                requirements: to_any_of_all_requirements(requirements)?.unwrap(),
            })
        })
        .collect()
}

fn to_pascal_case(camel_case: &str) -> String {
    camel_case[0..1]
        .to_uppercase()
        .chars()
        .chain(camel_case[1..].chars())
        .collect()
}

pub fn create_source(game_structure_files: GameStructureFiles) -> Result<Storage> {
    let start = std::time::Instant::now();

    let mut main_weapons = Vec::new();
    let mut sub_weapons = Vec::new();
    let mut chests = Vec::new();
    let mut seals = BTreeMap::new();
    let mut shops = Vec::new();
    let mut roms = BTreeMap::new();
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
        for (key, value) in field_data.seals {
            let any_of_all_requirements = to_any_of_all_requirements(value)?;
            let seal = items::Seal::from_str(&to_pascal_case(&key.replace("Seal", ""))).unwrap();
            let spot = SealSpot::new(
                field_id,
                seal,
                SpotName::new(key.clone()),
                any_of_all_requirements,
            );
            let item = Item::seal(StrategyFlag::new(key), seal);
            seals.insert(seal, Seal { spot, item });
        }
        for item in field_data.shops {
            shops.push((field_id, item));
        }
        for (key, value) in field_data.roms {
            let any_of_all_requirements = to_any_of_all_requirements(value)?
                .map(|mut any_of_all_requirements| {
                    for all_requirements in &mut any_of_all_requirements.0 {
                        let hand_scanner = RequirementFlag::new("handScanner".into());
                        all_requirements.0.push(hand_scanner);
                    }
                    any_of_all_requirements
                })
                .unwrap_or_else(|| {
                    let hand_scanner = RequirementFlag::new("handScanner".into());
                    AnyOfAllRequirements(Vec1::new(AllRequirements(Vec1::new(hand_scanner))))
                });
            let rom = items::Rom::from_str(&to_pascal_case(&key))?;
            let spot = RomSpot::new(
                field_id,
                rom,
                SpotName::new(key.clone()),
                any_of_all_requirements,
            );
            let item = Item::rom(StrategyFlag::new(key), rom);
            roms.insert(rom, Rom { spot, item });
        }
    }

    let main_weapons =
        parse_game_structure(main_weapons, |field_id, src_idx, name, requirements| {
            MainWeapon {
                spot: MainWeaponSpot::new(field_id, src_idx, name.clone(), requirements),
                item: Item::main_weapon(src_idx, name.into()),
            }
        })?;
    let sub_weapons =
        parse_game_structure(sub_weapons, |field_id, src_idx, name, requirements| {
            SubWeapon {
                spot: SubWeaponSpot::new(field_id, src_idx, name.clone(), requirements),
                item: Item::sub_weapon(src_idx, name.into()),
            }
        })?;
    let chests = parse_game_structure(chests, |field_id, src_idx, name, requirements| Chest {
        spot: ChestSpot::new(field_id, src_idx, name.clone(), requirements),
        item: Item::chest_item(src_idx, name.into()),
    })?;
    let shops = parse_shop_requirements(shops)?;
    let events = parse_event_requirements(game_structure_files.events.0)?;
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

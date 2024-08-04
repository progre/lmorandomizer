use std::{
    collections::{BTreeMap, BTreeSet},
    mem::take,
};

use anyhow::{bail, Result};

use crate::{dataset::game_structure::GameStructure, randomizer::RandomizeOptions};

use super::{
    item::{Item, StrategyFlag},
    Chest, Event, MainWeapon, Rom, Seal, Shop, Storage, SubWeapon,
};

pub fn create_source(
    game_structure: &GameStructure,
    options: &RandomizeOptions,
) -> Result<Storage> {
    let mut main_weapons = BTreeMap::new();
    for spot in game_structure.main_weapon_shutters.iter().cloned() {
        let item = Item::main_weapon(spot.main_weapon(), spot.name().clone().into());
        main_weapons.insert(spot.main_weapon(), MainWeapon { spot, item });
    }
    let mut sub_weapons = BTreeMap::new();
    for spot in game_structure.sub_weapon_shutters.iter().cloned() {
        let name = spot.name().clone().into();
        let item = Item::sub_weapon(spot.field_number(), spot.sub_weapon(), name);
        let key = (spot.field_number(), spot.sub_weapon());
        if sub_weapons.contains_key(&key) {
            bail!("duplicate sub weapon: {:?}", key);
        }
        sub_weapons.insert(key, SubWeapon { spot, item });
    }
    let mut chests = BTreeMap::new();
    for spot in game_structure.chests.iter().cloned() {
        let item = Item::chest_item(spot.field_number(), spot.item(), spot.name().clone().into());
        let key = (spot.field_number(), spot.item());
        if chests.contains_key(&key) {
            bail!("duplicate chest: {:?}", key);
        }
        chests.insert(key, Chest { spot, item });
    }
    let mut seals = BTreeMap::new();
    for spot in game_structure.seals.iter().cloned() {
        let item = Item::seal(spot.seal(), spot.name().clone().into());
        seals.insert(spot.seal(), Seal { spot, item });
    }
    let mut roms = BTreeMap::new();
    for spot in game_structure.roadside_roms.iter().cloned() {
        let item = Item::rom(spot.rom(), spot.name().clone().into());
        roms.insert(spot.rom(), Rom { spot, item });
    }
    let mut shops = Vec::new();
    let mut shop_set = BTreeSet::new();
    for spot in game_structure.shops.iter().cloned() {
        if shop_set.contains(&spot.items()) {
            bail!("duplicate shop: {:?}", spot.items());
        }
        shop_set.insert(spot.items());
        let mut names = spot
            .name()
            .get()
            .split(',')
            .map(|name| {
                let name = name.trim();
                (name != "_").then(|| StrategyFlag::new(name.into()))
            })
            .enumerate()
            .map(|(idx, name)| name.map(|name| Item::shop_item(spot.items(), idx, name)));
        let items = [
            names.next().unwrap(),
            names.next().unwrap(),
            names.next().unwrap(),
        ];
        shops.push(Shop { spot, items });
    }
    let mut events: Vec<_> = game_structure
        .events
        .iter()
        .map(|x| Event {
            name: x.name.clone().into(),
            requirements: x.requirements.clone(),
        })
        .collect();
    log::trace!(
        "options.shuffle_secret_roms: {:?}",
        options.shuffle_secret_roms
    );
    if !options.shuffle_secret_roms {
        let roms = take(&mut roms);
        events.append(
            &mut roms
                .into_values()
                .map(|x| Event {
                    name: x.spot.name().to_owned().into(),
                    requirements: x.spot.requirements().to_owned(),
                })
                .collect::<Vec<_>>(),
        );
    }

    Storage::new(
        main_weapons,
        sub_weapons,
        chests,
        seals,
        roms,
        shops,
        events,
    )
}

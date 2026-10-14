use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, bail};

use crate::{dataset::game_structure::GameStructure, randomizer::storage::Talk};

use super::{
    Chest, Event, MainWeapon, Rom, Seal, Shop, Storage, SubWeapon,
    ids::LocationId,
    item::{Item, StrategyFlag},
};

pub fn create_source(game_structure: &GameStructure) -> Result<Storage> {
    let field_number = |region| game_structure.regions.get(region).field_number();
    let mut next_index = 0;
    let mut next_id = || {
        let id = LocationId::new(next_index);
        next_index += 1;
        id
    };
    let mut main_weapons = BTreeMap::new();
    for spot in game_structure.main_weapon_shutters.iter().cloned() {
        let id = next_id();
        let item = Item::main_weapon(id.into(), spot.main_weapon(), spot.name().clone().into());
        main_weapons.insert(spot.main_weapon(), MainWeapon { id, spot, item });
    }
    let mut sub_weapons = BTreeMap::new();
    for spot in game_structure.sub_weapon_shutters.iter().cloned() {
        let id = next_id();
        let name = spot.name().clone().into();
        let field_number = field_number(spot.region());
        let item = Item::sub_weapon(id.into(), field_number, spot.sub_weapon(), name);
        let key = (field_number, spot.sub_weapon());
        if sub_weapons.contains_key(&key) {
            bail!("duplicate sub weapon: {:?}", key);
        }
        sub_weapons.insert(key, SubWeapon { id, spot, item });
    }
    let mut chests = BTreeMap::new();
    for spot in game_structure.chests.iter().cloned() {
        let id = next_id();
        let name = spot.name().clone().into();
        let field_number = field_number(spot.region());
        let item = Item::chest_item(id.into(), field_number, spot.item(), name);
        let key = (field_number, spot.item());
        if chests.contains_key(&key) {
            bail!("duplicate chest: {:?}", key);
        }
        chests.insert(key, Chest { id, spot, item });
    }
    let mut seals = BTreeMap::new();
    for spot in game_structure.seals.iter().cloned() {
        let id = next_id();
        let item = Item::seal(id.into(), spot.seal(), spot.name().clone().into());
        seals.insert(spot.seal(), Seal { id, spot, item });
    }
    let mut roms = BTreeMap::new();
    for spot in game_structure.roadside_roms.iter().cloned() {
        let id = next_id();
        let item = Item::rom(id.into(), spot.rom(), spot.name().clone().into());
        roms.insert(spot.rom(), Rom { id, spot, item });
    }
    let mut shops = Vec::new();
    let mut shop_set = BTreeSet::new();
    for spot in game_structure.shops.iter() {
        if shop_set.contains(&spot.items()) {
            bail!("duplicate shop: {:?}", spot.items());
        }
        shop_set.insert(spot.items());
        for (idx, name) in spot.name().get().split(',').enumerate() {
            let name = name.trim();
            if name == "_" {
                continue;
            }
            let id = next_id();
            let name = StrategyFlag::new(name.into());
            let item = Item::shop_item(id.into(), spot.items(), idx, name);
            let spot = spot.clone();
            shops.push(Shop {
                id,
                spot,
                idx,
                item,
            });
        }
    }
    let events: Vec<_> = game_structure
        .events
        .iter()
        .map(|x| Event {
            region: x.region,
            name: x.name.clone().into(),
            requirements: x.requirements.clone(),
        })
        .collect();
    let mut talks = Vec::new();
    for spot in game_structure.talks.iter().cloned() {
        let id = next_id();
        let item = Item::talk(id.into(), spot.item(), spot.name().clone().into());
        talks.push(Talk { id, spot, item });
    }

    Storage::new(
        game_structure.regions.clone(),
        main_weapons,
        sub_weapons,
        chests,
        seals,
        roms,
        talks,
        shops,
        events,
    )
}

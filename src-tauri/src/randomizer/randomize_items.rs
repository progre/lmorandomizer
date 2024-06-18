use std::collections::HashMap;

use anyhow::Result;
use log::info;
use rand::Rng;
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::Item,
        storage::{ItemSpot, Shop, Storage},
    },
    randomizer::{
        shuffle_utils::{select_random, shuffle_simply},
        validate::validate,
    },
    script::data::script::Script,
};

pub fn randomize_items(script: &mut Script, source: &Storage, seed: &str) -> Result<()> {
    debug_assert!(validate(source));
    assert_unique(source);
    let mut rng: Xoshiro256PlusPlus = Seeder::from(seed).make_rng();
    let shuffled = randomize_storage(source, &mut rng);
    assert_unique(&shuffled);
    script.replace_items(&shuffled)?;
    script.replace_shops(shuffled.shops())?;
    Ok(())
}

fn randomize_storage(source: &Storage, rng: &mut impl Rng) -> Storage {
    let mut shuffled = None;
    for i in 0..10000 {
        // itemをshuffleしてplaceと合わせる
        let storage = shuffle(source, rng);
        if validate(&storage) {
            shuffled = Some(storage);
            info!("Shuffle was tryed: {i} times");
            break;
        }
    }
    let Some(shuffled) = shuffled else {
        panic!("Shuffle failed");
    };
    shuffled
}

fn shuffle(source: &Storage, rng: &mut impl Rng) -> Storage {
    let all_items = source.all_items();
    let (
        mut new_main_weapon_shutters,
        mut new_sub_weapon_shutters,
        mut new_chest_items,
        mut new_seal_chest_items,
        mut new_shop_items,
    ) = distribute_items(&all_items, source, rng);
    debug_assert_eq!(
        source.main_weapon_shutters().len(),
        new_main_weapon_shutters.len()
    );
    debug_assert_eq!(
        source.sub_weapon_shutters().len(),
        new_sub_weapon_shutters.len()
    );
    debug_assert_eq!(source.chests().len(), new_chest_items.len());
    debug_assert_eq!(source.seal_chests().len(), new_seal_chest_items.len());
    debug_assert_eq!(source.shops().len(), new_shop_items.len());
    shuffle_simply(&mut new_main_weapon_shutters, rng);
    let main_weapon_shutters = new_main_weapon_shutters
        .into_iter()
        .enumerate()
        .map(|(i, item)| ItemSpot {
            item,
            spot: source.main_weapon_shutters()[i].spot.clone(),
        })
        .collect();
    shuffle_simply(&mut new_sub_weapon_shutters, rng);
    let sub_weapon_shutters = new_sub_weapon_shutters
        .into_iter()
        .enumerate()
        .map(|(i, item)| ItemSpot {
            item,
            spot: source.sub_weapon_shutters()[i].spot.clone(),
        })
        .collect();
    shuffle_simply(&mut new_chest_items, rng);
    let chests = new_chest_items
        .into_iter()
        .enumerate()
        .map(|(i, item)| ItemSpot {
            item,
            spot: source.chests()[i].spot.clone(),
        })
        .collect();
    shuffle_simply(&mut new_seal_chest_items, rng);
    let seal_chests = new_seal_chest_items
        .into_iter()
        .enumerate()
        .map(|(i, item)| ItemSpot {
            item,
            spot: source.seal_chests()[i].spot.clone(),
        })
        .collect();
    shuffle_simply(&mut new_shop_items, rng);
    let shops: Vec<_> = new_shop_items
        .into_iter()
        .enumerate()
        .map(|(i, items)| Shop {
            items,
            spot: source.shops()[i].spot.clone(),
        })
        .collect();
    debug_assert!(shops.iter().all(|x| x.spot.talk_number.is_some()));
    Storage::new(
        source.all_requirement_names().clone(),
        main_weapon_shutters,
        sub_weapon_shutters,
        chests,
        seal_chests,
        shops,
    )
}

#[allow(clippy::type_complexity)]
fn distribute_items(
    items: &[&Item],
    source: &Storage,
    rng: &mut impl Rng,
) -> (
    Vec<Item>,
    Vec<Item>,
    Vec<Item>,
    Vec<Item>,
    Vec<(Item, Item, Item)>,
) {
    debug_assert_eq!(
        items.len(),
        source.main_weapon_shutters().len()
            + source.sub_weapon_shutters().len()
            + source.chests().len()
            + source.seal_chests().len()
            + source.shops().len() * 3,
    );
    let mut new_main_weapon_shutters: Vec<Item> = Vec::new();
    let mut new_sub_weapon_shutters: Vec<Item> = Vec::new();
    let mut new_chest_items: Vec<Item> = Vec::new();
    let mut new_seal_chest_items: Vec<Item> = Vec::new();
    let mut new_shop_items: Vec<Item> = Vec::new();
    for &item in items {
        match select_random(
            &[
                source.main_weapon_shutters().len() - new_main_weapon_shutters.len(),
                source.sub_weapon_shutters().len() - new_sub_weapon_shutters.len(),
                source.chests().len() - new_chest_items.len(),
                source.seal_chests().len() - new_seal_chest_items.len(),
                if !item.can_display_in_shop() {
                    0
                } else {
                    source.shops().len() * 3 - new_shop_items.len()
                },
            ],
            rng,
        ) {
            0 => new_main_weapon_shutters.push(item.clone()),
            1 => new_sub_weapon_shutters.push(item.clone()),
            2 => new_chest_items.push(item.clone()),
            3 => new_seal_chest_items.push(item.clone()),
            4 => new_shop_items.push(item.clone()),
            _ => unreachable!(),
        }
    }
    (
        new_main_weapon_shutters,
        new_sub_weapon_shutters,
        new_chest_items,
        new_seal_chest_items,
        new_shop_items
            .chunks_exact(3)
            .map(|chunk| (chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
            .collect(),
    )
}

fn assert_unique(storage: &Storage) {
    let mut name_map: HashMap<String, ItemSpot> = HashMap::new();
    let mut flag_map: HashMap<String, ItemSpot> = HashMap::new();

    storage
        .main_weapon_shutters()
        .iter()
        .chain(storage.sub_weapon_shutters().iter())
        .chain(storage.chests().iter())
        .chain(storage.seal_chests().iter())
        .chain(
            storage
                .shops()
                .iter()
                .flat_map(|x| {
                    [x.items.0.clone(), x.items.1.clone(), x.items.2.clone()]
                        .into_iter()
                        .map(|item| ItemSpot {
                            item,
                            spot: x.spot.clone(),
                        })
                })
                .collect::<Vec<_>>()
                .iter(),
        )
        .for_each(|x| {
            if x.item.name != "weights"
                && x.item.name != "shurikenAmmo"
                && x.item.name != "toukenAmmo"
                && x.item.name != "spearAmmo"
                && x.item.name != "flareGunAmmo"
                && x.item.name != "bombAmmo"
                && x.item.name != "ammunition"
                && x.item.name != "shellHorn"
                && x.item.name != "finder"
            {
                let key = format!("{}:{}", x.spot.r#type, x.item.name);
                if name_map.contains_key(&key) {
                    panic!("Duplicate item: {}", key);
                }
                name_map.insert(key, x.clone());
            }

            if x.item.flag != 65279 && x.item.flag != 753 && x.item.flag != 754 {
                let key = x.item.flag.to_string();
                if flag_map.contains_key(&key) {
                    panic!("Duplicate flag: {}", key);
                }
                flag_map.insert(key, x.clone());
            }
        });
}

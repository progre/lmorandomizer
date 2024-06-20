use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::Item,
        storage::{ItemSpot, Shop, Storage},
        supplements::StrategyFlag,
    },
    randomizer::{
        shuffle_utils::{select_random, shuffle_simply},
        validate::validate,
    },
    script::data::script::Script,
};

pub fn randomize_items(script: &mut Script, source: &Storage, seed: &str) -> Result<()> {
    let start = std::time::Instant::now();
    // debug_assert!(validate(source));
    assert_unique(source);
    trace!("Assertion in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut rng: Xoshiro256PlusPlus = Seeder::from(seed).make_rng();
    let shuffled = randomize_storage(source, &mut rng);
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    assert_unique(&shuffled);
    script.replace_items(&shuffled)?;
    trace!("Replaced items in {:?}", start.elapsed());
    Ok(())
}

fn randomize_storage(source: &Storage, rng: &mut impl Rng) -> Storage {
    let mut shuffled = None;
    for i in 0..10000 {
        // itemをshuffleしてplaceと合わせる
        let start = std::time::Instant::now();
        let storage = shuffle(source, rng);
        trace!("Shuffled in {:?}", start.elapsed());
        let start = std::time::Instant::now();
        let result = validate(&storage);
        trace!("Validated in {:?}", start.elapsed());
        if result {
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
        .map(|(i, items)| {
            let old = &source.shops()[i];
            Shop {
                items,
                spot: old.spot.clone(),
            }
        })
        .collect();
    Storage::new(
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
    let mut names = HashSet::new();
    let mut flags = HashSet::new();

    storage
        .main_weapon_shutters()
        .iter()
        .map(|x| ("weaponShutter", x))
        .chain(
            storage
                .sub_weapon_shutters()
                .iter()
                .map(|x| ("weaponShutter", x)),
        )
        .chain(storage.chests().iter().map(|x| ("chest", x)))
        .chain(storage.seal_chests().iter().map(|x| ("sealChest", x)))
        .map(|(item_type, item_spot)| (item_type, item_spot.item.clone()))
        .chain(storage.shops().iter().flat_map(|x| {
            [&x.items.0, &x.items.1, &x.items.2]
                .into_iter()
                .cloned()
                .map(|item| ("shop", item))
        }))
        .for_each(|(item_type, item)| {
            if ![
                StrategyFlag::new("weights".to_owned()),
                StrategyFlag::new("shurikenAmmo".to_owned()),
                StrategyFlag::new("toukenAmmo".to_owned()),
                StrategyFlag::new("spearAmmo".to_owned()),
                StrategyFlag::new("flareGunAmmo".to_owned()),
                StrategyFlag::new("bombAmmo".to_owned()),
                StrategyFlag::new("ammunition".to_owned()),
                StrategyFlag::new("shellHorn".to_owned()),
                StrategyFlag::new("finder".to_owned()),
            ]
            .contains(item.name())
            {
                let key = format!("{}:{:?}", item_type, item.name());
                if names.contains(&key) {
                    panic!("Duplicate item: {}", key);
                }
                names.insert(key);
            }

            if ![65279, 753, 754].contains(&item.flag()) {
                if flags.contains(&item.flag()) {
                    panic!("Duplicate flag: {}", item.flag());
                }
                flags.insert(item.flag());
            }
        });
}

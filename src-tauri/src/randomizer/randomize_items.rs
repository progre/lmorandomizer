use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    dataset::{
        item::{Item, StrategyFlag},
        spot::Spot,
        storage::{ItemSpot, Shop, Storage},
    },
    randomizer::spoiler::{make_rng, spoiler},
    script::data::script::Script,
};

use super::spoiler_log::SpoilerLog;

pub fn randomize_items(script: &mut Script, source: &Storage, seed: &str) -> Result<SpoilerLog> {
    let start = std::time::Instant::now();
    // debug_assert!(validate(source));
    assert_unique(source);
    trace!("Assertion in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let (shuffled, spoiler_log) = randomize_storage(source, &mut rng);
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    assert_unique(&shuffled);
    script.replace_items(&script.clone(), &shuffled)?;
    trace!("Replaced items in {:?}", start.elapsed());
    Ok(spoiler_log)
}

fn randomize_storage(source: &Storage, rng: &mut impl Rng) -> (Storage, SpoilerLog) {
    let (sellable_items, unsellable_items): (Vec<_>, Vec<_>) = source
        .all_items()
        .cloned()
        .partition(|x| x.can_display_in_shop());

    debug_assert_eq!(
        sellable_items.len() + unsellable_items.len(),
        source.main_weapon_shutters().len()
            + source.sub_weapon_shutters().len()
            + source.chests().len()
            + source.seal_chests().len()
            + source.shops().len() * 3,
    );

    let start = std::time::Instant::now();
    let (storage, spoiler_log) = shuffle(
        source,
        &sellable_items.to_vec(),
        &unsellable_items.to_vec(),
        rng,
    );
    trace!("Shuffled in {:?}", start.elapsed());
    (storage, spoiler_log)
}

fn to_spots(src: &[ItemSpot]) -> Vec<Spot> {
    src.iter().map(|x| x.spot.clone()).collect()
}

fn shop_to_spots(src: &[Shop]) -> Vec<Spot> {
    src.iter().map(|x| x.spot.clone()).collect()
}

fn shuffle(
    source: &Storage,
    sellables: &[Item],
    unsellables: &[Item],
    rng: &mut impl Rng,
) -> (Storage, SpoilerLog) {
    let mut field_item_spots = to_spots(source.main_weapon_shutters());
    field_item_spots.append(&mut to_spots(source.sub_weapon_shutters()));
    field_item_spots.append(&mut to_spots(source.chests()));
    field_item_spots.append(&mut to_spots(source.seal_chests()));
    let field_item_spots: &Vec<_> = &field_item_spots.iter().collect();

    let shop_spots = shop_to_spots(source.shops());
    let shop_spots: &Vec<_> = &shop_spots.iter().collect();

    let thread_count = std::thread::available_parallelism().unwrap().get();

    let spoiler_log = std::thread::scope(|scope| {
        for i in 0..100000 {
            let handles = (0..thread_count)
                .map(|_| {
                    let seed = rng.next_u64();
                    scope.spawn(move || {
                        spoiler(seed, sellables, unsellables, field_item_spots, shop_spots)
                    })
                })
                .collect::<Vec<_>>();
            let Some(spoiler_log) = handles.into_iter().filter_map(|h| h.join().unwrap()).next()
            else {
                continue;
            };
            info!("Shuffle was tried: {} times", (i + 1) * thread_count);
            return spoiler_log;
        }
        unreachable!();
    });
    let mut storage = source.clone();
    for checkpoint in spoiler_log.progression.iter().flat_map(|sphere| &sphere.0) {
        match &checkpoint.spot {
            Spot::MainWeaponShutter(spot) => {
                storage.main_weapon_shutters_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::SubWeaponShutter(spot) => {
                storage.sub_weapon_shutters_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::Chest(spot) => {
                storage.chests_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::SealChest(spot) => {
                storage.seal_chests_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::Shop(spot) => {
                storage.shops_mut()[spot.src_idx].items = (
                    checkpoint.items[0].clone(),
                    checkpoint.items[1].clone(),
                    checkpoint.items[2].clone(),
                );
            }
        }
    }
    (storage, spoiler_log)
}

fn assert_unique(storage: &Storage) {
    let mut names = HashSet::new();

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
            if !item.name().is_consumable()
                && ![
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
        });
}

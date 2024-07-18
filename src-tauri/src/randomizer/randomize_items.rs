use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    dataset::{item::StrategyFlag, spot::Spot, storage::Storage},
    script::data::script::Script,
};

use super::{
    items_spots::{Items, Spots},
    spoiler::{make_rng, spoiler},
    spoiler_log::SpoilerLogRef,
};

pub fn randomize_items<'a>(
    script: &mut Script,
    source: &'a Storage,
    seed: &str,
) -> Result<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    assert_unique(source);
    trace!("Assertion in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let (shuffled, spoiler_log) = shuffle(&mut rng, source);
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    assert_unique(&shuffled);
    script.replace_items(&script.clone(), &shuffled)?;
    trace!("Replaced items in {:?}", start.elapsed());
    Ok(spoiler_log)
}

fn create_shuffled_storage(source: &Storage, spoiler_log: &SpoilerLogRef) -> Storage {
    let mut storage = source.clone();
    for checkpoint in spoiler_log
        .progression
        .iter()
        .flat_map(|sphere| &sphere.0)
        .chain(spoiler_log.maps.iter())
    {
        match &checkpoint.spot {
            Spot::MainWeapon(spot) => {
                storage.main_weapons_mut()[spot.src_idx].item = checkpoint.item.clone();
            }
            Spot::SubWeapon(spot) => {
                storage.sub_weapons_mut()[spot.src_idx].item = checkpoint.item.clone();
            }
            Spot::Chest(spot) => {
                storage.chests_mut()[spot.src_idx].item = checkpoint.item.clone();
            }
            Spot::Seal(spot) => {
                storage.seals_mut()[spot.src_idx].item = checkpoint.item.clone();
            }
            Spot::Shop(spot) => {
                let items = &mut storage.shops_mut()[spot.src_idx].items;
                *[&mut items.0, &mut items.1, &mut items.2][checkpoint.idx] =
                    checkpoint.item.clone();
            }
        }
    }
    storage
}

fn random_spoiler<'a>(rng: &mut impl Rng, source: &'a Storage) -> SpoilerLogRef<'a> {
    let start = std::time::Instant::now();
    let items = &Items::new(source);
    let spots = &Spots::new(source);
    debug_assert_eq!(
        spots.shops.len() - items.consumable_items().len(),
        spots
            .shops
            .iter()
            .map(|spot| (!spot.name.is_consumable()) as usize)
            .sum::<usize>(),
    );
    debug_assert_eq!(
        spots
            .shops
            .iter()
            .map(|spot| spot.name.is_consumable() as usize)
            .sum::<usize>(),
        items.consumable_items().len()
    );
    trace!("Prepared items and spots in {:?}", start.elapsed());

    let thread_count = std::thread::available_parallelism().unwrap().get();
    std::thread::scope(|scope| {
        for i in 0..100000 {
            let handles: Vec<_> = (0..thread_count)
                .map(|_| rng.next_u64())
                .map(|seed| scope.spawn(move || spoiler(seed, items, spots)))
                .collect();
            let Some(spoiler_log) = handles.into_iter().filter_map(|h| h.join().unwrap()).next()
            else {
                continue;
            };
            info!("Shuffle was tried: {} times", (i + 1) * thread_count);
            return spoiler_log;
        }
        unreachable!();
    })
}

fn shuffle<'a>(rng: &mut impl Rng, source: &'a Storage) -> (Storage, SpoilerLogRef<'a>) {
    let spoiler_log = random_spoiler(rng, source);
    let storage = create_shuffled_storage(source, &spoiler_log);
    (storage, spoiler_log)
}

fn assert_unique(storage: &Storage) {
    if cfg!(not(debug_assertions)) {
        return;
    }
    let mut names = HashSet::new();

    storage
        .main_weapons()
        .iter()
        .map(|x| ("weapon", x))
        .chain(storage.sub_weapons().iter().map(|x| ("weapon", x)))
        .chain(storage.chests().iter().map(|x| ("chest", x)))
        .chain(storage.seals().iter().map(|x| ("seal", x)))
        .map(|(item_type, item_spot)| (item_type, item_spot.item.clone()))
        .chain(storage.shops().iter().flat_map(|x| {
            [&x.items.0, &x.items.1, &x.items.2]
                .into_iter()
                .cloned()
                .map(|item| ("shop", item))
        }))
        .for_each(|(item_type, item)| {
            if !item.name.is_consumable()
                && ![
                    StrategyFlag::new("shellHorn".to_owned()),
                    StrategyFlag::new("finder".to_owned()),
                ]
                .contains(&item.name)
            {
                let key = format!("{}:{:?}", item_type, &item.name);
                if names.contains(&key) {
                    panic!("Duplicate item: {}", key);
                }
                names.insert(key);
            }
        });
}

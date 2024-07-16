use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    dataset::{item::StrategyFlag, spot::Spot, storage::Storage},
    randomizer::spoiler::{make_rng, spoiler},
    script::data::script::Script,
};

use super::{
    spoiler::{Items, Spots},
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

fn to_all_items(source: &Storage) -> Items {
    let (priority_items, remaining_items) = source.all_items().partition::<Vec<_>, _>(|item| {
        [
            "handScanner",
            "shellHorn",
            "holyGrail",
            "gameMaster",
            "glyphReader",
        ]
        .contains(&item.name.get())
    });
    let (sellable_items, unsellable_items): (Vec<_>, Vec<_>) = remaining_items
        .into_iter()
        .partition(|x| x.can_display_in_shop());
    debug_assert!(unsellable_items.iter().all(|x| !x.name.is_consumable()));
    let (consumable_items, sellable_items): (Vec<_>, Vec<_>) = sellable_items
        .into_iter()
        .partition(|x| x.name.is_consumable());

    debug_assert_eq!(
        priority_items.len()
            + unsellable_items.len()
            + sellable_items.len()
            + consumable_items.len(),
        source.all_items().count(),
    );
    debug_assert_eq!(
        priority_items.len()
            + unsellable_items.len()
            + sellable_items.len()
            + consumable_items.len(),
        source.main_weapons().len()
            + source.sub_weapons().len()
            + source.chests().len()
            + source.seals().len()
            + source
                .shops()
                .iter()
                .map(|_| true as usize + true as usize + true as usize)
                .sum::<usize>(),
    );
    debug_assert_eq!(
        priority_items.len() + unsellable_items.len() + sellable_items.len(),
        source.main_weapons().len()
            + source.sub_weapons().len()
            + source.chests().len()
            + source.seals().len()
            + source
                .shops()
                .iter()
                .map(|shop| shop.count_general_items())
                .sum::<usize>(),
    );

    Items {
        priority_items,
        sellable_items,
        unsellable_items,
        consumable_items,
    }
}

fn to_all_spots(source: &Storage) -> Spots {
    Spots {
        field_item_spots: source
            .main_weapons()
            .iter()
            .chain(source.sub_weapons())
            .chain(source.chests())
            .chain(source.seals())
            .map(|x| &x.spot)
            .collect(),
        shops: source.shops().iter().collect(),
    }
}

fn create_shuffled_storage(source: &Storage, spoiler_log: &SpoilerLogRef) -> Storage {
    let mut storage = source.clone();
    for checkpoint in spoiler_log.progression.iter().flat_map(|sphere| &sphere.0) {
        match &checkpoint.spot {
            Spot::MainWeapon(spot) => {
                storage.main_weapons_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::SubWeapon(spot) => {
                storage.sub_weapons_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::Chest(spot) => {
                storage.chests_mut()[spot.src_idx].item = checkpoint.items[0].clone();
            }
            Spot::Seal(spot) => {
                storage.seals_mut()[spot.src_idx].item = checkpoint.items[0].clone();
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
    storage
}

fn shuffle<'a>(rng: &mut impl Rng, source: &'a Storage) -> (Storage, SpoilerLogRef<'a>) {
    let start = std::time::Instant::now();
    let items = &to_all_items(source);
    let spots = &to_all_spots(source);
    debug_assert!(items
        .priority_items
        .iter()
        .all(|item| item.can_display_in_shop()));
    debug_assert_eq!(
        spots.shops.len() * 3 - items.consumable_items.len(),
        spots
            .shops
            .iter()
            .map(|shop| shop.count_general_items())
            .sum::<usize>(),
    );
    debug_assert_eq!(
        spots
            .shops
            .iter()
            .map(|shop| 3 - shop.count_general_items())
            .sum::<usize>(),
        items.consumable_items.len()
    );
    trace!("Prepared items and spots in {:?}", start.elapsed());

    let thread_count = std::thread::available_parallelism().unwrap().get();

    let spoiler_log = std::thread::scope(|scope| {
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
    });
    let storage = create_shuffled_storage(source, &spoiler_log);
    (storage, spoiler_log)
}

fn assert_unique(storage: &Storage) {
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

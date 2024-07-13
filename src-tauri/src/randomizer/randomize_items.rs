use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    dataset::{
        item::{Item, StrategyFlag},
        spot::Spot,
        storage::{ItemSpot, Storage},
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
    let (priority_items, remaining_items) =
        source.all_items().cloned().partition::<Vec<_>, _>(|item| {
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

    let start = std::time::Instant::now();
    let (storage, spoiler_log) = shuffle(
        source,
        &priority_items,
        &sellable_items,
        &unsellable_items,
        &consumable_items,
        rng,
    );
    trace!("Shuffled in {:?}", start.elapsed());
    (storage, spoiler_log)
}

fn to_spots(src: &[ItemSpot]) -> Vec<Spot> {
    src.iter().map(|x| x.spot.clone()).collect()
}

fn shuffle(
    source: &Storage,
    priority_items: &[Item],
    sellable_items: &[Item],
    unsellable_items: &[Item],
    consumable_items: &[Item],
    rng: &mut impl Rng,
) -> (Storage, SpoilerLog) {
    let mut field_item_spots = to_spots(source.main_weapons());
    field_item_spots.append(&mut to_spots(source.sub_weapons()));
    field_item_spots.append(&mut to_spots(source.chests()));
    field_item_spots.append(&mut to_spots(source.seals()));
    let field_item_spots: &Vec<_> = &field_item_spots.iter().collect();

    let shops: &Vec<_> = &source.shops().iter().collect();

    let thread_count = std::thread::available_parallelism().unwrap().get();

    let spoiler_log = std::thread::scope(|scope| {
        for i in 0..100000 {
            let handles = (0..thread_count)
                .map(|_| {
                    let seed = rng.next_u64();
                    scope.spawn(move || {
                        spoiler(
                            seed,
                            priority_items,
                            sellable_items,
                            unsellable_items,
                            consumable_items,
                            field_item_spots,
                            shops,
                        )
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

use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    randomizer::spoiler::{items::Items, locations::Locations},
    script::{data::script::Script, editor::apply_storage},
};

use super::{
    RandomizeOptions,
    spoiler::{make_rng, spoiler},
    spoiler_log::{CheckpointRef, SpoilerLogRef},
    storage::{
        Storage,
        ids::LocationTable,
        item::{Item, StrategyFlag},
    },
};

pub fn randomize_items<'a>(
    script: &mut Script,
    source: &'a Storage,
    options: &RandomizeOptions,
) -> Result<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    assert_unique(source);
    trace!("Assertion in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let (shuffled, spoiler_log) = shuffle(source, options);
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    assert_unique(&shuffled);
    apply_storage(script, &shuffled)?;
    trace!("Replaced items in {:?}", start.elapsed());
    Ok(spoiler_log)
}

fn create_shuffled_storage(source: &Storage, spoiler_log: &SpoilerLogRef) -> Storage {
    let placed: LocationTable<&Item> = spoiler_log
        .progression
        .iter()
        .flat_map(|sphere| sphere.iter())
        .chain(&spoiler_log.maps)
        .filter_map(|checkpoint| match checkpoint {
            CheckpointRef::Item { location, item } => Some((location.id(), *item)),
            CheckpointRef::Event(_) => None,
        })
        .collect();
    let mut storage = source.clone();
    for (id, item) in storage.items_mut() {
        if let Some(placed) = placed.get(id) {
            *item = (*placed).clone();
        }
    }
    storage
}

fn random_spoiler<'a>(
    rng: &mut impl Rng,
    source: &'a Storage,
    options: &RandomizeOptions,
) -> SpoilerLogRef<'a> {
    let start = std::time::Instant::now();
    let all_regions = &source.regions;
    let items = &Items::new(source);
    let locations = &Locations::new(source);
    debug_assert_eq!(
        locations.shops.len() - items.consumable_items().len(),
        locations
            .shops
            .iter()
            .map(|shop| (!shop.item.name.is_consumable()) as usize)
            .sum::<usize>(),
    );
    debug_assert_eq!(
        locations
            .shops
            .iter()
            .map(|shop| shop.item.name.is_consumable() as usize)
            .sum::<usize>(),
        items.consumable_items().len()
    );
    trace!("Prepared items and locations in {:?}", start.elapsed());

    let thread_count = std::thread::available_parallelism().unwrap().get();
    std::thread::scope(|scope| {
        for i in 0..100000 {
            let handles: Vec<_> = (0..thread_count)
                .map(|_| rng.next_u64())
                .map(|seed| {
                    scope.spawn(move || spoiler(seed, options, all_regions, items, locations))
                })
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

fn shuffle<'a>(source: &'a Storage, options: &RandomizeOptions) -> (Storage, SpoilerLogRef<'a>) {
    let mut rng = make_rng(&options.seed);
    let spoiler_log = random_spoiler(&mut rng, source, options);
    let storage = create_shuffled_storage(source, &spoiler_log);
    (storage, spoiler_log)
}

fn assert_unique(storage: &Storage) {
    if cfg!(not(debug_assertions)) {
        return;
    }
    let mut names = HashSet::new();

    storage
        .main_weapons
        .values()
        .map(|x| ("weapon", &x.item))
        .chain(storage.sub_weapons.values().map(|x| ("weapon", &x.item)))
        .chain(storage.chests.values().map(|x| ("chest", &x.item)))
        .chain(storage.seals.values().map(|x| ("seal", &x.item)))
        .chain(
            storage
                .shops
                .iter()
                .map(|x| &x.item)
                .map(|item| ("shop", item)),
        )
        .for_each(|(item_type, item)| {
            if !item.name.is_consumable()
                && ![
                    StrategyFlag::new("shellHorn".to_owned()),
                    StrategyFlag::new("finder".to_owned()),
                ]
                .contains(&item.name)
            {
                let key = format!("{}:{:?}", item_type, item.name);
                if names.contains(&key) {
                    panic!("Duplicate item: {}", key);
                }
                names.insert(key);
            }
        });
}

#[cfg(test)]
mod tests;

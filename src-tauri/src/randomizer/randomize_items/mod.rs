use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    randomizer::spoiler::{items::Items, regions::Regions, spots::Spots},
    script::{data::script::Script, editor::apply_storage},
};

use super::{
    RandomizeOptions,
    spoiler::{make_rng, spoiler},
    spoiler_log::{CheckpointRef, SpoilerLogRef},
    storage::{Storage, item::StrategyFlag},
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
    let mut storage = source.clone();
    for checkpoint in spoiler_log
        .progression
        .iter()
        .flat_map(|sphere| sphere.iter())
        .chain(&spoiler_log.maps)
    {
        match checkpoint {
            CheckpointRef::MainWeapon(main_weapon) => {
                let content = main_weapon.spot.main_weapon();
                storage.main_weapons.get_mut(&content).unwrap().item = main_weapon.item.clone();
            }
            CheckpointRef::SubWeapon(sub_weapon) => {
                let key = (
                    sub_weapon.spot.region().field_number(),
                    sub_weapon.spot.sub_weapon(),
                );
                storage.sub_weapons.get_mut(&key).unwrap().item = sub_weapon.item.clone();
            }
            CheckpointRef::Chest(chest) => {
                let key = (chest.spot.region().field_number(), chest.spot.item());
                storage.chests.get_mut(&key).unwrap().item = chest.item.clone();
            }
            CheckpointRef::Seal(seal) => {
                let content = seal.spot.seal();
                storage.seals.get_mut(&content).unwrap().item = seal.item.clone();
            }
            CheckpointRef::Shop(shop) => {
                let spot = &mut storage
                    .shops
                    .iter_mut()
                    .find(|x| x.spot.items() == shop.spot.items() && x.idx == shop.idx)
                    .unwrap();
                spot.item = shop.item.clone();
            }
            CheckpointRef::Rom(rom) => {
                let content = rom.spot.rom();
                storage.roms.get_mut(&content).unwrap().item = rom.item.clone();
            }
            CheckpointRef::Talk(talk) => {
                let item = talk.spot.item();
                storage
                    .talks
                    .iter_mut()
                    .find(|x| x.spot.item() == item)
                    .unwrap()
                    .item = talk.item.clone();
            }
            CheckpointRef::Event(_) => {}
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
    let all_regions = &Regions::new(source.regions.iter().collect());
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
                .map(|seed| scope.spawn(move || spoiler(seed, options, all_regions, items, spots)))
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

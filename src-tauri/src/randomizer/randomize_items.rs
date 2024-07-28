use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::Rng;

use crate::{
    dataset::{item::StrategyFlag, storage::Storage},
    script::data::script::Script,
};

use super::{
    items_spots::{Items, Spots},
    spoiler::{make_rng, spoiler},
    spoiler_log::{CheckpointRef, SpoilerLogRef},
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
    let (shuffled, spoiler_log) = shuffle(seed, source);
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
        .chain(&spoiler_log.maps)
    {
        match checkpoint {
            CheckpointRef::MainWeapon(checkpoint) => {
                storage.main_weapons[checkpoint.spot.src_idx()].item = checkpoint.item.clone();
            }
            CheckpointRef::SubWeapon(checkpoint) => {
                storage.sub_weapons[checkpoint.spot.src_idx()].item = checkpoint.item.clone();
            }
            CheckpointRef::Chest(checkpoint) => {
                storage.chests[checkpoint.spot.src_idx()].item = checkpoint.item.clone();
            }
            CheckpointRef::Seal(checkpoint) => {
                storage.seals[checkpoint.spot.src_idx()].item = checkpoint.item.clone();
            }
            CheckpointRef::Shop(checkpoint) => {
                let items = &mut storage.shops[checkpoint.spot.src_idx()].items;
                items.0 = checkpoint.items.0.clone();
                items.1 = checkpoint.items.1.clone();
                items.2 = checkpoint.items.2.clone();
            }
            CheckpointRef::Rom(checkpoint) => {
                storage.roms.get_mut(&checkpoint.spot.rom()).unwrap().item =
                    checkpoint.item.clone();
            }
            CheckpointRef::Event(_) => {}
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

fn shuffle<'a>(seed: &str, source: &'a Storage) -> (Storage, SpoilerLogRef<'a>) {
    let mut rng = make_rng(seed);
    let spoiler_log = random_spoiler(&mut rng, source);
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
        .iter()
        .map(|x| ("weapon", &x.item))
        .chain(storage.sub_weapons.iter().map(|x| ("weapon", &x.item)))
        .chain(storage.chests.iter().map(|x| ("chest", &x.item)))
        .chain(storage.seals.iter().map(|x| ("seal", &x.item)))
        .chain(
            storage
                .shops
                .iter()
                .flat_map(|x| [&x.items.0, &x.items.1, &x.items.2].map(|item| ("shop", item))),
        )
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

#[cfg(test)]
mod tests {
    use sha3::Digest;

    use crate::{app::read_game_structure_files_debug, dataset::create_source::create_source};

    use super::*;

    #[tokio::test]
    async fn test_shuffle() -> Result<()> {
        let game_structure_files = read_game_structure_files_debug().await?;
        let source = create_source(game_structure_files)?;
        let (shuffled, spoiler_log) = shuffle("test", &source);

        let shuffled_str = format!("{:?}", shuffled);
        let shuffled_hash = hex::encode(sha3::Sha3_512::digest(shuffled_str));
        const EXPECTED_SHUFFLED_HASH: &str = "064e2abfbf919e9696380de670b1e8f20cbe0f0920a903c312efb0c6ba98a1edbb01aecb5f5ad17c41b8c4cd1ab399ccdd15bb32eb2a0ac31a42f0c8152bf80f";
        assert_eq!(shuffled_hash, EXPECTED_SHUFFLED_HASH);

        let spoiler_log_str = format!("{:?}", spoiler_log.to_owned());
        let spoiler_log_hash = hex::encode(sha3::Sha3_512::digest(spoiler_log_str));
        const EXPECTED_SPOILER_LOG_HASH: &str = "1557e42db9c33277851cfc753b1b95131bff035aad9fd92b654a2798db0cdbb5e23947a6f9b2be06cbf30e9744819f9834a206426b9f15a89d9d523d0fbeb4da";
        assert_eq!(spoiler_log_hash, EXPECTED_SPOILER_LOG_HASH);

        Ok(())
    }
}

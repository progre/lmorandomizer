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
            CheckpointRef::MainWeapon(main_weapon) => {
                let content = main_weapon.spot.main_weapon();
                storage.main_weapons.get_mut(&content).unwrap().item = main_weapon.item.clone();
            }
            CheckpointRef::SubWeapon(sub_weapon) => {
                let key = (sub_weapon.spot.field_id(), sub_weapon.spot.sub_weapon());
                storage.sub_weapons.get_mut(&key).unwrap().item = sub_weapon.item.clone();
            }
            CheckpointRef::Chest(checkpoint) => {
                storage.chests[checkpoint.spot.src_idx()].item = checkpoint.item.clone();
            }
            CheckpointRef::Seal(seal) => {
                let content = seal.spot.seal();
                storage.seals.get_mut(&content).unwrap().item = seal.item.clone();
            }
            CheckpointRef::Shop(checkpoint) => {
                let items = &mut storage.shops[checkpoint.spot.src_idx()].items;
                items.0 = checkpoint.items.0.clone();
                items.1 = checkpoint.items.1.clone();
                items.2 = checkpoint.items.2.clone();
            }
            CheckpointRef::Rom(rom) => {
                let content = rom.spot.rom();
                storage.roms.get_mut(&content).unwrap().item = rom.item.clone();
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
        .values()
        .map(|x| ("weapon", &x.item))
        .chain(storage.sub_weapons.values().map(|x| ("weapon", &x.item)))
        .chain(storage.chests.iter().map(|x| ("chest", &x.item)))
        .chain(storage.seals.values().map(|x| ("seal", &x.item)))
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
        const EXPECTED_SHUFFLED_HASH: &str = "ada1d774e8a4bcb82f65327cd257069e16d76a9fae5065c1414ef4d7869db2ad3dae06fada040280da70356a2109fb71ccf9732e3cd1da36317ad9e00d05fb4d";
        assert_eq!(shuffled_hash, EXPECTED_SHUFFLED_HASH);

        let spoiler_log_str = format!("{:?}", spoiler_log.to_owned());
        let spoiler_log_hash = hex::encode(sha3::Sha3_512::digest(spoiler_log_str));
        const EXPECTED_SPOILER_LOG_HASH: &str = "27451559f9f5dd5999b1a4f1e6b002d2d0b7d5aaca46de127145f6cd885c340115f198b4984f56dab9ae8721daf5ab89e126fbeb11958cde358c7a53103668c4";
        assert_eq!(spoiler_log_hash, EXPECTED_SPOILER_LOG_HASH);

        Ok(())
    }
}

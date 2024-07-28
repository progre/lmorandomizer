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
            CheckpointRef::Chest(chest) => {
                let key = (chest.spot.field_id(), chest.spot.item());
                storage.chests.get_mut(&key).unwrap().item = chest.item.clone();
            }
            CheckpointRef::Seal(seal) => {
                let content = seal.spot.seal();
                storage.seals.get_mut(&content).unwrap().item = seal.item.clone();
            }
            CheckpointRef::Shop(shop) => {
                let items = &mut storage
                    .shops
                    .iter_mut()
                    .find(|x| x.spot.items() == shop.spot.items())
                    .unwrap()
                    .items;
                items.0 = shop.items.0.clone();
                items.1 = shop.items.1.clone();
                items.2 = shop.items.2.clone();
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
        .chain(storage.chests.values().map(|x| ("chest", &x.item)))
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
        const EXPECTED_SHUFFLED_HASH: &str = "4e3d61950d8ca12c6fe7826bea979d286aa76526cf9ca2eb9a1875bf5562c385f05acd041189c867c0b80ca7ea23fc5dad7aca2042deb1ce07c97c3335e8e060";
        assert_eq!(shuffled_hash, EXPECTED_SHUFFLED_HASH);

        let spoiler_log_str = format!("{:?}", spoiler_log.to_owned());
        let spoiler_log_hash = hex::encode(sha3::Sha3_512::digest(spoiler_log_str));
        const EXPECTED_SPOILER_LOG_HASH: &str = "3128eac4f51393b7d61e7597ecea6efdd1873080cfb87ac95fdab38a63630b72131964c91e1404184df2c1d18fcc5225d34b5e7b039e8fd2c40ce076573bfa7b";
        assert_eq!(spoiler_log_hash, EXPECTED_SPOILER_LOG_HASH);

        Ok(())
    }
}

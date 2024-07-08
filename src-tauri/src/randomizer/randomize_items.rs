use std::collections::HashSet;

use anyhow::Result;
use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::Item,
        storage::{ItemSpot, Shop, Storage},
        supplements::StrategyFlag,
    },
    randomizer::validate::validate,
    script::data::script::Script,
};

use super::spoiler_log::SpoilerLog;

pub fn randomize_items(script: &mut Script, source: &Storage, seed: &str) -> Result<SpoilerLog> {
    let start = std::time::Instant::now();
    // debug_assert!(validate(source));
    assert_unique(source);
    trace!("Assertion in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let mut rng: Xoshiro256PlusPlus = Seeder::from(seed).make_rng();
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

    for i in 0..10000 {
        // itemをshuffleしてplaceと合わせる
        let start = std::time::Instant::now();
        let storage = shuffle(
            source,
            sellable_items.to_vec(),
            unsellable_items.to_vec(),
            rng,
        );
        trace!("Shuffled in {:?}", start.elapsed());
        let start = std::time::Instant::now();
        let result = validate(&storage);
        trace!("Validated in {:?}", start.elapsed());
        if let Some(result) = result {
            info!("Shuffle was tryed: {i} times");
            return (storage, result);
        }
    }
    panic!("Shuffle failed")
}

fn shuffle(
    source: &Storage,
    mut sellable_items: Vec<Item>,
    mut unsellable_items: Vec<Item>,
    rng: &mut impl Rng,
) -> Storage {
    sellable_items.as_mut_slice().shuffle(rng);
    let new_shop_items: Vec<_> = sellable_items
        .drain((sellable_items.len() - source.shops().len() * 3)..)
        .collect();

    unsellable_items.append(&mut sellable_items);
    unsellable_items.as_mut_slice().shuffle(rng);
    let mut items = unsellable_items;

    let mut f = |src: &[ItemSpot]| -> Vec<_> {
        let drain = items.drain(items.len() - src.len()..);
        drain
            .zip(src)
            .map(|(item, src)| ItemSpot {
                item,
                spot: src.spot.clone(),
            })
            .collect()
    };

    let main_weapon_shutters = f(source.main_weapon_shutters());
    let sub_weapon_shutters = f(source.sub_weapon_shutters());
    let chests = f(source.chests());
    let seal_chests = f(source.seal_chests());
    let shops: Vec<_> = new_shop_items
        .chunks_exact(3)
        .zip(source.shops())
        .map(|(chunk, src)| Shop {
            items: (chunk[0].clone(), chunk[1].clone(), chunk[2].clone()),
            spot: src.spot.clone(),
        })
        .collect();

    debug_assert_eq!(
        source.main_weapon_shutters().len(),
        main_weapon_shutters.len()
    );
    debug_assert_eq!(
        source.sub_weapon_shutters().len(),
        sub_weapon_shutters.len()
    );
    debug_assert_eq!(source.chests().len(), chests.len());
    debug_assert_eq!(source.seal_chests().len(), seal_chests.len());
    debug_assert_eq!(source.shops().len(), shops.len());

    Storage::new(
        main_weapon_shutters,
        sub_weapon_shutters,
        chests,
        seal_chests,
        shops,
    )
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
        });
}

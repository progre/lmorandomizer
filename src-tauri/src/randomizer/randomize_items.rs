use std::{collections::HashSet, hash::Hash};

use anyhow::Result;
use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::Item,
        spot::Spot,
        storage::{ItemSpot, Shop, Storage},
        supplements::StrategyFlag,
    },
    script::data::script::Script,
};

use super::spoiler_log::{Checkpoint, Sphere, SpoilerLog};

fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

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
        sellable_items.to_vec(),
        unsellable_items.to_vec(),
        rng,
    );
    trace!("Shuffled in {:?}", start.elapsed());
    (storage, spoiler_log)
}

fn shuffle_items(
    rng: &mut impl Rng,
    mut sellable_items: Vec<Item>,
    unsellable_items: Vec<Item>,
    shop_count: usize,
) -> (Vec<Item>, Vec<Item>) {
    sellable_items.as_mut_slice().shuffle(rng);
    let new_shop_items = sellable_items.split_off(sellable_items.len() - shop_count * 3);
    let mut new_items = unsellable_items;
    new_items.append(&mut sellable_items);
    drop(sellable_items);
    new_items.as_mut_slice().shuffle(rng);
    (new_items, new_shop_items)
}

fn to_spots(src: &[ItemSpot]) -> Vec<Spot> {
    src.iter().map(|x| x.spot.clone()).collect()
}

fn shop_to_spots(src: &[Shop]) -> Vec<Spot> {
    src.iter().map(|x| x.spot.clone()).collect()
}

fn pick_items_include_requires(
    rng: &mut impl Rng,
    items_pool: &mut Vec<Item>,
    spots: &[&Spot],
    count: usize,
) -> Vec<Item> {
    let Some(pos) = items_pool.iter().position(|item| item.is_required(spots)) else {
        return (0..count).map(|_| items_pool.pop().unwrap()).collect();
    };
    let req_item = items_pool.swap_remove(pos);
    let mut field_items = vec![req_item];
    (0..count - 1).for_each(|_| {
        field_items.push(items_pool.pop().unwrap());
    });
    debug_assert!(!field_items.is_empty());
    field_items.as_mut_slice().shuffle(rng);
    field_items
}

type SphereRef<'a> = Vec<(&'a Spot, Vec<Item>)>;

fn sphere<'a>(
    mut rng: impl Rng,
    field_items_pool: &mut Vec<Item>,
    shop_items_pool: &mut Vec<Item>,
    field_item_spots: &mut Vec<&'a Spot>,
    shop_spots: &mut Vec<&'a Spot>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> Option<(SphereRef<'a>, bool)> {
    let spots: Vec<_> = field_item_spots
        .iter()
        .chain(shop_spots.iter())
        .copied()
        .collect();
    let strategy_flag_strings: Vec<_> = strategy_flags.iter().map(|x| x.get().to_owned()).collect();
    let strategy_flag_strs: HashSet<_> = strategy_flag_strings.iter().map(|x| x.as_str()).collect();
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;

    let mut sphere: Vec<_> = Default::default();

    let (reachables_field_item_spots, unreachables_field_item_spots) = field_item_spots
        .iter()
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    let (reachables_shop_spots, unreachable_shop_spots) = shop_spots
        .iter()
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    if reachables_field_item_spots.is_empty() && reachables_shop_spots.is_empty() {
        return None;
    }

    *field_item_spots = unreachables_field_item_spots;
    *shop_spots = unreachable_shop_spots;

    // 少なくとも一つは行動を広げるアイテムを配置する
    let numerator = reachables_field_item_spots.len() as u32;
    let denominator = (reachables_field_item_spots.len() + reachables_shop_spots.len() * 3) as u32;
    let (mut field_items, mut shop_items) = if rng.gen_ratio(numerator, denominator) {
        (
            pick_items_include_requires(
                &mut rng,
                field_items_pool,
                &spots,
                reachables_field_item_spots.len(),
            ),
            shop_items_pool.split_off(shop_items_pool.len() - reachables_shop_spots.len() * 3),
        )
    } else {
        (
            field_items_pool.split_off(field_items_pool.len() - reachables_field_item_spots.len()),
            pick_items_include_requires(
                &mut rng,
                shop_items_pool,
                &spots,
                reachables_shop_spots.len() * 3,
            ),
        )
    };

    reachables_field_item_spots.into_iter().for_each(|spot| {
        let item = field_items.pop().unwrap();
        strategy_flags.insert(item.name().clone());
        sphere.push((spot, vec![item]));
    });
    reachables_shop_spots.into_iter().for_each(|spot| {
        let items = vec![
            shop_items.pop().unwrap(),
            shop_items.pop().unwrap(),
            shop_items.pop().unwrap(),
        ];
        items.iter().for_each(|x| {
            strategy_flags.insert(x.name().clone());
        });
        sphere.push((spot, items));
    });

    if !field_item_spots.is_empty() || !shop_spots.is_empty() {
        return Some((sphere, false));
    }
    Some((sphere, true))
}

fn spoiler(
    seed: u64,
    sellable_items: &[Item],
    unsellable_items: &[Item],
    field_item_spots: &[&Spot],
    shop_spots: &[&Spot],
) -> Option<SpoilerLog> {
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let (mut field_items_pool, mut shop_items_pool) = shuffle_items(
        &mut rng,
        sellable_items.to_owned(),
        unsellable_items.to_owned(),
        shop_spots.len(),
    );

    let mut field_item_spots = field_item_spots.to_owned();
    let mut shop_spots = shop_spots.to_owned();

    let mut strategy_flags: HashSet<StrategyFlag> = Default::default();
    let mut progression = Vec::new();

    for i in 0..100 {
        let Some((sphere, complete)) = sphere(
            &mut rng,
            &mut field_items_pool,
            &mut shop_items_pool,
            &mut field_item_spots,
            &mut shop_spots,
            &mut strategy_flags,
        ) else {
            trace!("Retry (spheres: {}, time: {:?})", i, start.elapsed());
            return None;
        };
        progression.push(sphere);
        if !complete {
            continue;
        }
        info!("Sphere: {}, time: {:?}", i, start.elapsed());
        return Some(SpoilerLog {
            progression: progression
                .into_iter()
                .map(|sphere| {
                    Sphere(
                        sphere
                            .into_iter()
                            .map(|(spot, items)| Checkpoint {
                                spot: spot.to_owned(),
                                items,
                            })
                            .collect(),
                    )
                })
                .collect(),
        });
    }
    unreachable!();
}

fn shuffle(
    source: &Storage,
    sellable_items: Vec<Item>,
    unsellable_items: Vec<Item>,
    rng: &mut impl Rng,
) -> (Storage, SpoilerLog) {
    let mut field_item_spots = to_spots(source.main_weapon_shutters());
    field_item_spots.append(&mut to_spots(source.sub_weapon_shutters()));
    field_item_spots.append(&mut to_spots(source.chests()));
    field_item_spots.append(&mut to_spots(source.seal_chests()));
    let field_item_spots: Vec<_> = field_item_spots.iter().collect();

    let shop_spots = shop_to_spots(source.shops());
    let shop_spots: Vec<_> = shop_spots.iter().collect();

    let thread_count = std::thread::available_parallelism().unwrap().get();

    let spoiler_log = std::thread::scope(|scope| {
        for i in 0..100000 {
            let handles = (0..thread_count)
                .map(|_| {
                    let seed = rng.next_u64();
                    let sellables = &sellable_items;
                    let unsellables = &unsellable_items;
                    let field_item_spots = &field_item_spots;
                    let shop_spots = &shop_spots;
                    scope.spawn(move || {
                        spoiler(seed, sellables, unsellables, field_item_spots, shop_spots)
                    })
                })
                .collect::<Vec<_>>();
            if let Some(spoiler_log) = handles.into_iter().filter_map(|h| h.join().unwrap()).next()
            {
                info!("Shuffle was tried: {} times", (i + 1) * thread_count);
                return spoiler_log;
            }
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

use std::{collections::HashSet, hash::Hash};

use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::dataset::{
    item::{Item, StrategyFlag},
    spot::Spot,
};

use super::spoiler_log::{Checkpoint, Sphere, SpoilerLog};

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

fn shuffle_items(
    rng: &mut impl Rng,
    mut sellable_items: Vec<Item>,
    unsellable_items: Vec<Item>,
    shop_count: usize,
) -> (Vec<Item>, Vec<Item>) {
    sellable_items.as_mut_slice().shuffle(rng);
    let new_shop_items = sellable_items.split_off(sellable_items.len() - shop_count * 3);
    let mut new_field_items = unsellable_items;
    new_field_items.append(&mut sellable_items);
    drop(sellable_items);
    new_field_items.as_mut_slice().shuffle(rng);
    (new_field_items, new_shop_items)
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

fn explore<'a>(
    field_item_spots: &[&'a Spot],
    shop_spots: &[&'a Spot],
    strategy_flags: &HashSet<StrategyFlag>,
) -> (Vec<&'a Spot>, Vec<&'a Spot>, Vec<&'a Spot>, Vec<&'a Spot>) {
    let strategy_flag_strings: Vec<_> = strategy_flags.iter().map(|x| x.get().to_owned()).collect();
    let strategy_flag_strs: HashSet<_> = strategy_flag_strings.iter().map(|x| x.as_str()).collect();
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;

    let (reachables_field_item_spots, unreachables_field_item_spots) = field_item_spots
        .iter()
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    let (reachables_shop_spots, unreachable_shop_spots) = shop_spots
        .iter()
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    (
        reachables_field_item_spots,
        reachables_shop_spots,
        unreachables_field_item_spots,
        unreachable_shop_spots,
    )
}

type SphereRef<'a> = Vec<(&'a Spot, Vec<Item>)>;

fn place_items<'a>(
    rng: &mut impl Rng,
    (field_items_pool, shop_items_pool): (&mut Vec<Item>, &mut Vec<Item>),
    (reachables_field_item_spots, reachables_shop_spots): (Vec<&'a Spot>, Vec<&'a Spot>),
    (unreachable_field_item_spots, unreachable_shop_spots): (&[&'a Spot], &[&'a Spot]),
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> SphereRef<'a> {
    let remaining_spots: Vec<_> = unreachable_field_item_spots
        .iter()
        .chain(unreachable_shop_spots.iter())
        .copied()
        .collect();

    let mut sphere: Vec<_> = Default::default();

    // 少なくとも一つは行動を広げるアイテムを配置する
    let fi_spot_cnt = reachables_field_item_spots.len();
    let shop_spot_cnt = reachables_shop_spots.len() * 3;
    let numerator = fi_spot_cnt as u32;
    let denominator = (fi_spot_cnt + shop_spot_cnt) as u32;
    let (mut field_items, mut shop_items) = if rng.gen_ratio(numerator, denominator) {
        (
            pick_items_include_requires(rng, field_items_pool, &remaining_spots, fi_spot_cnt),
            shop_items_pool.split_off(shop_items_pool.len() - shop_spot_cnt),
        )
    } else {
        (
            field_items_pool.split_off(field_items_pool.len() - fi_spot_cnt),
            pick_items_include_requires(rng, shop_items_pool, &remaining_spots, shop_spot_cnt),
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
    sphere
}

fn sphere<'a>(
    mut rng: impl Rng,
    field_items_pool: &mut Vec<Item>,
    shop_items_pool: &mut Vec<Item>,
    field_item_spots: &mut Vec<&'a Spot>,
    shop_spots: &mut Vec<&'a Spot>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> Option<SphereRef<'a>> {
    let (
        reachables_field_item_spots,
        reachables_shop_spots,
        unreachables_field_item_spots,
        unreachable_shop_spots,
    ) = explore(&*field_item_spots, &*shop_spots, &*strategy_flags);
    if reachables_field_item_spots.is_empty() && reachables_shop_spots.is_empty() {
        return None;
    }

    let sphere = place_items(
        &mut rng,
        (field_items_pool, shop_items_pool),
        (reachables_field_item_spots, reachables_shop_spots),
        (&unreachables_field_item_spots, &unreachable_shop_spots),
        strategy_flags,
    );

    *field_item_spots = unreachables_field_item_spots;
    *shop_spots = unreachable_shop_spots;

    Some(sphere)
}

pub fn spoiler(
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
        let Some(sphere) = sphere(
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

        if !field_item_spots.is_empty() || !shop_spots.is_empty() {
            continue;
        }
        info!("Sphere: {}, time: {:?}", i, start.elapsed());
        let progression = progression
            .into_iter()
            .map(|sphere| {
                sphere
                    .into_iter()
                    .map(|(spot, items)| Checkpoint {
                        spot: spot.to_owned(),
                        items,
                    })
                    .collect()
            })
            .map(Sphere)
            .collect();
        return Some(SpoilerLog { progression });
    }
    unreachable!();
}

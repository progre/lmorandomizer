use std::{collections::HashSet, hash::Hash};

use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::{Item, StrategyFlag},
        spot::Spot,
        storage::Shop,
    },
    randomizer::sphere::{sphere, RemainingSpots},
};

use super::{
    pickup_items::ItemsPool,
    spoiler_log::{Checkpoint, Sphere, SpoilerLog},
};

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

fn shuffle_items(
    rng: &mut impl Rng,
    priority_items: &[Item],
    sellable_items: &[Item],
    unsellable_items: &[Item],
    consumable_items: &[Item],
    shop_display_count: usize,
) -> ItemsPool {
    let mut sellable_items = sellable_items.to_vec();
    sellable_items.as_mut_slice().shuffle(rng);
    let new_shop_items = sellable_items.split_off(sellable_items.len() - shop_display_count);
    let mut new_field_items = sellable_items;
    new_field_items.append(&mut unsellable_items.to_vec());
    new_field_items.as_mut_slice().shuffle(rng);
    let mut consumable_items = consumable_items.to_vec();
    consumable_items.shuffle(rng);
    ItemsPool {
        priority_items: Some(priority_items.to_vec()),
        field_items: new_field_items,
        shop_items: new_shop_items,
        consumable_items,
    }
}

pub fn spoiler(
    seed: u64,
    priority_items: &[Item],
    sellable_items: &[Item],
    unsellable_items: &[Item],
    consumable_items: &[Item],
    field_item_spots: &[&Spot],
    shops: &[&Shop],
) -> Option<SpoilerLog> {
    debug_assert!(priority_items.iter().all(|item| item.can_display_in_shop()));
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let shop_display_count = shops.len() * 3 - consumable_items.len();
    debug_assert_eq!(
        shops.len() * 3 - consumable_items.len(),
        shops
            .iter()
            .map(|shop| shop.count_general_items())
            .sum::<usize>(),
    );
    debug_assert_eq!(
        shops
            .iter()
            .map(|shop| 3 - shop.count_general_items())
            .sum::<usize>(),
        consumable_items.len()
    );
    let mut items_pool = shuffle_items(
        &mut rng,
        priority_items,
        sellable_items,
        unsellable_items,
        consumable_items,
        shop_display_count,
    );

    let mut remaining_spots = RemainingSpots {
        field_item_spots: field_item_spots.to_vec(),
        shops: shops.to_vec(),
    };

    let mut strategy_flags: HashSet<StrategyFlag> = Default::default();
    let mut progression = Vec::new();

    for i in 0..100 {
        let Some(sphere) = sphere(
            &mut rng,
            &mut items_pool,
            &mut remaining_spots,
            &mut strategy_flags,
        ) else {
            trace!("Retry (spheres: {}, time: {:?})", i, start.elapsed());
            return None;
        };
        progression.push(sphere);

        if !remaining_spots.is_empty() {
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

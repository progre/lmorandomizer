use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
    ptr,
};

use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::{
        item::{Item, StrategyFlag},
        spot::{FieldId, Spot},
    },
    randomizer::sphere::sphere,
};

use super::{
    items_spots::{Items, Spots},
    spoiler_log::{Checkpoint, SpoilerLogRef},
};

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

fn maps<'a>(
    rng: &mut impl Rng,
    maps: &BTreeMap<FieldId, &'a Item>,
    spots: &mut Spots<'a>,
) -> Vec<Checkpoint<&'a Spot, &'a Item>> {
    let mut hash_map: BTreeMap<FieldId, Vec<&'a Spot>> = Default::default();
    for spot in &spots.field_item_spots {
        hash_map.entry(spot.field_id()).or_default().push(spot);
    }
    maps.iter()
        .map(|(field_id, item)| {
            let spot = hash_map[field_id].choose(rng).unwrap();
            Checkpoint::<&Spot, &Item> {
                spot,
                idx: 0,
                item: *item,
            }
        })
        .inspect(|checkpoint| {
            let idx = spots
                .field_item_spots
                .iter()
                .position(|&x| ptr::eq(x, checkpoint.spot))
                .unwrap();
            spots.field_item_spots.swap_remove(idx);
        })
        .collect()
}

pub fn spoiler<'a>(seed: u64, items: &Items<'a>, spots: &Spots<'a>) -> Option<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let mut items_pool = items.to_items_pool(&mut rng, spots.shops.len());
    let mut remaining_spots = spots.clone();
    let maps = maps(&mut rng, items.maps(), &mut remaining_spots);

    debug_assert_eq!(
        items.priority_items().len()
            + items.unsellable_items().len()
            + items.consumable_items().len()
            + items.sellable_items().len(),
        remaining_spots.field_item_spots.len() + remaining_spots.shops.len()
    );

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
        return Some(SpoilerLogRef { progression, maps });
    }
    unreachable!();
}

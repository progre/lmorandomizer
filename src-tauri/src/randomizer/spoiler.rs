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

pub use super::sphere::Spots;
use super::{
    items_pool::{ItemsPool, UnorderedItems},
    spoiler_log::{Checkpoint, SpoilerLogRef},
};

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

pub struct Items<'a> {
    pub priority_items: Vec<&'a Item>,
    pub maps: BTreeMap<FieldId, &'a Item>,
    pub unsellable_items: Vec<&'a Item>,
    pub consumable_items: Vec<&'a Item>,
    pub sellable_items: Vec<&'a Item>,
}

impl<'a> Items<'a> {
    fn to_items_pool(&self, rng: &mut impl Rng, shop_display_count: usize) -> ItemsPool<'a> {
        let mut sellable_items = UnorderedItems::new(self.sellable_items.clone()).shuffle(rng);
        let shop_display_count = shop_display_count - self.consumable_items.len();
        let shop_items = sellable_items.split_off(sellable_items.len() - shop_display_count);
        let mut field_items = sellable_items.into_unordered();
        field_items.append(&mut self.unsellable_items.clone());
        ItemsPool {
            priority_items: Some(UnorderedItems::new(self.priority_items.clone())),
            field_items: field_items.shuffle(rng),
            shop_items,
            consumable_items: UnorderedItems::new(self.consumable_items.clone()).shuffle(rng),
        }
    }
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
    let maps = maps(&mut rng, &items.maps, &mut remaining_spots);

    debug_assert_eq!(
        items.priority_items.len()
            + items.unsellable_items.len()
            + items.consumable_items.len()
            + items.sellable_items.len(),
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

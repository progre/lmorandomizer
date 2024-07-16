use std::{collections::HashSet, hash::Hash};

use log::{info, trace};
use rand::Rng;
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::item::{Item, StrategyFlag},
    randomizer::sphere::sphere,
};

pub use super::sphere::Spots;
use super::{
    items_pool::{ItemsPool, UnorderedItems},
    spoiler_log::SpoilerLogRef,
};

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

pub struct Items<'a> {
    pub priority_items: Vec<&'a Item>,
    pub sellable_items: Vec<&'a Item>,
    pub unsellable_items: Vec<&'a Item>,
    pub consumable_items: Vec<&'a Item>,
}

impl<'a> Items<'a> {
    fn to_items_pool(&self, rng: &mut impl Rng, shop_count: usize) -> ItemsPool<'a> {
        let mut sellable_items = UnorderedItems::new(self.sellable_items.clone()).shuffle(rng);
        let shop_display_count = shop_count * 3 - self.consumable_items.len();
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

pub fn spoiler<'a>(seed: u64, items: &Items<'a>, spots: &Spots<'a>) -> Option<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let mut items_pool = items.to_items_pool(&mut rng, spots.shops.len());
    let mut remaining_spots = spots.clone();
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
        return Some(SpoilerLogRef { progression });
    }
    unreachable!();
}

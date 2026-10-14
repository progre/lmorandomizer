mod consts;
pub mod items;
mod items_pool;
pub mod locations;
mod sphere;

use std::{collections::BTreeMap, hash::Hash, sync::LazyLock};

use locations::LocationRef;
use log::{info, trace};
use rand::{Rng, prelude::IndexedRandom};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    dataset::regions::Regions, randomizer::spoiler::sphere::State, script::enums::FieldNumber,
};

use super::{
    RandomizeOptions,
    spoiler_log::{CheckpointRef, SpoilerLogRef},
    storage::item::{Item, StrategyFlag},
};

use {items::Items, locations::Locations, sphere::sphere};

static GLITCH: LazyLock<StrategyFlag> = LazyLock::new(|| StrategyFlag::new("option:glitch".into()));

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    let mut seeder = Seeder::from(seed);
    seeder.into_rng()
}

fn maps<'a>(
    rng: &mut impl Rng,
    maps: &BTreeMap<FieldNumber, &'a Item>,
    locations: &mut Locations<'a>,
    all_regions: &Regions,
) -> Vec<CheckpointRef<'a>> {
    let mut location_hash_map: BTreeMap<FieldNumber, Vec<LocationRef<'a>>> = Default::default();
    let mut twin_labrynths_locations = Vec::new();
    for location in &locations.field_item_locations {
        let field_number = all_regions.get(location.region()).field_number();
        if matches!(
            field_number,
            FieldNumber::TwinLabyrinthsLeft | FieldNumber::TwinLabyrinthsRight
        ) {
            twin_labrynths_locations.push(*location);
            continue;
        }
        location_hash_map
            .entry(field_number)
            .or_default()
            .push(*location);
    }
    maps.iter()
        .map(|(field_number, item)| {
            if *field_number == FieldNumber::TwinLabyrinthsLeft {
                let location = *twin_labrynths_locations.choose(rng).unwrap();
                return (location, *item);
            }
            let location = *location_hash_map[field_number].choose(rng).unwrap();
            (location, *item)
        })
        .inspect(|(location, _)| {
            let idx = locations
                .field_item_locations
                .iter()
                .position(|x| x.id() == location.id())
                .unwrap();
            locations.field_item_locations.swap_remove(idx);
        })
        .map(|(location, item)| CheckpointRef::Item { location, item })
        .collect()
}

pub fn spoiler<'a>(
    seed: u64,
    options: &RandomizeOptions,
    all_regions: &'a Regions,
    items: &Items<'a>,
    locations: &Locations<'a>,
) -> Option<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let mut items_pool = items.to_items_pool(
        &mut rng,
        locations.talk_locations.len(),
        locations.shops.len(),
    );
    let mut remaining_locations = locations.clone();
    let maps = maps(
        &mut rng,
        items.maps(),
        &mut remaining_locations,
        all_regions,
    );

    let mut state = State::new(
        all_regions
            .iter()
            .find(|x| matches!(x.name().get(), "surface/main"))
            .unwrap(),
    );
    let mut progression = Vec::new();

    if options.need_glitches {
        state.insert_flag(&GLITCH);
    }

    for i in 0..100 {
        let Some(sphere) = sphere(
            &mut rng,
            &mut items_pool,
            &mut remaining_locations,
            &mut state,
            all_regions,
        ) else {
            let reachable_names: std::collections::HashSet<_> =
                state.reachable_regions().map(|r| r.name().get()).collect();
            let unreachable_regions: Vec<_> = all_regions
                .iter()
                .filter(|r| !reachable_names.contains(r.name().get()))
                .map(|r| r.name().get())
                .collect();
            trace!(
                "Retry (spheres: {}, time: {:?}) unreachable regions: {:?}",
                i,
                start.elapsed(),
                unreachable_regions
            );
            return None;
        };
        progression.push(sphere);

        if !remaining_locations.is_empty() {
            debug_assert_eq!(
                remaining_locations.field_item_locations.len(),
                items_pool.field_items.len(),
            );
            debug_assert_eq!(
                remaining_locations.talk_locations.len(),
                items_pool.talk_items.len(),
            );
            debug_assert_eq!(
                remaining_locations.shops.len(),
                items_pool.shop_items.len() + items_pool.consumable_items.len(),
            );
            continue;
        }
        info!("Sphere: {}, time: {:?}", i, start.elapsed());
        return Some(SpoilerLogRef { progression, maps });
    }
    unreachable!();
}

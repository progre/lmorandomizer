pub mod items;
mod items_pool;
mod sphere;
pub mod spots;

use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
    ptr,
    sync::LazyLock,
};

use log::{info, trace};
use rand::{seq::SliceRandom, Rng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256PlusPlus;
use spots::SpotRef;

use crate::script::enums::FieldNumber;

use super::{
    spoiler_log::{CheckpointRef, SpoilerLogRef},
    storage::item::{Item, StrategyFlag},
    RandomizeOptions,
};

use {items::Items, sphere::sphere, spots::Spots};

static GLITCH: LazyLock<StrategyFlag> = LazyLock::new(|| StrategyFlag::new("option:glitch".into()));

pub fn make_rng<H: Hash>(seed: H) -> Xoshiro256PlusPlus {
    Seeder::from(seed).make_rng()
}

fn ptr_eq<'a>(a: SpotRef<'a>, b: &CheckpointRef<'a>) -> bool {
    match (a, b) {
        (SpotRef::MainWeapon(a), CheckpointRef::MainWeapon(b)) => ptr::eq(a, b.spot),
        (SpotRef::SubWeapon(a), CheckpointRef::SubWeapon(b)) => ptr::eq(a, b.spot),
        (SpotRef::Chest(a), CheckpointRef::Chest(b)) => ptr::eq(a, b.spot),
        (SpotRef::Seal(a), CheckpointRef::Seal(b)) => ptr::eq(a, b.spot),
        (SpotRef::Rom(a), CheckpointRef::Rom(b)) => ptr::eq(a, b.spot),
        (SpotRef::Talk(a), CheckpointRef::Talk(b)) => ptr::eq(a, b.spot),
        (SpotRef::Shop(a), CheckpointRef::Shop(b)) => ptr::eq(a, b.spot),
        _ => false,
    }
}

fn maps<'a>(
    rng: &mut impl Rng,
    maps: &BTreeMap<FieldNumber, &'a Item>,
    spots: &mut Spots<'a>,
) -> Vec<CheckpointRef<'a>> {
    let mut spot_hash_map: BTreeMap<FieldNumber, Vec<SpotRef<'a>>> = Default::default();
    let mut twin_labrynths_spots = Vec::new();
    for spot in &spots.field_item_spots {
        if matches!(
            spot.field_number(),
            FieldNumber::TwinLabyrinthsLeft | FieldNumber::TwinLabyrinthsRight
        ) {
            twin_labrynths_spots.push(*spot);
            continue;
        }
        spot_hash_map
            .entry(spot.field_number())
            .or_default()
            .push(*spot);
    }
    maps.iter()
        .map(|(field_number, item)| {
            if *field_number == FieldNumber::TwinLabyrinthsLeft {
                let spot = *twin_labrynths_spots.choose(rng).unwrap();
                return CheckpointRef::from_field_spot_item(spot, item);
            }
            let spot = spot_hash_map[field_number].choose(rng).unwrap();
            CheckpointRef::from_field_spot_item(*spot, item)
        })
        .inspect(|checkpoint| {
            let idx = spots
                .field_item_spots
                .iter()
                .position(|&x| ptr_eq(x, checkpoint))
                .unwrap();
            spots.field_item_spots.swap_remove(idx);
        })
        .collect()
}

pub fn spoiler<'a>(
    seed: u64,
    options: &RandomizeOptions,
    items: &Items<'a>,
    spots: &Spots<'a>,
) -> Option<SpoilerLogRef<'a>> {
    let start = std::time::Instant::now();
    let mut rng = make_rng(seed);
    let mut items_pool = items.to_items_pool(&mut rng, spots.talk_spots.len(), spots.shops.len());
    let mut remaining_spots = spots.clone();
    let maps = maps(&mut rng, items.maps(), &mut remaining_spots);

    let mut strategy_flags: HashSet<&'a StrategyFlag> = Default::default();
    let mut progression = Vec::new();

    if options.need_glitches {
        strategy_flags.insert(&GLITCH);
    }

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
            debug_assert_eq!(
                remaining_spots.field_item_spots.len(),
                items_pool.field_items.len(),
            );
            debug_assert_eq!(
                remaining_spots.talk_spots.len(),
                items_pool.talk_items.len(),
            );
            debug_assert_eq!(
                remaining_spots.shops.len(),
                items_pool.shop_items.len() + items_pool.consumable_items.len(),
            );
            continue;
        }
        info!("Sphere: {}, time: {:?}", i, start.elapsed());
        return Some(SpoilerLogRef { progression, maps });
    }
    unreachable!();
}

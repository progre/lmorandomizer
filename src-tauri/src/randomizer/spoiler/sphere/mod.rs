mod events;
mod explore;
mod place_items;
mod pre_sphere;
mod progression;
mod state;

use std::ops::Deref;

use events::achieve_events;
use explore::explore;
use place_items::place_items;
use pre_sphere::pre_sphere;
use progression::progression_flags;
use rand::Rng;

use crate::{
    dataset::spot::ShopSpot,
    randomizer::{
        spoiler::regions::Regions,
        spoiler_log::{CheckpointRef, SphereRef},
        storage::item::StrategyFlag,
    },
};

use super::{items_pool::ItemsPool, spots::Spots};

pub use state::State;

#[derive(Clone, Debug)]
pub struct ShopItemDisplay<'a> {
    pub spot: &'a ShopSpot,
    pub idx: usize,
    pub name: &'a StrategyFlag,
}

fn item_shop_spot_count(spots: &Spots<'_>) -> usize {
    spots
        .shops
        .iter()
        .map(|shop| (!shop.name.is_consumable()) as usize)
        .sum::<usize>()
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool<'a>,
    remaining_spots: &mut Spots<'a>,
    state: &mut State<'a>,
    all_regions: &Regions<'a>,
) -> Option<SphereRef<'a>> {
    debug_assert_eq!(
        items_pool.shop_items.len() + items_pool.consumable_items.len(),
        remaining_spots.shops.len()
    );

    state.explore_regions(all_regions);

    if let Some(priority_items) = items_pool.priority_items.take() {
        let sphere = pre_sphere(rng, priority_items, remaining_spots, state);
        let shop_count = sphere
            .iter()
            .filter(|x| match x {
                CheckpointRef::MainWeapon(_)
                | CheckpointRef::SubWeapon(_)
                | CheckpointRef::Chest(_)
                | CheckpointRef::Seal(_)
                | CheckpointRef::Rom(_)
                | CheckpointRef::Talk(_)
                | CheckpointRef::Event(_) => false,
                CheckpointRef::Shop(_) => true,
            })
            .count();
        items_pool.move_shop_items_to_field_items(rng, shop_count);
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
        return Some(sphere);
    }
    let (reachables, unreachables) = explore(remaining_spots.deref(), state.deref());

    if reachables.is_empty() {
        return None;
    }

    // 「進行候補を少なくとも1つ含む」という条件付きで一様に item 群を選ぶ
    let progression = progression_flags(items_pool, state, &unreachables, all_regions);
    let (field_items, talk_items, shop_items) = items_pool.pick_items_with_retry(
        rng,
        reachables.field_item_spots.len(),
        reachables.talk_spots.len(),
        item_shop_spot_count(&reachables),
        |item| progression.contains(&item.name),
    )?;

    let mut sphere = place_items(
        rng,
        field_items,
        talk_items,
        shop_items,
        &mut items_pool.consumable_items,
        reachables,
    )?;
    state.append_flags(&sphere);
    *remaining_spots = unreachables;

    let checkpoints = achieve_events(&mut remaining_spots.events, state, all_regions);
    sphere.append_checkpoints(checkpoints.into_iter().map(CheckpointRef::Event).collect());

    Some(sphere)
}

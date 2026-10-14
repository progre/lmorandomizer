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
    dataset::regions::Regions,
    randomizer::{
        placement::Sphere,
        storage::{
            ids::{ItemId, SpotId},
            item::Item,
        },
    },
};

use super::{
    items_pool::ItemsPool,
    spots::{SpotRef, Spots},
};

pub use state::State;

fn to_ids(placed: &[(SpotRef, &Item)]) -> Vec<(SpotId, ItemId)> {
    placed
        .iter()
        .map(|(spot, item)| (spot.id(), item.id))
        .collect()
}

fn item_shop_spot_count(spots: &Spots<'_>) -> usize {
    spots
        .shops
        .iter()
        .map(|shop| (!shop.item.name.is_consumable()) as usize)
        .sum::<usize>()
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool<'a>,
    remaining_spots: &mut Spots<'a>,
    state: &mut State<'a>,
    all_regions: &'a Regions,
) -> Option<Sphere<'a>> {
    debug_assert_eq!(
        items_pool.shop_items.len() + items_pool.consumable_items.len(),
        remaining_spots.shops.len()
    );

    state.explore_regions(all_regions);

    if let Some(priority_items) = items_pool.priority_items.take() {
        let placed = pre_sphere(rng, priority_items, remaining_spots, state, all_regions);
        let shop_count = placed
            .iter()
            .filter(|(spot, _)| matches!(spot, SpotRef::Shop(_)))
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
        let items = to_ids(&placed);
        return Some(Sphere {
            items,
            events: vec![],
        });
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

    let placed = place_items(
        rng,
        field_items,
        talk_items,
        shop_items,
        &mut items_pool.consumable_items,
        reachables,
    )?;
    state.append_flags(placed.iter().map(|(_, item)| &item.name));
    *remaining_spots = unreachables;

    let events = achieve_events(&mut remaining_spots.events, state, all_regions);
    let items = to_ids(&placed);
    Some(Sphere { items, events })
}

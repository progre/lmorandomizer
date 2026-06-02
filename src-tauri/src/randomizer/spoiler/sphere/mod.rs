mod pre_sphere;
mod state;

use std::{collections::HashSet, mem::take, ops::Deref};

use pre_sphere::pre_sphere;
use rand::Rng;

use crate::{
    dataset::spot::{Region, ShopSpot},
    randomizer::{
        spoiler::regions::Regions,
        spoiler_log::{CheckpointRef, SphereRef},
        storage::{Event, ShopRef, item::StrategyFlag},
    },
};

use super::{
    items_pool::{ItemsPool, ShuffledItems, UnorderedItems},
    spots::{SpotRef, Spots},
};

pub use state::State;

#[derive(Clone, Debug)]
pub struct ShopItemDisplay<'a> {
    pub spot: &'a ShopSpot,
    pub idx: usize,
    pub name: &'a StrategyFlag,
}

fn explore<'a>(remaining_spots: &Spots<'a>, state: &State<'a>) -> (Spots<'a>, Spots<'a>) {
    let (reachables_field_item_spots, unreachables_field_item_spots) = remaining_spots
        .field_item_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_reachable(x.region(), x.requirements()));
    let (reachables_talk_spots, unreachables_talk_spots) = remaining_spots
        .talk_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_reachable(x.region(), x.requirements()));
    let (reachables_shops, unreachable_shops) = remaining_spots
        .shops
        .iter()
        .cloned()
        .partition::<Vec<_>, _>(|shop| {
            state.is_reachable(shop.spot.region(), shop.spot.requirements())
        });

    let reachables = Spots {
        field_item_spots: reachables_field_item_spots,
        talk_spots: reachables_talk_spots,
        shops: reachables_shops,
        events: vec![],
    };
    let unreachables = Spots {
        field_item_spots: unreachables_field_item_spots,
        talk_spots: unreachables_talk_spots,
        shops: unreachable_shops,
        events: remaining_spots.events.clone(),
    };
    (reachables, unreachables)
}

fn place_items<'a>(
    rng: &mut impl Rng,
    mut field_items: ShuffledItems<'a>,
    mut talk_items: ShuffledItems<'a>,
    mut shop_items: ShuffledItems<'a>,
    consumable_items_pool: &mut UnorderedItems<'a>,
    reachable_regions: Vec<&'a Region>,
    reachables: Spots<'a>,
) -> Option<SphereRef<'a>> {
    let mut sphere: Vec<_> = Default::default();
    reachables
        .field_item_spots
        .into_iter()
        .for_each(|spot| match spot {
            SpotRef::MainWeapon(_)
            | SpotRef::SubWeapon(_)
            | SpotRef::Chest(_)
            | SpotRef::Seal(_)
            | SpotRef::Rom(_) => {
                let item = field_items.pop().unwrap();
                sphere.push(CheckpointRef::from_field_spot_item(spot, item));
            }
            SpotRef::Talk(_) | SpotRef::Shop(_) => unreachable!(),
        });
    reachables.talk_spots.into_iter().for_each(|spot| {
        let item = talk_items.pop().unwrap();
        let spot = SpotRef::Talk(spot);
        sphere.push(CheckpointRef::from_field_spot_item(spot, item));
    });
    for shop in reachables.shops {
        let item = if shop.name.is_consumable() {
            // Do not sell the same item to the same store
            let placed_items: HashSet<_> = sphere
                .iter()
                .filter_map(|x| match x {
                    CheckpointRef::Shop(x) => Some(x),
                    CheckpointRef::MainWeapon(_)
                    | CheckpointRef::SubWeapon(_)
                    | CheckpointRef::Chest(_)
                    | CheckpointRef::Seal(_)
                    | CheckpointRef::Rom(_)
                    | CheckpointRef::Talk(_)
                    | CheckpointRef::Event(_) => None,
                })
                .filter(|x| x.spot.items() == shop.spot.items())
                .map(|x| &x.item.name)
                .collect();
            let mut items_pool = take(consumable_items_pool).shuffle(rng).into_inner();
            let idx = items_pool
                .iter()
                .position(|x| !placed_items.contains(&&x.name))?;
            let item = items_pool.swap_remove(idx);
            *consumable_items_pool = UnorderedItems::new(items_pool);
            item
        } else {
            shop_items.pop().unwrap()
        };
        let spot = &shop.spot;
        let idx = shop.idx;
        sphere.push(CheckpointRef::Shop(ShopRef { spot, idx, item }));
    }
    Some(SphereRef::new(reachable_regions, sphere))
}

fn take_achieved<'a>(events: &mut Vec<&'a Event>, state: &State) -> Vec<&'a Event> {
    let (achieved, unachieved) = take(events).into_iter().partition(|event| {
        if let Some(region) = &event.region {
            state.is_reachable(region, Some(&event.requirements))
        } else {
            state.is_reachable_without_region(Some(&event.requirements))
        }
    });
    *events = unachieved;
    achieved
}

fn achieve_events<'a>(
    events: &mut Vec<&'a Event>,
    state: &mut State<'a>,
    all_regions: &Regions<'a>,
) -> Vec<&'a StrategyFlag> {
    let mut checkpoints = vec![];
    while !events.is_empty() {
        state.explore_regions(all_regions);
        let achieved = take_achieved(events, state);
        if achieved.is_empty() {
            return checkpoints;
        }
        achieved.into_iter().for_each(|event| {
            state.insert_flag(&event.name);
            checkpoints.push(&event.name);
        });
    }
    checkpoints
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

    let (field_items, talk_items, shop_items) =
        items_pool.pick_items_randomly(rng, &reachables, &unreachables);

    let mut sphere = place_items(
        rng,
        field_items,
        talk_items,
        shop_items,
        &mut items_pool.consumable_items,
        state.reachable_regions().collect(),
        reachables,
    )?;
    state.append_flags(&sphere);
    *remaining_spots = unreachables;

    let checkpoints = achieve_events(&mut remaining_spots.events, state, all_regions);
    sphere.append_checkpoints(checkpoints.into_iter().map(CheckpointRef::Event).collect());

    Some(sphere)
}

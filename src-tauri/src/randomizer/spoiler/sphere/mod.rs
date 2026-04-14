mod pre_sphere;

use std::{collections::HashSet, mem::take, ops::Deref};

use pre_sphere::pre_sphere;
use rand::Rng;

use crate::{
    dataset::spot::{AnyOfAllRequirements, ShopSpot},
    randomizer::{
        spoiler_log::{CheckpointRef, SphereRef},
        storage::{item::StrategyFlag, Event, ShopRef},
    },
};

use super::{
    items_pool::{ItemsPool, ShuffledItems, UnorderedItems},
    spots::{SpotRef, Spots},
};

#[derive(Clone, Debug)]
pub struct ShopItemDisplay<'a> {
    pub spot: &'a ShopSpot,
    pub idx: usize,
    pub name: &'a StrategyFlag,
}

fn is_reachable(
    requirements: Option<&AnyOfAllRequirements>,
    current_strategy_flags: &HashSet<&str>,
    sacred_orb_count: u8,
) -> bool {
    let Some(any) = requirements else {
        return true;
    };
    any.0.iter().any(|all| {
        all.0.iter().all(|x| {
            x.is_sacred_orb() && x.sacred_orb_count() <= sacred_orb_count
                || current_strategy_flags.contains(x.get())
        })
    })
}

fn explore<'a>(
    remaining_spots: &Spots<'a>,
    strategy_flags: &HashSet<&'a StrategyFlag>,
) -> (Spots<'a>, Spots<'a>) {
    let strategy_flag_strings: Vec<_> = strategy_flags.iter().map(|x| x.get().to_owned()).collect();
    let strategy_flag_strs: HashSet<_> = strategy_flag_strings.iter().map(|x| x.as_str()).collect();
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;

    let (reachables_field_item_spots, unreachables_field_item_spots) = remaining_spots
        .field_item_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| {
            is_reachable(x.requirements(), &strategy_flag_strs, sacred_orb_count)
        });
    let (reachables_talk_spots, unreachables_talk_spots) = remaining_spots
        .talk_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| {
            is_reachable(x.requirements(), &strategy_flag_strs, sacred_orb_count)
        });
    let (reachables_shops, unreachable_shops) = remaining_spots
        .shops
        .iter()
        .cloned()
        .partition::<Vec<_>, _>(|shop| {
            is_reachable(
                shop.spot.requirements(),
                &strategy_flag_strs,
                sacred_orb_count,
            )
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
    Some(SphereRef(sphere))
}

fn append_flags<'a>(strategy_flags: &mut HashSet<&'a StrategyFlag>, sphere: &SphereRef<'a>) {
    for checkpoint in &sphere.0 {
        match checkpoint {
            CheckpointRef::MainWeapon(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::SubWeapon(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Chest(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Seal(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Shop(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Rom(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Talk(checkpoint) => {
                strategy_flags.insert(&checkpoint.item.name);
            }
            CheckpointRef::Event(flag) => {
                strategy_flags.insert(flag);
            }
        }
    }
}

fn take_achieved<'a>(
    events: &mut Vec<&'a Event>,
    strategy_flags: &HashSet<&'a StrategyFlag>,
    sacred_orb_count: u8,
) -> Vec<&'a Event> {
    let current_strategy_flags: HashSet<_> = strategy_flags.iter().map(|x| x.get()).collect();
    let (achieved, unachieved) = take(events).into_iter().partition(|event| {
        is_reachable(
            Some(&event.requirements),
            &current_strategy_flags,
            sacred_orb_count,
        )
    });
    *events = unachieved;
    achieved
}

fn achieve_events<'a>(
    events: &mut Vec<&'a Event>,
    strategy_flags: &mut HashSet<&'a StrategyFlag>,
) -> Vec<&'a StrategyFlag> {
    let mut checkpoints = vec![];
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;
    while !events.is_empty() {
        let achieved = take_achieved(events, strategy_flags, sacred_orb_count);
        if achieved.is_empty() {
            return checkpoints;
        }
        achieved.into_iter().for_each(|event| {
            strategy_flags.insert(&event.name);
            checkpoints.push(&event.name);
        });
    }
    checkpoints
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool<'a>,
    remaining_spots: &mut Spots<'a>,
    strategy_flags: &mut HashSet<&'a StrategyFlag>,
) -> Option<SphereRef<'a>> {
    debug_assert_eq!(
        items_pool.shop_items.len() + items_pool.consumable_items.len(),
        remaining_spots.shops.len()
    );

    if let Some(priority_items) = items_pool.priority_items.take() {
        let sphere = pre_sphere(rng, priority_items, remaining_spots, strategy_flags);
        let shop_count = sphere
            .0
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
    let (reachables, unreachables) = explore(remaining_spots.deref(), strategy_flags);

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
        reachables,
    )?;
    append_flags(strategy_flags, &sphere);
    *remaining_spots = unreachables;

    let checkpoints = achieve_events(&mut remaining_spots.events, strategy_flags);
    sphere
        .0
        .append(&mut checkpoints.into_iter().map(CheckpointRef::Event).collect());

    Some(sphere)
}

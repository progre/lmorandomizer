use std::{collections::HashSet, mem::take, ops::Deref};

use rand::Rng;

use crate::{
    dataset::spot::{AnyOfAllRequirements, ShopSpot},
    randomizer::{
        spoiler_log::{CheckpointRef, SphereRef},
        storage::{
            item::StrategyFlag,
            {Event, ShopRef},
        },
    },
};

use super::{
    items_pool::{ItemsPool, ShuffledItems},
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
    remainging_spots: &Spots<'a>,
    strategy_flags: &HashSet<&'a StrategyFlag>,
) -> (Spots<'a>, Spots<'a>) {
    let strategy_flag_strings: Vec<_> = strategy_flags.iter().map(|x| x.get().to_owned()).collect();
    let strategy_flag_strs: HashSet<_> = strategy_flag_strings.iter().map(|x| x.as_str()).collect();
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;

    let (reachables_field_item_spots, unreachables_field_item_spots) = remainging_spots
        .field_item_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| {
            is_reachable(x.requirements(), &strategy_flag_strs, sacred_orb_count)
        });
    let (reachables_talk_spots, unreachables_talk_spots) = remainging_spots
        .talk_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| {
            is_reachable(x.requirements(), &strategy_flag_strs, sacred_orb_count)
        });
    let (reachables_shops, unreachable_shops) = remainging_spots
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
        events: remainging_spots.events.clone(),
    };
    (reachables, unreachables)
}

fn place_items<'a>(
    mut field_items: ShuffledItems<'a>,
    mut talk_items: ShuffledItems<'a>,
    mut shop_items: ShuffledItems<'a>,
    consumable_items_pool: &mut ShuffledItems<'a>,
    reachables: Spots<'a>,
) -> SphereRef<'a> {
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
    reachables.shops.into_iter().for_each(|shop| {
        let item = if shop.name.is_consumable() {
            consumable_items_pool.pop().unwrap()
        } else {
            shop_items.pop().unwrap()
        };
        let spot = &shop.spot;
        let idx = shop.idx;
        sphere.push(CheckpointRef::Shop(ShopRef { spot, idx, item }));
    });
    SphereRef(sphere)
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

    let (reachables, unreachables) = explore(remaining_spots.deref(), strategy_flags);

    if reachables.is_empty() {
        return None;
    }

    let (field_items, talk_items, shop_items) =
        items_pool.pick_items_randomly(rng, &reachables, &unreachables);

    let mut sphere = place_items(
        field_items,
        talk_items,
        shop_items,
        &mut items_pool.consumable_items,
        reachables,
    );
    append_flags(strategy_flags, &sphere);
    *remaining_spots = unreachables;

    let checkpoints = achieve_events(&mut remaining_spots.events, strategy_flags);
    sphere
        .0
        .append(&mut checkpoints.into_iter().map(CheckpointRef::Event).collect());

    Some(sphere)
}

use std::{
    collections::{BTreeMap, HashSet},
    ops::Deref,
};

use rand::Rng;

use crate::dataset::{
    item::StrategyFlag,
    spot::{AnyOfAllRequirements, ShopSpot},
    storage::ShopRef,
};

use super::{
    items_pool::{ItemsPool, ShuffledItems},
    items_spots::Spots,
    spoiler_log::{CheckpointRef, SphereRef},
};

#[derive(Clone)]
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
    strategy_flags: &HashSet<StrategyFlag>,
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
        shops: reachables_shops,
    };
    let unreachables = Spots {
        field_item_spots: unreachables_field_item_spots,
        shops: unreachable_shops,
    };
    (reachables, unreachables)
}

fn place_items<'a>(
    mut field_items: ShuffledItems<'a>,
    mut shop_items: ShuffledItems<'a>,
    consumable_items_pool: &mut ShuffledItems<'a>,
    reachables: Spots<'a>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> SphereRef<'a> {
    let mut sphere: Vec<_> = Default::default();
    reachables.field_item_spots.into_iter().for_each(|spot| {
        let item = field_items.pop().unwrap();
        strategy_flags.insert(item.name.clone());
        sphere.push(CheckpointRef::from_field_spot_item(spot, item));
    });
    let shops = reachables
        .shops
        .into_iter()
        .fold(BTreeMap::<_, Vec<_>>::new(), |mut map, shop| {
            map.entry(shop.spot as *const ShopSpot)
                .or_default()
                .push(shop);
            map
        });
    for mut shops in shops.into_values() {
        shops.sort_by_key(|x| x.idx);
        let spot = shops[0].spot;
        let items: Vec<_> = shops
            .into_iter()
            .map(|shop| {
                if shop.name.is_consumable() {
                    consumable_items_pool.pop().unwrap()
                } else {
                    shop_items.pop().unwrap()
                }
            })
            .collect();
        for item in &items {
            strategy_flags.insert(item.name.clone());
        }
        sphere.push(CheckpointRef::Shop(ShopRef {
            spot,
            items: (items[0], items[1], items[2]),
        }));
    }
    SphereRef(sphere)
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool<'a>,
    remaining_spots: &mut Spots<'a>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> Option<SphereRef<'a>> {
    debug_assert_eq!(
        items_pool.priority_items.as_ref().map_or(0, |x| x.len())
            + items_pool.field_items.len()
            + items_pool.shop_items.len()
            + items_pool.consumable_items.len(),
        remaining_spots.field_item_spots.len() + remaining_spots.shops.len()
    );

    let (reachables, unreachables) = explore(remaining_spots.deref(), strategy_flags.deref());

    if reachables.is_empty() {
        return None;
    }

    let (field_items, shop_items) = items_pool.pick_items_randomly(rng, &reachables, &unreachables);

    let sphere = place_items(
        field_items,
        shop_items,
        &mut items_pool.consumable_items,
        reachables,
        strategy_flags,
    );

    *remaining_spots = unreachables;

    Some(sphere)
}

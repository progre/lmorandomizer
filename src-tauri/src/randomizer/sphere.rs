use std::{collections::HashSet, ops::Deref};

use rand::Rng;

use crate::dataset::{
    item::{Item, StrategyFlag},
    spot::Spot,
    storage::Shop,
};

use super::pickup_items::{pickup_items, ItemsPool};

fn explore<'a>(
    remainging_spots: &RemainingSpots<'a>,
    strategy_flags: &HashSet<StrategyFlag>,
) -> (Vec<&'a Spot>, Vec<&'a Shop>, Vec<&'a Spot>, Vec<&'a Shop>) {
    let strategy_flag_strings: Vec<_> = strategy_flags.iter().map(|x| x.get().to_owned()).collect();
    let strategy_flag_strs: HashSet<_> = strategy_flag_strings.iter().map(|x| x.as_str()).collect();
    let sacred_orb_count = strategy_flags.iter().filter(|x| x.is_sacred_orb()).count() as u8;

    let (reachables_field_item_spots, unreachables_field_item_spots) = remainging_spots
        .field_item_spots
        .iter()
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    let (reachables_shops, unreachable_shops) = remainging_spots
        .shops
        .iter()
        .partition::<Vec<_>, _>(|x| x.spot.is_reachable(&strategy_flag_strs, sacred_orb_count));
    (
        reachables_field_item_spots,
        reachables_shops,
        unreachables_field_item_spots,
        unreachable_shops,
    )
}

type SphereRef<'a> = Vec<(&'a Spot, Vec<Item>)>;

fn place_items<'a>(
    mut field_items: Vec<Item>,
    mut shop_items: Vec<Item>,
    consumable_items_pool: &mut Vec<Item>,
    (reachables_field_item_spots, reachables_shops): (Vec<&'a Spot>, Vec<&'a Shop>),
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> SphereRef<'a> {
    let mut sphere: Vec<_> = Default::default();
    reachables_field_item_spots.into_iter().for_each(|spot| {
        let item = field_items.pop().unwrap();
        strategy_flags.insert(item.name.clone());
        sphere.push((spot, vec![item]));
    });
    reachables_shops.into_iter().for_each(|shop| {
        let items: Vec<_> = [&shop.items.0, &shop.items.1, &shop.items.2]
            .into_iter()
            .map(|item| {
                if item.name.is_consumable() {
                    consumable_items_pool.pop().unwrap()
                } else {
                    shop_items.pop().unwrap()
                }
            })
            .inspect(|item| {
                strategy_flags.insert(item.name.clone());
            })
            .collect();
        sphere.push((&shop.spot, items));
    });
    sphere
}

fn is_empty_shop_displays(shops: &[&Shop]) -> bool {
    shops.iter().all(|shop| shop.count_general_items() == 0)
}

pub struct RemainingSpots<'a> {
    pub field_item_spots: Vec<&'a Spot>,
    pub shops: Vec<&'a Shop>,
}

impl RemainingSpots<'_> {
    pub fn is_empty(&self) -> bool {
        self.field_item_spots.is_empty() && is_empty_shop_displays(&self.shops)
    }
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool,
    remainging_spots: &mut RemainingSpots<'a>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> Option<SphereRef<'a>> {
    let (
        reachable_field_item_spots,
        reachable_shops,
        unreachable_field_item_spots,
        unreachable_shops,
    ) = explore(remainging_spots.deref(), strategy_flags.deref());
    if reachable_field_item_spots.is_empty() && is_empty_shop_displays(&reachable_shops) {
        return None;
    }

    let (field_items, shop_items) = pickup_items(
        rng,
        items_pool,
        (&reachable_field_item_spots, &reachable_shops),
        (&unreachable_field_item_spots, &unreachable_shops),
    );

    let sphere = place_items(
        field_items,
        shop_items,
        &mut items_pool.consumable_items,
        (reachable_field_item_spots, reachable_shops),
        strategy_flags,
    );

    remainging_spots.field_item_spots = unreachable_field_item_spots;
    remainging_spots.shops = unreachable_shops;

    Some(sphere)
}

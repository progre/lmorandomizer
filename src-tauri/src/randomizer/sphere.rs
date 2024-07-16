use std::{collections::HashSet, ops::Deref};

use rand::Rng;

use crate::dataset::{
    item::{Item, StrategyFlag},
    spot::Spot,
    storage::Shop,
};

use super::{
    items_pool::{ItemsPool, ShuffledItems},
    spoiler_log::{Checkpoint, Sphere},
};

fn is_empty_shop_displays(shops: &[&Shop]) -> bool {
    shops.iter().all(|shop| shop.count_general_items() == 0)
}

#[derive(Clone)]
pub struct Spots<'a> {
    pub field_item_spots: Vec<&'a Spot>,
    pub shops: Vec<&'a Shop>,
}

impl Spots<'_> {
    pub fn is_empty(&self) -> bool {
        self.field_item_spots.is_empty() && is_empty_shop_displays(&self.shops)
    }
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
        .partition::<Vec<_>, _>(|x| x.is_reachable(&strategy_flag_strs, sacred_orb_count));
    let (reachables_shops, unreachable_shops) = remainging_spots
        .shops
        .iter()
        .partition::<Vec<_>, _>(|x| x.spot.is_reachable(&strategy_flag_strs, sacred_orb_count));

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
) -> Sphere<&'a Spot, &'a Item> {
    let mut sphere: Vec<_> = Default::default();
    reachables.field_item_spots.into_iter().for_each(|spot| {
        let item = field_items.pop().unwrap();
        strategy_flags.insert(item.name.clone());
        sphere.push(Checkpoint {
            spot,
            items: vec![item],
        });
    });
    reachables.shops.into_iter().for_each(|shop| {
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
        sphere.push(Checkpoint {
            spot: &shop.spot,
            items,
        });
    });
    Sphere(sphere)
}

pub fn sphere<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ItemsPool<'a>,
    remainging_spots: &mut Spots<'a>,
    strategy_flags: &mut HashSet<StrategyFlag>,
) -> Option<Sphere<&'a Spot, &'a Item>> {
    let (reachables, unreachables) = explore(remainging_spots.deref(), strategy_flags.deref());
    if reachables.field_item_spots.is_empty() && is_empty_shop_displays(&reachables.shops) {
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

    *remainging_spots = unreachables;

    Some(sphere)
}

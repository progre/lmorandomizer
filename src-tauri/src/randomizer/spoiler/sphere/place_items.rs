use std::{collections::HashSet, mem::take};

use rand::Rng;

use crate::randomizer::{
    spoiler::{
        items_pool::{ShuffledItems, UnorderedItems},
        spots::{SpotRef, Spots},
    },
    storage::item::Item,
};

pub fn place_items<'a>(
    rng: &mut impl Rng,
    mut field_items: ShuffledItems<'a>,
    mut talk_items: ShuffledItems<'a>,
    mut shop_items: ShuffledItems<'a>,
    consumable_items_pool: &mut UnorderedItems<'a>,
    reachables: Spots<'a>,
) -> Option<Vec<(SpotRef<'a>, &'a Item)>> {
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
                sphere.push((spot, item));
            }
            SpotRef::Talk(_) | SpotRef::Shop(_) => unreachable!(),
        });
    reachables.talk_spots.into_iter().for_each(|spot| {
        let item = talk_items.pop().unwrap();
        sphere.push((SpotRef::Talk(spot), item));
    });
    for shop in reachables.shops {
        let item = if shop.item.name.is_consumable() {
            // Do not sell the same item to the same store
            let placed_items: HashSet<_> = sphere
                .iter()
                .filter_map(|(spot, item)| match spot {
                    SpotRef::Shop(spot) => Some((spot, item)),
                    SpotRef::MainWeapon(_)
                    | SpotRef::SubWeapon(_)
                    | SpotRef::Chest(_)
                    | SpotRef::Seal(_)
                    | SpotRef::Rom(_)
                    | SpotRef::Talk(_) => None,
                })
                .filter(|(spot, _)| spot.spot.items() == shop.spot.items())
                .map(|(_, item)| &item.name)
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
        sphere.push((SpotRef::Shop(shop), item));
    }
    Some(sphere)
}

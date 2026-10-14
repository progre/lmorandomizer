use std::{collections::HashSet, mem::take};

use rand::Rng;

use crate::randomizer::{
    spoiler::{
        items_pool::{ShuffledItems, UnorderedItems},
        locations::{LocationRef, Locations},
    },
    spoiler_log::{CheckpointRef, SphereRef},
};

pub fn place_items<'a>(
    rng: &mut impl Rng,
    mut field_items: ShuffledItems<'a>,
    mut talk_items: ShuffledItems<'a>,
    mut shop_items: ShuffledItems<'a>,
    consumable_items_pool: &mut UnorderedItems<'a>,
    reachables: Locations<'a>,
) -> Option<SphereRef<'a>> {
    let mut sphere: Vec<_> = Default::default();
    reachables
        .field_item_locations
        .into_iter()
        .for_each(|location| match location {
            LocationRef::MainWeapon(_)
            | LocationRef::SubWeapon(_)
            | LocationRef::Chest(_)
            | LocationRef::Seal(_)
            | LocationRef::Rom(_) => {
                let item = field_items.pop().unwrap();
                sphere.push(CheckpointRef::Item { location, item });
            }
            LocationRef::Talk(_) | LocationRef::Shop(_) => unreachable!(),
        });
    reachables.talk_locations.into_iter().for_each(|location| {
        let item = talk_items.pop().unwrap();
        let location = LocationRef::Talk(location);
        sphere.push(CheckpointRef::Item { location, item });
    });
    for shop in reachables.shops {
        let item = if shop.item.name.is_consumable() {
            // Do not sell the same item to the same store
            let placed_items: HashSet<_> = sphere
                .iter()
                .filter_map(|x| match x {
                    CheckpointRef::Item {
                        location: LocationRef::Shop(location),
                        item,
                    } => Some((location, item)),
                    CheckpointRef::Item { .. } | CheckpointRef::Event(_) => None,
                })
                .filter(|(location, _)| location.spot.items() == shop.spot.items())
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
        let location = LocationRef::Shop(shop);
        sphere.push(CheckpointRef::Item { location, item });
    }
    Some(SphereRef::new(sphere))
}

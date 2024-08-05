use std::{collections::HashSet, ops::Deref};

use rand::Rng;

use crate::{
    randomizer::{
        spoiler::{items_pool::UnorderedItems, spots::Spots},
        spoiler_log::{CheckpointRef, SphereRef},
        storage::{
            item::{Item, StrategyFlag},
            ShopRef,
        },
    },
    script::enums::FieldNumber,
};

use super::{append_flags, explore};

fn explorer_neighborhood<'a>(
    remaining_spots: &Spots<'a>,
    strategy_flags: &HashSet<&'a StrategyFlag>,
) -> (Spots<'a>, Spots<'a>) {
    let (mut working, mut remainings) = explore(remaining_spots, strategy_flags);
    let is_early = |field_number: FieldNumber| {
        matches!(
            field_number,
            FieldNumber::Surface | FieldNumber::GateOfGuidance
        )
    };
    let (reachables, mut unreachables) = working
        .field_item_spots
        .into_iter()
        .partition(|x| is_early(x.field_number()));
    working.field_item_spots = reachables;
    remainings.field_item_spots.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .talk_spots
        .into_iter()
        .partition(|x| is_early(x.field_number()));
    working.talk_spots = reachables;
    remainings.talk_spots.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .shops
        .into_iter()
        .partition(|x| is_early(x.spot.field_number()) && !x.name.is_consumable());
    working.shops = reachables;
    remainings.shops.append(&mut unreachables);
    (working, remainings)
}

fn place_items<'a>(
    rng: &mut impl Rng,
    priority_items: impl Iterator<Item = &'a Item>,
    working: &mut Spots<'a>,
) -> Vec<CheckpointRef<'a>> {
    priority_items
        .map(|item| {
            let dice = rng.gen_range(
                0..(working.field_item_spots.len()
                    + working.talk_spots.len()
                    + working.shops.len()),
            );
            if dice < working.field_item_spots.len() {
                let spot = working.field_item_spots.swap_remove(dice);
                CheckpointRef::from_field_spot_item(spot, item)
            } else if dice < working.field_item_spots.len() + working.talk_spots.len() {
                unreachable!()
            } else {
                let idx = dice - working.field_item_spots.len() - working.talk_spots.len();
                let item_spot = working.shops.swap_remove(idx);
                let spot = &item_spot.spot;
                let idx = item_spot.idx;
                CheckpointRef::Shop(ShopRef { spot, idx, item })
            }
        })
        .collect()
}

pub fn pre_sphere<'a>(
    rng: &mut impl Rng,
    priority_items: UnorderedItems<'a>,
    remaining_spots: &mut Spots<'a>,
    strategy_flags: &mut HashSet<&'a StrategyFlag>,
) -> SphereRef<'a> {
    let mut priority_items = priority_items.into_inner();
    let mut spheres = Vec::new();

    // Placing a Hand Scanner
    if let Some(idx) = priority_items
        .iter()
        .position(|x| x.name == StrategyFlag::new("handScanner".into()))
    {
        let item = priority_items.swap_remove(idx);
        let (mut working, remainings) =
            explorer_neighborhood(remaining_spots.deref(), strategy_flags.deref());
        *remaining_spots = remainings;
        let mut checkpoints = SphereRef(place_items(rng, [item].into_iter(), &mut working));
        remaining_spots.extend(working);
        append_flags(strategy_flags, &checkpoints);
        spheres.append(&mut checkpoints.0);
    }
    let (mut working, remainings) =
        explorer_neighborhood(remaining_spots.deref(), strategy_flags.deref());
    *remaining_spots = remainings;
    let checkpoints: Vec<_> = place_items(rng, priority_items.iter().copied(), &mut working);
    remaining_spots.extend(working);
    let mut checkpoints = SphereRef(checkpoints);
    append_flags(strategy_flags, &checkpoints);
    spheres.append(&mut checkpoints.0);
    SphereRef(spheres)
}

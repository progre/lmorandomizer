use std::ops::Deref;

use rand::Rng;

use crate::{
    dataset::spot::Region,
    randomizer::{
        spoiler::{
            items_pool::UnorderedItems, regions::Regions, sphere::state::State, spots::Spots,
        },
        spoiler_log::{CheckpointRef, SphereRef},
        storage::{
            ShopRef,
            item::{Item, StrategyFlag},
        },
    },
    script::enums::FieldNumber,
};

use super::explore;

fn explorer_neighborhood<'a>(
    remaining_spots: &Spots<'a>,
    state: &State<'a>,
) -> (Spots<'a>, Spots<'a>) {
    let (mut working, mut remainings) = explore(remaining_spots, state);
    let is_early = |region: &Region| {
        matches!(
            region.field_number(),
            FieldNumber::Surface | FieldNumber::GateOfGuidance
        )
    };
    let (reachables, mut unreachables) = working
        .field_item_spots
        .into_iter()
        .partition(|x| is_early(x.region()));
    working.field_item_spots = reachables;
    remainings.field_item_spots.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .talk_spots
        .into_iter()
        .partition(|x| is_early(x.region()));
    working.talk_spots = reachables;
    remainings.talk_spots.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .shops
        .into_iter()
        .partition(|x| is_early(x.spot.region()) && !x.name.is_consumable());
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
    state: &mut State<'a>,
    all_regions: &Regions<'a>,
) -> SphereRef<'a> {
    let mut priority_items = priority_items.into_inner();
    let mut spheres = Vec::new();

    // Placing a Hand Scanner
    if let Some(idx) = priority_items
        .iter()
        .position(|x| x.name == StrategyFlag::new("handScanner".into()))
    {
        let item = priority_items.swap_remove(idx);
        let (mut working, remainings) = explorer_neighborhood(remaining_spots.deref(), state);
        *remaining_spots = remainings;
        let checkpoints = place_items(rng, [item].into_iter(), &mut working);
        let checkpoints = SphereRef::new(state.reachable_regions().collect(), checkpoints);
        remaining_spots.extend(working);
        state.append_flags(&checkpoints, all_regions);
        spheres.append(&mut checkpoints.into_inner());
    }
    let (mut working, remainings) = explorer_neighborhood(remaining_spots.deref(), state);
    *remaining_spots = remainings;
    let checkpoints: Vec<_> = place_items(rng, priority_items.iter().copied(), &mut working);
    remaining_spots.extend(working);
    let reachable_regions: Vec<_> = state.reachable_regions().collect();
    let checkpoints = SphereRef::new(reachable_regions.clone(), checkpoints);
    state.append_flags(&checkpoints, all_regions);
    spheres.append(&mut checkpoints.into_inner());
    SphereRef::new(reachable_regions, spheres)
}

use std::ops::Deref;

use rand::{Rng, RngExt};

use crate::{
    dataset::regions::{RegionId, Regions},
    randomizer::{
        spoiler::{
            items_pool::UnorderedItems,
            locations::{LocationRef, Locations},
            sphere::state::State,
        },
        spoiler_log::{CheckpointRef, SphereRef},
        storage::item::{Item, StrategyFlag},
    },
    script::enums::FieldNumber,
};

use super::explore;

fn explorer_neighborhood<'a>(
    remaining_locations: &Locations<'a>,
    state: &State<'a>,
    all_regions: &Regions,
) -> (Locations<'a>, Locations<'a>) {
    let (mut working, mut remainings) = explore(remaining_locations, state);
    let is_early = |region: RegionId| {
        matches!(
            all_regions.get(region).field_number(),
            FieldNumber::Surface | FieldNumber::GateOfGuidance
        )
    };
    let (reachables, mut unreachables) = working
        .field_item_locations
        .into_iter()
        .partition(|x| is_early(x.region()));
    working.field_item_locations = reachables;
    remainings.field_item_locations.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .talk_locations
        .into_iter()
        .partition(|x| is_early(x.spot.region()));
    working.talk_locations = reachables;
    remainings.talk_locations.append(&mut unreachables);
    let (reachables, mut unreachables) = working
        .shops
        .into_iter()
        .partition(|x| is_early(x.spot.region()) && !x.item.name.is_consumable());
    working.shops = reachables;
    remainings.shops.append(&mut unreachables);
    (working, remainings)
}

fn place_items<'a>(
    rng: &mut impl Rng,
    priority_items: impl Iterator<Item = &'a Item>,
    working: &mut Locations<'a>,
) -> Vec<CheckpointRef<'a>> {
    priority_items
        .map(|item| {
            let dice = rng.random_range(
                0..(working.field_item_locations.len()
                    + working.talk_locations.len()
                    + working.shops.len()),
            );
            if dice < working.field_item_locations.len() {
                let location = working.field_item_locations.swap_remove(dice);
                CheckpointRef::Item { location, item }
            } else if dice < working.field_item_locations.len() + working.talk_locations.len() {
                unreachable!()
            } else {
                let idx = dice - working.field_item_locations.len() - working.talk_locations.len();
                let location = LocationRef::Shop(working.shops.swap_remove(idx));
                CheckpointRef::Item { location, item }
            }
        })
        .collect()
}

pub fn pre_sphere<'a>(
    rng: &mut impl Rng,
    priority_items: UnorderedItems<'a>,
    remaining_locations: &mut Locations<'a>,
    state: &mut State<'a>,
    all_regions: &Regions,
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
            explorer_neighborhood(remaining_locations.deref(), state, all_regions);
        *remaining_locations = remainings;
        let checkpoints = place_items(rng, [item].into_iter(), &mut working);
        let checkpoints = SphereRef::new(checkpoints);
        remaining_locations.extend(working);
        state.append_flags(&checkpoints);
        spheres.append(&mut checkpoints.into_inner());
    }
    let (mut working, remainings) =
        explorer_neighborhood(remaining_locations.deref(), state, all_regions);
    *remaining_locations = remainings;
    let checkpoints: Vec<_> = place_items(rng, priority_items.iter().copied(), &mut working);
    remaining_locations.extend(working);
    let checkpoints = SphereRef::new(checkpoints);
    state.append_flags(&checkpoints);
    spheres.append(&mut checkpoints.into_inner());
    SphereRef::new(spheres)
}

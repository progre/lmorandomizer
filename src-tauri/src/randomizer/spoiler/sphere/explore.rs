use crate::randomizer::spoiler::locations::Locations;

use super::State;

pub fn explore<'a>(
    remaining_locations: &Locations<'a>,
    state: &State<'a>,
) -> (Locations<'a>, Locations<'a>) {
    let (reachables_field_item_locations, unreachables_field_item_locations) = remaining_locations
        .field_item_locations
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_reachable(x.region(), x.requirements()));
    let (reachables_talk_locations, unreachables_talk_locations) = remaining_locations
        .talk_locations
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_reachable(x.spot.region(), x.spot.requirements()));
    let (reachables_shops, unreachable_shops) = remaining_locations
        .shops
        .iter()
        .cloned()
        .partition::<Vec<_>, _>(|shop| {
            state.is_reachable(shop.spot.region(), shop.spot.requirements())
        });

    let reachables = Locations {
        field_item_locations: reachables_field_item_locations,
        talk_locations: reachables_talk_locations,
        shops: reachables_shops,
        events: vec![],
    };
    let unreachables = Locations {
        field_item_locations: unreachables_field_item_locations,
        talk_locations: unreachables_talk_locations,
        shops: unreachable_shops,
        events: remaining_locations.events.clone(),
    };
    (reachables, unreachables)
}

use crate::randomizer::spoiler::spots::Spots;

use super::State;

pub fn explore<'a>(remaining_spots: &Spots<'a>, state: &State<'a>) -> (Spots<'a>, Spots<'a>) {
    let (reachables_field_item_spots, unreachables_field_item_spots) = remaining_spots
        .field_item_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_satisfied(x.requirements()));
    let (reachables_talk_spots, unreachables_talk_spots) = remaining_spots
        .talk_spots
        .iter()
        .copied()
        .partition::<Vec<_>, _>(|x| state.is_satisfied(x.spot.requirements()));
    let (reachables_shops, unreachable_shops) = remaining_spots
        .shops
        .iter()
        .cloned()
        .partition::<Vec<_>, _>(|shop| state.is_satisfied(shop.spot.requirements()));

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
        events: remaining_spots.events.clone(),
    };
    (reachables, unreachables)
}

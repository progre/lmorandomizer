use std::mem::take;

use crate::randomizer::{
    spoiler::regions::Regions,
    storage::{Event, item::StrategyFlag},
};

use super::{State, progression::is_event_achievable};

fn take_achieved<'a>(events: &mut Vec<&'a Event>, state: &State) -> Vec<&'a Event> {
    let (achieved, unachieved) = take(events)
        .into_iter()
        .partition(|event| is_event_achievable(state, event));
    *events = unachieved;
    achieved
}

pub fn achieve_events<'a>(
    events: &mut Vec<&'a Event>,
    state: &mut State<'a>,
    all_regions: &Regions<'a>,
) -> Vec<&'a StrategyFlag> {
    let mut checkpoints = vec![];
    while !events.is_empty() {
        state.explore_regions(all_regions);
        let achieved = take_achieved(events, state);
        if achieved.is_empty() {
            return checkpoints;
        }
        achieved.into_iter().for_each(|event| {
            state.insert_flag(&event.name);
            checkpoints.push(&event.name);
        });
    }
    checkpoints
}

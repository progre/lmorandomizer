use std::collections::HashSet;

use crate::{
    dataset::spot::AnyOfAllRequirements,
    randomizer::{
        spoiler::{items_pool::ItemsPool, regions::Regions, spots::Spots},
        storage::{
            Event,
            item::{Item, StrategyFlag},
        },
    },
};

use super::state::State;

pub fn is_event_achievable(state: &State, event: &Event) -> bool {
    if let Some(region) = &event.region {
        state.is_reachable(region, event.requirements.as_ref())
    } else {
        state.is_reachable_without_region(event.requirements.as_ref())
    }
}

struct RequirementNames {
    names: HashSet<String>,
    includes_sacred_orb: bool,
}

impl RequirementNames {
    fn collect(unreachables: &Spots, all_regions: &Regions) -> Self {
        let mut result = Self {
            names: HashSet::new(),
            includes_sacred_orb: false,
        };
        for spot in &unreachables.field_item_spots {
            result.add(spot.requirements());
        }
        for spot in &unreachables.talk_spots {
            result.add(spot.requirements());
        }
        for shop in &unreachables.shops {
            result.add(shop.spot.requirements());
        }
        for event in &unreachables.events {
            result.add(event.requirements.as_ref());
        }
        for region in all_regions.iter() {
            result.add(region.access_rule());
            for (_, access_rule) in region.exits().all_exits() {
                let requirements = access_rule
                    .clone()
                    .try_into_any_of_all_requirements()
                    .unwrap();
                result.add(requirements.as_ref());
            }
        }
        result
    }

    fn add(&mut self, requirements: Option<&AnyOfAllRequirements>) {
        let Some(any) = requirements else {
            return;
        };
        for all in &any.0 {
            for requirement in &all.0 {
                if requirement.is_sacred_orb() {
                    self.includes_sacred_orb = true;
                } else {
                    self.names.insert(requirement.get().to_owned());
                }
            }
        }
    }

    fn contains(&self, name: &StrategyFlag) -> bool {
        self.names.contains(name.get()) || self.includes_sacred_orb && name.is_sacred_orb()
    }
}

/// state に item を単体で追加したとき、未到達の spot / event / region の
/// いずれかが新たに到達可能になるか
fn expands_reachability<'a>(
    state: &State<'a>,
    item: &'a Item,
    unreachables: &Spots<'a>,
    pending_events: &[&'a Event],
    all_regions: &Regions<'a>,
    baseline_region_count: usize,
) -> bool {
    let mut state = state.clone();
    state.insert_flag(&item.name);
    state.explore_regions(all_regions);
    state.reachable_regions().count() > baseline_region_count
        || unreachables
            .field_item_spots
            .iter()
            .any(|x| state.is_reachable(x.region(), x.requirements()))
        || unreachables
            .talk_spots
            .iter()
            .any(|x| state.is_reachable(x.region(), x.requirements()))
        || unreachables
            .shops
            .iter()
            .any(|x| state.is_reachable(x.spot.region(), x.spot.requirements()))
        || pending_events
            .iter()
            .any(|&event| is_event_achievable(&state, event))
}

/// items_pool のうち、現在の state に単体で追加すると未到達の
/// spot / event / region を新たに到達可能にする item 名の集合
pub fn progression_flags<'a>(
    items_pool: &ItemsPool<'a>,
    state: &State<'a>,
    unreachables: &Spots<'a>,
    all_regions: &Regions<'a>,
) -> HashSet<&'a StrategyFlag> {
    // どの requirement にも名前が現れない item は到達可能範囲を変えられないため、
    // 事前に除外して到達可能性の再計算回数を減らす
    let requirement_names = RequirementNames::collect(unreachables, all_regions);
    let baseline_region_count = state.reachable_regions().count();
    // 既に達成可能な event は「新たに」達成可能になるわけではないため除外する
    let pending_events: Vec<_> = unreachables
        .events
        .iter()
        .copied()
        .filter(|event| !is_event_achievable(state, event))
        .collect();

    let mut visited = HashSet::new();
    items_pool
        .field_items
        .as_slice()
        .iter()
        .chain(items_pool.talk_items.as_slice())
        .chain(items_pool.shop_items.as_slice())
        .copied()
        .filter(|item| visited.insert(&item.name))
        .filter(|item| requirement_names.contains(&item.name))
        .filter(|&item| {
            expands_reachability(
                state,
                item,
                unreachables,
                &pending_events,
                all_regions,
                baseline_region_count,
            )
        })
        .map(|item| &item.name)
        .collect()
}

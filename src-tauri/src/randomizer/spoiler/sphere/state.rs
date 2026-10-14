use std::{collections::HashSet, iter::once};

use crate::{
    dataset::{
        regions::{Region, RegionId, Regions},
        spot::AnyOfAllRequirements,
    },
    randomizer::{
        spoiler_log::{CheckpointRef, SphereRef},
        storage::item::StrategyFlag,
    },
};

#[derive(Clone)]
pub struct State<'a> {
    reachable_regions: Vec<&'a Region>,
    reachable_region_ids: HashSet<RegionId>,
    strategy_flags: HashSet<&'a str>,
    sacred_orb_count: u8,
}

impl<'a> State<'a> {
    pub fn new(initial_region: &'a Region) -> Self {
        Self {
            reachable_regions: vec![initial_region],
            reachable_region_ids: HashSet::from([initial_region.id()]),
            strategy_flags: HashSet::default(),
            sacred_orb_count: 0,
        }
    }

    pub fn is_reachable(
        &self,
        region: RegionId,
        requirements: Option<&AnyOfAllRequirements>,
    ) -> bool {
        self.reachable_region_ids.contains(&region)
            && self.is_reachable_without_region(requirements)
    }

    pub fn is_reachable_without_region(&self, requirements: Option<&AnyOfAllRequirements>) -> bool {
        let Some(any) = requirements else {
            return true;
        };
        any.0.iter().any(|all| {
            all.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= self.sacred_orb_count
                    || self.strategy_flags.contains(x.get())
            })
        })
    }

    pub fn reachable_regions(&self) -> impl Iterator<Item = &'a Region> {
        self.reachable_regions.iter().copied()
    }

    fn new_exit_targets(&self, regions: &[&'a Region]) -> impl Iterator<Item = RegionId> {
        regions
            .iter()
            .flat_map(|x| x.exits())
            .filter(|exit| {
                self.is_reachable_without_region(exit.requirements.as_ref())
                    && !self.reachable_region_ids.contains(&exit.target_id)
            })
            .map(|exit| exit.target_id)
    }

    pub fn explore_regions(&mut self, all_regions: &'a Regions) {
        let mut searching_regions = self.reachable_regions.clone();
        loop {
            let found_regions: Vec<_> = self
                .new_exit_targets(&searching_regions)
                .map(|region_id| all_regions.get(region_id))
                .filter(|region| self.is_reachable_without_region(region.access_rule()))
                .collect();
            if found_regions.is_empty() {
                break;
            }
            self.reachable_region_ids
                .extend(found_regions.iter().map(|region| region.id()));
            self.reachable_regions.append(&mut found_regions.clone());
            searching_regions = found_regions;
        }
    }

    pub fn insert_flag(&mut self, flag: &'a StrategyFlag) {
        self.append_flags_internal(once(flag));
    }

    pub fn append_flags(&mut self, sphere: &SphereRef<'a>) {
        self.append_flags_internal(sphere.iter().map(|checkpoint| match checkpoint {
            CheckpointRef::Item { item, .. } => &item.name,
            CheckpointRef::Event(flag) => flag,
        }));
    }

    fn append_flags_internal(&mut self, flags: impl Iterator<Item = &'a StrategyFlag>) {
        for flag in flags {
            if flag.is_sacred_orb() {
                self.sacred_orb_count += 1;
            }
            self.strategy_flags.insert(flag.get());
        }
    }
}

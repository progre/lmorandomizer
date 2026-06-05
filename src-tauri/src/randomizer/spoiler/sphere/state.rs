use std::{collections::HashSet, iter::once};

use crate::{
    dataset::{
        game_structure::RegionName,
        spot::{AnyOfAllRequirements, Region},
    },
    randomizer::{
        spoiler::regions::Regions,
        spoiler_log::{CheckpointRef, SphereRef},
        storage::item::StrategyFlag,
    },
};

pub struct State<'a> {
    reachable_regions: Vec<&'a Region>,
    strategy_flags: HashSet<&'a StrategyFlag>,
    sacred_orb_count: u8,
}

impl<'a> State<'a> {
    pub fn new(initial_region: &'a Region) -> Self {
        Self {
            reachable_regions: vec![initial_region],
            strategy_flags: HashSet::default(),
            sacred_orb_count: 0,
        }
    }

    pub fn is_reachable(
        &self,
        region: &Region,
        requirements: Option<&AnyOfAllRequirements>,
    ) -> bool {
        self.reachable_regions
            .iter()
            .any(|x| x.name() == region.name())
            && self.is_reachable_without_region(requirements)
    }

    pub fn is_reachable_without_region(&self, requirements: Option<&AnyOfAllRequirements>) -> bool {
        let Some(any) = requirements else {
            return true;
        };
        let strategy_flag_strings: HashSet<_> = self
            .strategy_flags
            .iter()
            .map(|x| x.get().to_owned())
            .collect();
        any.0.iter().any(|all| {
            all.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= self.sacred_orb_count
                    || strategy_flag_strings.contains(x.get())
            })
        })
    }

    pub fn reachable_regions(&self) -> impl Iterator<Item = &'a Region> {
        self.reachable_regions.iter().copied()
    }

    fn new_exit_names(&self, regions: &[&'a Region]) -> impl Iterator<Item = &'a RegionName> {
        regions
            .iter()
            .flat_map(|x| x.exits().all_exits())
            .map(|(name, _)| name)
            .filter(|&region_name| {
                self.reachable_regions
                    .iter()
                    .all(|x| x.name() != region_name)
            })
    }

    pub fn explore_regions(&mut self, all_regions: &Regions<'a>) {
        let mut searching_regions = self.reachable_regions.clone();
        loop {
            let found_regions: Vec<_> = self
                .new_exit_names(&searching_regions)
                .map(|region_name| {
                    all_regions
                        .iter()
                        .find(|x| x.name() == region_name)
                        .unwrap_or_else(|| {
                            panic!("Region '{}' is not found in all_regions", region_name.get())
                        })
                })
                .filter(|region| self.is_reachable_without_region(region.access_rule()))
                .collect();
            if found_regions.is_empty() {
                break;
            }
            self.reachable_regions.append(&mut found_regions.clone());
            searching_regions = found_regions;
        }
    }

    pub fn insert_flag(&mut self, flag: &'a StrategyFlag) {
        self.append_flags_internal(once(flag));
    }

    pub fn append_flags(&mut self, sphere: &SphereRef<'a>) {
        self.append_flags_internal(sphere.iter().map(|checkpoint| match checkpoint {
            CheckpointRef::MainWeapon(checkpoint) => &checkpoint.item.name,
            CheckpointRef::SubWeapon(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Chest(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Seal(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Shop(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Rom(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Talk(checkpoint) => &checkpoint.item.name,
            CheckpointRef::Event(flag) => flag,
        }));
    }

    fn append_flags_internal(&mut self, flags: impl Iterator<Item = &'a StrategyFlag>) {
        for flag in flags {
            if flag.is_sacred_orb() {
                self.sacred_orb_count += 1;
            }
            self.strategy_flags.insert(flag);
        }
    }
}

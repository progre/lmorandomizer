use std::{collections::HashSet, iter::once};

use crate::{
    dataset::spot::{AnyOfAllRequirements, Region},
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
    pub fn new(all_regions: &Regions<'a>) -> Self {
        Self {
            reachable_regions: all_regions
                .iter()
                .filter(|x| x.access_rule().is_none())
                .collect(),
            strategy_flags: HashSet::default(),
            sacred_orb_count: 0,
        }
    }

    pub fn is_reachable(
        &self,
        region: &Region,
        requirements: Option<&AnyOfAllRequirements>,
    ) -> bool {
        self.reachable_regions.contains(&region) && self.is_reachable_without_region(requirements)
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

    pub fn explore_regions(&mut self, all_regions: &Regions<'a>) {
        self.reachable_regions = all_regions
            .iter()
            .filter(|region| self.is_reachable_without_region(region.access_rule()))
            .collect();
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

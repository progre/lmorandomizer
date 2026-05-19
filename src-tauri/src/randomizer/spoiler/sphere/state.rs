use std::collections::HashSet;

use crate::{
    dataset::spot::AnyOfAllRequirements,
    randomizer::{
        spoiler_log::{CheckpointRef, SphereRef},
        storage::item::StrategyFlag,
    },
};

pub struct State<'a> {
    strategy_flags: HashSet<&'a StrategyFlag>,
    sacred_orb_count: u8,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            strategy_flags: HashSet::default(),
            sacred_orb_count: 0,
        }
    }

    pub fn is_reachable(&self, requirements: Option<&AnyOfAllRequirements>) -> bool {
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

    pub fn insert_flag(&mut self, flag: &'a StrategyFlag) {
        if flag.is_sacred_orb() {
            self.sacred_orb_count += 1;
        }
        self.strategy_flags.insert(flag);
    }

    pub fn append_flags(&mut self, sphere: &SphereRef<'a>) {
        for checkpoint in sphere.iter() {
            let flag = match checkpoint {
                CheckpointRef::MainWeapon(checkpoint) => &checkpoint.item.name,
                CheckpointRef::SubWeapon(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Chest(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Seal(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Shop(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Rom(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Talk(checkpoint) => &checkpoint.item.name,
                CheckpointRef::Event(flag) => flag,
            };
            self.insert_flag(flag);
        }
    }
}

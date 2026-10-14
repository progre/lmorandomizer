use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use crate::{
    dataset::{
        regions::{Region, RegionId, Regions},
        spot::AnyOfAllRequirements,
    },
    randomizer::storage::item::StrategyFlag,
};

#[derive(Clone)]
pub struct State<'a> {
    reachable_region_ids: HashSet<RegionId>,
    strategy_flags: HashSet<&'a str>,
    /// flag の接頭辞（`sacredOrb:surface` なら `sacredOrb`）ごとの個数
    counts: HashMap<&'a str, u8>,
}

impl<'a> State<'a> {
    pub fn new(initial_region: &'a Region) -> Self {
        Self {
            reachable_region_ids: HashSet::from([initial_region.id()]),
            strategy_flags: HashSet::from([initial_region.flag().get()]),
            counts: HashMap::default(),
        }
    }

    /// 到達した region の名前も flag として持つので、spot や event の到達もこれで判定できる
    pub fn is_satisfied(&self, requirements: Option<&AnyOfAllRequirements>) -> bool {
        let Some(any) = requirements else {
            return true;
        };
        any.0.iter().any(|all| {
            all.0.iter().all(|x| match x.at_least() {
                Some((prefix, count)) => self.counts.get(prefix).is_some_and(|&x| x >= count),
                None => self.strategy_flags.contains(x.get()),
            })
        })
    }

    pub fn is_region_reachable(&self, region: RegionId) -> bool {
        self.reachable_region_ids.contains(&region)
    }

    pub fn reachable_region_count(&self) -> usize {
        self.reachable_region_ids.len()
    }

    /// 要求を満たした region を、到達済みの集合に繰り返し足す。新しく足す region がなくなったら終える。
    /// 1 周目は全 region を調べる。2 周目以降に調べるのは、前の周で足した region の依存先だけである
    pub fn explore_regions(&mut self, all_regions: &'a Regions) {
        let mut candidates: Vec<_> = all_regions.iter().map(|region| region.id()).collect();
        loop {
            let found_regions: Vec<_> = candidates
                .iter()
                .copied()
                .filter(|&id| !self.is_region_reachable(id))
                .filter(|&id| {
                    all_regions
                        .requirements(id)
                        .is_some_and(|x| self.is_satisfied(Some(x)))
                })
                .collect();
            if found_regions.is_empty() {
                break;
            }
            for &id in &found_regions {
                self.reachable_region_ids.insert(id);
                self.strategy_flags.insert(all_regions.get(id).flag().get());
            }
            candidates = found_regions
                .iter()
                .flat_map(|&id| all_regions.dependents(id))
                .copied()
                .collect();
        }
    }

    pub fn insert_flag(&mut self, flag: &'a StrategyFlag) {
        self.append_flags(once(flag));
    }

    pub fn append_flags(&mut self, flags: impl Iterator<Item = &'a StrategyFlag>) {
        for flag in flags {
            if let Some((prefix, _)) = flag.get().split_once(':') {
                *self.counts.entry(prefix).or_default() += 1;
            }
            self.strategy_flags.insert(flag.get());
        }
    }
}

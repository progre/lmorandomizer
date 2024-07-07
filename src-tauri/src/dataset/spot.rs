use std::{collections::HashSet, fmt};

use super::supplements::StrategyFlag;

#[derive(Clone, Debug, serde::Serialize)]
pub struct SpotName(pub String);

impl SpotName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SpotSource {
    MainWeaponShutter(usize),
    SubWeaponShutter(usize),
    Chest(usize),
    SealChest(usize),
    Shop(usize),
}

impl fmt::Display for SpotSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainWeaponShutter(x) => write!(f, "MainWeaponShutter{}", x),
            Self::SubWeaponShutter(x) => write!(f, "SubWeaponShutter{}", x),
            Self::Chest(x) => write!(f, "Chest{}", x),
            Self::SealChest(x) => write!(f, "SealChest{}", x),
            Self::Shop(x) => write!(f, "Shop{}", x),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequirementFlag(String);

impl RequirementFlag {
    pub fn new(requirement: String) -> Self {
        debug_assert!(
            !requirement.starts_with("sacredOrb:")
                || requirement
                    .split(':')
                    .nth(1)
                    .map_or(false, |x| x.parse::<u8>().is_ok())
        );
        Self(requirement)
    }

    pub fn is_sacred_orb(&self) -> bool {
        self.0.starts_with("sacredOrb:")
    }

    pub fn sacred_orb_count(&self) -> u8 {
        self.0.split(':').nth(1).unwrap().parse().unwrap()
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<StrategyFlag> for RequirementFlag {
    fn eq(&self, other: &StrategyFlag) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec<AllRequirements>);

#[derive(Clone, Debug)]
pub struct Spot {
    pub name: SpotName,
    pub source: SpotSource,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl Spot {
    pub fn new(
        name: SpotName,
        source: SpotSource,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            name,
            source,
            requirements,
        }
    }

    pub fn is_reachable<'a>(
        &self,
        current_item_names: impl Iterator<Item = &'a StrategyFlag>,
        sacred_orb_count: u8,
    ) -> bool {
        let Some(any) = &self.requirements else {
            return true;
        };
        let current_item_names: HashSet<_> = current_item_names.map(|x| x.get()).collect();
        any.0.iter().any(|all| {
            all.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= sacred_orb_count
                    || current_item_names.contains(x.get())
            })
        })
    }
}

use std::{any::type_name, collections::HashSet, fmt};

use super::supplements::StrategyFlag;

#[derive(Clone, Debug, serde::Serialize)]
pub struct SpotName(String);

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
pub struct MainWeaponShutter {
    pub src_idx: usize,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl MainWeaponShutter {
    pub fn new(src_idx: usize, name: SpotName, requirements: Option<AnyOfAllRequirements>) -> Self {
        Self {
            src_idx,
            name,
            requirements,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubWeaponShutter {
    pub src_idx: usize,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl SubWeaponShutter {
    pub fn new(src_idx: usize, name: SpotName, requirements: Option<AnyOfAllRequirements>) -> Self {
        Self {
            src_idx,
            name,
            requirements,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chest {
    pub src_idx: usize,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl Chest {
    pub fn new(src_idx: usize, name: SpotName, requirements: Option<AnyOfAllRequirements>) -> Self {
        Self {
            src_idx,
            name,
            requirements,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SealChest {
    pub src_idx: usize,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl SealChest {
    pub fn new(src_idx: usize, name: SpotName, requirements: Option<AnyOfAllRequirements>) -> Self {
        Self {
            src_idx,
            name,
            requirements,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Shop {
    pub src_idx: usize,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl Shop {
    pub fn new(src_idx: usize, name: SpotName, requirements: Option<AnyOfAllRequirements>) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.0.split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self {
            src_idx,
            name,
            requirements,
        }
    }

    pub fn to_strategy_flags(&self) -> (StrategyFlag, StrategyFlag, StrategyFlag) {
        let mut names = self.name.0.split(',').map(|x| x.trim());
        (
            StrategyFlag::new(names.next().unwrap().to_string()),
            StrategyFlag::new(names.next().unwrap().to_string()),
            StrategyFlag::new(names.next().unwrap().to_string()),
        )
    }
}

#[derive(Clone, Debug)]
pub enum Spot {
    MainWeaponShutter(MainWeaponShutter),
    SubWeaponShutter(SubWeaponShutter),
    Chest(Chest),
    SealChest(SealChest),
    Shop(Shop),
}

impl Spot {
    fn name(&self) -> &SpotName {
        match self {
            Self::MainWeaponShutter(x) => &x.name,
            Self::SubWeaponShutter(x) => &x.name,
            Self::Chest(x) => &x.name,
            Self::SealChest(x) => &x.name,
            Self::Shop(x) => &x.name,
        }
    }

    pub fn requirements(&self) -> &Option<AnyOfAllRequirements> {
        match self {
            Self::MainWeaponShutter(x) => &x.requirements,
            Self::SubWeaponShutter(x) => &x.requirements,
            Self::Chest(x) => &x.requirements,
            Self::SealChest(x) => &x.requirements,
            Self::Shop(x) => &x.requirements,
        }
    }

    pub fn is_reachable(
        &self,
        current_strategy_flags: &HashSet<&str>,
        sacred_orb_count: u8,
    ) -> bool {
        let Some(any) = self.requirements() else {
            return true;
        };
        any.0.iter().any(|all| {
            all.0.iter().all(|x| {
                x.is_sacred_orb() && x.sacred_orb_count() <= sacred_orb_count
                    || current_strategy_flags.contains(x.get())
            })
        })
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_type_name = match self {
            Self::MainWeaponShutter(_) => type_name::<MainWeaponShutter>(),
            Self::SubWeaponShutter(_) => type_name::<SubWeaponShutter>(),
            Self::Chest(_) => type_name::<Chest>(),
            Self::SealChest(_) => type_name::<SealChest>(),
            Self::Shop(_) => type_name::<Shop>(),
        };
        let src_idx = match self {
            Self::MainWeaponShutter(x) => x.src_idx,
            Self::SubWeaponShutter(x) => x.src_idx,
            Self::Chest(x) => x.src_idx,
            Self::SealChest(x) => x.src_idx,
            Self::Shop(x) => x.src_idx,
        };
        write!(
            f,
            "{}{}({})",
            full_type_name.split("::").last().unwrap(),
            src_idx,
            self.name().get()
        )
    }
}

use std::{any::type_name, fmt};

use super::super::item::StrategyFlag;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
pub enum FieldId {
    Surface = 0,
    GateOfGuidance,
    MausoleumOfTheGiants,
    TempleOfTheSun,
    SpringInTheSky,
    InfernoCavern,
    ChamberOfExtinction,
    TwinLabyrinthsLeft,
    EndlessCorridor,
    ShrineOfTheMother,
    GateOfIllusion = 11,
    GraveyardOfTheGiants,
    TempleOfMoonlight,
    TowerOfTheGoddess,
    TowerOfRuin,
    ChamberOfBirth,
    TwinLabyrinthsRight,
    DimensionalCorridor,
    TrueShrineOfTheMother,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct SpotParams {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl SpotParams {
    fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            field_id,
            src_idx,
            name,
            requirements,
        }
    }

    fn fmt<T>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_id = &self.field_id;
        let type_name = type_name::<T>().split("::").last().unwrap();
        let name = self.name.get();
        write!(f, "{:?}_{}{}({})", field_id, type_name, self.src_idx, name)
    }
}

#[derive(Clone)]
pub struct MainWeaponSpot(SpotParams);

impl MainWeaponSpot {
    pub fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, src_idx, name, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.0.src_idx
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for MainWeaponSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt::<Self>(f)
    }
}

#[derive(Clone)]
pub struct SubWeaponSpot(SpotParams);

impl SubWeaponSpot {
    pub fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, src_idx, name, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.0.src_idx
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for SubWeaponSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt::<Self>(f)
    }
}

#[derive(Clone)]
pub struct ChestSpot(SpotParams);

impl ChestSpot {
    pub fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, src_idx, name, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.0.src_idx
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for ChestSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt::<Self>(f)
    }
}

#[derive(Clone)]
pub struct SealSpot(SpotParams);

impl SealSpot {
    pub fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, src_idx, name, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.0.src_idx
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for SealSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt::<Self>(f)
    }
}

#[derive(Clone)]
pub struct ShopSpot(SpotParams);

impl ShopSpot {
    pub fn new(
        field_id: FieldId,
        src_idx: usize,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.0.split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self(SpotParams::new(field_id, src_idx, name, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.0.src_idx
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }

    pub fn to_strategy_flags(&self) -> (StrategyFlag, StrategyFlag, StrategyFlag) {
        let mut names = self.0.name.0.split(',').map(|x| x.trim());
        (
            StrategyFlag::new(names.next().unwrap().to_string()),
            StrategyFlag::new(names.next().unwrap().to_string()),
            StrategyFlag::new(names.next().unwrap().to_string()),
        )
    }
}

impl fmt::Display for ShopSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt::<Self>(f)
    }
}

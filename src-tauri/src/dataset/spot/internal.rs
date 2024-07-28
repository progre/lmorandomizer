use std::fmt;

use vec1::Vec1;

use crate::script::data::items::{Rom, Seal};

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

impl fmt::Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
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
pub struct AllRequirements(pub Vec1<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec1<AllRequirements>);

#[derive(Clone, Debug)]
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

    fn fmt(&self, f: &mut fmt::Formatter<'_>, type_name: &str) -> fmt::Result {
        write!(f, "{}_{}({})", self.field_id, type_name, self.name.get())
    }
}

#[derive(Clone, Debug)]
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
        self.0.fmt(f, "MainWeaponSpot")
    }
}

#[derive(Clone, Debug)]
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
        self.0.fmt(f, "SubWeaponSpot")
    }
}

#[derive(Clone, Debug)]
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
        self.0.fmt(f, "Chest")
    }
}

#[derive(Clone, Debug)]
pub struct SealSpot {
    field_id: FieldId,
    seal: Seal,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl SealSpot {
    pub fn new(
        field_id: FieldId,
        seal: Seal,
        name: SpotName,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            field_id,
            seal,
            name,
            requirements,
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn seal(&self) -> Seal {
        self.seal
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
}

impl fmt::Display for SealSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = "SealSpot";
        write!(f, "{}_{}({})", self.field_id, type_name, self.name.get())
    }
}

#[derive(Clone, Debug)]
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
        self.0.fmt(f, "Shop")
    }
}

#[derive(Clone, Debug)]
pub struct RomSpot {
    field_id: FieldId,
    rom: Rom,
    name: SpotName,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        field_id: FieldId,
        rom: Rom,
        name: SpotName,
        requirements: AnyOfAllRequirements,
    ) -> Self {
        Self {
            field_id,
            rom,
            name,
            requirements,
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn rom(&self) -> Rom {
        self.rom
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> &AnyOfAllRequirements {
        &self.requirements
    }
}

impl fmt::Display for RomSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}_RomSpot({})", self.field_id(), self.name().get())
    }
}

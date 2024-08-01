use std::fmt;

use vec1::Vec1;

use crate::script::data::items::{self, MainWeapon, Rom, Seal, SubWeapon};

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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec1<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec1<AllRequirements>);

#[derive(Clone, Debug)]
struct SpotParams<T> {
    field_id: FieldId,
    name: SpotName,
    content: T,
    requirements: Option<AnyOfAllRequirements>,
}

impl<T> SpotParams<T> {
    fn new(
        field_id: FieldId,
        name: SpotName,
        content: T,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            field_id,
            name,
            content,
            requirements,
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>, type_name: &str) -> fmt::Result {
        write!(f, "{}_{}({})", self.field_id, type_name, self.name.get())
    }
}

#[derive(Clone, Debug)]
pub struct MainWeaponSpot(SpotParams<MainWeapon>);

impl MainWeaponSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: MainWeapon,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn main_weapon(&self) -> MainWeapon {
        self.0.content
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
pub struct SubWeaponSpot(SpotParams<SubWeapon>);

impl SubWeaponSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: SubWeapon,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn sub_weapon(&self) -> SubWeapon {
        self.0.content
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ChestItem {
    Equipment(items::Equipment),
    Rom(items::Rom),
}

#[derive(Clone, Debug)]
pub struct ChestSpot(SpotParams<ChestItem>);

impl ChestSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: ChestItem,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn item(&self) -> ChestItem {
        self.0.content
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
pub struct SealSpot(SpotParams<Seal>);

impl SealSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: Seal,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn seal(&self) -> Seal {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for SealSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "SealSpot")
    }
}

#[derive(Clone, Debug)]
pub struct RomSpot {
    field_id: FieldId,
    name: SpotName,
    rom: Rom,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: Rom,
        requirements: AnyOfAllRequirements,
    ) -> Self {
        Self {
            field_id,
            name,
            rom: content,
            requirements,
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn rom(&self) -> Rom {
        self.rom
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ShopItem {
    Equipment(items::Equipment),
    Rom(items::Rom),
    SubWeapon(items::SubWeapon),
}

impl ShopItem {
    pub fn matches_items(
        left: (Self, Self, Self),
        right: (Option<Self>, Option<Self>, Option<Self>),
    ) -> bool {
        left.0.matches(right.0.as_ref())
            && left.1.matches(right.1.as_ref())
            && left.2.matches(right.2.as_ref())
    }

    pub fn matches(&self, right: Option<&Self>) -> bool {
        right.map_or(true, |x| self == x)
    }
}

#[derive(Clone, Debug)]
pub struct ShopSpot(SpotParams<(Option<ShopItem>, Option<ShopItem>, Option<ShopItem>)>);

impl ShopSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: (Option<ShopItem>, Option<ShopItem>, Option<ShopItem>),
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.0.split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn items(&self) -> (Option<ShopItem>, Option<ShopItem>, Option<ShopItem>) {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for ShopSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "Shop")
    }
}

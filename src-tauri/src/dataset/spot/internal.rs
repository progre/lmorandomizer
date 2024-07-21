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
pub struct MainWeaponSpot {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl MainWeaponSpot {
    pub fn new(
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

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
    pub fn requirements_mut(&mut self) -> &mut Option<AnyOfAllRequirements> {
        &mut self.requirements
    }
}

#[derive(Clone)]
pub struct SubWeaponSpot {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl SubWeaponSpot {
    pub fn new(
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

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
    pub fn requirements_mut(&mut self) -> &mut Option<AnyOfAllRequirements> {
        &mut self.requirements
    }
}

#[derive(Clone)]
pub struct ChestSpot {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl ChestSpot {
    pub fn new(
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

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
    pub fn requirements_mut(&mut self) -> &mut Option<AnyOfAllRequirements> {
        &mut self.requirements
    }
}

#[derive(Clone)]
pub struct SealSpot {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

impl SealSpot {
    pub fn new(
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

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
    pub fn requirements_mut(&mut self) -> &mut Option<AnyOfAllRequirements> {
        &mut self.requirements
    }
}

#[derive(Clone)]
pub struct ShopSpot {
    field_id: FieldId,
    src_idx: usize,
    name: SpotName,
    requirements: Option<AnyOfAllRequirements>,
}

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
        Self {
            field_id,
            src_idx,
            name,
            requirements,
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
    }
    pub fn src_idx(&self) -> usize {
        self.src_idx
    }
    pub fn name(&self) -> &SpotName {
        &self.name
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.requirements.as_ref()
    }
    pub fn requirements_mut(&mut self) -> &mut Option<AnyOfAllRequirements> {
        &mut self.requirements
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

use std::fmt;

use crate::script::enums::{FieldNumber, SubWeapon};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct SubWeaponSpot(SpotParams<SubWeapon>);

impl SubWeaponSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: SubWeapon,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_number, name, content, requirements))
    }

    pub fn field_number(&self) -> FieldNumber {
        self.0.field_number
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

use std::fmt;

use crate::script::enums::{FieldNumber, MainWeapon};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct MainWeaponSpot(SpotParams<MainWeapon>);

impl MainWeaponSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: MainWeapon,
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

use std::fmt;

use crate::script::enums::MainWeapon;

use super::{
    params::{AnyOfAllRequirements, SpotName, SpotParams},
    FieldId,
};

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

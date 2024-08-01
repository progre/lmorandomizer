use std::fmt;

use crate::script::data::items::SubWeapon;

use super::{
    params::{AnyOfAllRequirements, SpotName, SpotParams},
    FieldId,
};

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

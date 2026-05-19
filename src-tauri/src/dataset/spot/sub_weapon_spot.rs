use std::fmt;

use crate::{dataset::spot::params::Region, script::enums::SubWeapon};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct SubWeaponSpot(SpotParams<SubWeapon>);

impl SubWeaponSpot {
    pub fn new(
        region: Region,
        name: SpotName,
        content: SubWeapon,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(region, name, content, requirements))
    }

    pub fn region(&self) -> &Region {
        &self.0.region
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

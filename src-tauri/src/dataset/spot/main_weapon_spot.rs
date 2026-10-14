use crate::{
    dataset::regions::{RegionId, Regions},
    script::enums::MainWeapon,
};

use super::params::{AnyOfAllRequirements, SpotDisplay, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct MainWeaponSpot(SpotParams<MainWeapon>);

impl MainWeaponSpot {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: MainWeapon,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(region, name, content, requirements))
    }

    pub fn region(&self) -> RegionId {
        self.0.region
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

    pub fn display<'a>(&'a self, regions: &'a Regions) -> SpotDisplay<'a> {
        self.0.display(regions, "MainWeaponSpot")
    }
}

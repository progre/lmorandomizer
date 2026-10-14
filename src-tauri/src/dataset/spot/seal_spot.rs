use crate::{
    dataset::regions::{RegionId, Regions},
    script::enums::Seal,
};

use super::params::{AnyOfAllRequirements, SpotDisplay, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct SealSpot(SpotParams<Seal>);

impl SealSpot {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: Seal,
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
    pub fn seal(&self) -> Seal {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }

    pub fn display<'a>(&'a self, regions: &'a Regions) -> SpotDisplay<'a> {
        self.0.display(regions, "SealSpot")
    }
}

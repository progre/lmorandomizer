use crate::{
    dataset::regions::{RegionId, Regions},
    script::enums::ChestItem,
};

use super::params::{AnyOfAllRequirements, SpotDisplay, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct ChestSpot(SpotParams<ChestItem>);

impl ChestSpot {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: ChestItem,
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
    pub fn item(&self) -> ChestItem {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }

    pub fn display<'a>(&'a self, regions: &'a Regions) -> SpotDisplay<'a> {
        self.0.display(regions, "Chest")
    }
}

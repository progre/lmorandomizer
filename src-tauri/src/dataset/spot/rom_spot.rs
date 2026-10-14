use crate::{
    dataset::regions::{RegionId, Regions},
    script::enums::Rom,
};

use super::params::{AnyOfAllRequirements, SpotDisplay, SpotName};

#[derive(Clone, Debug)]
pub struct RomSpot {
    region: RegionId,
    name: SpotName,
    rom: Rom,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: Rom,
        requirements: AnyOfAllRequirements,
    ) -> Self {
        Self {
            region,
            name,
            rom: content,
            requirements,
        }
    }

    pub fn region(&self) -> RegionId {
        self.region
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

    pub fn display<'a>(&'a self, regions: &'a Regions) -> SpotDisplay<'a> {
        SpotDisplay::new(regions.get(self.region), "RomSpot", &self.name)
    }
}

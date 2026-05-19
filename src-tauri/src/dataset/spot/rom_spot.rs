use std::fmt;

use crate::{dataset::spot::Region, script::enums::Rom};

use super::params::{AnyOfAllRequirements, SpotName};

#[derive(Clone, Debug)]
pub struct RomSpot {
    region: Region,
    name: SpotName,
    rom: Rom,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        region: Region,
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

    pub fn region(&self) -> &Region {
        &self.region
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
}

impl fmt::Display for RomSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/RomSpot({})", self.region, self.name().get())
    }
}

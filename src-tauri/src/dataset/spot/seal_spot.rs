use std::fmt;

use crate::{dataset::spot::params::Region, script::enums::Seal};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct SealSpot(SpotParams<Seal>);

impl SealSpot {
    pub fn new(
        region: Region,
        name: SpotName,
        content: Seal,
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
    pub fn seal(&self) -> Seal {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for SealSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "SealSpot")
    }
}

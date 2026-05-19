use std::fmt;

use crate::{dataset::spot::params::Region, script::enums::TalkItem};

use super::{AnyOfAllRequirements, SpotName, params::SpotParams};

#[derive(Clone, Debug)]
pub struct TalkSpot(SpotParams<TalkItem>);

impl TalkSpot {
    pub fn new(
        region: Region,
        name: SpotName,
        content: TalkItem,
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
    pub fn item(&self) -> TalkItem {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for TalkSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "Talk")
    }
}

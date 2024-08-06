use std::fmt;

use crate::script::enums::{FieldNumber, TalkItem};

use super::{params::SpotParams, AnyOfAllRequirements, SpotName};

#[derive(Clone, Debug)]
pub struct TalkSpot(SpotParams<TalkItem>);

impl TalkSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: TalkItem,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_number, name, content, requirements))
    }

    pub fn field_number(&self) -> FieldNumber {
        self.0.field_number
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

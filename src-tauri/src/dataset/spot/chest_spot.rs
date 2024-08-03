use std::fmt;

use crate::script::enums::{ChestItem, FieldNumber};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct ChestSpot(SpotParams<ChestItem>);

impl ChestSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: ChestItem,
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
    pub fn item(&self) -> ChestItem {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for ChestSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "Chest")
    }
}

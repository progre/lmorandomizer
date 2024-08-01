use std::fmt;

use crate::script::data::items::ChestItem;

use super::{
    params::{AnyOfAllRequirements, SpotName, SpotParams},
    FieldId,
};

#[derive(Clone, Debug)]
pub struct ChestSpot(SpotParams<ChestItem>);

impl ChestSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: ChestItem,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
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

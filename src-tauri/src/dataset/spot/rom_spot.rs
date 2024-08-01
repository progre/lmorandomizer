use std::fmt;

use crate::script::data::items::Rom;

use super::{
    params::{AnyOfAllRequirements, SpotName},
    FieldId,
};

#[derive(Clone, Debug)]
pub struct RomSpot {
    field_id: FieldId,
    name: SpotName,
    rom: Rom,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: Rom,
        requirements: AnyOfAllRequirements,
    ) -> Self {
        Self {
            field_id,
            name,
            rom: content,
            requirements,
        }
    }

    pub fn field_id(&self) -> FieldId {
        self.field_id
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
        write!(f, "{:?}_RomSpot({})", self.field_id(), self.name().get())
    }
}

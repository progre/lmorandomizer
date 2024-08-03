use std::fmt;

use crate::script::enums::{FieldNumber, Rom};

use super::params::{AnyOfAllRequirements, SpotName};

#[derive(Clone, Debug)]
pub struct RomSpot {
    field_number: FieldNumber,
    name: SpotName,
    rom: Rom,
    requirements: AnyOfAllRequirements,
}

impl RomSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: Rom,
        requirements: AnyOfAllRequirements,
    ) -> Self {
        Self {
            field_number,
            name,
            rom: content,
            requirements,
        }
    }

    pub fn field_number(&self) -> FieldNumber {
        self.field_number
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
        let field_number = self.field_number();
        write!(f, "{:?}_RomSpot({})", field_number, self.name().get())
    }
}

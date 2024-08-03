use std::fmt;

use crate::script::enums::{FieldNumber, ShopItem};

use super::params::{AnyOfAllRequirements, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct ShopSpot(SpotParams<[Option<ShopItem>; 3]>);

impl ShopSpot {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: [Option<ShopItem>; 3],
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.get().split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self(SpotParams::new(field_number, name, content, requirements))
    }

    pub fn field_number(&self) -> FieldNumber {
        self.0.field_number
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn items(&self) -> [Option<ShopItem>; 3] {
        self.0.content
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        self.0.requirements.as_ref()
    }
}

impl fmt::Display for ShopSpot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, "Shop")
    }
}

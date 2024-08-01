use std::fmt;

use crate::script::data::items::ShopItem;

use super::{
    params::{AnyOfAllRequirements, SpotName, SpotParams},
    FieldId,
};

#[derive(Clone, Debug)]
pub struct ShopSpot(SpotParams<(Option<ShopItem>, Option<ShopItem>, Option<ShopItem>)>);

impl ShopSpot {
    pub fn new(
        field_id: FieldId,
        name: SpotName,
        content: (Option<ShopItem>, Option<ShopItem>, Option<ShopItem>),
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.get().split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self(SpotParams::new(field_id, name, content, requirements))
    }

    pub fn field_id(&self) -> FieldId {
        self.0.field_id
    }
    pub fn name(&self) -> &SpotName {
        &self.0.name
    }
    pub fn items(&self) -> (Option<ShopItem>, Option<ShopItem>, Option<ShopItem>) {
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

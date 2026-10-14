use crate::{
    dataset::regions::{RegionId, Regions},
    script::enums::ShopItem,
};

use super::params::{AnyOfAllRequirements, SpotDisplay, SpotName, SpotParams};

#[derive(Clone, Debug)]
pub struct ShopSpot(SpotParams<[Option<ShopItem>; 3]>);

impl ShopSpot {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: [Option<ShopItem>; 3],
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        if cfg!(debug_assertions) {
            let names: Vec<_> = name.get().split(',').map(|x| x.trim()).collect();
            debug_assert_eq!(names.len(), 3);
        }
        Self(SpotParams::new(region, name, content, requirements))
    }

    pub fn region(&self) -> RegionId {
        self.0.region
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

    pub fn display<'a>(&'a self, regions: &'a Regions) -> SpotDisplay<'a> {
        self.0.display(regions, "Shop")
    }
}

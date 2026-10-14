use std::fmt::{self, Display};

use crate::{
    dataset::{
        files::RegionName,
        spot::{AnyOfAllRequirements, RequirementFlag},
    },
    script::enums::FieldNumber,
};

/// region 一覧の中での位置
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RegionId(usize);

impl RegionId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// パース済みの exit。access rule を到達判定のたびにパースしなくて済むよう、
/// ロード時に一度だけ変換して保持する
#[derive(Clone, Debug)]
pub struct RegionExit {
    pub direction: &'static str,
    pub target: RegionName,
    pub target_id: RegionId,
    pub requirements: Option<AnyOfAllRequirements>,
}

#[derive(Clone, Debug)]
pub struct Region {
    id: RegionId,
    field_number: FieldNumber,
    name: RegionName,
    flag: RequirementFlag,
    access_rule: Option<AnyOfAllRequirements>,
    exits: Vec<RegionExit>,
}

impl Region {
    pub fn new(
        id: RegionId,
        field_number: FieldNumber,
        name: RegionName,
        access_rule: Option<AnyOfAllRequirements>,
        exits: Vec<RegionExit>,
    ) -> Self {
        // lmo には ROM と同名の region（`mazeOfGalious`）があるため、接頭辞を付けて区別する
        let flag = RequirementFlag::new(format!("region:{}", name.get()));
        Self {
            id,
            field_number,
            name,
            flag,
            access_rule,
            exits,
        }
    }

    /// この region に到達したことを表す flag
    pub fn flag(&self) -> &RequirementFlag {
        &self.flag
    }

    pub fn id(&self) -> RegionId {
        self.id
    }

    pub fn field_number(&self) -> FieldNumber {
        self.field_number
    }

    pub fn name(&self) -> &RegionName {
        &self.name
    }

    pub fn access_rule(&self) -> Option<&AnyOfAllRequirements> {
        self.access_rule.as_ref()
    }

    pub fn exits(&self) -> &[RegionExit] {
        &self.exits
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.get().fmt(f)
    }
}

/// `RegionId` で引ける region の表
#[derive(Clone, Debug)]
pub struct Regions(Vec<Region>);

impl Regions {
    pub fn new(regions: Vec<Region>) -> Self {
        debug_assert!(
            regions
                .iter()
                .enumerate()
                .all(|(index, region)| region.id() == RegionId::new(index))
        );
        Self(regions)
    }

    pub fn get(&self, id: RegionId) -> &Region {
        &self.0[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Region> {
        self.0.iter()
    }
}

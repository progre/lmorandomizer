use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

use vec1::Vec1;

use crate::{
    dataset::{
        files::RegionName,
        spot::{AllRequirements, AnyOfAllRequirements, RequirementFlag},
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

/// 2 つの要求の AND。各要求セットの組み合わせごとに 1 つのセットになる
fn and(a: AnyOfAllRequirements, b: Option<&AnyOfAllRequirements>) -> Vec<AllRequirements> {
    let Some(b) = b else {
        return a.0.into_vec();
    };
    a.0.iter()
        .flat_map(|x| b.0.iter().map(move |y| x.0.iter().chain(&y.0).cloned()))
        .map(|all| AllRequirements(Vec1::try_from_vec(all.collect()).unwrap()))
        .collect()
}

/// region への到達要求を、入ってくる exit ごとの要求セットの OR にする。
/// 各セットは「出発 region、exit の要求、行き先の access rule」の AND で、
/// 入ってくる exit が無い region は `None` になる
fn incoming_requirements(regions: &[Region]) -> Vec<Option<AnyOfAllRequirements>> {
    let mut sets: Vec<Vec<AllRequirements>> = vec![vec![]; regions.len()];
    for source in regions {
        for exit in source.exits() {
            let target = &regions[exit.target_id.0];
            let from_source =
                AnyOfAllRequirements::require(source.flag().clone(), exit.requirements.clone());
            sets[exit.target_id.0].append(&mut and(from_source, target.access_rule()));
        }
    }
    sets.into_iter()
        .map(|x| Vec1::try_from_vec(x).ok().map(AnyOfAllRequirements))
        .collect()
}

/// region ごとに、その region の flag を要求に含む region の一覧
fn dependents(
    regions: &[Region],
    requirements: &[Option<AnyOfAllRequirements>],
) -> Vec<Vec<RegionId>> {
    let ids: HashMap<_, _> = regions.iter().map(|x| (x.flag(), x.id())).collect();
    let mut dependents = vec![vec![]; regions.len()];
    for (index, any) in requirements.iter().enumerate() {
        let flags: HashSet<_> = any.iter().flat_map(|x| &x.0).flat_map(|x| &x.0).collect();
        for flag in flags {
            if let Some(id) = ids.get(flag) {
                dependents[id.0].push(RegionId(index));
            }
        }
    }
    dependents
}

/// `RegionId` で引ける region の表
#[derive(Clone, Debug)]
pub struct Regions {
    regions: Vec<Region>,
    requirements: Vec<Option<AnyOfAllRequirements>>,
    dependents: Vec<Vec<RegionId>>,
}

impl Regions {
    pub fn new(regions: Vec<Region>) -> Self {
        debug_assert!(
            regions
                .iter()
                .enumerate()
                .all(|(index, region)| region.id() == RegionId::new(index))
        );
        let requirements = incoming_requirements(&regions);
        let dependents = dependents(&regions, &requirements);
        Self {
            regions,
            requirements,
            dependents,
        }
    }

    /// `id` の region に到達したとき、到達できるようになりうる region
    pub fn dependents(&self, id: RegionId) -> &[RegionId] {
        &self.dependents[id.0]
    }

    pub fn get(&self, id: RegionId) -> &Region {
        &self.regions[id.0]
    }

    /// region に到達するための要求。`None` は exit からは到達できないことを表す
    pub fn requirements(&self, id: RegionId) -> Option<&AnyOfAllRequirements> {
        self.requirements[id.0].as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
    }
}

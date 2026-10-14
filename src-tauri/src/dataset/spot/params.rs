use std::fmt::{self, Display};

use vec1::Vec1;

use crate::dataset::regions::{Region, RegionId, Regions};

#[derive(Clone, Debug, PartialEq)]
pub struct SpotName(String);

impl SpotName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequirementFlag(String);

impl RequirementFlag {
    pub fn new(requirement: String) -> Self {
        Self(requirement)
    }

    /// `sacredOrb:3` のような「接頭辞を持つ flag を N 個以上」の要求を (接頭辞, N) に分ける
    pub fn at_least(&self) -> Option<(&str, u8)> {
        let (prefix, count) = self.0.split_once(':')?;
        Some((prefix, count.parse().ok()?))
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec1<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec1<AllRequirements>);

/// spot を `region/種類(名前)` の形で表示する
pub struct SpotDisplay<'a> {
    region: &'a Region,
    type_name: &'static str,
    name: &'a SpotName,
}

impl<'a> SpotDisplay<'a> {
    pub fn new(region: &'a Region, type_name: &'static str, name: &'a SpotName) -> Self {
        Self {
            region,
            type_name,
            name,
        }
    }
}

impl Display for SpotDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}({})", self.region, self.type_name, self.name.get())
    }
}

#[derive(Clone, Debug)]
pub struct SpotParams<T> {
    pub region: RegionId,
    pub name: SpotName,
    pub content: T,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl<T> SpotParams<T> {
    pub fn new(
        region: RegionId,
        name: SpotName,
        content: T,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            region,
            name,
            content,
            requirements,
        }
    }

    pub fn display<'a>(&'a self, regions: &'a Regions, type_name: &'static str) -> SpotDisplay<'a> {
        SpotDisplay::new(regions.get(self.region), type_name, &self.name)
    }
}

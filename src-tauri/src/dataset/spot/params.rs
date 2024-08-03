use std::fmt;

use vec1::Vec1;

use crate::script::enums::FieldNumber;

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
        debug_assert!(
            !requirement.starts_with("sacredOrb:")
                || requirement
                    .split(':')
                    .nth(1)
                    .map_or(false, |x| x.parse::<u8>().is_ok())
        );
        Self(requirement)
    }

    pub fn is_sacred_orb(&self) -> bool {
        self.0.starts_with("sacredOrb:")
    }

    pub fn sacred_orb_count(&self) -> u8 {
        self.0.split(':').nth(1).unwrap().parse().unwrap()
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec1<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec1<AllRequirements>);

#[derive(Clone, Debug)]
pub struct SpotParams<T> {
    pub field_number: FieldNumber,
    pub name: SpotName,
    pub content: T,
    pub requirements: Option<AnyOfAllRequirements>,
}

impl<T> SpotParams<T> {
    pub fn new(
        field_number: FieldNumber,
        name: SpotName,
        content: T,
        requirements: Option<AnyOfAllRequirements>,
    ) -> Self {
        Self {
            field_number,
            name,
            content,
            requirements,
        }
    }

    pub fn fmt(&self, f: &mut fmt::Formatter<'_>, type_name: &str) -> fmt::Result {
        write!(
            f,
            "{:?}_{}({})",
            self.field_number,
            type_name,
            self.name.get()
        )
    }
}

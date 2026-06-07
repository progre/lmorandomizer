use std::collections::BTreeMap;

use anyhow::Result;

use super::spot::{AllRequirements, AnyOfAllRequirements, RequirementFlag};

#[derive(serde::Deserialize)]
pub struct FieldYaml(pub BTreeMap<RegionName, FieldYamlRegion>);

impl FieldYaml {
    pub fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct FieldYamlAccessRule(Vec<String>);

impl FieldYamlAccessRule {
    pub fn try_into_any_of_all_requirements(self) -> Result<Option<AnyOfAllRequirements>> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let requirements = self
            .0
            .into_iter()
            .map(|y| {
                y.split(',')
                    .map(|z| RequirementFlag::new(z.trim().to_owned()))
                    .collect::<Vec<_>>()
            })
            .map(|x| Ok(AllRequirements(x.try_into()?)))
            .collect::<Result<Vec<_>>>()?
            .try_into()?;
        Ok(Some(AnyOfAllRequirements(requirements)))
    }
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
pub struct RegionName(String);

impl RegionName {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Exits {
    pub up: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub down: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub left: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub right: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub door: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub warp: BTreeMap<RegionName, FieldYamlAccessRule>,
    pub fixed: BTreeMap<RegionName, FieldYamlAccessRule>,
}

impl Exits {
    pub fn all_exits(&self) -> impl Iterator<Item = (&RegionName, &FieldYamlAccessRule)> {
        self.up
            .iter()
            .chain(self.down.iter())
            .chain(self.left.iter())
            .chain(self.right.iter())
            .chain(self.door.iter())
            .chain(self.warp.iter())
            .chain(self.fixed.iter())
    }
}

impl<'de> serde::Deserialize<'de> for Exits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = BTreeMap::<String, FieldYamlAccessRule>::deserialize(deserializer)?;
        // up.name, down.name のような形式なので、up, down, left, right, door に分割する
        let mut exits = Self::default();
        for (key, value) in map {
            let (direction, name) = key.split_once('.').ok_or_else(|| {
                let msg = format!("invalid exit key: {key}, expected format is direction.name");
                serde::de::Error::custom(msg)
            })?;
            match direction {
                "up" => exits.up.insert(RegionName(name.to_owned()), value),
                "down" => exits.down.insert(RegionName(name.to_owned()), value),
                "left" => exits.left.insert(RegionName(name.to_owned()), value),
                "right" => exits.right.insert(RegionName(name.to_owned()), value),
                "door" => exits.door.insert(RegionName(name.to_owned()), value),
                "warp" => exits.warp.insert(RegionName(name.to_owned()), value),
                "fixed" => exits.fixed.insert(RegionName(name.to_owned()), value),
                _ => {
                    let msg = format!(
                        "invalid exit direction: {direction}, expected one of up, down, left, right, door"
                    );
                    return Err(serde::de::Error::custom(msg));
                }
            };
        }
        Ok(exits)
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldYamlRegion {
    #[serde(default)]
    pub access_rule: FieldYamlAccessRule,
    #[serde(default)]
    pub exits: Exits,
    #[serde(default)]
    pub main_weapons: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub sub_weapons: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub chests: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub seals: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub roms: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub shops: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub talks: BTreeMap<String, FieldYamlAccessRule>,
    #[serde(default)]
    pub events: BTreeMap<String, FieldYamlAccessRule>,
}

#[derive(serde::Deserialize)]
pub struct EventsYaml(pub BTreeMap<String, FieldYamlAccessRule>);

impl EventsYaml {
    pub fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

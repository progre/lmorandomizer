use std::collections::BTreeMap;

use anyhow::Result;

use super::spot::{AllRequirements, AnyOfAllRequirements, RequirementFlag};

#[derive(serde::Deserialize)]
pub struct FieldYaml(pub BTreeMap<String, FieldYamlRegion>);

impl FieldYaml {
    pub fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

#[derive(Default, serde::Deserialize)]
pub struct FieldYamlAccessRule(Vec<String>);

impl FieldYamlAccessRule {
    pub fn into_any_of_all_requirements(self) -> Result<Option<AnyOfAllRequirements>> {
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

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldYamlRegion {
    #[serde(default)]
    #[serde(rename = "accessRules")]
    pub access_rule: FieldYamlAccessRule,
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
}

#[derive(serde::Deserialize)]
pub struct EventsYaml(pub BTreeMap<String, FieldYamlAccessRule>);

impl EventsYaml {
    pub fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

use anyhow::{bail, Result};
use serde_yaml::{Mapping, Value};

use super::spot::FieldId;

fn try_parse_mapping(map: Mapping) -> Result<(String, Vec<String>)> {
    debug_assert_eq!(map.len(), 1);
    let (key, requirements) = map.into_iter().next().unwrap();
    let Value::String(key) = key else {
        bail!("key is not string")
    };
    let requirements = match requirements {
        Value::Null => vec![],
        Value::Sequence(requirements) => requirements,
        _ => bail!("requirements is not sequence"),
    };
    let requirements: Vec<_> = requirements
        .into_iter()
        .map(|x| {
            if let Value::String(x) = x {
                Ok(x)
            } else {
                bail!("requirement is not string")
            }
        })
        .collect::<Result<_>>()?;
    Ok((key, requirements))
}

fn try_parse_events(vec: Vec<Mapping>) -> Result<Vec<YamlSpot>> {
    vec.into_iter()
        .map(|map| {
            let (name, requirements) = try_parse_mapping(map)?;
            Ok(YamlSpot { name, requirements })
        })
        .collect()
}

fn try_parse_spots(vec: Vec<(FieldId, Mapping)>) -> Result<Vec<(FieldId, YamlSpot)>> {
    vec.into_iter()
        .map(|(field_id, map)| {
            debug_assert_eq!(map.len(), 1);
            let (name, requirements) = try_parse_mapping(map)?;
            Ok((field_id, YamlSpot { name, requirements }))
        })
        .collect()
}

fn try_parse_shops(vec: Vec<(FieldId, Mapping)>) -> Result<Vec<(FieldId, YamlShop)>> {
    vec.into_iter()
        .map(|(field_id, map)| {
            let (names, requirements) = try_parse_mapping(map)?;
            let shop = YamlShop {
                names,
                requirements,
            };
            Ok((field_id, shop))
        })
        .collect()
}

pub struct GameStructureFiles {
    pub fields: Vec<(FieldId, String)>,
    pub events: String,
}

impl GameStructureFiles {
    #[allow(clippy::type_complexity)]
    pub fn try_parse(
        &self,
    ) -> Result<(
        Vec<(FieldId, YamlSpot)>,
        Vec<(FieldId, YamlSpot)>,
        Vec<(FieldId, YamlSpot)>,
        Vec<(FieldId, YamlSpot)>,
        Vec<(FieldId, YamlShop)>,
        Vec<YamlSpot>,
    )> {
        let mut main_weapons = Vec::new();
        let mut sub_weapons = Vec::new();
        let mut chests = Vec::new();
        let mut seals = Vec::new();
        let mut shops = Vec::new();
        for (field_id, field_data) in &self.fields {
            let yaml: GameStructureYaml = serde_yaml::from_str(field_data)?;
            for item in yaml.main_weapons {
                main_weapons.push((*field_id, item));
            }
            for item in yaml.sub_weapons {
                sub_weapons.push((*field_id, item));
            }
            for item in yaml.chests {
                chests.push((*field_id, item));
            }
            for item in yaml.seals {
                seals.push((*field_id, item));
            }
            for item in yaml.shops {
                shops.push((*field_id, item));
            }
        }
        let events: SpotYaml = serde_yaml::from_str(&self.events)?;
        Ok((
            try_parse_spots(main_weapons)?,
            try_parse_spots(sub_weapons)?,
            try_parse_spots(chests)?,
            try_parse_spots(seals)?,
            try_parse_shops(shops)?,
            try_parse_events(events.0)?,
        ))
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameStructureYaml {
    #[serde(default)]
    pub main_weapons: Vec<Mapping>,
    #[serde(default)]
    pub sub_weapons: Vec<Mapping>,
    #[serde(default)]
    pub chests: Vec<Mapping>,
    #[serde(default)]
    pub seals: Vec<Mapping>,
    #[serde(default)]
    pub shops: Vec<Mapping>,
}

#[derive(serde::Deserialize)]
struct SpotYaml(Vec<Mapping>);

pub struct YamlSpot {
    pub name: String,
    pub requirements: Vec<String>,
}

pub struct YamlShop {
    pub names: String,
    pub requirements: Vec<String>,
}

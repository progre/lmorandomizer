use anyhow::{bail, Result};
use serde_yaml::{Mapping, Value};

fn try_parse_spots(vec: Vec<Mapping>) -> Result<Vec<YamlSpot>> {
    vec.into_iter()
        .map(|map| {
            debug_assert_eq!(map.len(), 1);
            let (name, requirements) = map.into_iter().next().unwrap();
            let Value::String(name) = name else {
                bail!("name is not string")
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
            Ok(YamlSpot { name, requirements })
        })
        .collect()
}

fn try_parse_shops(vec: Vec<Mapping>) -> Result<Vec<YamlShop>> {
    vec.into_iter()
        .map(|map| {
            debug_assert_eq!(map.len(), 1);
            let (names, requirements) = map.into_iter().next().unwrap();
            let Value::String(names) = names else {
                bail!("name is not string")
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
            Ok(YamlShop {
                names,
                requirements,
            })
        })
        .collect()
}

pub struct GameStructureFiles {
    pub fields: Vec<String>,
    pub events: String,
}

impl GameStructureFiles {
    #[allow(clippy::type_complexity)]
    pub fn try_parse(
        &self,
    ) -> Result<(
        Vec<YamlSpot>,
        Vec<YamlSpot>,
        Vec<YamlSpot>,
        Vec<YamlSpot>,
        Vec<YamlShop>,
        Vec<YamlSpot>,
    )> {
        let mut main_weapons = Vec::new();
        let mut sub_weapons = Vec::new();
        let mut chests = Vec::new();
        let mut seals = Vec::new();
        let mut shops = Vec::new();
        for field in &self.fields {
            let mut yaml: GameStructureYaml = serde_yaml::from_str(field)?;
            main_weapons.append(&mut yaml.main_weapons);
            sub_weapons.append(&mut yaml.sub_weapons);
            chests.append(&mut yaml.chests);
            seals.append(&mut yaml.seals);
            shops.append(&mut yaml.shops);
        }
        let events: SpotYaml = serde_yaml::from_str(&self.events)?;
        Ok((
            try_parse_spots(main_weapons)?,
            try_parse_spots(sub_weapons)?,
            try_parse_spots(chests)?,
            try_parse_spots(seals)?,
            try_parse_shops(shops)?,
            try_parse_spots(events.0)?,
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

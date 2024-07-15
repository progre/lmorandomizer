use anyhow::{bail, Result};
use serde_yaml::{Mapping, Value};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplementFiles {
    pub weapons_yml: String,
    pub chests_yml: String,
    pub seals_yml: String,
    pub shops_yml: String,
    pub events_yml: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct YamlSpot {
    pub name: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct YamlShop {
    pub names: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

fn try_parse(vec: Vec<Mapping>) -> Result<Vec<YamlSpot>> {
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponsYaml {
    main_weapons: Vec<Mapping>,
    sub_weapons: Vec<Mapping>,
}

impl WeaponsYaml {
    pub fn try_into_parsed_inner(self) -> Result<(Vec<YamlSpot>, Vec<YamlSpot>)> {
        Ok((try_parse(self.main_weapons)?, try_parse(self.sub_weapons)?))
    }
}

#[derive(serde::Deserialize)]
pub struct SpotYaml(Vec<Mapping>);

impl SpotYaml {
    pub fn try_into_parsed_inner(self) -> Result<Vec<YamlSpot>> {
        try_parse(self.0)
    }
}

#[derive(serde::Deserialize)]
pub struct ShopYaml(Vec<Mapping>);

impl ShopYaml {
    pub fn try_into_parsed_inner(self) -> Result<Vec<YamlShop>> {
        self.0
            .into_iter()
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
}

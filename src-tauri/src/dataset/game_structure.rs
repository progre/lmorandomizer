use std::collections::{BTreeMap, HashMap};

use anyhow::{bail, Result};
use num_traits::FromPrimitive;

use super::spot::FieldId;

pub struct GameStructureFiles {
    pub fields: Vec<(FieldId, FieldYaml)>,
    pub events: EventsYaml,
}

impl GameStructureFiles {
    pub fn new(fields: BTreeMap<FieldId, String>, events: String) -> Result<GameStructureFiles> {
        const ORDER_OF_FIELD_DATA: [u8; 19] = [
            1, 0, 2, 3, 4, 5, 6, 8, 9, 7, 17, 11, 12, 14, 13, 15, 16, 18, 19,
        ];
        let fields = ORDER_OF_FIELD_DATA
            .map(|x| {
                let field_id = FieldId::from_u8(x).unwrap();
                let yaml = FieldYaml::new(&fields[&field_id])?;
                Ok((field_id, yaml))
            })
            .into_iter()
            .collect::<Result<_>>()?;
        let events = EventsYaml::new(&events)?;
        Ok(Self { fields, events })
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldYaml {
    #[serde(default)]
    pub main_weapons: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub sub_weapons: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub chests: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub seals: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub shops: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub roms: HashMap<String, Vec<String>>,
}

impl FieldYaml {
    fn new(raw_str: &str) -> Result<Self> {
        let zelf: Self = serde_yaml::from_str(raw_str)?;
        if zelf
            .main_weapons
            .iter()
            .chain(&zelf.sub_weapons)
            .chain(&zelf.chests)
            .chain(&zelf.seals)
            .chain(&zelf.shops)
            .any(|x| x.len() != 1)
        {
            bail!("invalid data format");
        }
        Ok(zelf)
    }
}

#[derive(serde::Deserialize)]
pub struct EventsYaml(pub Vec<HashMap<String, Vec<String>>>);

impl EventsYaml {
    fn new(raw_str: &str) -> Result<Self> {
        let zelf: Self = serde_yaml::from_str(raw_str)?;
        if zelf.0.iter().any(|x| x.len() != 1) {
            bail!("invalid data format");
        }
        if zelf
            .0
            .iter()
            .flat_map(|x| x.values())
            .any(|requirements| requirements.is_empty())
        {
            bail!("invalid data format");
        }
        Ok(zelf)
    }
}

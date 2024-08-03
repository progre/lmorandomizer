use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use strum::ParseError;
use vec1::Vec1;

use crate::{
    dataset::spot::SpotName,
    script::enums::{
        ChestItem, Equipment, FieldNumber, MainWeapon, Rom, Seal, ShopItem, SubWeapon,
    },
};

use super::spot::{
    AllRequirements, AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RequirementFlag, RomSpot,
    SealSpot, ShopSpot, SubWeaponSpot,
};

pub struct GameStructureFiles {
    pub fields: Vec<(FieldNumber, FieldYaml)>,
    pub events: EventsYaml,
}

impl GameStructureFiles {
    pub fn new(fields: BTreeMap<u8, String>, events: String) -> Result<GameStructureFiles> {
        let fields = fields
            .into_iter()
            .map(|(field_logic_number, string)| {
                let field_number = FieldNumber::from_logic_number(field_logic_number).unwrap();
                let yaml = FieldYaml::new(&string)?;
                Ok((field_number, yaml))
            })
            .collect::<Result<_>>()?;
        let events = EventsYaml::new(&events)?;
        Ok(Self { fields, events })
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldYaml {
    #[serde(default)]
    pub main_weapons: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub sub_weapons: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub chests: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub seals: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub roms: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub shops: BTreeMap<String, Vec<String>>,
}

impl FieldYaml {
    fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

#[derive(serde::Deserialize)]
pub struct EventsYaml(pub BTreeMap<String, Vec<String>>);

impl EventsYaml {
    fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

fn to_any_of_all_requirements(requirements: Vec<String>) -> Result<Option<AnyOfAllRequirements>> {
    if requirements.is_empty() {
        return Ok(None);
    }
    let requirements = requirements
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

fn parse_event_requirements(items: BTreeMap<String, Vec<String>>) -> Result<Vec<Event>> {
    items
        .into_iter()
        .map(|(name, requirements)| {
            Ok(Event {
                name: SpotName::new(name),
                requirements: to_any_of_all_requirements(requirements)?.unwrap(),
            })
        })
        .collect()
}

fn to_pascal_case(camel_case: &str) -> String {
    camel_case[0..1]
        .to_uppercase()
        .chars()
        .chain(camel_case[1..].chars())
        .collect()
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: SpotName,
    pub requirements: AnyOfAllRequirements,
}

pub struct GameStructure {
    pub main_weapon_shutters: Vec<MainWeaponSpot>,
    pub sub_weapon_shutters: Vec<SubWeaponSpot>,
    pub chests: Vec<ChestSpot>,
    pub seals: Vec<SealSpot>,
    pub roadside_roms: Vec<RomSpot>,
    pub shops: Vec<ShopSpot>,
    pub events: Vec<Event>,
}

impl GameStructure {
    pub fn new(mut game_structure_files: GameStructureFiles) -> Result<Self> {
        let mut main_weapon_shutters = Vec::new();
        let mut sub_weapon_shutters = Vec::new();
        let mut chests = Vec::new();
        let mut seals = Vec::new();
        let mut roadside_roms = Vec::new();
        let mut shops = Vec::new();
        game_structure_files
            .fields
            .sort_by_key(|(field_number, _)| *field_number as u8);
        for (field_number, field_data) in game_structure_files.fields {
            for (key, value) in field_data.main_weapons {
                let main_weapon = MainWeapon::from_str(&to_pascal_case(&key))?;
                let name = SpotName::new(key.clone());
                let requirements = to_any_of_all_requirements(value)?;
                let spot = MainWeaponSpot::new(field_number, name, main_weapon, requirements);
                main_weapon_shutters.push(spot);
            }
            for (key, value) in field_data.sub_weapons {
                let sub_weapon =
                    SubWeapon::from_str(to_pascal_case(&key).split(":").next().unwrap())?;
                let name = SpotName::new(key.clone());
                let requirements = to_any_of_all_requirements(value)?;
                let spot = SubWeaponSpot::new(field_number, name, sub_weapon, requirements);
                sub_weapon_shutters.push(spot);
            }
            for (key, value) in field_data.chests {
                let pascal_case = to_pascal_case(&key);
                let pascal_case = pascal_case.split(":").next().unwrap();
                let item = Equipment::from_str(pascal_case)
                    .map(ChestItem::Equipment)
                    .or_else(|_| Rom::from_str(pascal_case).map(ChestItem::Rom))?;
                let name = SpotName::new(key.clone());
                let requirements = to_any_of_all_requirements(value)?;
                let spot = ChestSpot::new(field_number, name, item, requirements);
                chests.push(spot);
            }
            for (key, value) in field_data.seals {
                let seal = Seal::from_str(&to_pascal_case(&key.replace("Seal", "")))?;
                let name = SpotName::new(key.clone());
                let requirements = to_any_of_all_requirements(value)?;
                let spot = SealSpot::new(field_number, name, seal, requirements);
                seals.push(spot);
            }
            for (key, value) in field_data.roms {
                let rom = Rom::from_str(&to_pascal_case(&key))?;
                let name = SpotName::new(key.clone());
                let requirements = to_any_of_all_requirements(value)?
                    .map(|mut any_of_all_requirements| {
                        for all_requirements in &mut any_of_all_requirements.0 {
                            let hand_scanner = RequirementFlag::new("handScanner".into());
                            all_requirements.0.push(hand_scanner);
                        }
                        any_of_all_requirements
                    })
                    .unwrap_or_else(|| {
                        let hand_scanner = RequirementFlag::new("handScanner".into());
                        AnyOfAllRequirements(Vec1::new(AllRequirements(Vec1::new(hand_scanner))))
                    });
                roadside_roms.push(RomSpot::new(field_number, name, rom, requirements));
            }
            for (key, value) in field_data.shops {
                let items: Vec<_> = key
                    .split(',')
                    .map(|x| {
                        let name = x.trim();
                        if name == "_" {
                            return Ok(None);
                        }
                        let pascal_case = to_pascal_case(name);
                        let pascal_case = pascal_case
                            .split(":")
                            .next()
                            .unwrap()
                            .split("Ammo")
                            .next()
                            .unwrap();
                        let item = SubWeapon::from_str(pascal_case)
                            .map(ShopItem::SubWeapon)
                            .or_else(|_| Equipment::from_str(pascal_case).map(ShopItem::Equipment))
                            .or_else(|_| Rom::from_str(pascal_case).map(ShopItem::Rom))?;
                        Ok(Some(item))
                    })
                    .collect::<Result<_, ParseError>>()?;
                let name = SpotName::new(key);
                let any_of_all_requirements = to_any_of_all_requirements(value)?;
                let mut items = items.into_iter();
                let items = (
                    items.next().unwrap(),
                    items.next().unwrap(),
                    items.next().unwrap(),
                );
                let spot = ShopSpot::new(field_number, name, items, any_of_all_requirements);
                shops.push(spot)
            }
        }
        let events = parse_event_requirements(game_structure_files.events.0)?;

        Ok(Self {
            main_weapon_shutters,
            sub_weapon_shutters,
            chests,
            seals,
            shops,
            roadside_roms,
            events,
        })
    }
}

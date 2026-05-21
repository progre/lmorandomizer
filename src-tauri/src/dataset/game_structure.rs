use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use strum::ParseError;
use vec1::Vec1;

use crate::{
    dataset::spot::{Region, SpotName},
    script::enums::{
        ChestItem, Equipment, FieldNumber, MainWeapon, Rom, Seal, ShopItem, SubWeapon, TalkItem,
    },
};

use super::spot::{
    AllRequirements, AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RequirementFlag, RomSpot,
    SealSpot, ShopSpot, SubWeaponSpot, TalkSpot,
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
                let field_tables = FieldYaml::new(&string)?;
                Ok((field_number, field_tables))
            })
            .collect::<Result<Vec<_>>>()?;
        let events = EventsYaml::new(&events)?;
        Ok(Self { fields, events })
    }
}

#[derive(serde::Deserialize)]
pub struct FieldYaml(pub BTreeMap<String, FieldYamlRegion>);

impl FieldYaml {
    fn new(raw_str: &str) -> serde_yaml::Result<Self> {
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
    fn new(raw_str: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(raw_str)
    }
}

fn parse_event_requirements(items: BTreeMap<String, FieldYamlAccessRule>) -> Result<Vec<Event>> {
    items
        .into_iter()
        .map(|(name, access_rule)| {
            Ok(Event {
                name: SpotName::new(name),
                requirements: access_rule.into_any_of_all_requirements()?.unwrap(),
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
    pub regions: Vec<Region>,
    pub main_weapon_shutters: Vec<MainWeaponSpot>,
    pub sub_weapon_shutters: Vec<SubWeaponSpot>,
    pub chests: Vec<ChestSpot>,
    pub seals: Vec<SealSpot>,
    pub roadside_roms: Vec<RomSpot>,
    pub shops: Vec<ShopSpot>,
    pub talks: Vec<TalkSpot>,
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
        let mut talks = Vec::new();
        game_structure_files
            .fields
            .sort_by_key(|(field_number, _)| *field_number as u8);
        let mut regions = vec![];
        for (field_number, field_yaml) in game_structure_files.fields {
            for (region_name, field_yaml_region) in field_yaml.0 {
                let region = Region::new(
                    field_number,
                    region_name,
                    field_yaml_region
                        .access_rule
                        .into_any_of_all_requirements()?,
                );
                for (item, access_rule) in field_yaml_region.main_weapons {
                    let location = main_weapon_location(region.clone(), item, access_rule)?;
                    main_weapon_shutters.push(location);
                }
                for (key, value) in field_yaml_region.sub_weapons {
                    sub_weapon_shutters.push(sub_weapon_location(region.clone(), key, value)?);
                }
                for (key, value) in field_yaml_region.chests {
                    chests.push(chest_location(region.clone(), key, value)?);
                }
                for (key, value) in field_yaml_region.seals {
                    seals.push(seals_location(region.clone(), key, value)?);
                }
                for (key, value) in field_yaml_region.roms {
                    roadside_roms.push(rom_location(region.clone(), key, value)?);
                }
                for (key, value) in field_yaml_region.shops {
                    shops.push(shop_locations(region.clone(), key, value)?)
                }
                for (key, value) in field_yaml_region.talks {
                    talks.push(talk_location(region.clone(), key, value)?);
                }
                regions.push(region);
            }
        }
        let events = parse_event_requirements(game_structure_files.events.0)?;

        Ok(Self {
            regions,
            main_weapon_shutters,
            sub_weapon_shutters,
            chests,
            seals,
            roadside_roms,
            shops,
            talks,
            events,
        })
    }
}

fn talk_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<TalkSpot, anyhow::Error> {
    let pascal_case = to_pascal_case(&key);
    let item = Equipment::from_str(&pascal_case)
        .map(TalkItem::Equipment)
        .or_else(|_| Rom::from_str(&pascal_case).map(TalkItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = value.into_any_of_all_requirements()?;
    let spot = TalkSpot::new(region, name, item, requirements);
    Ok(spot)
}

fn shop_locations(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<ShopSpot, anyhow::Error> {
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
    let any_of_all_requirements = value.into_any_of_all_requirements()?;
    let items = [items[0], items[1], items[2]];
    let spot = ShopSpot::new(region, name, items, any_of_all_requirements);
    Ok(spot)
}

fn rom_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<RomSpot, anyhow::Error> {
    let rom = Rom::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let requirements = value
        .into_any_of_all_requirements()?
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
    let spot = RomSpot::new(region, name, rom, requirements);
    Ok(spot)
}

fn seals_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<SealSpot, anyhow::Error> {
    let seal = Seal::from_str(&to_pascal_case(&key.replace("Seal", "")))?;
    let name = SpotName::new(key.clone());
    let requirements = value.into_any_of_all_requirements()?;
    let spot = SealSpot::new(region, name, seal, requirements);
    Ok(spot)
}

fn chest_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<ChestSpot, anyhow::Error> {
    let pascal_case = to_pascal_case(&key);
    let pascal_case = pascal_case.split(":").next().unwrap();
    let item = Equipment::from_str(pascal_case)
        .map(ChestItem::Equipment)
        .or_else(|_| Rom::from_str(pascal_case).map(ChestItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = value.into_any_of_all_requirements()?;
    let spot = ChestSpot::new(region, name, item, requirements);
    Ok(spot)
}

fn sub_weapon_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<SubWeaponSpot, anyhow::Error> {
    let sub_weapon = SubWeapon::from_str(to_pascal_case(&key).split(":").next().unwrap())?;
    let name = SpotName::new(key.clone());
    let requirements = value.into_any_of_all_requirements()?;
    let spot = SubWeaponSpot::new(region, name, sub_weapon, requirements);
    Ok(spot)
}

fn main_weapon_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<MainWeaponSpot, anyhow::Error> {
    let main_weapon = MainWeapon::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let requirements = value.into_any_of_all_requirements()?;
    let spot = MainWeaponSpot::new(region, name, main_weapon, requirements);
    Ok(spot)
}

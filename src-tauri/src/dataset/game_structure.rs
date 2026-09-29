use std::collections::BTreeMap;

use anyhow::Result;

use crate::{
    dataset::{
        files::{EventsYaml, FieldYaml, FieldYamlAccessRule},
        spot::{Region, RegionExit, SpotName},
        spot_locations::{
            chest_location, main_weapon_location, rom_location, seals_location, shop_locations,
            sub_weapon_location, talk_location,
        },
        validate_exits::validate_exits,
    },
    script::enums::FieldNumber,
};

use super::spot::{
    AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RomSpot, SealSpot, ShopSpot, SubWeaponSpot,
    TalkSpot,
};

pub use super::files::RegionName;

fn parse_event_requirements(items: BTreeMap<String, FieldYamlAccessRule>) -> Result<Vec<Event>> {
    items
        .into_iter()
        .map(|(name, access_rule)| {
            Ok(Event {
                region: None,
                name: SpotName::new(name),
                requirements: access_rule.try_into_any_of_all_requirements()?,
            })
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct Event {
    pub region: Option<Region>,
    pub name: SpotName,
    pub requirements: Option<AnyOfAllRequirements>,
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
    pub fn new(fields: BTreeMap<u8, String>, events: String) -> Result<Self> {
        let mut fields = fields
            .into_iter()
            .map(|(field_logic_number, string)| {
                let field_number = FieldNumber::from_logic_number(field_logic_number).unwrap();
                let field_tables = FieldYaml::new(&string)?;
                Ok((field_number, field_tables))
            })
            .collect::<Result<Vec<_>>>()?;
        fields.sort_by_key(|(field_number, _)| *field_number as u8);
        let events = EventsYaml::new(&events)?;

        let mut events = parse_event_requirements(events.0)?;

        let mut main_weapon_shutters = Vec::new();
        let mut sub_weapon_shutters = Vec::new();
        let mut chests = Vec::new();
        let mut seals = Vec::new();
        let mut roadside_roms = Vec::new();
        let mut shops = Vec::new();
        let mut talks = Vec::new();
        let mut regions = vec![];
        for (field_number, field_yaml) in fields {
            for (region_name, field_yaml_region) in field_yaml.0 {
                let exits = field_yaml_region
                    .exits
                    .into_all_exits()
                    .map(|(direction, target, access_rule)| {
                        Ok(RegionExit {
                            direction,
                            target,
                            requirements: access_rule.try_into_any_of_all_requirements()?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let region = Region::new(
                    field_number,
                    region_name,
                    field_yaml_region
                        .access_rule
                        .try_into_any_of_all_requirements()?,
                    exits,
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
                for (key, value) in field_yaml_region.events {
                    events.push(event_location(region.clone(), key, value)?);
                }
                regions.push(region);
            }
        }

        if cfg!(debug_assertions) {
            validate_exits(&regions);
        }

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

fn event_location(region: Region, key: String, value: FieldYamlAccessRule) -> Result<Event> {
    Ok(Event {
        region: Some(region),
        name: SpotName::new(key),
        requirements: value.try_into_any_of_all_requirements()?,
    })
}

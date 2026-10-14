use std::str::FromStr;

use anyhow::Result;
use strum::ParseError;

use crate::{
    dataset::{
        files::FieldYamlAccessRule,
        regions::Region,
        spot::{
            AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RequirementFlag, RomSpot, SealSpot,
            ShopSpot, SpotName, SubWeaponSpot, TalkSpot,
        },
    },
    script::enums::{ChestItem, Equipment, MainWeapon, Rom, Seal, ShopItem, SubWeapon, TalkItem},
};

fn to_pascal_case(camel_case: &str) -> String {
    camel_case[0..1]
        .to_uppercase()
        .chars()
        .chain(camel_case[1..].chars())
        .collect()
}

/// spot の要求に、spot がある region への到達を足す
pub fn requirements_in_region(
    region: &Region,
    value: FieldYamlAccessRule,
) -> Result<AnyOfAllRequirements> {
    let requirements = value.try_into_any_of_all_requirements()?;
    Ok(AnyOfAllRequirements::require(
        region.flag().clone(),
        requirements,
    ))
}

pub fn talk_location(region: &Region, key: String, value: FieldYamlAccessRule) -> Result<TalkSpot> {
    let pascal_case = to_pascal_case(&key);
    let item = Equipment::from_str(&pascal_case)
        .map(TalkItem::Equipment)
        .or_else(|_| Rom::from_str(&pascal_case).map(TalkItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = requirements_in_region(region, value)?;
    let spot = TalkSpot::new(region.id(), name, item, Some(requirements));
    Ok(spot)
}

pub fn shop_locations(
    region: &Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<ShopSpot> {
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
    let requirements = requirements_in_region(region, value)?;
    let items = [items[0], items[1], items[2]];
    let spot = ShopSpot::new(region.id(), name, items, Some(requirements));
    Ok(spot)
}

pub fn rom_location(region: &Region, key: String, value: FieldYamlAccessRule) -> Result<RomSpot> {
    let rom = Rom::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let hand_scanner = RequirementFlag::new("handScanner".into());
    let requirements = requirements_in_region(region, value)?;
    let requirements = AnyOfAllRequirements::require(hand_scanner, Some(requirements));
    let spot = RomSpot::new(region.id(), name, rom, requirements);
    Ok(spot)
}

pub fn seals_location(
    region: &Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<SealSpot> {
    let seal = Seal::from_str(&to_pascal_case(&key.replace("Seal", "")))?;
    let name = SpotName::new(key.clone());
    let requirements = requirements_in_region(region, value)?;
    let spot = SealSpot::new(region.id(), name, seal, Some(requirements));
    Ok(spot)
}

pub fn chest_location(
    region: &Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<ChestSpot> {
    let pascal_case = to_pascal_case(&key);
    let pascal_case = pascal_case.split(":").next().unwrap();
    let item = Equipment::from_str(pascal_case)
        .map(ChestItem::Equipment)
        .or_else(|_| Rom::from_str(pascal_case).map(ChestItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = requirements_in_region(region, value)?;
    let spot = ChestSpot::new(region.id(), name, item, Some(requirements));
    Ok(spot)
}

pub fn sub_weapon_location(
    region: &Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<SubWeaponSpot> {
    let sub_weapon = SubWeapon::from_str(to_pascal_case(&key).split(":").next().unwrap())?;
    let name = SpotName::new(key.clone());
    let requirements = requirements_in_region(region, value)?;
    let spot = SubWeaponSpot::new(region.id(), name, sub_weapon, Some(requirements));
    Ok(spot)
}

pub fn main_weapon_location(
    region: &Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<MainWeaponSpot> {
    let main_weapon = MainWeapon::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let requirements = requirements_in_region(region, value)?;
    let spot = MainWeaponSpot::new(region.id(), name, main_weapon, Some(requirements));
    Ok(spot)
}

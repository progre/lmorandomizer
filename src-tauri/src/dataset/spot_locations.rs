use std::str::FromStr;

use anyhow::Result;
use strum::ParseError;
use vec1::Vec1;

use crate::{
    dataset::{
        files::FieldYamlAccessRule,
        spot::{
            AllRequirements, AnyOfAllRequirements, ChestSpot, MainWeaponSpot, Region,
            RequirementFlag, RomSpot, SealSpot, ShopSpot, SpotName, SubWeaponSpot, TalkSpot,
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

pub fn talk_location(region: Region, key: String, value: FieldYamlAccessRule) -> Result<TalkSpot> {
    let pascal_case = to_pascal_case(&key);
    let item = Equipment::from_str(&pascal_case)
        .map(TalkItem::Equipment)
        .or_else(|_| Rom::from_str(&pascal_case).map(TalkItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = value.try_into_any_of_all_requirements()?;
    let spot = TalkSpot::new(region, name, item, requirements);
    Ok(spot)
}

pub fn shop_locations(region: Region, key: String, value: FieldYamlAccessRule) -> Result<ShopSpot> {
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
    let any_of_all_requirements = value.try_into_any_of_all_requirements()?;
    let items = [items[0], items[1], items[2]];
    let spot = ShopSpot::new(region, name, items, any_of_all_requirements);
    Ok(spot)
}

pub fn rom_location(region: Region, key: String, value: FieldYamlAccessRule) -> Result<RomSpot> {
    let rom = Rom::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let requirements = value
        .try_into_any_of_all_requirements()?
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

pub fn seals_location(region: Region, key: String, value: FieldYamlAccessRule) -> Result<SealSpot> {
    let seal = Seal::from_str(&to_pascal_case(&key.replace("Seal", "")))?;
    let name = SpotName::new(key.clone());
    let requirements = value.try_into_any_of_all_requirements()?;
    let spot = SealSpot::new(region, name, seal, requirements);
    Ok(spot)
}

pub fn chest_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<ChestSpot> {
    let pascal_case = to_pascal_case(&key);
    let pascal_case = pascal_case.split(":").next().unwrap();
    let item = Equipment::from_str(pascal_case)
        .map(ChestItem::Equipment)
        .or_else(|_| Rom::from_str(pascal_case).map(ChestItem::Rom))?;
    let name = SpotName::new(key.clone());
    let requirements = value.try_into_any_of_all_requirements()?;
    let spot = ChestSpot::new(region, name, item, requirements);
    Ok(spot)
}

pub fn sub_weapon_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<SubWeaponSpot> {
    let sub_weapon = SubWeapon::from_str(to_pascal_case(&key).split(":").next().unwrap())?;
    let name = SpotName::new(key.clone());
    let requirements = value.try_into_any_of_all_requirements()?;
    let spot = SubWeaponSpot::new(region, name, sub_weapon, requirements);
    Ok(spot)
}

pub fn main_weapon_location(
    region: Region,
    key: String,
    value: FieldYamlAccessRule,
) -> Result<MainWeaponSpot> {
    let main_weapon = MainWeapon::from_str(&to_pascal_case(&key))?;
    let name = SpotName::new(key.clone());
    let requirements = value.try_into_any_of_all_requirements()?;
    let spot = MainWeaponSpot::new(region, name, main_weapon, requirements);
    Ok(spot)
}

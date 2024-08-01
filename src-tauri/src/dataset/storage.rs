use std::collections::BTreeMap;

use anyhow::Result;

use crate::script::data::items;

use super::{
    assertions::ware_missing_requirements,
    item::{Item, StrategyFlag},
    spot::{
        AnyOfAllRequirements, ChestItem, ChestSpot, FieldId, MainWeaponSpot, RomSpot, SealSpot,
        ShopSpot, SubWeaponSpot,
    },
};

#[derive(Clone, Debug)]
pub struct MainWeapon {
    pub spot: MainWeaponSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct SubWeapon {
    pub spot: SubWeaponSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Chest {
    pub spot: ChestSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Seal {
    pub spot: SealSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Shop {
    pub spot: ShopSpot,
    pub items: (Option<Item>, Option<Item>, Option<Item>),
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub spot: RomSpot,
    pub item: Item,
}

pub struct MainWeaponRef<'a> {
    pub spot: &'a MainWeaponSpot,
    pub item: &'a Item,
}

pub struct SubWeaponRef<'a> {
    pub spot: &'a SubWeaponSpot,
    pub item: &'a Item,
}

pub struct ChestRef<'a> {
    pub spot: &'a ChestSpot,
    pub item: &'a Item,
}

pub struct SealRef<'a> {
    pub spot: &'a SealSpot,
    pub item: &'a Item,
}

pub struct ShopRef<'a> {
    pub spot: &'a ShopSpot,
    pub items: (Option<&'a Item>, Option<&'a Item>, Option<&'a Item>),
}

pub struct RomRef<'a> {
    pub spot: &'a RomSpot,
    pub item: &'a Item,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: StrategyFlag,
    pub requirements: AnyOfAllRequirements,
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub main_weapons: BTreeMap<items::MainWeapon, MainWeapon>,
    pub sub_weapons: BTreeMap<(FieldId, items::SubWeapon), SubWeapon>,
    pub chests: BTreeMap<(FieldId, ChestItem), Chest>,
    pub seals: BTreeMap<items::Seal, Seal>,
    pub shops: Vec<Shop>,
    pub roms: BTreeMap<items::Rom, Rom>,
    pub events: Vec<Event>,
}

impl Storage {
    pub fn new(
        main_weapons: BTreeMap<items::MainWeapon, MainWeapon>,
        sub_weapons: BTreeMap<(FieldId, items::SubWeapon), SubWeapon>,
        chests: BTreeMap<(FieldId, ChestItem), Chest>,
        seals: BTreeMap<items::Seal, Seal>,
        shops: Vec<Shop>,
        roms: BTreeMap<items::Rom, Rom>,
        events: Vec<Event>,
    ) -> Result<Self> {
        let zelf = Self {
            main_weapons,
            sub_weapons,
            chests,
            seals,
            shops,
            roms,
            events,
        };
        if cfg!(debug_assertions) {
            ware_missing_requirements(&zelf)?;
        }
        Ok(zelf)
    }

    pub fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.main_weapons
            .values()
            .map(|x| &x.item)
            .chain(self.sub_weapons.values().map(|x| &x.item))
            .chain(self.chests.values().map(|x| &x.item))
            .chain(self.seals.values().map(|x| &x.item))
            .chain(
                self.shops
                    .iter()
                    .flat_map(|x| [&x.items.0, &x.items.1, &x.items.2])
                    .filter_map(|x| x.as_ref()),
            )
            .chain(self.roms.values().map(|x| &x.item))
    }
}

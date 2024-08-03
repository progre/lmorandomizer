mod assertions;
pub mod create_source;
pub mod item;

use std::collections::BTreeMap;

use anyhow::Result;

use crate::{
    dataset::spot::{
        AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RomSpot, SealSpot, ShopSpot,
        SubWeaponSpot, TalkSpot,
    },
    script::enums::{self, FieldNumber},
};

use {
    assertions::ware_missing_requirements,
    item::{Item, StrategyFlag},
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
    pub items: [Option<Item>; 3],
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub spot: RomSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Talk {
    pub spot: TalkSpot,
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
    pub items: [Option<&'a Item>; 3],
}

pub struct RomRef<'a> {
    pub spot: &'a RomSpot,
    pub item: &'a Item,
}

pub struct TalkRef<'a> {
    pub spot: &'a TalkSpot,
    pub item: &'a Item,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: StrategyFlag,
    pub requirements: AnyOfAllRequirements,
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub main_weapons: BTreeMap<enums::MainWeapon, MainWeapon>,
    pub sub_weapons: BTreeMap<(FieldNumber, enums::SubWeapon), SubWeapon>,
    pub chests: BTreeMap<(FieldNumber, enums::ChestItem), Chest>,
    pub seals: BTreeMap<enums::Seal, Seal>,
    pub roms: BTreeMap<enums::Rom, Rom>,
    pub talks: Vec<Talk>,
    pub shops: Vec<Shop>,
    pub events: Vec<Event>,
}

impl Storage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        main_weapons: BTreeMap<enums::MainWeapon, MainWeapon>,
        sub_weapons: BTreeMap<(FieldNumber, enums::SubWeapon), SubWeapon>,
        chests: BTreeMap<(FieldNumber, enums::ChestItem), Chest>,
        seals: BTreeMap<enums::Seal, Seal>,
        roms: BTreeMap<enums::Rom, Rom>,
        talks: Vec<Talk>,
        shops: Vec<Shop>,
        events: Vec<Event>,
    ) -> Result<Self> {
        let zelf = Self {
            main_weapons,
            sub_weapons,
            chests,
            seals,
            roms,
            talks,
            shops,
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
                    .flat_map(|x| &x.items)
                    .filter_map(|x| x.as_ref()),
            )
            .chain(self.roms.values().map(|x| &x.item))
            .chain(self.talks.iter().map(|x| &x.item))
    }
}

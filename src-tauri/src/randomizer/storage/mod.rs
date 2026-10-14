pub mod apply_options;
mod assertions;
pub mod create_source;
pub mod ids;
pub mod item;

use std::collections::BTreeMap;

use anyhow::Result;

use crate::{
    dataset::{
        regions::{RegionId, Regions},
        spot::{
            AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RomSpot, SealSpot, ShopSpot,
            SubWeaponSpot, TalkSpot,
        },
    },
    script::enums::{self, FieldNumber},
};

use {
    assertions::ware_missing_requirements,
    ids::LocationId,
    item::{Item, StrategyFlag},
};

#[derive(Clone, Debug)]
pub struct MainWeapon {
    pub id: LocationId,
    pub spot: MainWeaponSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct SubWeapon {
    pub id: LocationId,
    pub spot: SubWeaponSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Chest {
    pub id: LocationId,
    pub spot: ChestSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Seal {
    pub id: LocationId,
    pub spot: SealSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Shop {
    pub id: LocationId,
    pub spot: ShopSpot,
    pub idx: usize,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub id: LocationId,
    pub spot: RomSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Talk {
    pub id: LocationId,
    pub spot: TalkSpot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub region: Option<RegionId>,
    pub name: StrategyFlag,
    pub requirements: Option<AnyOfAllRequirements>,
}

#[derive(Clone, Debug)]
pub struct Storage {
    pub regions: Regions,
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
        regions: Regions,
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
            regions,
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
            .chain(self.shops.iter().map(|x| &x.item))
            .chain(self.roms.values().map(|x| &x.item))
            .chain(self.talks.iter().map(|x| &x.item))
    }

    /// 全 location の ID と、そこに置かれている item
    pub fn items_mut(&mut self) -> impl Iterator<Item = (LocationId, &mut Item)> {
        let main_weapons = self.main_weapons.values_mut().map(|x| (x.id, &mut x.item));
        let sub_weapons = self.sub_weapons.values_mut().map(|x| (x.id, &mut x.item));
        let chests = self.chests.values_mut().map(|x| (x.id, &mut x.item));
        let seals = self.seals.values_mut().map(|x| (x.id, &mut x.item));
        let shops = self.shops.iter_mut().map(|x| (x.id, &mut x.item));
        let roms = self.roms.values_mut().map(|x| (x.id, &mut x.item));
        let talks = self.talks.iter_mut().map(|x| (x.id, &mut x.item));
        main_weapons
            .chain(sub_weapons)
            .chain(chests)
            .chain(seals)
            .chain(shops)
            .chain(roms)
            .chain(talks)
    }
}

use crate::{
    dataset::spot::{
        AnyOfAllRequirements, ChestSpot, MainWeaponSpot, RomSpot, SealSpot, ShopSpot, SubWeaponSpot,
    },
    randomizer::storage::{Event, Storage},
    script::enums::FieldNumber,
};

use super::sphere::ShopItemDisplay;

#[derive(Clone, Copy)]
pub enum SpotRef<'a> {
    MainWeapon(&'a MainWeaponSpot),
    SubWeapon(&'a SubWeaponSpot),
    Chest(&'a ChestSpot),
    Seal(&'a SealSpot),
    Shop(&'a ShopSpot),
    Rom(&'a RomSpot),
}

impl SpotRef<'_> {
    pub fn field_number(&self) -> FieldNumber {
        match self {
            Self::MainWeapon(x) => x.field_number(),
            Self::SubWeapon(x) => x.field_number(),
            Self::Chest(x) => x.field_number(),
            Self::Seal(x) => x.field_number(),
            Self::Shop(x) => x.field_number(),
            Self::Rom(x) => x.field_number(),
        }
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        match self {
            Self::MainWeapon(x) => x.requirements(),
            Self::SubWeapon(x) => x.requirements(),
            Self::Chest(x) => x.requirements(),
            Self::Seal(x) => x.requirements(),
            Self::Shop(x) => x.requirements(),
            Self::Rom(x) => Some(x.requirements()),
        }
    }
}

#[derive(Clone)]
pub struct Spots<'a> {
    pub field_item_spots: Vec<SpotRef<'a>>,
    pub shops: Vec<ShopItemDisplay<'a>>,
    pub events: Vec<&'a Event>,
}

impl<'a> Spots<'a> {
    pub fn new(source: &'a Storage) -> Self {
        Self {
            field_item_spots: source
                .main_weapons
                .values()
                .map(|x| SpotRef::MainWeapon(&x.spot))
                .chain(
                    source
                        .sub_weapons
                        .values()
                        .map(|x| SpotRef::SubWeapon(&x.spot)),
                )
                .chain(source.chests.values().map(|x| SpotRef::Chest(&x.spot)))
                .chain(source.seals.values().map(|x| SpotRef::Seal(&x.spot)))
                .chain(source.roms.values().map(|x| SpotRef::Rom(&x.spot)))
                .collect(),
            shops: source
                .shops
                .iter()
                .flat_map(|shop| {
                    [&shop.items.0, &shop.items.1, &shop.items.2]
                        .into_iter()
                        .enumerate()
                        .filter_map(|(idx, item)| {
                            item.as_ref().map(|item| ShopItemDisplay {
                                spot: &shop.spot,
                                idx,
                                name: &item.name,
                            })
                        })
                })
                .collect(),
            events: source.events.iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.field_item_spots.is_empty() && self.shops.iter().all(|shop| shop.name.is_consumable())
    }
}

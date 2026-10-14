use crate::{
    dataset::{regions::RegionId, spot::AnyOfAllRequirements},
    randomizer::storage::{
        Chest, Event, MainWeapon, Rom, Seal, Shop, Storage, SubWeapon, Talk, ids::LocationId,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum LocationRef<'a> {
    MainWeapon(&'a MainWeapon),
    SubWeapon(&'a SubWeapon),
    Chest(&'a Chest),
    Seal(&'a Seal),
    Rom(&'a Rom),
    Talk(&'a Talk),
    Shop(&'a Shop),
}

impl LocationRef<'_> {
    pub fn id(&self) -> LocationId {
        match self {
            Self::MainWeapon(x) => x.id,
            Self::SubWeapon(x) => x.id,
            Self::Chest(x) => x.id,
            Self::Seal(x) => x.id,
            Self::Rom(x) => x.id,
            Self::Talk(x) => x.id,
            Self::Shop(x) => x.id,
        }
    }
    pub fn region(&self) -> RegionId {
        match self {
            Self::MainWeapon(x) => x.spot.region(),
            Self::SubWeapon(x) => x.spot.region(),
            Self::Chest(x) => x.spot.region(),
            Self::Seal(x) => x.spot.region(),
            Self::Rom(x) => x.spot.region(),
            Self::Talk(x) => x.spot.region(),
            Self::Shop(x) => x.spot.region(),
        }
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        match self {
            Self::MainWeapon(x) => x.spot.requirements(),
            Self::SubWeapon(x) => x.spot.requirements(),
            Self::Chest(x) => x.spot.requirements(),
            Self::Seal(x) => x.spot.requirements(),
            Self::Rom(x) => Some(x.spot.requirements()),
            Self::Talk(x) => x.spot.requirements(),
            Self::Shop(x) => x.spot.requirements(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Locations<'a> {
    pub field_item_locations: Vec<LocationRef<'a>>,
    pub talk_locations: Vec<&'a Talk>,
    pub shops: Vec<&'a Shop>,
    pub events: Vec<&'a Event>,
}

impl<'a> Locations<'a> {
    pub fn new(source: &'a Storage) -> Self {
        Self {
            field_item_locations: source
                .main_weapons
                .values()
                .map(LocationRef::MainWeapon)
                .chain(source.sub_weapons.values().map(LocationRef::SubWeapon))
                .chain(source.chests.values().map(LocationRef::Chest))
                .chain(source.seals.values().map(LocationRef::Seal))
                .chain(source.roms.values().map(LocationRef::Rom))
                .collect(),
            talk_locations: source.talks.iter().collect(),
            shops: source.shops.iter().collect(),
            events: source.events.iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.field_item_locations.is_empty()
            && self.talk_locations.is_empty()
            && self.shops.iter().all(|shop| shop.item.name.is_consumable())
    }

    pub fn extend(&mut self, other: Self) {
        self.field_item_locations.extend(other.field_item_locations);
        self.talk_locations.extend(other.talk_locations);
        self.shops.extend(other.shops);
        self.events.extend(other.events);
    }
}

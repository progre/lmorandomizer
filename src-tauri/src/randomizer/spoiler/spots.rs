use crate::{
    dataset::{
        regions::{RegionId, Regions},
        spot::{AnyOfAllRequirements, SpotDisplay},
    },
    randomizer::storage::{
        Chest, Event, MainWeapon, Rom, Seal, Shop, Storage, SubWeapon, Talk,
        ids::{SpotId, SpotTable},
        item::Item,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum SpotRef<'a> {
    MainWeapon(&'a MainWeapon),
    SubWeapon(&'a SubWeapon),
    Chest(&'a Chest),
    Seal(&'a Seal),
    Rom(&'a Rom),
    Talk(&'a Talk),
    Shop(&'a Shop),
}

impl<'a> SpotRef<'a> {
    /// この spot に元々置かれている item
    pub fn item(&self) -> &'a Item {
        match self {
            Self::MainWeapon(x) => &x.item,
            Self::SubWeapon(x) => &x.item,
            Self::Chest(x) => &x.item,
            Self::Seal(x) => &x.item,
            Self::Rom(x) => &x.item,
            Self::Talk(x) => &x.item,
            Self::Shop(x) => &x.item,
        }
    }
    pub fn display(&self, regions: &'a Regions) -> SpotDisplay<'a> {
        match self {
            Self::MainWeapon(x) => x.spot.display(regions),
            Self::SubWeapon(x) => x.spot.display(regions),
            Self::Chest(x) => x.spot.display(regions),
            Self::Seal(x) => x.spot.display(regions),
            Self::Rom(x) => x.spot.display(regions),
            Self::Talk(x) => x.spot.display(regions),
            Self::Shop(x) => x.spot.display(regions),
        }
    }
    pub fn id(&self) -> SpotId {
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
pub struct Spots<'a> {
    pub field_item_spots: Vec<SpotRef<'a>>,
    pub talk_spots: Vec<&'a Talk>,
    pub shops: Vec<&'a Shop>,
    pub events: Vec<&'a Event>,
}

impl<'a> Spots<'a> {
    pub fn new(source: &'a Storage) -> Self {
        Self {
            field_item_spots: source
                .main_weapons
                .values()
                .map(SpotRef::MainWeapon)
                .chain(source.sub_weapons.values().map(SpotRef::SubWeapon))
                .chain(source.chests.values().map(SpotRef::Chest))
                .chain(source.seals.values().map(SpotRef::Seal))
                .chain(source.roms.values().map(SpotRef::Rom))
                .collect(),
            talk_spots: source.talks.iter().collect(),
            shops: source.shops.iter().collect(),
            events: source.events.iter().collect(),
        }
    }

    pub fn to_table(&self) -> SpotTable<SpotRef<'a>> {
        self.field_item_spots
            .iter()
            .copied()
            .chain(self.talk_spots.iter().map(|&x| SpotRef::Talk(x)))
            .chain(self.shops.iter().map(|&x| SpotRef::Shop(x)))
            .map(|spot| (spot.id(), spot))
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.field_item_spots.is_empty()
            && self.talk_spots.is_empty()
            && self.shops.iter().all(|shop| shop.item.name.is_consumable())
    }

    pub fn extend(&mut self, other: Self) {
        self.field_item_spots.extend(other.field_item_spots);
        self.talk_spots.extend(other.talk_spots);
        self.shops.extend(other.shops);
        self.events.extend(other.events);
    }
}

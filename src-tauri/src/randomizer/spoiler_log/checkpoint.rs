use crate::randomizer::{
    spoiler::locations::LocationRef,
    storage::{
        Chest, MainWeapon, Rom, Seal, Shop, SubWeapon, Talk,
        item::{Item, StrategyFlag},
    },
};

#[derive(Debug)]
pub enum Checkpoint {
    MainWeapon(MainWeapon),
    SubWeapon(SubWeapon),
    Chest(Chest),
    Seal(Seal),
    Rom(Rom),
    Talk(Talk),
    Shop(Shop),
    Event(StrategyFlag),
}

pub enum CheckpointRef<'a> {
    Item {
        location: LocationRef<'a>,
        item: &'a Item,
    },
    Event(&'a StrategyFlag),
}

impl CheckpointRef<'_> {
    pub fn to_owned(&self) -> Checkpoint {
        match self {
            Self::Item { location, item } => placed_location(*location, (*item).to_owned()),
            Self::Event(flag) => Checkpoint::Event((*flag).to_owned()),
        }
    }
}

fn placed_location(location: LocationRef, item: Item) -> Checkpoint {
    match location {
        LocationRef::MainWeapon(x) => Checkpoint::MainWeapon(MainWeapon { item, ..x.clone() }),
        LocationRef::SubWeapon(x) => Checkpoint::SubWeapon(SubWeapon { item, ..x.clone() }),
        LocationRef::Chest(x) => Checkpoint::Chest(Chest { item, ..x.clone() }),
        LocationRef::Seal(x) => Checkpoint::Seal(Seal { item, ..x.clone() }),
        LocationRef::Rom(x) => Checkpoint::Rom(Rom { item, ..x.clone() }),
        LocationRef::Talk(x) => Checkpoint::Talk(Talk { item, ..x.clone() }),
        LocationRef::Shop(x) => Checkpoint::Shop(Shop { item, ..x.clone() }),
    }
}

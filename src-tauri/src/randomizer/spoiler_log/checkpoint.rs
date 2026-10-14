use crate::randomizer::{
    spoiler::spots::SpotRef,
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
    Item { spot: SpotRef<'a>, item: &'a Item },
    Event(&'a StrategyFlag),
}

impl CheckpointRef<'_> {
    pub fn to_owned(&self) -> Checkpoint {
        match self {
            Self::Item { spot, item } => placed_spot(*spot, (*item).to_owned()),
            Self::Event(flag) => Checkpoint::Event((*flag).to_owned()),
        }
    }
}

fn placed_spot(spot: SpotRef, item: Item) -> Checkpoint {
    match spot {
        SpotRef::MainWeapon(x) => Checkpoint::MainWeapon(MainWeapon { item, ..x.clone() }),
        SpotRef::SubWeapon(x) => Checkpoint::SubWeapon(SubWeapon { item, ..x.clone() }),
        SpotRef::Chest(x) => Checkpoint::Chest(Chest { item, ..x.clone() }),
        SpotRef::Seal(x) => Checkpoint::Seal(Seal { item, ..x.clone() }),
        SpotRef::Rom(x) => Checkpoint::Rom(Rom { item, ..x.clone() }),
        SpotRef::Talk(x) => Checkpoint::Talk(Talk { item, ..x.clone() }),
        SpotRef::Shop(x) => Checkpoint::Shop(Shop { item, ..x.clone() }),
    }
}

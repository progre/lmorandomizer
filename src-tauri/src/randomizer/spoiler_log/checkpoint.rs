use crate::randomizer::{
    spoiler::spots::SpotRef,
    storage::{
        Chest, ChestRef, MainWeapon, MainWeaponRef, Rom, RomRef, Seal, SealRef, Shop, ShopRef,
        SubWeapon, SubWeaponRef, Talk, TalkRef,
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
    MainWeapon(MainWeaponRef<'a>),
    SubWeapon(SubWeaponRef<'a>),
    Chest(ChestRef<'a>),
    Seal(SealRef<'a>),
    Rom(RomRef<'a>),
    Talk(TalkRef<'a>),
    Shop(ShopRef<'a>),
    Event(&'a StrategyFlag),
}

impl<'a> CheckpointRef<'a> {
    pub fn from_field_spot_item(spot: SpotRef<'a>, item: &'a Item) -> Self {
        match spot {
            SpotRef::MainWeapon(spot) => Self::MainWeapon(MainWeaponRef { spot, item }),
            SpotRef::SubWeapon(spot) => Self::SubWeapon(SubWeaponRef { spot, item }),
            SpotRef::Chest(spot) => Self::Chest(ChestRef { spot, item }),
            SpotRef::Seal(spot) => Self::Seal(SealRef { spot, item }),
            SpotRef::Rom(spot) => Self::Rom(RomRef { spot, item }),
            SpotRef::Talk(spot) => Self::Talk(TalkRef { spot, item }),
        }
    }

    pub fn to_owned(&self) -> Checkpoint {
        match self {
            Self::MainWeapon(checkpoint) => Checkpoint::MainWeapon(MainWeapon {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::SubWeapon(checkpoint) => Checkpoint::SubWeapon(SubWeapon {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Chest(checkpoint) => Checkpoint::Chest(Chest {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Seal(checkpoint) => Checkpoint::Seal(Seal {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Rom(checkpoint) => Checkpoint::Rom(Rom {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Talk(checkpoint) => Checkpoint::Talk(Talk {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Shop(checkpoint) => Checkpoint::Shop(Shop {
                spot: checkpoint.spot.to_owned(),
                idx: checkpoint.idx,
                item: checkpoint.item.to_owned(),
            }),
            Self::Event(flag) => Checkpoint::Event((*flag).to_owned()),
        }
    }
}

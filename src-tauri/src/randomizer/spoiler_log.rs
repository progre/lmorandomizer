use std::fmt;

use crate::dataset::{
    item::{ChestItem, Item, StrategyFlag},
    spot::{FieldId, SpotRef},
    storage::{
        Chest, ChestRef, MainWeapon, MainWeaponRef, Rom, RomRef, Seal, SealRef, Shop, ShopRef,
        SubWeapon, SubWeaponRef,
    },
};

fn compare_key_for_spoiler_log(field_id: FieldId) -> u8 {
    if matches!(field_id, FieldId::TwinLabyrinthsRight) {
        FieldId::TwinLabyrinthsLeft as u8 * 10 + 1
    } else {
        field_id as u8 * 10
    }
}

#[derive(Debug)]
pub enum Checkpoint {
    MainWeapon(MainWeapon),
    SubWeapon(SubWeapon),
    Chest(Chest),
    Seal(Seal),
    Shop(Shop),
    Rom(Rom),
    Event(StrategyFlag),
}

impl fmt::Display for Checkpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainWeapon(checkpoint) => {
                write!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())
            }
            Self::SubWeapon(checkpoint) => {
                write!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())
            }
            Self::Chest(checkpoint) => {
                write!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())
            }
            Self::Seal(checkpoint) => {
                write!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())
            }
            Self::Shop(checkpoint) => {
                let spot = &checkpoint.spot;
                let items = &checkpoint.items;
                let items = (items.0.name.get(), items.1.name.get(), items.2.name.get());
                write!(f, "{} = {}, {}, {}", spot, items.0, items.1, items.2)
            }
            Self::Rom(checkpoint) => {
                write!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())
            }
            Self::Event(flag) => write!(f, "{}", flag.get()),
        }
    }
}

pub enum CheckpointRef<'a> {
    MainWeapon(MainWeaponRef<'a>),
    SubWeapon(SubWeaponRef<'a>),
    Chest(ChestRef<'a>),
    Seal(SealRef<'a>),
    Shop(ShopRef<'a>),
    Rom(RomRef<'a>),
    Event(&'a StrategyFlag),
}

impl<'a> CheckpointRef<'a> {
    pub fn from_field_spot_item(spot: SpotRef<'a>, item: &'a Item) -> Self {
        match spot {
            SpotRef::MainWeapon(spot) => Self::MainWeapon(MainWeaponRef { spot, item }),
            SpotRef::SubWeapon(spot) => Self::SubWeapon(SubWeaponRef { spot, item }),
            SpotRef::Chest(spot) => Self::Chest(ChestRef { spot, item }),
            SpotRef::Seal(spot) => Self::Seal(SealRef { spot, item }),
            SpotRef::Shop(_) => unreachable!(),
            SpotRef::Rom(spot) => Self::Rom(RomRef { spot, item }),
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
            Self::Shop(checkpoint) => Checkpoint::Shop(Shop {
                spot: checkpoint.spot.to_owned(),
                items: (
                    checkpoint.items.0.to_owned(),
                    checkpoint.items.1.to_owned(),
                    checkpoint.items.2.to_owned(),
                ),
            }),
            Self::Rom(checkpoint) => Checkpoint::Rom(Rom {
                spot: checkpoint.spot.to_owned(),
                item: checkpoint.item.to_owned(),
            }),
            Self::Event(flag) => Checkpoint::Event((*flag).to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct Sphere(pub Vec<Checkpoint>);

pub struct SphereRef<'a>(pub Vec<CheckpointRef<'a>>);

#[derive(Debug)]
pub struct SpoilerLog {
    progression: Vec<Sphere>,
    maps: Vec<Checkpoint>,
}

impl fmt::Display for SpoilerLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, sphere) in self.progression.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            writeln!(f, "[Sphere {}]", i + 1)?;
            let mut checkpoints: Vec<_> = sphere.0.iter().collect();
            checkpoints.sort_by_key(|checkpoint| {
                let (field, type_num, src_idx) = match checkpoint {
                    Checkpoint::MainWeapon(x) => {
                        (x.spot.field_id(), 1, x.spot.main_weapon() as usize)
                    }
                    Checkpoint::SubWeapon(x) => {
                        (x.spot.field_id(), 2, x.spot.sub_weapon() as usize)
                    }
                    Checkpoint::Chest(x) => {
                        let number = match x.spot.item() {
                            ChestItem::Equipment(equipment) => equipment as usize,
                            ChestItem::Rom(rom) => 100 + rom as usize,
                        };
                        (x.spot.field_id(), 3, number)
                    }
                    Checkpoint::Seal(x) => (x.spot.field_id(), 4, x.spot.seal() as usize),
                    Checkpoint::Shop(x) => (x.spot.field_id(), 5, x.spot.src_idx()),
                    Checkpoint::Rom(x) => (x.spot.field_id(), 6, 0),
                    Checkpoint::Event(_) => return 10000000,
                };
                compare_key_for_spoiler_log(field) as usize * 10000 + type_num * 1000 + src_idx
            });
            for checkpoint in checkpoints {
                writeln!(f, "{}", checkpoint)?;
            }
        }
        writeln!(f)?;
        writeln!(f, "[Maps]")?;
        for checkpoint in &self.maps {
            writeln!(f, "{}", checkpoint)?;
        }
        Ok(())
    }
}

pub struct SpoilerLogRef<'a> {
    pub progression: Vec<SphereRef<'a>>,
    pub maps: Vec<CheckpointRef<'a>>,
}

impl SpoilerLogRef<'_> {
    pub fn to_owned(&self) -> SpoilerLog {
        SpoilerLog {
            progression: self
                .progression
                .iter()
                .map(|sphere| {
                    sphere
                        .0
                        .iter()
                        .map(|checkpoint| checkpoint.to_owned())
                        .collect()
                })
                .map(Sphere)
                .collect(),
            maps: self
                .maps
                .iter()
                .map(|checkpoint| checkpoint.to_owned())
                .collect(),
        }
    }
}

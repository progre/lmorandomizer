use std::fmt;

use crate::{
    dataset::spot::ShopSpot,
    script::enums::{ChestItem, FieldNumber},
};

use super::{
    spoiler::spots::SpotRef,
    storage::{
        item::{Item, StrategyFlag},
        Chest, ChestRef, MainWeapon, MainWeaponRef, Rom, RomRef, Seal, SealRef, Shop, ShopRef,
        SubWeapon, SubWeaponRef, Talk, TalkRef,
    },
};

fn compare_key_for_spoiler_log(field_number: FieldNumber) -> u8 {
    if matches!(field_number, FieldNumber::TwinLabyrinthsRight) {
        FieldNumber::TwinLabyrinthsLeft.to_logic_number().unwrap() * 10 + 1
    } else {
        field_number.to_logic_number().unwrap() * 10
    }
}

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

fn fmt_checkpoints(checkpoints: &[&Checkpoint], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let fmt_shop = |f: &mut fmt::Formatter<'_>, spot: &ShopSpot, shop: &Vec<&Shop>| {
        let item0 = shop.iter().find(|x| x.idx == 0);
        let item0 = item0.as_ref().map_or("_", |x| x.item.name.get());
        let item1 = shop.iter().find(|x| x.idx == 1);
        let item1 = item1.as_ref().map_or("_", |x| x.item.name.get());
        let item2 = shop.iter().find(|x| x.idx == 2);
        let item2 = item2.as_ref().map_or("_", |x| x.item.name.get());
        writeln!(f, "{} = {}, {}, {}", spot, item0, item1, item2)
    };
    let mut shop: Vec<&Shop> = Vec::new();
    for checkpoint in checkpoints {
        if !shop.is_empty() {
            let spot = &shop[0].spot;
            let different = match checkpoint {
                Checkpoint::MainWeapon(_)
                | Checkpoint::SubWeapon(_)
                | Checkpoint::Chest(_)
                | Checkpoint::Seal(_)
                | Checkpoint::Rom(_)
                | Checkpoint::Talk(_)
                | Checkpoint::Event(_) => true,
                Checkpoint::Shop(checkpoint) => checkpoint.spot.items() != spot.items(),
            };
            if different {
                fmt_shop(f, spot, &shop)?;
                shop.clear();
            }
        }
        match checkpoint {
            Checkpoint::MainWeapon(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::SubWeapon(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::Chest(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::Seal(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::Rom(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::Talk(checkpoint) => {
                writeln!(f, "{} = {}", checkpoint.spot, checkpoint.item.name.get())?
            }
            Checkpoint::Shop(checkpoint) => {
                shop.push(checkpoint);
            }
            Checkpoint::Event(flag) => writeln!(f, "{}", flag.get())?,
        }
    }
    if !shop.is_empty() {
        fmt_shop(f, &shop[0].spot, &shop)?;
    }
    Ok(())
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
            SpotRef::Shop(_) => unreachable!(),
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
            let shop_list: Vec<_> = checkpoints
                .iter()
                .filter_map(|x| match x {
                    Checkpoint::Shop(x) => Some(x),
                    Checkpoint::MainWeapon(_)
                    | Checkpoint::SubWeapon(_)
                    | Checkpoint::Chest(_)
                    | Checkpoint::Seal(_)
                    | Checkpoint::Rom(_)
                    | Checkpoint::Talk(_)
                    | Checkpoint::Event(_) => None,
                })
                .map(|x| x.spot.items())
                .collect();
            checkpoints.sort_by_key(|checkpoint| {
                let (field, type_num, src_idx) = match checkpoint {
                    Checkpoint::MainWeapon(x) => {
                        (x.spot.field_number(), 1, x.spot.main_weapon() as usize)
                    }
                    Checkpoint::SubWeapon(x) => {
                        (x.spot.field_number(), 2, x.spot.sub_weapon() as usize)
                    }
                    Checkpoint::Chest(x) => {
                        let number = match x.spot.item() {
                            ChestItem::Equipment(equipment) => equipment as usize,
                            ChestItem::Rom(rom) => 100 + rom as usize,
                        };
                        (x.spot.field_number(), 3, number)
                    }
                    Checkpoint::Seal(x) => (x.spot.field_number(), 4, x.spot.seal() as usize),
                    Checkpoint::Rom(x) => (x.spot.field_number(), 5, 0),
                    Checkpoint::Talk(x) => (x.spot.field_number(), 6, 0),
                    Checkpoint::Shop(x) => (
                        x.spot.field_number(),
                        7,
                        shop_list.iter().position(|&y| y == x.spot.items()).unwrap(),
                    ),
                    Checkpoint::Event(_) => return 10000000,
                };
                compare_key_for_spoiler_log(field) as usize * 10000 + type_num * 1000 + src_idx
            });
            fmt_checkpoints(&checkpoints, f)?;
        }
        writeln!(f)?;
        writeln!(f, "[Maps]")?;
        fmt_checkpoints(&self.maps.iter().collect::<Vec<_>>(), f)
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

use std::fmt;

use crate::{
    dataset::{regions::Regions, spot::ShopSpot},
    randomizer::storage::{Chest, MainWeapon, Rom, Seal, Shop, SubWeapon, Talk},
    script::enums::{ChestItem, FieldNumber},
};

use super::{Checkpoint, SpoilerLog};

fn compare_key_for_spoiler_log(field_number: FieldNumber) -> u8 {
    if matches!(field_number, FieldNumber::TwinLabyrinthsRight) {
        FieldNumber::TwinLabyrinthsLeft.to_logic_number().unwrap() * 10 + 1
    } else {
        field_number.to_logic_number().unwrap() * 10
    }
}

fn fmt_checkpoints(
    checkpoints: &[&Checkpoint],
    regions: &Regions,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let fmt_shop = |f: &mut fmt::Formatter<'_>, spot: &ShopSpot, shop: &Vec<&Shop>| {
        let item0 = shop.iter().find(|x| x.idx == 0);
        let item0 = item0.as_ref().map_or("_", |x| x.item.name.get());
        let item1 = shop.iter().find(|x| x.idx == 1);
        let item1 = item1.as_ref().map_or("_", |x| x.item.name.get());
        let item2 = shop.iter().find(|x| x.idx == 2);
        let item2 = item2.as_ref().map_or("_", |x| x.item.name.get());
        writeln!(
            f,
            "{} = {}, {}, {}",
            spot.display(regions),
            item0,
            item1,
            item2
        )
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
            Checkpoint::MainWeapon(MainWeapon { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
            }
            Checkpoint::SubWeapon(SubWeapon { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
            }
            Checkpoint::Chest(Chest { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
            }
            Checkpoint::Seal(Seal { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
            }
            Checkpoint::Rom(Rom { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
            }
            Checkpoint::Talk(Talk { spot, item, .. }) => {
                writeln!(f, "{} = {}", spot.display(regions), item.name.get())?
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

impl fmt::Display for SpoilerLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_number_of = |region| self.regions.get(region).field_number();
        for (i, sphere) in self.progression.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            writeln!(f, "[Sphere {}]", i)?;
            let mut checkpoints: Vec<_> = sphere.checkpoints.iter().collect();
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
                    Checkpoint::MainWeapon(x) => (
                        field_number_of(x.spot.region()),
                        1,
                        x.spot.main_weapon() as usize,
                    ),
                    Checkpoint::SubWeapon(x) => (
                        field_number_of(x.spot.region()),
                        2,
                        x.spot.sub_weapon() as usize,
                    ),
                    Checkpoint::Chest(x) => {
                        let number = match x.spot.item() {
                            ChestItem::Equipment(equipment) => equipment as usize,
                            ChestItem::Rom(rom) => 100 + rom as usize,
                        };
                        (field_number_of(x.spot.region()), 3, number)
                    }
                    Checkpoint::Seal(x) => {
                        (field_number_of(x.spot.region()), 4, x.spot.seal() as usize)
                    }
                    Checkpoint::Rom(x) => (field_number_of(x.spot.region()), 5, 0),
                    Checkpoint::Talk(x) => (field_number_of(x.spot.region()), 6, 0),
                    Checkpoint::Shop(x) => (
                        field_number_of(x.spot.region()),
                        7,
                        shop_list.iter().position(|&y| y == x.spot.items()).unwrap(),
                    ),
                    Checkpoint::Event(_) => return 10000000,
                };
                compare_key_for_spoiler_log(field) as usize * 10000 + type_num * 1000 + src_idx
            });
            fmt_checkpoints(&checkpoints, &self.regions, f)?;
        }
        writeln!(f)?;
        writeln!(f, "[Maps]")?;
        let maps: Vec<_> = self.maps.iter().collect();
        fmt_checkpoints(&maps, &self.regions, f)
    }
}

use std::{collections::BTreeMap, fmt};

use crate::dataset::{
    item::Item,
    spot::{FieldId, Spot},
};

fn compare_key_for_spoiler_log(field_id: FieldId) -> u8 {
    if matches!(field_id, FieldId::TwinLabyrinthsRight) {
        FieldId::TwinLabyrinthsLeft as u8 * 10 + 1
    } else {
        field_id as u8 * 10
    }
}

fn spot_idx(spot: &Spot) -> usize {
    compare_key_for_spoiler_log(spot.field_id()) as usize * 10000
        + match spot {
            Spot::MainWeapon(spot) => 1000 + spot.src_idx,
            Spot::SubWeapon(spot) => 2000 + spot.src_idx,
            Spot::Chest(spot) => 3000 + spot.src_idx,
            Spot::Seal(spot) => 4000 + spot.src_idx,
            Spot::Shop(spot) => 5000 + spot.src_idx,
        }
}

pub struct Checkpoint<TSpot, TItem> {
    pub spot: TSpot,
    pub idx: usize,
    pub item: TItem,
}

impl<'a> Checkpoint<&'a Spot, &'a Item> {
    pub fn into_owned(self) -> Checkpoint<Spot, Item> {
        Checkpoint {
            spot: self.spot.to_owned(),
            idx: self.idx,
            item: self.item.to_owned(),
        }
    }
}

pub struct Sphere<TSpot, TItem>(pub Vec<Checkpoint<TSpot, TItem>>);

pub struct SpoilerLog {
    progression: Vec<Sphere<Spot, Item>>,
    maps: Vec<Checkpoint<Spot, Item>>,
}

impl fmt::Display for SpoilerLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, sphere) in self.progression.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            writeln!(f, "[Sphere {}]", i + 1)?;
            let mut map: BTreeMap<_, Vec<_>> = BTreeMap::new();
            for checkpoint in &sphere.0 {
                map.entry(spot_idx(&checkpoint.spot))
                    .or_default()
                    .push(checkpoint);
            }
            for checkpoints in map.values_mut() {
                if checkpoints.len() == 1 {
                    let spot = &checkpoints[0].spot;
                    let name = checkpoints[0].item.name.get();
                    writeln!(f, "{} = {}", spot, name)?;
                } else {
                    let spot = &checkpoints[0].spot;
                    checkpoints.sort_by_key(|x| x.idx);
                    let names: Vec<_> = checkpoints.iter().map(|x| x.item.name.get()).collect();
                    let names = names.join(", ");
                    writeln!(f, "{} = {}", spot, names)?;
                }
            }
        }
        writeln!(f)?;
        writeln!(f, "[Maps]")?;
        for checkpoint in &self.maps {
            let spot = &checkpoint.spot;
            let name = checkpoint.item.name.get();
            writeln!(f, "{} = {}", spot, name)?;
        }
        Ok(())
    }
}

pub struct SpoilerLogRef<'a> {
    pub progression: Vec<Sphere<&'a Spot, &'a Item>>,
    pub maps: Vec<Checkpoint<&'a Spot, &'a Item>>,
}

impl SpoilerLogRef<'_> {
    pub fn into_owned(self) -> SpoilerLog {
        SpoilerLog {
            progression: self
                .progression
                .into_iter()
                .map(|sphere| {
                    sphere
                        .0
                        .into_iter()
                        .map(|checkpoint| checkpoint.into_owned())
                        .collect()
                })
                .map(Sphere)
                .collect(),
            maps: self
                .maps
                .into_iter()
                .map(|checkpoint| checkpoint.into_owned())
                .collect(),
        }
    }
}

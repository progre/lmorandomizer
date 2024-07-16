use std::fmt;

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

pub struct Checkpoint<TSpot, TItem> {
    pub spot: TSpot,
    pub items: Vec<TItem>,
}

impl<'a> Checkpoint<&'a Spot, &'a Item> {
    pub fn into_owned(self) -> Checkpoint<Spot, Item> {
        Checkpoint {
            spot: self.spot.to_owned(),
            items: self.items.into_iter().map(|x| x.to_owned()).collect(),
        }
    }
}

pub struct Sphere<TSpot, TItem>(pub Vec<Checkpoint<TSpot, TItem>>);

pub struct SpoilerLog {
    pub progression: Vec<Sphere<Spot, Item>>,
}

impl fmt::Display for SpoilerLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, sphere) in self.progression.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            writeln!(f, "[Sphere {}]", i + 1)?;
            let mut sphere: Vec<_> = sphere.0.iter().collect();
            sphere.sort_by(|a, b| {
                compare_key_for_spoiler_log(a.spot.field_id())
                    .cmp(&compare_key_for_spoiler_log(b.spot.field_id()))
            });
            for checkpoint in sphere {
                writeln!(
                    f,
                    "{} = {}",
                    checkpoint.spot,
                    checkpoint
                        .items
                        .iter()
                        .map(|x| x.name.get())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }
        Ok(())
    }
}

pub struct SpoilerLogRef<'a> {
    pub progression: Vec<Sphere<&'a Spot, &'a Item>>,
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
        }
    }
}

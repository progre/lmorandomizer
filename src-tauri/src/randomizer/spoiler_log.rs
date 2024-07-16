use std::fmt;

use crate::dataset::{
    item::Item,
    spot::{FieldId, Spot},
};

#[derive(Clone)]
pub struct Checkpoint {
    pub spot: Spot,
    pub items: Vec<Item>,
}

pub struct Sphere(pub Vec<Checkpoint>);

fn compare_key_for_spoiler_log(field_id: FieldId) -> u8 {
    if matches!(field_id, FieldId::TwinLabyrinthsRight) {
        FieldId::TwinLabyrinthsLeft as u8 * 10 + 1
    } else {
        field_id as u8 * 10
    }
}

pub struct SpoilerLog {
    pub progression: Vec<Sphere>,
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

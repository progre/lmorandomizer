use std::fmt;

use crate::dataset::{item::Item, spot::Spot};

#[derive(Clone)]
pub struct Checkpoint {
    pub spot: Spot,
    pub items: Vec<Item>,
}

pub struct Sphere(pub Vec<Checkpoint>);

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
            for checkpoint in &sphere.0 {
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

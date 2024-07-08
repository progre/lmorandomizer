use std::fmt;

use crate::dataset::{spot::Spot, supplements::StrategyFlag};

#[derive(Clone)]
pub struct Checkpoint {
    pub spot: Spot,
    pub strategy_flags: Vec<StrategyFlag>,
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
                    "{}({}) = {}",
                    checkpoint.spot.source,
                    checkpoint.spot.name.0,
                    checkpoint
                        .strategy_flags
                        .iter()
                        .map(|x| x.get())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }
        Ok(())
    }
}

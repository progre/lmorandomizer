mod checkpoint;
mod display;

pub use checkpoint::{Checkpoint, CheckpointRef};

#[derive(Debug)]
pub struct Sphere {
    checkpoints: Vec<Checkpoint>,
}
impl Sphere {
    pub fn new(checkpoints: Vec<Checkpoint>) -> Self {
        Self { checkpoints }
    }
}

pub struct SphereRef<'a> {
    checkpoints: Vec<CheckpointRef<'a>>,
}

impl<'a> SphereRef<'a> {
    pub fn new(checkpoints: Vec<CheckpointRef<'a>>) -> Self {
        Self { checkpoints }
    }

    pub fn iter(&self) -> impl Iterator<Item = &CheckpointRef<'a>> {
        self.checkpoints.iter()
    }

    pub fn into_inner(self) -> Vec<CheckpointRef<'a>> {
        self.checkpoints
    }

    pub fn append_checkpoints(&mut self, mut checkpoints: Vec<CheckpointRef<'a>>) {
        self.checkpoints.append(&mut checkpoints);
    }
}

#[derive(Debug)]
pub struct SpoilerLog {
    progression: Vec<Sphere>,
    maps: Vec<Checkpoint>,
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
                    Sphere::new(
                        sphere
                            .checkpoints
                            .iter()
                            .map(|checkpoint| checkpoint.to_owned())
                            .collect(),
                    )
                })
                .collect(),
            maps: self
                .maps
                .iter()
                .map(|checkpoint| checkpoint.to_owned())
                .collect(),
        }
    }

    #[cfg(test)]
    pub fn count_checkpoints(&self) -> usize {
        self.progression
            .iter()
            .map(|sphere| sphere.checkpoints.len())
            .sum::<usize>()
            + self.maps.len()
    }
}

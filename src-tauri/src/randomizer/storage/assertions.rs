use std::collections::HashSet;

use anyhow::bail;

use crate::dataset::spot::{AnyOfAllRequirements, RequirementFlag};

use super::{item::StrategyFlag, Storage};

fn append<'a>(
    set: &mut HashSet<RequirementFlag>,
    any_of_requirements: impl Iterator<Item = Option<&'a AnyOfAllRequirements>>,
) {
    any_of_requirements
        .filter_map(|item| item.as_ref().map(|x| &x.0))
        .flatten()
        .flat_map(|group| &group.0)
        .for_each(|x| {
            set.insert(x.clone());
        });
}

pub fn ware_missing_requirements(storage: &Storage) -> anyhow::Result<()> {
    let glitch = StrategyFlag::new("option:glitch".into());
    let all_items: Vec<_> = storage
        .all_items()
        .map(|x| &x.name)
        .chain(storage.events.iter().map(|y| &y.name))
        .chain([&glitch])
        .collect();
    let mut set = HashSet::new();
    let iter = storage
        .main_weapons
        .values()
        .map(|x| x.spot.requirements())
        .chain(storage.sub_weapons.values().map(|x| x.spot.requirements()))
        .chain(storage.chests.values().map(|x| x.spot.requirements()))
        .chain(storage.seals.values().map(|x| x.spot.requirements()))
        .chain(storage.shops.iter().map(|x| x.spot.requirements()))
        .chain(storage.talks.iter().map(|x| x.spot.requirements()))
        .chain(storage.events.iter().map(|x| Some(&x.requirements)));
    append(&mut set, iter);
    let mut vec: Vec<_> = set
        .iter()
        .filter(|&x| all_items.iter().all(|&name| name != x))
        .filter(|x| !x.is_sacred_orb())
        .collect();
    vec.sort();
    if !vec.is_empty() {
        bail!("Missing items: {:?}", vec);
    }
    Ok(())
}

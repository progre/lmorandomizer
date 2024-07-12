use std::collections::HashSet;

use log::warn;

use crate::dataset::spot::AllRequirements;

use super::{
    spot::{AnyOfAllRequirements, RequirementFlag},
    storage::{ItemSpot, Storage},
};

pub fn assert_chests(chests: &[ItemSpot]) {
    debug_assert_eq!(
        chests
            .iter()
            .find(|x| x.item.name.get() == "iceCape")
            .unwrap()
            .spot
            .requirements(),
        Some(&AnyOfAllRequirements(vec![
            AllRequirements(vec![
                RequirementFlag::new("ankhJewel:templeOfTheSun".into()),
                RequirementFlag::new("bronzeMirror".into()),
                RequirementFlag::new("shuriken".into()),
                RequirementFlag::new("shurikenAmmo".into()),
            ]),
            AllRequirements(vec![
                RequirementFlag::new("holyGrail".into()),
                RequirementFlag::new("flareGun".into()),
                RequirementFlag::new("grappleClaw".into()),
            ]),
            // tslint:disable-next-line:max-line-length
            // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "grappleClaw"],
            AllRequirements(vec![
                RequirementFlag::new("bronzeMirror".into()),
                RequirementFlag::new("ankhJewel:mausoleumOfTheGiants".into()),
                RequirementFlag::new("flareGun".into()),
                RequirementFlag::new("grappleClaw".into()),
            ]),
            AllRequirements(vec![
                RequirementFlag::new("holyGrail".into()),
                RequirementFlag::new("flareGun".into()),
                RequirementFlag::new("feather".into()),
            ]),
            // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "feather"],
            AllRequirements(vec![
                RequirementFlag::new("bronzeMirror".into()),
                RequirementFlag::new("ankhJewel:mausoleumOfTheGiants".into()),
                RequirementFlag::new("flareGun".into()),
                RequirementFlag::new("feather".into()),
            ]),
        ]))
    );
}

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

pub fn ware_missing_requirements(storage: &Storage) {
    let all_items: Vec<_> = storage.all_items().cloned().collect();
    let mut set = HashSet::new();
    let main_weapon_requirements = storage.main_weapons().iter().map(|x| x.spot.requirements());
    append(&mut set, main_weapon_requirements);
    let sub_weapon_requirements = storage.sub_weapons().iter().map(|x| x.spot.requirements());
    append(&mut set, sub_weapon_requirements);
    append(
        &mut set,
        storage.chests().iter().map(|x| x.spot.requirements()),
    );
    append(
        &mut set,
        storage.seals().iter().map(|x| x.spot.requirements()),
    );
    append(
        &mut set,
        storage.shops().iter().map(|x| x.spot.requirements()),
    );
    let mut vec: Vec<_> = set
        .iter()
        .filter(|&x| all_items.iter().all(|y| &y.name != x))
        .filter(|x| !x.is_sacred_orb())
        .collect();
    vec.sort();
    for x in vec {
        warn!("Missing item: {:?}", x);
    }
}

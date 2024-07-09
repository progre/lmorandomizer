use std::collections::HashSet;

use log::warn;

use crate::dataset::{
    item::Item,
    spot::Spot,
    storage::{ItemSpot, Shop, Storage},
    supplements::Supplements,
};

use super::{
    get_all_items::{get_all_items, AllItems},
    spot::{AnyOfAllRequirements, RequirementFlag},
};

#[test]
fn test_create_source() {
    use sha3::{Digest, Sha3_512};

    use crate::dataset::{
        create_source::create_source,
        supplements::{SupplementFiles, Supplements},
    };

    let files = SupplementFiles {
        weapons_yml: include_str!("../../../public/res/weapons.yml").to_owned(),
        chests_yml: include_str!("../../../public/res/chests.yml").to_owned(),
        seals_yml: include_str!("../../../public/res/seals.yml").to_owned(),
        shops_yml: include_str!("../../../public/res/shops.yml").to_owned(),
        events_yml: include_str!("../../../public/res/events.yml").to_owned(),
    };
    let supplements = Supplements::new(&files);
    let source = create_source(supplements);
    let script_dat_hash = Sha3_512::digest(format!("{:?}", source)).to_vec();
    const HASH: &str = "3390414b584a1728d18c5cc067e42607159f0f8908d8e02b87bfaf8fc8caff68ecddb6fdd0de3bc9de377d035c0203f8226910670c246e9851b7b8e1ebd30967";
    assert_eq!(script_dat_hash, hex::decode(HASH).unwrap());
}

pub fn create_source(supplements: Supplements) -> Storage {
    let all_items = get_all_items(&supplements);
    let enumerate_items: Vec<_> = all_items
        .main_weapons
        .iter()
        .chain(all_items.sub_weapons.iter())
        .chain(all_items.chests.iter())
        .chain(all_items.seals.iter())
        .chain(all_items.shops.iter().flat_map(|(a, b, c)| [a, b, c]))
        .cloned()
        .collect();
    ware_missing_requirements(&supplements, &enumerate_items);
    let AllItems {
        main_weapons: all_items_main_weapons,
        sub_weapons: all_items_sub_weapons,
        chests: all_items_chests,
        seals: all_items_seals,
        shops: all_items_shops,
    } = all_items;
    let Supplements {
        main_weapons: supplements_main_weapons,
        sub_weapons: supplements_sub_weapons,
        chests: supplements_chests,
        seals: supplements_seals,
        shops: supplements_shops,
    } = supplements;
    Storage::new(
        all_items_main_weapons
            .into_iter()
            .zip(supplements_main_weapons)
            .map(|(item, spot)| ItemSpot {
                spot: Spot::MainWeaponShutter(spot),
                item,
            })
            .collect(),
        all_items_sub_weapons
            .into_iter()
            .zip(supplements_sub_weapons)
            .map(|(item, spot)| ItemSpot {
                spot: Spot::SubWeaponShutter(spot),
                item,
            })
            .collect(),
        all_items_chests
            .into_iter()
            .zip(supplements_chests)
            .map(|(item, spot)| ItemSpot {
                spot: Spot::Chest(spot),
                item,
            })
            .collect(),
        all_items_seals
            .into_iter()
            .zip(supplements_seals)
            .map(|(item, spot)| ItemSpot {
                spot: Spot::SealChest(spot),
                item,
            })
            .collect(),
        all_items_shops
            .into_iter()
            .zip(supplements_shops)
            .map(|(items, spot)| Shop {
                spot: Spot::Shop(spot),
                items,
            })
            .collect(),
    )
}

fn ware_missing_requirements(supplements: &Supplements, all_items: &[Item]) {
    let mut set = HashSet::new();
    let main_weapon_requirements = supplements.main_weapons.iter().map(|x| &x.requirements);
    append(&mut set, main_weapon_requirements);
    let sub_weapon_requirements = supplements.sub_weapons.iter().map(|x| &x.requirements);
    append(&mut set, sub_weapon_requirements);
    append(&mut set, supplements.chests.iter().map(|x| &x.requirements));
    append(&mut set, supplements.seals.iter().map(|x| &x.requirements));
    append(&mut set, supplements.shops.iter().map(|x| &x.requirements));
    if cfg!(debug_assertions) {
        let mut vec: Vec<_> = set
            .iter()
            .filter(|&x| all_items.iter().all(|y| y.name() != x))
            .filter(|x| !x.is_sacred_orb())
            .collect();
        vec.sort();
        for x in vec {
            warn!("Missing item: {:?}", x);
        }
    }
}

fn append<'a>(
    set: &mut HashSet<RequirementFlag>,
    any_of_requirements: impl Iterator<Item = &'a Option<AnyOfAllRequirements>>,
) {
    any_of_requirements
        .filter_map(|item| item.as_ref().map(|x| &x.0))
        .flatten()
        .flat_map(|group| &group.0)
        .for_each(|x| {
            set.insert(x.clone());
        });
}

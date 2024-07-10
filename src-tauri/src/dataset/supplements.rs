use crate::dataset::spot::{
    AllRequirements, Chest, MainWeaponShutter, SealChest, SubWeaponShutter,
};

use super::spot::{self, AnyOfAllRequirements, RequirementFlag, Shop, SpotName};

pub const NIGHT_SURFACE_SUB_WEAPON_COUNT: usize = 1;
pub const NIGHT_SURFACE_CHEST_COUNT: usize = 3;
pub const TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT: usize = 1;
pub const NIGHT_SURFACE_SEAL_COUNT: usize = 1;
pub const WARE_NO_MISE_COUNT: usize = 1;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct StrategyFlag(pub String);

impl StrategyFlag {
    pub fn new(spot_name: String) -> Self {
        debug_assert!(
            !spot_name.starts_with("sacredOrb:")
                || spot_name
                    .split(':')
                    .nth(1)
                    .map_or(false, |x| x.parse::<u8>().is_err())
        );
        Self(spot_name)
    }

    pub fn is_sacred_orb(&self) -> bool {
        self.0.starts_with("sacredOrb:")
    }
    pub fn is_map(&self) -> bool {
        self.0.starts_with("map:")
    }

    pub fn is_consumable(&self) -> bool {
        [
            "weights",
            "shurikenAmmo",
            "toukenAmmo",
            "spearAmmo",
            "flareGunAmmo",
            "bombAmmo",
            "ammunition",
        ]
        .contains(&self.0.as_str())
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<RequirementFlag> for StrategyFlag {
    fn eq(&self, other: &RequirementFlag) -> bool {
        self.0 == other.get()
    }
}

impl From<SpotName> for StrategyFlag {
    fn from(x: SpotName) -> Self {
        Self::new(x.into_inner())
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplementFiles {
    pub weapons_yml: String,
    pub chests_yml: String,
    pub seals_yml: String,
    pub shops_yml: String,
    pub events_yml: String,
}

#[derive(Debug, serde::Deserialize)]
struct YamlSpot {
    pub name: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[derive(serde::Deserialize)]
struct YamlShop {
    pub names: String,
    #[serde(default)]
    pub requirements: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WeaponsYaml {
    pub main_weapons: Vec<YamlSpot>,
    pub sub_weapons: Vec<YamlSpot>,
}

#[derive(Clone)]
struct Event {
    name: String,
    requirements: AnyOfAllRequirements,
}

pub struct Supplements {
    pub main_weapons: Vec<MainWeaponShutter>,
    pub sub_weapons: Vec<SubWeaponShutter>,
    pub chests: Vec<Chest>,
    pub seals: Vec<SealChest>,
    pub shops: Vec<Shop>,
}

impl Supplements {
    pub fn new(supplement_files: &SupplementFiles) -> Self {
        let weapons: WeaponsYaml = serde_yaml::from_str(&supplement_files.weapons_yml).unwrap();
        let main_weapons = weapons.main_weapons;
        let sub_weapons = weapons.sub_weapons;
        let chests: Vec<YamlSpot> = serde_yaml::from_str(&supplement_files.chests_yml).unwrap();
        let seals: Vec<YamlSpot> = serde_yaml::from_str(&supplement_files.seals_yml).unwrap();
        let shops: Vec<YamlShop> = serde_yaml::from_str(&supplement_files.shops_yml).unwrap();
        let events: Vec<YamlSpot> = serde_yaml::from_str(&supplement_files.events_yml).unwrap();
        let events = parse_requirements_of_events(events);

        let mut main_weapons = parse_item_spot_requirements(MainWeaponShutter::new, main_weapons);
        main_weapons.iter_mut().for_each(|x| {
            if let Some(requirements) = x.requirements.take() {
                let requirements = merge_events(requirements, &events);
                x.requirements = Some(requirements);
            }
        });
        let mut sub_weapons = parse_item_spot_requirements(SubWeaponShutter::new, sub_weapons);
        sub_weapons.iter_mut().for_each(|x| {
            if let Some(requirements) = x.requirements.take() {
                let requirements = merge_events(requirements, &events);
                x.requirements = Some(requirements);
            }
        });
        let mut chests = parse_item_spot_requirements(Chest::new, chests);
        chests.iter_mut().for_each(|x| {
            if let Some(requirements) = x.requirements.take() {
                let requirements = merge_events(requirements, &events);
                x.requirements = Some(requirements);
            }
        });
        let mut seals = parse_item_spot_requirements(SealChest::new, seals);
        seals.iter_mut().for_each(|x| {
            if let Some(requirements) = x.requirements.take() {
                let requirements = merge_events(requirements, &events);
                x.requirements = Some(requirements);
            }
        });
        let mut shops = parse_shop_requirements(shops);
        shops.iter_mut().for_each(|x| {
            if let Some(requirements) = x.requirements.take() {
                let requirements = merge_events(requirements, &events);
                x.requirements = Some(requirements);
            }
        });

        debug_assert_eq!(
            &chests
                .iter()
                .find(|x| x.name.get() == "iceCape")
                .unwrap()
                .requirements,
            &Some(AnyOfAllRequirements(vec![
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
        Self {
            main_weapons,
            sub_weapons,
            chests,
            seals,
            shops,
        }
    }
}

fn to_any_of_all_requirements(requirements: Vec<String>) -> Option<AnyOfAllRequirements> {
    if requirements.is_empty() {
        None
    } else {
        Some(AnyOfAllRequirements(
            requirements
                .into_iter()
                .map(|y| {
                    AllRequirements(
                        y.split(',')
                            .map(|z| RequirementFlag::new(z.trim().to_owned()))
                            .collect(),
                    )
                })
                .collect(),
        ))
    }
}

fn parse_item_spot_requirements<T>(
    create_spot: impl Fn(usize, SpotName, Option<AnyOfAllRequirements>) -> T,
    items: Vec<YamlSpot>,
) -> Vec<T> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, item)| {
            create_spot(
                src_idx,
                SpotName::new(item.name),
                to_any_of_all_requirements(item.requirements),
            )
        })
        .collect()
}

fn parse_shop_requirements(items: Vec<YamlShop>) -> Vec<spot::Shop> {
    items
        .into_iter()
        .enumerate()
        .map(|(src_idx, item)| {
            Shop::new(
                src_idx,
                SpotName::new(item.names),
                to_any_of_all_requirements(item.requirements),
            )
        })
        .collect()
}

fn parse_requirements_of_events(items: Vec<YamlSpot>) -> Vec<Event> {
    let mut current: Vec<Event> = items
        .into_iter()
        .map(|x| Event {
            name: x.name,
            requirements: AnyOfAllRequirements(
                x.requirements
                    .into_iter()
                    .map(|y| {
                        AllRequirements(
                            y.split(',')
                                .map(|z| RequirementFlag::new(z.trim().to_owned()))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        })
        .collect();
    for _ in 0..100 {
        let events: Vec<_> = current
            .iter()
            .filter(|x| {
                x.requirements
                    .0
                    .iter()
                    .all(|y| y.0.iter().all(|z| !z.get().starts_with("event:")))
            })
            .cloned()
            .collect();
        if events.len() == current.len() {
            return current;
        }
        current = current
            .into_iter()
            .map(|x| Event {
                name: x.name,
                requirements: merge_events(x.requirements, &events),
            })
            .collect();
    }
    unreachable!();
}

fn merge_events(requirements: AnyOfAllRequirements, events: &[Event]) -> AnyOfAllRequirements {
    // [['event:a', 'event:b', 'c']]
    // 'event:a': [['d', 'e', 'f']]
    // 'event:b': [['g', 'h'], ['i', 'j']]
    // ↓
    // [['event:b', 'c', 'd', 'e', 'f']]
    // ↓
    // [
    //   ['c', 'd', 'e', 'f', 'g', 'h']
    //   ['c', 'd', 'e', 'f', 'i', 'j']
    // ]
    let mut current = requirements;
    for event in events {
        if current
            .0
            .iter()
            .all(|target_group| !target_group.0.iter().any(|x| x.get() == event.name))
        {
            continue;
        }
        current = AnyOfAllRequirements(
            current
                .0
                .into_iter()
                .flat_map(|target_group| -> Vec<AllRequirements> {
                    if !target_group.0.iter().any(|x| x.get() == event.name) {
                        return vec![target_group];
                    }
                    event
                        .requirements
                        .0
                        .iter()
                        .map(|event_group| -> AllRequirements {
                            AllRequirements(
                                event_group
                                    .0
                                    .clone()
                                    .into_iter()
                                    .chain(
                                        target_group
                                            .0
                                            .clone()
                                            .into_iter()
                                            .filter(|x| {
                                                x.get() != event.name
                                                    && !event_group.0.iter().any(|y| y == x)
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                                    .collect(),
                            )
                        })
                        .collect()
                })
                .collect(),
        );
    }
    current
}

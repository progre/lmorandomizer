pub const NIGHT_SURFACE_SUB_WEAPON_COUNT: usize = 1;
pub const NIGHT_SURFACE_CHEST_COUNT: usize = 3;
pub const TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT: usize = 1;
pub const NIGHT_SURFACE_SEAL_COUNT: usize = 1;
pub const WARE_NO_MISE_COUNT: usize = 1;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct StrategyFlag(String);

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

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<RequirementFlag> for StrategyFlag {
    fn eq(&self, other: &RequirementFlag) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequirementFlag(String);

impl RequirementFlag {
    fn new(requirement: String) -> Self {
        debug_assert!(
            !requirement.starts_with("sacredOrb:")
                || requirement
                    .split(':')
                    .nth(1)
                    .map_or(false, |x| x.parse::<u8>().is_ok())
        );
        Self(requirement)
    }

    pub fn is_sacred_orb(&self) -> bool {
        self.0.starts_with("sacredOrb:")
    }

    pub fn sacred_orb_count(&self) -> u8 {
        self.0.split(':').nth(1).unwrap().parse().unwrap()
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<StrategyFlag> for RequirementFlag {
    fn eq(&self, other: &StrategyFlag) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllRequirements(pub Vec<RequirementFlag>);

#[derive(Clone, Debug, PartialEq)]
pub struct AnyOfAllRequirements(pub Vec<AllRequirements>);

pub struct Spot {
    pub name: StrategyFlag,
    pub requirements: Option<AnyOfAllRequirements>,
}

pub struct Shop {
    pub names: (StrategyFlag, StrategyFlag, StrategyFlag),
    pub requirements: Option<AnyOfAllRequirements>,
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
    pub main_weapons: Vec<Spot>,
    pub sub_weapons: Vec<Spot>,
    pub chests: Vec<Spot>,
    pub seals: Vec<Spot>,
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

        let main_weapons = parse_item_spots_events_in_supplement(
            parse_item_spot_requirements(main_weapons),
            &events,
        );
        let sub_weapons = parse_item_spots_events_in_supplement(
            parse_item_spot_requirements(sub_weapons),
            &events,
        );
        let chests =
            parse_item_spots_events_in_supplement(parse_item_spot_requirements(chests), &events);
        let seals =
            parse_item_spots_events_in_supplement(parse_item_spot_requirements(seals), &events);
        let shops = parse_shops_events_in_supplement(parse_shop_requirements(shops), &events);
        debug_assert_eq!(
            &chests
                .iter()
                .find(|x| x.name.0 == "iceCape")
                .unwrap()
                .requirements,
            &Some(AnyOfAllRequirements(vec![
                AllRequirements(vec![
                    RequirementFlag("ankhJewel:templeOfTheSun".into()),
                    RequirementFlag("bronzeMirror".into()),
                    RequirementFlag("shuriken".into()),
                    RequirementFlag("shurikenAmmo".into()),
                ]),
                AllRequirements(vec![
                    RequirementFlag("holyGrail".into()),
                    RequirementFlag("flareGun".into()),
                    RequirementFlag("grappleClaw".into()),
                ]),
                // tslint:disable-next-line:max-line-length
                // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "grappleClaw"],
                AllRequirements(vec![
                    RequirementFlag("bronzeMirror".into()),
                    RequirementFlag("ankhJewel:mausoleumOfTheGiants".into()),
                    RequirementFlag("flareGun".into()),
                    RequirementFlag("grappleClaw".into()),
                ]),
                AllRequirements(vec![
                    RequirementFlag("holyGrail".into()),
                    RequirementFlag("flareGun".into()),
                    RequirementFlag("feather".into()),
                ]),
                // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "feather"],
                AllRequirements(vec![
                    RequirementFlag("bronzeMirror".into()),
                    RequirementFlag("ankhJewel:mausoleumOfTheGiants".into()),
                    RequirementFlag("flareGun".into()),
                    RequirementFlag("feather".into()),
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

fn parse_item_spot_requirements(items: Vec<YamlSpot>) -> Vec<Spot> {
    items
        .into_iter()
        .map(|item| Spot {
            name: StrategyFlag::new(item.name),
            requirements: if item.requirements.is_empty() {
                None
            } else {
                Some(AnyOfAllRequirements(
                    item.requirements
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
            },
        })
        .collect()
}

fn parse_shop_requirements(items: Vec<YamlShop>) -> Vec<Shop> {
    items
        .into_iter()
        .map(|item| Shop {
            names: {
                let names: Vec<_> = item.names.split(',').map(|x| x.trim()).collect();
                debug_assert_eq!(names.len(), 3);
                (
                    StrategyFlag::new(names[0].to_owned()),
                    StrategyFlag::new(names[1].to_owned()),
                    StrategyFlag::new(names[2].to_owned()),
                )
            },
            requirements: if item.requirements.is_empty() {
                None
            } else {
                Some(AnyOfAllRequirements(
                    item.requirements
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
            },
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
                    .all(|y| y.0.iter().all(|z| !z.0.starts_with("event:")))
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
                requirements: parse_events(x.requirements, &events),
            })
            .collect();
    }
    unreachable!();
}

fn parse_item_spots_events_in_supplement(list: Vec<Spot>, events: &[Event]) -> Vec<Spot> {
    list.into_iter()
        .map(|x| {
            if let Some(requirements) = x.requirements {
                Spot {
                    name: x.name.clone(),
                    requirements: Some(parse_events(requirements, events)),
                }
            } else {
                x
            }
        })
        .collect()
}

fn parse_shops_events_in_supplement(list: Vec<Shop>, events: &[Event]) -> Vec<Shop> {
    list.into_iter()
        .map(|x| {
            if let Some(requirements) = x.requirements {
                Shop {
                    names: x.names.clone(),
                    requirements: Some(parse_events(requirements, events)),
                }
            } else {
                x
            }
        })
        .collect()
}

fn parse_events(requirements: AnyOfAllRequirements, events: &[Event]) -> AnyOfAllRequirements {
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
            .all(|target_group| !target_group.0.iter().any(|x| x.0 == event.name))
        {
            continue;
        }
        current = AnyOfAllRequirements(
            current
                .0
                .into_iter()
                .flat_map(|target_group| -> Vec<AllRequirements> {
                    if !target_group.0.iter().any(|x| x.0 == event.name) {
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
                                                x.0 != event.name
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

pub const NIGHT_SURFACE_SUB_WEAPON_COUNT: u8 = 1;
pub const NIGHT_SURFACE_CHEST_COUNT: u8 = 3;
pub const TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT: u8 = 1;
pub const NIGHT_SURFACE_SEAL_COUNT: u8 = 1;
pub const WARE_NO_MISE_COUNT: u8 = 1;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Requirement(pub String);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Spot {
    name: String,
    requirements: Option<Vec<Vec<Requirement>>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Shop {
    names: String,
    requirements: Option<Vec<Vec<Requirement>>>,
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
    requirements: Vec<Vec<Requirement>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Supplements {
    main_weapons: Vec<Spot>,
    sub_weapons: Vec<Spot>,
    chests: Vec<Spot>,
    seals: Vec<Spot>,
    shops: Vec<Shop>,
}

impl Supplements {
    pub fn new(supplement_files: SupplementFiles) -> Self {
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
                .find(|x| x.name == "iceCape")
                .unwrap()
                .requirements,
            &Some(vec![
                vec![
                    Requirement("ankhJewel:templeOfTheSun".into()),
                    Requirement("bronzeMirror".into()),
                    Requirement("shuriken".into()),
                    Requirement("shurikenAmmo".into()),
                ],
                vec![
                    Requirement("holyGrail".into()),
                    Requirement("flareGun".into()),
                    Requirement("grappleClaw".into()),
                ],
                // tslint:disable-next-line:max-line-length
                // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "grappleClaw"],
                vec![
                    Requirement("bronzeMirror".into()),
                    Requirement("ankhJewel:mausoleumOfTheGiants".into()),
                    Requirement("flareGun".into()),
                    Requirement("grappleClaw".into()),
                ],
                vec![
                    Requirement("holyGrail".into()),
                    Requirement("flareGun".into()),
                    Requirement("feather".into()),
                ],
                // vec!["anchor", "knife", "bronzeMirror", "ankhJewel:gateOfGuidance", "flareGun", "feather"],
                vec![
                    Requirement("bronzeMirror".into()),
                    Requirement("ankhJewel:mausoleumOfTheGiants".into()),
                    Requirement("flareGun".into()),
                    Requirement("feather".into()),
                ],
            ])
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
            name: item.name,
            requirements: if item.requirements.is_empty() {
                None
            } else {
                Some(
                    item.requirements
                        .into_iter()
                        .map(|y| {
                            y.split(',')
                                .map(|z| Requirement(z.trim().to_owned()))
                                .collect()
                        })
                        .collect(),
                )
            },
        })
        .collect()
}

fn parse_shop_requirements(items: Vec<YamlShop>) -> Vec<Shop> {
    items
        .into_iter()
        .map(|item| Shop {
            names: item.names,
            requirements: if item.requirements.is_empty() {
                None
            } else {
                Some(
                    item.requirements
                        .into_iter()
                        .map(|y| {
                            y.split(',')
                                .map(|z| Requirement(z.trim().to_owned()))
                                .collect()
                        })
                        .collect(),
                )
            },
        })
        .collect()
}

fn parse_requirements_of_events(items: Vec<YamlSpot>) -> Vec<Event> {
    let mut current: Vec<Event> = items
        .into_iter()
        .map(|x| Event {
            name: x.name,
            requirements: x
                .requirements
                .into_iter()
                .map(|y| {
                    y.split(',')
                        .map(|z| Requirement(z.trim().to_owned()))
                        .collect()
                })
                .collect(),
        })
        .collect();
    for _ in 0..100 {
        let events: Vec<_> = current
            .iter()
            .filter(|x| {
                x.requirements
                    .iter()
                    .all(|y| y.iter().all(|z| !z.0.starts_with("event:")))
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

fn parse_events(requirements: Vec<Vec<Requirement>>, events: &[Event]) -> Vec<Vec<Requirement>> {
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
            .iter()
            .all(|target_group| !target_group.iter().any(|Requirement(x)| x == &event.name))
        {
            continue;
        }
        current = current
            .into_iter()
            .flat_map(|target_group| -> Vec<Vec<Requirement>> {
                if !target_group.iter().any(|Requirement(x)| x == &event.name) {
                    return vec![target_group];
                }
                event
                    .requirements
                    .iter()
                    .map(|event_group| -> Vec<Requirement> {
                        event_group
                            .clone()
                            .into_iter()
                            .chain(
                                target_group
                                    .clone()
                                    .into_iter()
                                    .filter(|Requirement(x)| {
                                        x != &event.name
                                            && !event_group.iter().any(|Requirement(y)| y == x)
                                    })
                                    .collect::<Vec<Requirement>>(),
                            )
                            .collect()
                    })
                    .collect()
            })
            .collect();
    }
    current
}

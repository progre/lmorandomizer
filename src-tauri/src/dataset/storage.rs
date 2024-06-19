use std::{collections::HashSet, thread::spawn};

use super::{item::Item, spot::Spot};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ItemSpot {
    pub spot: Spot,
    pub item: Item,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Shop {
    pub spot: Spot,
    pub talk_number: u16,
    pub items: (Item, Item, Item),
}

#[derive(Clone, getset::Getters, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    #[get = "pub"]
    main_weapon_shutters: Vec<ItemSpot>,
    #[get = "pub"]
    sub_weapon_shutters: Vec<ItemSpot>,
    #[get = "pub"]
    chests: Vec<ItemSpot>,
    #[get = "pub"]
    seal_chests: Vec<ItemSpot>,
    #[get = "pub"]
    shops: Vec<Shop>,
}

impl Storage {
    pub fn new(
        main_weapon_shutters: Vec<ItemSpot>,
        sub_weapon_shutters: Vec<ItemSpot>,
        chests: Vec<ItemSpot>,
        seal_chests: Vec<ItemSpot>,
        shops: Vec<Shop>,
    ) -> Self {
        Self {
            main_weapon_shutters,
            sub_weapon_shutters,
            chests,
            seal_chests,
            shops,
        }
    }

    pub fn all_items(&self) -> Vec<&Item> {
        let mut all_items: Vec<_> = self
            .main_weapon_shutters
            .iter()
            .map(|x| &x.item)
            .chain(self.sub_weapon_shutters.iter().map(|x| &x.item))
            .chain(self.chests.iter().map(|x| &x.item))
            .chain(self.seal_chests.iter().map(|x| &x.item))
            .chain(
                self.shops
                    .iter()
                    .flat_map(|x| [&x.items.0, &x.items.1, &x.items.2]),
            )
            .collect();
        all_items.sort_by_cached_key(|x| x.can_display_in_shop());
        all_items
    }

    pub fn all_requirement_names(&self) -> Vec<String> {
        let mut set = HashSet::new();
        add_item_spot_requirement_item_names_to(&mut set, &self.main_weapon_shutters);
        add_item_spot_requirement_item_names_to(&mut set, &self.sub_weapon_shutters);
        add_item_spot_requirement_item_names_to(&mut set, &self.chests);
        add_item_spot_requirement_item_names_to(&mut set, &self.seal_chests);
        add_shop_requirement_item_names_to(&mut set, &self.shops);
        let mut vec: Vec<_> = set.into_iter().collect();
        vec.sort();
        vec
    }

    pub fn split_reachables_unreachables(
        self,
        current_item_names: &[String],
        sacred_orb_count: u8,
    ) -> (Vec<String>, Self) {
        let (reached_item_names_tx, reached_item_names_rx) = std::sync::mpsc::channel();

        let Self {
            main_weapon_shutters,
            sub_weapon_shutters,
            chests,
            seal_chests,
            shops,
        } = self;

        let main_weapon_shutter_handle = spawn({
            let current_item_names = current_item_names.to_owned();
            let reached_item_names_tx = reached_item_names_tx.clone();
            move || {
                let mut unreached_main_weapon_shutters = Vec::new();
                for item_spot in main_weapon_shutters {
                    if item_spot
                        .spot
                        .is_reachable(&current_item_names, sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name.clone())
                            .unwrap();
                    } else {
                        unreached_main_weapon_shutters.push(item_spot);
                    }
                }
                unreached_main_weapon_shutters
            }
        });
        let sub_weapon_shutters_handle = spawn({
            let current_item_names = current_item_names.to_owned();
            let reached_item_names_tx = reached_item_names_tx.clone();
            move || {
                let mut unreached_sub_weapon_shutters = Vec::new();
                for item_spot in sub_weapon_shutters {
                    if item_spot
                        .spot
                        .is_reachable(&current_item_names, sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name.clone())
                            .unwrap();
                    } else {
                        unreached_sub_weapon_shutters.push(item_spot);
                    }
                }
                unreached_sub_weapon_shutters
            }
        });
        let chests_handle = spawn({
            let current_item_names = current_item_names.to_owned();
            let reached_item_names_tx = reached_item_names_tx.clone();
            move || {
                let mut unreached_chests = Vec::new();
                for item_spot in chests {
                    if item_spot
                        .spot
                        .is_reachable(&current_item_names, sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name.clone())
                            .unwrap();
                    } else {
                        unreached_chests.push(item_spot);
                    }
                }
                unreached_chests
            }
        });
        let seal_chests_handle = spawn({
            let current_item_names = current_item_names.to_owned();
            let reached_item_names_tx = reached_item_names_tx.clone();
            move || {
                let mut unreached_seal_chests = Vec::new();
                for item_spot in seal_chests {
                    if item_spot
                        .spot
                        .is_reachable(&current_item_names, sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name.clone())
                            .unwrap();
                    } else {
                        unreached_seal_chests.push(item_spot);
                    }
                }
                unreached_seal_chests
            }
        });
        let shops_handle = spawn({
            let current_item_names = current_item_names.to_owned();
            let reached_item_names_tx = reached_item_names_tx.clone();
            move || {
                let mut unreached_shops = Vec::new();
                for shop in shops {
                    if shop
                        .spot
                        .is_reachable(&current_item_names, sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(shop.items.0.name.clone())
                            .unwrap();
                        reached_item_names_tx
                            .send(shop.items.1.name.clone())
                            .unwrap();
                        reached_item_names_tx
                            .send(shop.items.2.name.clone())
                            .unwrap();
                    } else {
                        unreached_shops.push(shop);
                    }
                }
                unreached_shops
            }
        });

        let unreached_main_weapon_shutters = main_weapon_shutter_handle.join().unwrap();
        let unreached_sub_weapon_shutters = sub_weapon_shutters_handle.join().unwrap();
        let unreached_chests = chests_handle.join().unwrap();
        let unreached_seal_chests = seal_chests_handle.join().unwrap();
        let unreached_shops = shops_handle.join().unwrap();

        drop(reached_item_names_tx);

        (
            reached_item_names_rx.iter().collect(),
            Self::new(
                unreached_main_weapon_shutters,
                unreached_sub_weapon_shutters,
                unreached_chests,
                unreached_seal_chests,
                unreached_shops,
            ),
        )
    }
}

fn add_item_spot_requirement_item_names_to(set: &mut HashSet<String>, items: &[ItemSpot]) {
    for item in items {
        for group in item.spot.requirement_items().unwrap_or(&Vec::new()) {
            for i in group {
                set.insert(i.name.clone());
            }
        }
    }
}

fn add_shop_requirement_item_names_to(set: &mut HashSet<String>, items: &[Shop]) {
    for item in items {
        for group in item.spot.requirement_items().unwrap_or(&Vec::new()) {
            for i in group {
                set.insert(i.name.clone());
            }
        }
    }
}

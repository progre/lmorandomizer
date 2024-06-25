use std::thread::spawn;

use super::{item::Item, spot::Spot, supplements::StrategyFlag};

#[derive(Clone)]
pub struct ItemSpot {
    pub spot: Spot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Shop {
    pub spot: Spot,
    pub items: (Item, Item, Item),
}

#[derive(Default)]
pub struct StorageIndices {
    pub main_weapon_spot_idx: usize,
    pub sub_weapon_spot_idx: usize,
    pub chest_idx: usize,
    pub seal_chest_idx: usize,
}

#[derive(Clone, getset::Getters)]
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

    pub fn split_reachables_unreachables(
        self,
        current_item_names: &[StrategyFlag],
        current_sacred_orb_count: u8,
    ) -> (Vec<StrategyFlag>, Self) {
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
                        .is_reachable(&current_item_names, current_sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name().to_owned())
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
                        .is_reachable(&current_item_names, current_sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name().to_owned())
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
                        .is_reachable(&current_item_names, current_sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name().to_owned())
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
                        .is_reachable(&current_item_names, current_sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(item_spot.item.name().to_owned())
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
                        .is_reachable(&current_item_names, current_sacred_orb_count)
                    {
                        reached_item_names_tx
                            .send(shop.items.0.name().to_owned())
                            .unwrap();
                        reached_item_names_tx
                            .send(shop.items.1.name().to_owned())
                            .unwrap();
                        reached_item_names_tx
                            .send(shop.items.2.name().to_owned())
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

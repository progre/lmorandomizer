use std::collections::HashSet;

use super::{item::Item, spot::Spot};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ItemSpot {
    pub spot: Spot,
    pub item: Item,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Shop {
    pub spot: Spot,
    pub items: (Item, Item, Item),
}

#[derive(Clone, getset::Getters, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    #[get = "pub"]
    all_requirement_names: Vec<String>,
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
    pub fn create(
        main_weapon_shutters: Vec<ItemSpot>,
        sub_weapon_shutters: Vec<ItemSpot>,
        chests: Vec<ItemSpot>,
        seal_chests: Vec<ItemSpot>,
        shops: Vec<Shop>,
    ) -> Self {
        Self::new(
            {
                let mut set = HashSet::new();
                add_item_spot_requirement_item_names_to(&mut set, &main_weapon_shutters);
                add_item_spot_requirement_item_names_to(&mut set, &sub_weapon_shutters);
                add_item_spot_requirement_item_names_to(&mut set, &chests);
                add_item_spot_requirement_item_names_to(&mut set, &seal_chests);
                add_shop_requirement_item_names_to(&mut set, &shops);
                let mut vec: Vec<_> = set.into_iter().collect();
                vec.sort();
                vec
            },
            main_weapon_shutters,
            sub_weapon_shutters,
            chests,
            seal_chests,
            shops,
        )
    }

    pub fn new(
        all_requirement_names: Vec<String>,
        main_weapon_shutters: Vec<ItemSpot>,
        sub_weapon_shutters: Vec<ItemSpot>,
        chests: Vec<ItemSpot>,
        seal_chests: Vec<ItemSpot>,
        shops: Vec<Shop>,
    ) -> Self {
        debug_assert!(main_weapon_shutters
            .iter()
            .all(|x| x.spot.r#type == "weaponShutter"));
        debug_assert!(sub_weapon_shutters
            .iter()
            .all(|x| x.spot.r#type == "weaponShutter"));
        debug_assert!(chests.iter().all(|x| x.spot.r#type == "chest"));
        debug_assert!(seal_chests.iter().all(|x| x.spot.r#type == "sealChest"));
        debug_assert!(shops.iter().all(|x| x.spot.r#type == "shop"));

        Self {
            all_requirement_names,
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

    pub fn reachable_item_names(
        &self,
        current_item_names: &[String],
        sacred_orb_count: usize,
    ) -> Vec<String> {
        self.main_weapon_shutters
            .iter()
            .filter(|x| x.spot.is_reachable(current_item_names, sacred_orb_count))
            .map(|x| x.item.name.clone())
            .chain(
                self.sub_weapon_shutters
                    .iter()
                    .filter(|x| x.spot.is_reachable(current_item_names, sacred_orb_count))
                    .map(|x| x.item.name.clone()),
            )
            .chain(
                self.chests
                    .iter()
                    .filter(|x| x.spot.is_reachable(current_item_names, sacred_orb_count))
                    .map(|x| x.item.name.clone()),
            )
            .chain(
                self.seal_chests
                    .iter()
                    .filter(|x| x.spot.is_reachable(current_item_names, sacred_orb_count))
                    .map(|x| x.item.name.clone()),
            )
            .chain(
                self.shops
                    .iter()
                    .filter(|x| x.spot.is_reachable(current_item_names, sacred_orb_count))
                    .flat_map(|x| [x.items.0.clone(), x.items.1.clone(), x.items.2.clone()])
                    .map(|x| x.name.clone()),
            )
            .collect()
    }

    pub fn unreachables(self, current_item_names: &[String], sacred_orb_count: usize) -> Self {
        Self::new(
            self.all_requirement_names.clone(),
            self.main_weapon_shutters
                .into_iter()
                .filter(|x| !x.spot.is_reachable(current_item_names, sacred_orb_count))
                .collect(),
            self.sub_weapon_shutters
                .into_iter()
                .filter(|x| !x.spot.is_reachable(current_item_names, sacred_orb_count))
                .collect(),
            self.chests
                .into_iter()
                .filter(|x| !x.spot.is_reachable(current_item_names, sacred_orb_count))
                .collect(),
            self.seal_chests
                .into_iter()
                .filter(|x| !x.spot.is_reachable(current_item_names, sacred_orb_count))
                .collect(),
            self.shops
                .into_iter()
                .filter(|x| !x.spot.is_reachable(current_item_names, sacred_orb_count))
                .collect(),
        )
    }
}

fn add_item_spot_requirement_item_names_to(set: &mut HashSet<String>, items: &[ItemSpot]) {
    for item in items {
        for group in item.spot.requirement_items.as_ref().unwrap_or(&Vec::new()) {
            for i in group {
                set.insert(i.name.clone());
            }
        }
    }
}

fn add_shop_requirement_item_names_to(set: &mut HashSet<String>, items: &[Shop]) {
    for item in items {
        for group in item.spot.requirement_items.as_ref().unwrap_or(&Vec::new()) {
            for i in group {
                set.insert(i.name.clone());
            }
        }
    }
}

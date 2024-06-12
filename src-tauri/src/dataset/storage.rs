use super::{item::Item, spot::Spot};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ItemSpot {
    pub spot: Spot,
    pub item: Item,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Shop {
    pub spot: Spot,
    pub items: (Item, Item, Item),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub all_items: Vec<Item>,
    pub all_requirement_names: Vec<String>,
    pub main_weapon_shutters: Vec<ItemSpot>,
    pub sub_weapon_shutters: Vec<ItemSpot>,
    pub chests: Vec<ItemSpot>,
    pub seal_chests: Vec<ItemSpot>,
    pub shops: Vec<Shop>,
}

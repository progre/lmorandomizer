use crate::dataset::{item::Item, supplements::Supplements};

pub struct AllItems {
    pub main_weapons: Vec<Item>,
    pub sub_weapons: Vec<Item>,
    pub chests: Vec<Item>,
    pub seals: Vec<Item>,
    pub shops: Vec<(Item, Item, Item)>,
}

pub fn get_all_items(supplements: &Supplements) -> AllItems {
    AllItems {
        main_weapons: main_weapons(supplements),
        sub_weapons: sub_weapons(supplements),
        chests: chests(supplements),
        seals: seals(supplements),
        shops: shops(supplements),
    }
}

fn main_weapons(supplements: &Supplements) -> Vec<Item> {
    supplements
        .main_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| Item::main_weapon(i as u8, supplement.name.clone()))
        .collect()
}

fn sub_weapons(supplements: &Supplements) -> Vec<Item> {
    supplements
        .sub_weapons
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            if supplement.name.get().starts_with("ankhJewel:") {
                return Item::sub_weapon_ammo(i as u8, supplement.name.clone());
            }
            Item::sub_weapon_body(i as u8, supplement.name.clone())
        })
        .collect()
}

fn chests(supplements: &Supplements) -> Vec<Item> {
    supplements
        .chests
        .iter()
        .enumerate()
        .map(|(i, supplement)| Item::chest_item(i as u8, supplement.name.clone()))
        .collect()
}

fn seals(supplements: &Supplements) -> Vec<Item> {
    supplements
        .seals
        .iter()
        .enumerate()
        .map(|(i, supplement)| Item::seal(i as u8, supplement.name.clone()))
        .collect()
}

fn shops(supplements: &Supplements) -> Vec<(Item, Item, Item)> {
    supplements
        .shops
        .iter()
        .enumerate()
        .map(|(i, supplement)| {
            (
                Item::shop_item(i as u8, 0, supplement.names.0.clone()),
                Item::shop_item(i as u8, 1, supplement.names.1.clone()),
                Item::shop_item(i as u8, 2, supplement.names.2.clone()),
            )
        })
        .collect()
}

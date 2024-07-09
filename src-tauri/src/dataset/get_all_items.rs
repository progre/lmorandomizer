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
        .map(|(i, spot)| Item::main_weapon(i, spot.name.clone().into()))
        .collect()
}

fn sub_weapons(supplements: &Supplements) -> Vec<Item> {
    supplements
        .sub_weapons
        .iter()
        .enumerate()
        .map(|(i, spot)| {
            if spot.name.get().starts_with("ankhJewel:") {
                return Item::sub_weapon_ammo(i, spot.name.clone().into());
            }
            Item::sub_weapon_body(i, spot.name.clone().into())
        })
        .collect()
}

fn chests(supplements: &Supplements) -> Vec<Item> {
    supplements
        .chests
        .iter()
        .enumerate()
        .map(|(i, spot)| Item::chest_item(i, spot.name.clone().into()))
        .collect()
}

fn seals(supplements: &Supplements) -> Vec<Item> {
    supplements
        .seals
        .iter()
        .enumerate()
        .map(|(i, spot)| Item::seal(i, spot.name.clone().into()))
        .collect()
}

fn shops(supplements: &Supplements) -> Vec<(Item, Item, Item)> {
    supplements
        .shops
        .iter()
        .enumerate()
        .map(|(i, spot)| {
            let flags = spot.to_strategy_flags();
            (
                Item::shop_item(i, 0, flags.0),
                Item::shop_item(i, 1, flags.1),
                Item::shop_item(i, 2, flags.2),
            )
        })
        .collect()
}

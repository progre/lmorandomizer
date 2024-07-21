use super::{
    item::Item,
    spot::{ChestSpot, MainWeaponSpot, SealSpot, ShopSpot, SubWeaponSpot},
};

#[derive(Default)]
pub struct StorageIndices {
    pub main_weapon_spot_idx: usize,
    pub sub_weapon_spot_idx: usize,
    pub chest_idx: usize,
    pub seal_chest_idx: usize,
}

#[derive(Clone)]
pub struct MainWeapon {
    pub spot: MainWeaponSpot,
    pub item: Item,
}

#[derive(Clone)]
pub struct SubWeapon {
    pub spot: SubWeaponSpot,
    pub item: Item,
}

#[derive(Clone)]
pub struct Chest {
    pub spot: ChestSpot,
    pub item: Item,
}

#[derive(Clone)]
pub struct Seal {
    pub spot: SealSpot,
    pub item: Item,
}

#[derive(Clone)]
pub struct Shop {
    pub spot: ShopSpot,
    pub items: (Item, Item, Item),
}

impl Shop {
    pub fn count_general_items(&self) -> usize {
        !self.items.0.name.is_consumable() as usize
            + !self.items.1.name.is_consumable() as usize
            + !self.items.2.name.is_consumable() as usize
    }
}

#[derive(Clone)]
pub struct Storage {
    pub main_weapons: Vec<MainWeapon>,
    pub sub_weapons: Vec<SubWeapon>,
    pub chests: Vec<Chest>,
    pub seals: Vec<Seal>,
    pub shops: Vec<Shop>,
}

impl Storage {
    pub fn new(
        main_weapons: Vec<MainWeapon>,
        sub_weapons: Vec<SubWeapon>,
        chests: Vec<Chest>,
        seals: Vec<Seal>,
        shops: Vec<Shop>,
    ) -> Self {
        Self {
            main_weapons,
            sub_weapons,
            chests,
            seals,
            shops,
        }
    }

    pub fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.main_weapons
            .iter()
            .map(|x| &x.item)
            .chain(self.sub_weapons.iter().map(|x| &x.item))
            .chain(self.chests.iter().map(|x| &x.item))
            .chain(self.seals.iter().map(|x| &x.item))
            .chain(
                self.shops
                    .iter()
                    .flat_map(|x| [&x.items.0, &x.items.1, &x.items.2]),
            )
    }
}

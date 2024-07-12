use super::{item::Item, spot::Spot};

#[derive(Clone, Debug)]
pub struct ItemSpot {
    pub spot: Spot,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct Shop {
    pub spot: Spot,
    pub items: (Item, Item, Item),
}
impl Shop {
    pub fn count_general_items(&self) -> usize {
        !self.items.0.name.is_consumable() as usize
            + !self.items.1.name.is_consumable() as usize
            + !self.items.2.name.is_consumable() as usize
    }
}

#[derive(Default)]
pub struct StorageIndices {
    pub main_weapon_spot_idx: usize,
    pub sub_weapon_spot_idx: usize,
    pub chest_idx: usize,
    pub seal_chest_idx: usize,
}

#[derive(Clone, Debug, getset::Getters, getset::MutGetters)]
pub struct Storage {
    #[getset(get = "pub", get_mut = "pub")]
    main_weapons: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    sub_weapons: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    chests: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    seals: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    shops: Vec<Shop>,
}

impl Storage {
    pub fn new(
        main_weapons: Vec<ItemSpot>,
        sub_weapons: Vec<ItemSpot>,
        chests: Vec<ItemSpot>,
        seals: Vec<ItemSpot>,
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

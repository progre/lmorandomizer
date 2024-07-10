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
    main_weapon_shutters: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    sub_weapon_shutters: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    chests: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
    seal_chests: Vec<ItemSpot>,
    #[getset(get = "pub", get_mut = "pub")]
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

    pub fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.main_weapon_shutters
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
    }
}

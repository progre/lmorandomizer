use std::collections::BTreeMap;

use rand::Rng;

use crate::{
    randomizer::storage::{item::Item, Storage},
    script::enums::FieldNumber,
};

use super::items_pool::{ItemsPool, UnorderedItems};

pub struct Items<'a> {
    priority_items: Vec<&'a Item>,
    maps: BTreeMap<FieldNumber, &'a Item>,
    consumable_items: Vec<&'a Item>,
    general_items: Vec<&'a Item>,
}

impl<'a> Items<'a> {
    pub fn new(source: &'a Storage) -> Self {
        let (maps, chests) = source
            .chests
            .values()
            .partition::<Vec<_>, _>(|x| x.item.name.is_map());
        let maps: BTreeMap<FieldNumber, &Item> = maps
            .into_iter()
            .map(|x| (x.spot.field_number(), &x.item))
            .collect();

        let items = source
            .main_weapons
            .values()
            .map(|x| &x.item)
            .chain(source.sub_weapons.values().map(|x| &x.item))
            .chain(chests.iter().map(|x| &x.item))
            .chain(source.seals.values().map(|x| &x.item))
            .chain(source.shops.iter().map(|x| &x.item))
            .chain(source.roms.values().map(|x| &x.item))
            .chain(source.talks.iter().map(|x| &x.item));
        let (priority_items, remaining_items) = items.partition::<Vec<_>, _>(|item| {
            [
                "handScanner",
                "shellHorn",
                "holyGrail",
                "gameMaster",
                "glyphReader",
            ]
            .contains(&item.name.get())
        });
        let (consumable_items, general_items): (Vec<_>, Vec<_>) = remaining_items
            .into_iter()
            .partition(|x| x.can_display_in_shop() && x.name.is_consumable());

        debug_assert!(priority_items.iter().all(|item| item.can_display_in_shop()));

        Self {
            priority_items,
            maps,
            consumable_items,
            general_items,
        }
    }

    pub fn priority_items(&self) -> &[&'a Item] {
        &self.priority_items
    }
    pub fn maps(&self) -> &BTreeMap<FieldNumber, &'a Item> {
        &self.maps
    }
    pub fn consumable_items(&self) -> &[&'a Item] {
        &self.consumable_items
    }

    pub fn to_items_pool(
        &self,
        rng: &mut impl Rng,
        talk_items_count: usize,
        shop_items_count: usize,
    ) -> ItemsPool<'a> {
        let list = self.general_items.clone();
        let (candidate, mut list) = list.into_iter().partition(|x| x.can_talk());
        let mut candidate = UnorderedItems::new(candidate).shuffle(rng);
        let talk_items = candidate.split_off(candidate.len() - talk_items_count);
        list.append(&mut candidate.into_inner());

        let (candidate, mut list) = list.into_iter().partition(|x| x.can_display_in_shop());
        let mut candidate = UnorderedItems::new(candidate).shuffle(rng);
        let shop_items =
            candidate.split_off(candidate.len() - (shop_items_count - self.consumable_items.len()));
        list.append(&mut candidate.into_inner());
        let field_items = UnorderedItems::new(list).shuffle(rng);

        ItemsPool {
            priority_items: Some(UnorderedItems::new(self.priority_items.clone())),
            consumable_items: UnorderedItems::new(self.consumable_items.clone()),
            field_items,
            talk_items,
            shop_items,
        }
    }
}

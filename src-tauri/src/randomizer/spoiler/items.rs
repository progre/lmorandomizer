use std::collections::BTreeMap;

use rand::Rng;

use crate::{
    dataset::spot::FieldId,
    randomizer::storage::{item::Item, Storage},
};

use super::items_pool::{ItemsPool, UnorderedItems};

pub struct Items<'a> {
    priority_items: Vec<&'a Item>,
    maps: BTreeMap<FieldId, &'a Item>,
    unsellable_items: Vec<&'a Item>,
    consumable_items: Vec<&'a Item>,
    sellable_items: Vec<&'a Item>,
}

impl<'a> Items<'a> {
    pub fn new(source: &'a Storage) -> Self {
        let (maps, chests) = source
            .chests
            .values()
            .partition::<Vec<_>, _>(|x| x.item.name.is_map());
        let maps: BTreeMap<FieldId, &Item> = maps
            .into_iter()
            .map(|x| (x.spot.field_id(), &x.item))
            .collect();

        let items = source
            .main_weapons
            .values()
            .map(|x| &x.item)
            .chain(source.sub_weapons.values().map(|x| &x.item))
            .chain(chests.iter().map(|x| &x.item))
            .chain(source.seals.values().map(|x| &x.item))
            .chain(
                source
                    .shops
                    .iter()
                    .flat_map(|x| [&x.items.0, &x.items.1, &x.items.2])
                    .filter_map(|x| x.as_ref()),
            )
            .chain(source.roms.values().map(|x| &x.item));
        let (priority_items, remaining_items) = items.partition::<Vec<_>, _>(|item| {
            [
                "handScanner",
                "shellHorn",
                "holyGrail",
                "gameMaster",
                "gameMaster2",
                "glyphReader",
            ]
            .contains(&item.name.get())
        });
        let (sellable_items, unsellable_items): (Vec<_>, Vec<_>) = remaining_items
            .into_iter()
            .partition(|x| x.can_display_in_shop());
        let (consumable_items, sellable_items): (Vec<_>, Vec<_>) = sellable_items
            .into_iter()
            .partition(|x| x.name.is_consumable());

        debug_assert!(unsellable_items.iter().all(|x| !x.name.is_consumable()));
        debug_assert_eq!(
            priority_items.len()
                + unsellable_items.len()
                + sellable_items.len()
                + consumable_items.len()
                + maps.len(),
            source.all_items().count(),
        );
        debug_assert_eq!(
            priority_items.len() + unsellable_items.len() + sellable_items.len() + maps.len(),
            source.main_weapons.len()
                + source.sub_weapons.len()
                + source.chests.len()
                + source.seals.len()
                + source
                    .shops
                    .iter()
                    .map(|shop| {
                        let count = |item: &Option<Item>| {
                            item.as_ref()
                                .map_or(0, |x| !x.name.is_consumable() as usize)
                        };
                        count(&shop.items.0) + count(&shop.items.1) + count(&shop.items.2)
                    })
                    .sum::<usize>()
                + source.roms.len(),
        );
        debug_assert!(priority_items.iter().all(|item| item.can_display_in_shop()));

        Self {
            priority_items,
            maps,
            unsellable_items,
            consumable_items,
            sellable_items,
        }
    }

    pub fn priority_items(&self) -> &[&'a Item] {
        &self.priority_items
    }
    pub fn maps(&self) -> &BTreeMap<FieldId, &'a Item> {
        &self.maps
    }
    pub fn unsellable_items(&self) -> &[&'a Item] {
        &self.unsellable_items
    }
    pub fn consumable_items(&self) -> &[&'a Item] {
        &self.consumable_items
    }
    pub fn sellable_items(&self) -> &[&'a Item] {
        &self.sellable_items
    }

    pub fn to_items_pool(&self, rng: &mut impl Rng, shop_display_count: usize) -> ItemsPool<'a> {
        let mut sellable_items = UnorderedItems::new(self.sellable_items.clone()).shuffle(rng);
        let shop_display_count = shop_display_count - self.consumable_items.len();
        let shop_items = sellable_items.split_off(sellable_items.len() - shop_display_count);
        let mut field_items = sellable_items.into_unordered();
        field_items.append(&mut self.unsellable_items.clone());
        ItemsPool {
            priority_items: Some(UnorderedItems::new(self.priority_items.clone())),
            field_items: field_items.shuffle(rng),
            shop_items,
            consumable_items: UnorderedItems::new(self.consumable_items.clone()).shuffle(rng),
        }
    }
}

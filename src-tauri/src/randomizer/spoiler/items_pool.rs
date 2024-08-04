mod items;

use std::mem::take;

use rand::Rng;

use super::spots::{SpotRef, Spots};

use items::{
    fill_items_from, fill_items_including_requires_from, move_items_to, partition_randomly,
};

pub use items::{ShuffledItems, UnorderedItems};

pub struct ItemsPool<'a> {
    pub priority_items: Option<UnorderedItems<'a>>,
    pub field_items: ShuffledItems<'a>,
    pub shop_items: ShuffledItems<'a>,
    pub consumable_items: ShuffledItems<'a>,
}

impl<'a> ItemsPool<'a> {
    fn pick_priority_items(
        &mut self,
        rng: &mut impl Rng,
        fi_spot_cnt: usize,
        shop_display_cnt: usize,
    ) -> (UnorderedItems<'a>, UnorderedItems<'a>) {
        let Some(priority_items) = self.priority_items.take() else {
            return Default::default();
        };
        let (picked_field_items, picked_shop_items) =
            partition_randomly(rng, priority_items, fi_spot_cnt, shop_display_cnt);
        let cnt = picked_shop_items.len();
        let field_items = take(&mut self.field_items);
        self.field_items = move_items_to(field_items, &mut self.shop_items, cnt).shuffle(rng);
        (picked_field_items, picked_shop_items)
    }

    pub fn pick_items_randomly(
        &mut self,
        rng: &mut impl Rng,
        reachables: &Spots<'a>,
        unreachables: &Spots<'a>,
    ) -> (ShuffledItems<'a>, ShuffledItems<'a>) {
        debug_assert_eq!(
            reachables.field_item_spots.len()
                + unreachables.field_item_spots.len()
                + reachables.shops.len()
                + unreachables.shops.len(),
            self.priority_items.as_ref().map_or(0, |x| x.len())
                + self.field_items.len()
                + self.shop_items.len()
                + self.consumable_items.len(),
        );

        let shops: Vec<_> = unreachables
            .shops
            .iter()
            .map(|shop| SpotRef::Shop(shop.spot))
            .collect();
        let remaining_spots: Vec<_> = unreachables
            .field_item_spots
            .iter()
            .chain(shops.iter())
            .collect();
        let fi_spot_cnt = reachables.field_item_spots.len();
        let shop_display_cnt = reachables
            .shops
            .iter()
            .map(|shop| (!shop.name.is_consumable()) as usize)
            .sum::<usize>();

        let (mut field_items, mut shop_items) =
            self.pick_priority_items(rng, fi_spot_cnt, shop_display_cnt);

        // 少なくとも一つは行動を広げるアイテムを配置する
        if field_items
            .iter()
            .chain(shop_items.iter())
            .any(|item| item.is_required(&remaining_spots))
        {
            fill_items_from(&mut field_items, fi_spot_cnt, &mut self.field_items);
            fill_items_from(&mut shop_items, shop_display_cnt, &mut self.shop_items);
        } else {
            let numerator = fi_spot_cnt as u32;
            let denominator = (fi_spot_cnt + shop_display_cnt) as u32;
            if rng.gen_ratio(numerator, denominator) {
                let dst = &mut field_items;
                let src = &mut self.field_items;
                fill_items_including_requires_from(dst, fi_spot_cnt, src, &remaining_spots);
                fill_items_from(&mut shop_items, shop_display_cnt, &mut self.shop_items);
            } else {
                fill_items_from(&mut field_items, fi_spot_cnt, &mut self.field_items);
                let dst = &mut shop_items;
                let src = &mut self.shop_items;
                fill_items_including_requires_from(dst, shop_display_cnt, src, &remaining_spots);
            };
        }
        let field_items = field_items.shuffle(rng);
        let shop_items = shop_items.shuffle(rng);
        (field_items, shop_items)
    }
}

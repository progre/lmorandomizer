mod items;

use std::mem::take;

use rand::Rng;

use super::spots::{SpotRef, Spots};

use items::{fill_items_from, move_one_required_item, partition_randomly};

pub use items::{ShuffledItems, UnorderedItems};

pub struct ItemsPool<'a> {
    pub priority_items: Option<UnorderedItems<'a>>,
    pub field_items: ShuffledItems<'a>,
    pub shop_items: ShuffledItems<'a>,
    pub consumable_items: ShuffledItems<'a>,
}

impl<'a> ItemsPool<'a> {
    fn move_shop_items_to_field_items(&mut self, rng: &mut impl Rng, cnt: usize) {
        self.field_items = take(&mut self.field_items)
            .append_count(&mut self.shop_items, cnt)
            .shuffle(rng);
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
        // 残っているスポット
        let remaining_spots: Vec<_> = unreachables
            .field_item_spots
            .iter()
            .chain(shops.iter())
            .collect();
        // 必要な通常アイテムの数
        let req_f_items = reachables.field_item_spots.len();
        // 必要なショップアイテムの数
        let req_s_items = reachables
            .shops
            .iter()
            .map(|shop| (!shop.name.is_consumable()) as usize)
            .sum::<usize>();

        // 初期配置アイテムの取得、なければなし
        let (mut field_items, mut shop_items) =
            if let Some(priority_items) = self.priority_items.take() {
                let (field_items, shop_items) =
                    partition_randomly(rng, priority_items, req_f_items, req_s_items);
                self.move_shop_items_to_field_items(rng, shop_items.len());
                (field_items, shop_items)
            } else {
                Default::default()
            };
        // 少なくとも一つは行動を広げるアイテムを配置する
        let has_already_required_items = field_items
            .iter()
            .chain(shop_items.iter())
            .any(|item| item.is_required(&remaining_spots));
        if !has_already_required_items {
            let numerator = req_f_items as u32;
            let denominator = (req_f_items + req_s_items) as u32;
            let (dst, src) = if rng.gen_ratio(numerator, denominator) {
                (&mut field_items, &mut self.field_items)
            } else {
                (&mut shop_items, &mut self.shop_items)
            };
            move_one_required_item(dst, src, &remaining_spots);
        }
        fill_items_from(&mut field_items, req_f_items, &mut self.field_items);
        fill_items_from(&mut shop_items, req_s_items, &mut self.shop_items);

        (field_items.shuffle(rng), shop_items.shuffle(rng))
    }
}

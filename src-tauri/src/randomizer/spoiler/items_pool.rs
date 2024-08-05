mod items;

use std::mem::take;

use rand::Rng;

use super::spots::{SpotRef, Spots};

use items::{fill_items_from, move_one_required_item};

pub use items::{ShuffledItems, UnorderedItems};

fn item_shop_spot_count(spots: &Spots<'_>) -> usize {
    spots
        .shops
        .iter()
        .map(|shop| (!shop.name.is_consumable()) as usize)
        .sum::<usize>()
}

pub struct ItemsPool<'a> {
    pub priority_items: Option<UnorderedItems<'a>>,
    pub field_items: ShuffledItems<'a>,
    pub talk_items: ShuffledItems<'a>,
    pub shop_items: ShuffledItems<'a>,
    pub consumable_items: ShuffledItems<'a>,
}

impl<'a> ItemsPool<'a> {
    pub fn move_shop_items_to_field_items(&mut self, rng: &mut impl Rng, cnt: usize) {
        self.field_items = take(&mut self.field_items)
            .append_count(&mut self.shop_items, cnt)
            .shuffle(rng);
    }

    pub fn pick_items_randomly(
        &mut self,
        rng: &mut impl Rng,
        reachables: &Spots<'a>,
        unreachables: &Spots<'a>,
    ) -> (ShuffledItems<'a>, ShuffledItems<'a>, ShuffledItems<'a>) {
        debug_assert_eq!(
            self.shop_items.len() + self.consumable_items.len(),
            reachables.shops.len() + unreachables.shops.len(),
        );
        debug_assert!(self.priority_items.is_none());

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
        // 必要なトークアイテムの数
        let req_t_items = reachables.talk_spots.len();
        // 必要なショップアイテムの数
        let req_s_items = item_shop_spot_count(reachables);

        let mut field_items = Default::default();
        let mut shop_items = Default::default();
        let mut talk_items = Default::default();
        // 少なくとも一つは行動を広げるアイテムを配置する
        let dice = rng.gen_range(0..(req_f_items + req_t_items + req_s_items));
        let (dst, src) = match dice {
            dice if (0..req_f_items).contains(&dice) => (&mut field_items, &mut self.field_items),
            dice if (req_f_items..(req_f_items + req_t_items)).contains(&dice) => {
                (&mut talk_items, &mut self.talk_items)
            }
            _ => (&mut shop_items, &mut self.shop_items),
        };
        move_one_required_item(dst, src, &remaining_spots);
        fill_items_from(&mut field_items, req_f_items, &mut self.field_items);
        fill_items_from(&mut talk_items, req_t_items, &mut self.talk_items);
        fill_items_from(&mut shop_items, req_s_items, &mut self.shop_items);

        (
            field_items.shuffle(rng),
            talk_items.shuffle(rng),
            shop_items.shuffle(rng),
        )
    }
}

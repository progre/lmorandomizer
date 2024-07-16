use std::mem::take;

use rand::{seq::SliceRandom, Rng};

use crate::dataset::{item::Item, spot::Spot};

use super::sphere::Spots;

fn move_items<'a>(dst: &mut UnorderedItems<'a>, src: &mut ShuffledItems<'a>, cnt: usize) {
    dst.0.append(&mut src.split_off(src.len() - cnt).0);
}

fn fill_items_from<'a>(
    dst: &mut UnorderedItems<'a>,
    target_len: usize,
    src: &mut ShuffledItems<'a>,
) {
    let cnt = target_len - dst.0.len();
    move_items(dst, src, cnt);
}

fn pick_items_including_requires<'a>(
    rng: &mut impl Rng,
    items_pool: &mut ShuffledItems<'a>,
    spots: &[&Spot],
    cnt: usize,
) -> UnorderedItems<'a> {
    let Some(pos) = items_pool.0.iter().position(|item| item.is_required(spots)) else {
        let mut list = items_pool.split_off(items_pool.len() - cnt);
        list.0.reverse(); // TODO: reverse() は不要
        return UnorderedItems(list.0);
    };
    let req_item = items_pool.0.swap_remove(pos);
    let mut field_items = UnorderedItems(vec![req_item]);
    let mut items_pool = items_pool.split_off(items_pool.len() - (cnt - 1)); // TODO: reverse() は不要
    items_pool.0.reverse();
    move_items(&mut field_items, &mut items_pool, cnt - 1);
    debug_assert!(!field_items.0.is_empty());
    field_items.0.shuffle(rng); // TODO: このタイミングでのソートは不要
    field_items
}

fn fill_items_including_requires_from<'a>(
    rng: &mut impl Rng,
    dst: &mut UnorderedItems<'a>,
    target_len: usize,
    src: &mut ShuffledItems<'a>,
    spots: &[&Spot],
) {
    let cnt = target_len - dst.len();
    let mut items = pick_items_including_requires(rng, src, spots, cnt);
    dst.0.append(&mut items.0);
}

fn partition_randomly<'a>(
    rng: &mut impl Rng,
    items: UnorderedItems<'a>,
    mut field_spot_count: usize,
    mut shop_display_count: usize,
) -> (UnorderedItems<'a>, UnorderedItems<'a>) {
    let mut field_items: UnorderedItems<'a> = Default::default();
    let mut shop_items: UnorderedItems<'a> = Default::default();
    for item in items.0 {
        let numerator = field_spot_count as u32;
        let denominator = (field_spot_count + shop_display_count) as u32;
        if rng.gen_ratio(numerator, denominator) {
            field_items.0.push(item);
            field_spot_count -= 1;
        } else {
            shop_items.0.push(item);
            shop_display_count -= 1;
        }
    }
    (field_items, shop_items)
}

#[derive(Default)]
pub struct UnorderedItems<'a>(Vec<&'a Item>);

impl<'a> UnorderedItems<'a> {
    pub fn new(items: Vec<&'a Item>) -> Self {
        UnorderedItems(items)
    }

    pub fn append(&mut self, other: &mut Vec<&'a Item>) {
        self.0.append(other);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn shuffle(mut self, rng: &mut impl Rng) -> ShuffledItems<'a> {
        self.0.shuffle(rng);
        ShuffledItems(self.0)
    }
}

#[derive(Default)]
pub struct ShuffledItems<'a>(Vec<&'a Item>);

impl<'a> ShuffledItems<'a> {
    pub fn into_unordered(self) -> UnorderedItems<'a> {
        UnorderedItems(self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<&'a Item> {
        self.0.pop()
    }

    pub fn split_off(&mut self, at: usize) -> ShuffledItems<'a> {
        ShuffledItems(self.0.split_off(at))
    }
}

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
        let (field_items, shop_items) =
            partition_randomly(rng, priority_items, fi_spot_cnt, shop_display_cnt);
        let mut list = self
            .shop_items
            .split_off(self.shop_items.0.len() - shop_items.len()); // TODO: reverse() は不要
        list.0.reverse();
        let mut unordered_field_items = take(&mut self.field_items).into_unordered();
        move_items(&mut unordered_field_items, &mut list, shop_items.len());
        // self.field_items = unordered_field_items.shuffle(rng); TODO: シャッフルする必要がある
        self.field_items = ShuffledItems(unordered_field_items.0);
        (field_items, shop_items)
    }

    pub fn pick_items_randomly(
        &mut self,
        rng: &mut impl Rng,
        reachables: &Spots<'a>,
        unreachables: &Spots<'a>,
    ) -> (ShuffledItems<'a>, ShuffledItems<'a>) {
        let remaining_spots: Vec<_> = unreachables
            .field_item_spots
            .iter()
            .copied()
            .chain(unreachables.shops.iter().map(|shop| &shop.spot))
            .collect();
        let fi_spot_cnt = reachables.field_item_spots.len();
        let shop_display_cnt = reachables
            .shops
            .iter()
            .map(|shop| shop.count_general_items())
            .sum::<usize>();

        let (mut field_items, mut shop_items) =
            self.pick_priority_items(rng, fi_spot_cnt, shop_display_cnt);

        // 少なくとも一つは行動を広げるアイテムを配置する
        if field_items
            .0
            .iter()
            .chain(shop_items.0.iter())
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
                fill_items_including_requires_from(rng, dst, fi_spot_cnt, src, &remaining_spots);
                fill_items_from(&mut shop_items, shop_display_cnt, &mut self.shop_items);
            } else {
                fill_items_from(&mut field_items, fi_spot_cnt, &mut self.field_items);
                let dst = &mut shop_items;
                let src = &mut self.shop_items;
                fill_items_including_requires_from(
                    rng,
                    dst,
                    shop_display_cnt,
                    src,
                    &remaining_spots,
                );
            };
        }
        let field_items = field_items.shuffle(rng);
        let shop_items = shop_items.shuffle(rng);
        (field_items, shop_items)
    }
}

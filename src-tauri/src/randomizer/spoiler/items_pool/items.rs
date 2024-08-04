use rand::{seq::SliceRandom, Rng};

use crate::randomizer::{spoiler::spots::SpotRef, storage::item::Item};

pub fn fill_items_from<'a>(
    dst: &mut UnorderedItems<'a>,
    target_len: usize,
    src: &mut ShuffledItems<'a>,
) {
    let cnt = target_len - dst.0.len();
    dst.append_count(src, cnt);
}

fn pick_items_including_requires<'a>(
    items_pool: &mut ShuffledItems<'a>,
    spots: &[&SpotRef<'a>],
    cnt: usize,
) -> UnorderedItems<'a> {
    debug_assert!(items_pool.len() >= cnt);
    let Some(pos) = items_pool.0.iter().position(|item| item.is_required(spots)) else {
        let shuffled_items = items_pool.split_off(items_pool.len() - cnt);
        return shuffled_items.into_unordered();
    };
    let req_item = items_pool.0.swap_remove(pos);
    let mut field_items = UnorderedItems(vec![req_item]);
    field_items.append_count(items_pool, cnt - 1);
    debug_assert!(!field_items.0.is_empty());
    field_items
}

pub fn fill_items_including_requires_from<'a>(
    dst: &mut UnorderedItems<'a>,
    target_len: usize,
    src: &mut ShuffledItems<'a>,
    spots: &[&SpotRef<'a>],
) {
    debug_assert!(dst.len() + src.len() >= target_len);
    let cnt = target_len - dst.len();
    let mut items = pick_items_including_requires(src, spots, cnt);
    dst.0.append(&mut items.0);
}

pub fn partition_randomly<'a>(
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

    fn append_count(&mut self, other: &mut ShuffledItems<'a>, cnt: usize) {
        self.0.append(&mut other.split_off(other.len() - cnt).0);
    }

    pub fn iter(&self) -> impl Iterator<Item = &&'a Item> {
        self.0.iter()
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

    pub fn append_count(mut self, other: &mut ShuffledItems<'a>, cnt: usize) -> UnorderedItems<'a> {
        self.0.append(&mut other.split_off(other.len() - cnt).0);
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

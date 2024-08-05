use rand::{seq::SliceRandom, Rng};

use crate::randomizer::{spoiler::spots::SpotRef, storage::item::Item};

pub fn fill_items_from<'a>(
    dst: &mut UnorderedItems<'a>,
    target_len: usize,
    src: &mut ShuffledItems<'a>,
) {
    debug_assert!(target_len <= src.len() + dst.len());
    dst.append_count(src, target_len - dst.len());
}

pub fn move_one_required_item<'a>(
    dst: &mut UnorderedItems<'a>,
    src: &mut ShuffledItems<'a>,
    spots: &[&SpotRef<'a>],
) {
    let Some(pos) = src.0.iter().position(|item| item.is_required(spots)) else {
        return;
    };
    dst.0.push(src.0.swap_remove(pos));
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

    pub fn into_inner(self) -> Vec<&'a Item> {
        self.0
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

#[derive(Default, Debug)]
pub struct ShuffledItems<'a>(Vec<&'a Item>);

impl<'a> ShuffledItems<'a> {
    pub fn into_unordered(self) -> UnorderedItems<'a> {
        UnorderedItems(self.0)
    }

    pub fn into_inner(self) -> Vec<&'a Item> {
        self.0
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

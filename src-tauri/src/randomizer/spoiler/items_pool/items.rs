use rand::{Rng, seq::SliceRandom};

use crate::randomizer::storage::item::Item;

#[derive(Clone, Default)]
pub struct UnorderedItems<'a>(Vec<&'a Item>);

impl<'a> UnorderedItems<'a> {
    pub fn new(items: Vec<&'a Item>) -> Self {
        UnorderedItems(items)
    }

    pub fn into_inner(self) -> Vec<&'a Item> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn shuffle(mut self, rng: &mut impl Rng) -> ShuffledItems<'a> {
        self.0.shuffle(rng);
        ShuffledItems(self.0)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ShuffledItems<'a>(Vec<&'a Item>);

impl<'a> ShuffledItems<'a> {
    pub fn as_slice(&self) -> &[&'a Item] {
        &self.0
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

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.0.shuffle(rng);
    }

    pub fn split_off(&mut self, at: usize) -> ShuffledItems<'a> {
        ShuffledItems(self.0.split_off(at))
    }
}

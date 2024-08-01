mod equipment;
mod others;
mod rom;

pub use {
    equipment::Equipment,
    others::{MainWeapon, Seal, SubWeapon},
    rom::Rom,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ChestItem {
    Equipment(Equipment),
    Rom(Rom),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ShopItem {
    Equipment(Equipment),
    Rom(Rom),
    SubWeapon(SubWeapon),
}

impl ShopItem {
    pub fn matches_items(
        left: (Self, Self, Self),
        right: (Option<Self>, Option<Self>, Option<Self>),
    ) -> bool {
        left.0.matches(right.0.as_ref())
            && left.1.matches(right.1.as_ref())
            && left.2.matches(right.2.as_ref())
    }

    pub fn matches(&self, right: Option<&Self>) -> bool {
        right.map_or(true, |x| self == x)
    }
}

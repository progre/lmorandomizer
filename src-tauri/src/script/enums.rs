mod equipment;
mod others;
mod rom;

pub use {
    equipment::Equipment,
    others::{FieldNumber, MainWeapon, Seal, SubWeapon},
    rom::Rom,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChestItem {
    Equipment(Equipment),
    Rom(Rom),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ShopItem {
    Equipment(Equipment),
    Rom(Rom),
    SubWeapon(SubWeapon),
}

impl ShopItem {
    pub fn matches_items(left: [Self; 3], right: [Option<Self>; 3]) -> bool {
        left.iter()
            .zip(right)
            .all(|(left, right)| left.matches(right.as_ref()))
    }

    pub fn matches(&self, right: Option<&Self>) -> bool {
        right.map_or(true, |x| self == x)
    }
}

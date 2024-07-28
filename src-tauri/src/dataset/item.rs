use crate::script::data::items;

use super::spot::{RequirementFlag, SpotName, SpotRef};

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct StrategyFlag(pub String);

impl StrategyFlag {
    pub fn new(spot_name: String) -> Self {
        debug_assert!(
            !spot_name.starts_with("sacredOrb:")
                || spot_name
                    .split(':')
                    .nth(1)
                    .map_or(false, |x| x.parse::<u8>().is_err())
        );
        Self(spot_name)
    }

    pub fn is_sacred_orb(&self) -> bool {
        self.0.starts_with("sacredOrb:")
    }
    pub fn is_map(&self) -> bool {
        self.0.starts_with("map:")
    }

    pub fn is_consumable(&self) -> bool {
        [
            "weights",
            "shurikenAmmo",
            "toukenAmmo",
            "spearAmmo",
            "flareGunAmmo",
            "bombAmmo",
            "ammunition",
        ]
        .contains(&self.0.as_str())
    }

    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<RequirementFlag> for StrategyFlag {
    fn eq(&self, other: &RequirementFlag) -> bool {
        self.0 == other.get()
    }
}

impl From<SpotName> for StrategyFlag {
    fn from(x: SpotName) -> Self {
        Self::new(x.into_inner())
    }
}

#[derive(Clone, Debug)]
pub enum ItemSource {
    MainWeapon(items::MainWeapon),
    SubWeapon(usize),
    Chest(usize),
    Seal(items::Seal),
    Shop(usize, usize),
    Rom(items::Rom),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub src: ItemSource,
    pub name: StrategyFlag,
}

impl Item {
    pub fn main_weapon(name: StrategyFlag, main_weapon: items::MainWeapon) -> Self {
        let src = ItemSource::MainWeapon(main_weapon);
        Self { src, name }
    }
    pub fn sub_weapon(src_idx: usize, name: StrategyFlag) -> Self {
        let src = ItemSource::SubWeapon(src_idx);
        Self { src, name }
    }
    pub fn chest_item(src_idx: usize, name: StrategyFlag) -> Self {
        let src = ItemSource::Chest(src_idx);
        Self { src, name }
    }
    pub fn seal(name: StrategyFlag, seal: items::Seal) -> Self {
        let src = ItemSource::Seal(seal);
        Self { src, name }
    }
    pub fn shop_item(shop_idx: usize, item_idx: usize, name: StrategyFlag) -> Self {
        let src = ItemSource::Shop(shop_idx, item_idx);
        Self { src, name }
    }
    pub fn rom(name: StrategyFlag, rom: items::Rom) -> Self {
        let src = ItemSource::Rom(rom);
        Self { src, name }
    }

    // chests -> equipments / rom
    // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
    // shops -> equipments / rom
    // shops <- subWeapon / subWeaponAmmo / equipments / rom
    pub fn can_display_in_shop(&self) -> bool {
        match &self.src {
            ItemSource::MainWeapon(_) | ItemSource::Seal(_) => false,
            ItemSource::Shop(..) | ItemSource::Rom(_) => true,
            ItemSource::SubWeapon(_) => self.name.get() == "pistol",
            ItemSource::Chest(_) => {
                !self.name.is_map()
                    && !self.name.is_sacred_orb()
                    && (
                        // Boots with set flag 768 (multiples of 256) cannot be sold in shops
                        self.name.get() != "boots"
                    )
            }
        }
    }

    /// 与えられたスポット一覧のいずれかから必要とされているか
    pub fn is_required(&self, spots: &[&SpotRef]) -> bool {
        spots
            .iter()
            .flat_map(|spot| spot.requirements())
            .any(|reqs| {
                reqs.0
                    .iter()
                    .any(|all| all.0.iter().any(|req| req == &self.name))
            })
    }
}

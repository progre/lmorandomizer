use super::{spot::Spot, supplements::StrategyFlag};

#[derive(Clone, Debug)]
pub struct MainWeapon {
    pub src_idx: usize,
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub struct SubWeaponBody {
    pub src_idx: usize,
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub struct SubWeaponAmmo {
    pub src_idx: usize,
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub struct ChestItem {
    pub src_idx: usize,
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub struct Seal {
    pub src_idx: usize,
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub struct ShopItem {
    pub src_idx: (usize, usize),
    pub name: StrategyFlag,
}

#[derive(Clone, Debug)]
pub enum Item {
    MainWeapon(MainWeapon),
    SubWeaponBody(SubWeaponBody),
    SubWeaponAmmo(SubWeaponAmmo),
    #[allow(clippy::enum_variant_names)]
    ChestItem(ChestItem),
    Seal(Seal),
    #[allow(clippy::enum_variant_names)]
    ShopItem(ShopItem),
}

impl Item {
    pub fn main_weapon(src_idx: usize, name: StrategyFlag) -> Self {
        Self::MainWeapon(MainWeapon { src_idx, name })
    }
    pub fn sub_weapon_body(src_idx: usize, name: StrategyFlag) -> Self {
        Self::SubWeaponBody(SubWeaponBody { src_idx, name })
    }
    pub fn sub_weapon_ammo(src_idx: usize, name: StrategyFlag) -> Self {
        Self::SubWeaponAmmo(SubWeaponAmmo { src_idx, name })
    }
    pub fn chest_item(src_idx: usize, name: StrategyFlag) -> Self {
        Self::ChestItem(ChestItem { src_idx, name })
    }
    pub fn seal(src_idx: usize, name: StrategyFlag) -> Self {
        Self::Seal(Seal { src_idx, name })
    }
    pub fn shop_item(shop_idx: usize, item_idx: usize, name: StrategyFlag) -> Self {
        Self::ShopItem(ShopItem {
            src_idx: (shop_idx, item_idx),
            name,
        })
    }

    pub fn name(&self) -> &StrategyFlag {
        match self {
            Self::MainWeapon(x) => &x.name,
            Self::SubWeaponBody(x) => &x.name,
            Self::SubWeaponAmmo(x) => &x.name,
            Self::ChestItem(x) => &x.name,
            Self::Seal(x) => &x.name,
            Self::ShopItem(x) => &x.name,
        }
    }

    // chests -> equipments / rom
    // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
    // shops -> equipments / rom
    // shops <- subWeapon / subWeaponAmmo / equipments / rom
    pub fn can_display_in_shop(&self) -> bool {
        match self {
            Self::MainWeapon(_) => false,
            Self::SubWeaponBody(x) => {
                x.name.get() == "pistol"
                    || x.name.get() == "buckler"
                    || x.name.get() == "handScanner"
            }
            Self::SubWeaponAmmo(_) => true,
            Self::ChestItem(x) => {
                !x.name.is_map()
                    && !x.name.is_sacred_orb()
                    && (
                        // Boots with set flag 768 (multiples of 256) cannot be sold in shops
                        x.name.get() != "boots"
                    )
            }
            Self::Seal(_) => false,
            Self::ShopItem(_) => true,
        }
    }

    /// 与えられたスポット一覧のいずれかから必要とされているか
    pub fn is_required(&self, spots: &[&Spot]) -> bool {
        let name = match self {
            Self::MainWeapon(x) => &x.name,
            Self::SubWeaponBody(x) => &x.name,
            Self::SubWeaponAmmo(x) => &x.name,
            Self::ChestItem(x) => &x.name,
            Self::Seal(x) => &x.name,
            Self::ShopItem(x) => &x.name,
        };
        spots
            .iter()
            .flat_map(|spot| spot.requirements())
            .any(|reqs| reqs.0.iter().any(|all| all.0.iter().any(|req| req == name)))
    }
}

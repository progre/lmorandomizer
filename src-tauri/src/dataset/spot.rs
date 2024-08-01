mod chest_spot;
mod main_weapon_spot;
mod params;
mod rom_spot;
mod seal_spot;
mod shop_spot;
mod sub_weapon_spot;

pub use {
    chest_spot::{ChestItem, ChestSpot},
    main_weapon_spot::MainWeaponSpot,
    params::{AllRequirements, AnyOfAllRequirements, FieldId, RequirementFlag, SpotName},
    rom_spot::RomSpot,
    seal_spot::SealSpot,
    shop_spot::{ShopItem, ShopSpot},
    sub_weapon_spot::SubWeaponSpot,
};

#[derive(Clone, Copy)]
pub enum SpotRef<'a> {
    MainWeapon(&'a MainWeaponSpot),
    SubWeapon(&'a SubWeaponSpot),
    Chest(&'a ChestSpot),
    Seal(&'a SealSpot),
    Shop(&'a ShopSpot),
    Rom(&'a RomSpot),
}

impl SpotRef<'_> {
    pub fn field_id(&self) -> FieldId {
        match self {
            Self::MainWeapon(x) => x.field_id(),
            Self::SubWeapon(x) => x.field_id(),
            Self::Chest(x) => x.field_id(),
            Self::Seal(x) => x.field_id(),
            Self::Shop(x) => x.field_id(),
            Self::Rom(x) => x.field_id(),
        }
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        match self {
            Self::MainWeapon(x) => x.requirements(),
            Self::SubWeapon(x) => x.requirements(),
            Self::Chest(x) => x.requirements(),
            Self::Seal(x) => x.requirements(),
            Self::Shop(x) => x.requirements(),
            Self::Rom(x) => Some(x.requirements()),
        }
    }
}

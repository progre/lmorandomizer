mod internal;

pub use internal::{
    AllRequirements, AnyOfAllRequirements, ChestSpot, FieldId, MainWeaponSpot, RequirementFlag,
    SealSpot, ShopSpot, SpotName, SubWeaponSpot,
};

#[derive(Clone, Copy)]
pub enum SpotRef<'a> {
    MainWeapon(&'a MainWeaponSpot),
    SubWeapon(&'a SubWeaponSpot),
    Chest(&'a ChestSpot),
    Seal(&'a SealSpot),
    Shop(&'a ShopSpot),
}

impl SpotRef<'_> {
    pub fn field_id(&self) -> FieldId {
        match self {
            Self::MainWeapon(x) => x.field_id(),
            Self::SubWeapon(x) => x.field_id(),
            Self::Chest(x) => x.field_id(),
            Self::Seal(x) => x.field_id(),
            Self::Shop(x) => x.field_id(),
        }
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        match self {
            Self::MainWeapon(x) => x.requirements(),
            Self::SubWeapon(x) => x.requirements(),
            Self::Chest(x) => x.requirements(),
            Self::Seal(x) => x.requirements(),
            Self::Shop(x) => x.requirements(),
        }
    }
}

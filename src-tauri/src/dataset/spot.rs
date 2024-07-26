mod internal;

use std::{any::type_name, fmt};

pub use internal::{
    AllRequirements, AnyOfAllRequirements, ChestSpot, FieldId, MainWeaponSpot, RequirementFlag,
    SealSpot, ShopSpot, SpotName, SubWeaponSpot,
};

#[derive(Clone)]
pub enum Spot {
    MainWeapon(MainWeaponSpot),
    SubWeapon(SubWeaponSpot),
    Chest(ChestSpot),
    Seal(SealSpot),
    Shop(ShopSpot),
}

impl Spot {
    pub fn field_id(&self) -> FieldId {
        match self {
            Self::MainWeapon(x) => x.field_id(),
            Self::SubWeapon(x) => x.field_id(),
            Self::Chest(x) => x.field_id(),
            Self::Seal(x) => x.field_id(),
            Self::Shop(x) => x.field_id(),
        }
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_type_name = match self {
            Self::MainWeapon(_) => type_name::<MainWeaponSpot>(),
            Self::SubWeapon(_) => type_name::<SubWeaponSpot>(),
            Self::Chest(_) => type_name::<ChestSpot>(),
            Self::Seal(_) => type_name::<SealSpot>(),
            Self::Shop(_) => type_name::<ShopSpot>(),
        };
        let src_idx = match self {
            Self::MainWeapon(x) => x.src_idx(),
            Self::SubWeapon(x) => x.src_idx(),
            Self::Chest(x) => x.src_idx(),
            Self::Seal(x) => x.src_idx(),
            Self::Shop(x) => x.src_idx(),
        };
        let name = match self {
            Self::MainWeapon(x) => &x.name(),
            Self::SubWeapon(x) => &x.name(),
            Self::Chest(x) => &x.name(),
            Self::Seal(x) => &x.name(),
            Self::Shop(x) => &x.name(),
        };
        write!(
            f,
            "{:?}_{}{}({})",
            self.field_id(),
            full_type_name.split("::").last().unwrap(),
            src_idx,
            name.get()
        )
    }
}

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

    pub fn to_owned(self) -> Spot {
        match self {
            Self::MainWeapon(x) => Spot::MainWeapon((*x).to_owned()),
            Self::SubWeapon(x) => Spot::SubWeapon((*x).to_owned()),
            Self::Chest(x) => Spot::Chest((*x).to_owned()),
            Self::Seal(x) => Spot::Seal((*x).to_owned()),
            Self::Shop(x) => Spot::Shop((*x).to_owned()),
        }
    }
}

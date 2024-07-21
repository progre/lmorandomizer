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

impl From<SpotRef<'_>> for Spot {
    fn from(spot_ref: SpotRef) -> Self {
        match spot_ref {
            SpotRef::MainWeapon(x) => Spot::MainWeapon((*x).clone()),
            SpotRef::SubWeapon(x) => Spot::SubWeapon((*x).clone()),
            SpotRef::Chest(x) => Spot::Chest((*x).clone()),
            SpotRef::Seal(x) => Spot::Seal((*x).clone()),
            SpotRef::Shop(x) => Spot::Shop((*x).clone()),
        }
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
            SpotRef::MainWeapon(x) => x.field_id(),
            SpotRef::SubWeapon(x) => x.field_id(),
            SpotRef::Chest(x) => x.field_id(),
            SpotRef::Seal(x) => x.field_id(),
            SpotRef::Shop(x) => x.field_id(),
        }
    }
    pub fn requirements(&self) -> Option<&AnyOfAllRequirements> {
        match self {
            SpotRef::MainWeapon(x) => x.requirements(),
            SpotRef::SubWeapon(x) => x.requirements(),
            SpotRef::Chest(x) => x.requirements(),
            SpotRef::Seal(x) => x.requirements(),
            SpotRef::Shop(x) => x.requirements(),
        }
    }
}

impl<'a> From<&'a Spot> for SpotRef<'a> {
    fn from(spot: &'a Spot) -> Self {
        match spot {
            Spot::MainWeapon(spot) => Self::MainWeapon(spot),
            Spot::SubWeapon(spot) => Self::SubWeapon(spot),
            Spot::Chest(spot) => Self::Chest(spot),
            Spot::Seal(spot) => Self::Seal(spot),
            Spot::Shop(spot) => Self::Shop(spot),
        }
    }
}

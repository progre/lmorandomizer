use std::fmt;

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::EnumString,
)]
#[repr(u8)]
pub enum MainWeapon {
    Whip,
    ChainWhip,
    Mace,
    Knife,
    KeySword,
    Axe,
    Katana,
}

impl fmt::Display for MainWeapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::EnumString,
)]
#[repr(u8)]
pub enum SubWeapon {
    Shuriken = 0,
    Touken,
    Spear,
    FlareGun,
    Bomb,
    Pistol,
    Weights,
    AnkhJewel,
    Buckler,
    HandScanner,
    SilverShield,
    AngelShield,
    Ammunition,
}

impl fmt::Display for SubWeapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::EnumString,
)]
#[repr(u8)]
pub enum Seal {
    Origin,
    Birth,
    Life,
    Death,
}

impl fmt::Display for Seal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

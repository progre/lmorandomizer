use std::{collections::BTreeMap, fmt, sync::LazyLock};

use num_traits::FromPrimitive;

static REVERSE_MAP: LazyLock<BTreeMap<u8, FieldNumber>> = LazyLock::new(|| {
    let mut map = BTreeMap::new();
    for i in (FieldNumber::GateOfGuidance as u8)..=(FieldNumber::TrueShrineOfTheMother as u8) {
        let field_number = FieldNumber::from_u8(i).unwrap();
        map.insert(field_number.to_logic_number().unwrap(), field_number);
    }
    map
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum FieldNumber {
    GateOfGuidance = 0,
    Surface,
    MausoleumOfTheGiants,
    TempleOfTheSun,
    SpringInTheSky,
    InfernoCavern,
    ChamberOfExtinction,
    EndlessCorridor,
    ShrineOfTheMother,
    TwinLabyrinthsLeft,
    TwinLabyrinthsRight,
    GateOfIllusion,
    GraveyardOfTheGiants,
    TowerOfTheGoddess,
    TempleOfMoonlight,
    TowerOfRuin,
    ChamberOfBirth,
    DimensionalCorridor,
    GaliousCastle,
    TrueShrineOfTheMother,
    GaliousWorld,
    GaliousSmallShrine,
    GaliousMagicSquare,
    GaliousBoss,
    LegendaryTree,
    GaliousFanfare,
    HellTemple1,
    HellTemple2,
    SurfaceNight,
    LavaPit,
    GaliousStagesSwitch,
    Pr3,
    MukimukiMemorial,
    HellTemple3,
    Swimsuit,
}

impl FieldNumber {
    pub fn from_logic_number(logic_number: u8) -> Option<Self> {
        REVERSE_MAP.get(&logic_number).copied()
    }

    pub fn to_logic_number(self) -> Option<u8> {
        match self {
            FieldNumber::Surface => Some(0),
            FieldNumber::GateOfGuidance => Some(1),
            FieldNumber::MausoleumOfTheGiants => Some(2),
            FieldNumber::TempleOfTheSun => Some(3),
            FieldNumber::SpringInTheSky => Some(4),
            FieldNumber::InfernoCavern => Some(5),
            FieldNumber::ChamberOfExtinction => Some(6),
            FieldNumber::TwinLabyrinthsLeft => Some(7),
            FieldNumber::EndlessCorridor => Some(8),
            FieldNumber::ShrineOfTheMother => Some(9),
            FieldNumber::GateOfIllusion => Some(11),
            FieldNumber::GraveyardOfTheGiants => Some(12),
            FieldNumber::TempleOfMoonlight => Some(13),
            FieldNumber::TowerOfTheGoddess => Some(14),
            FieldNumber::TowerOfRuin => Some(15),
            FieldNumber::ChamberOfBirth => Some(16),
            FieldNumber::TwinLabyrinthsRight => Some(17),
            FieldNumber::DimensionalCorridor => Some(18),
            FieldNumber::TrueShrineOfTheMother => Some(19),
            FieldNumber::GaliousCastle => Some(20),
            FieldNumber::GaliousWorld
            | FieldNumber::GaliousSmallShrine
            | FieldNumber::GaliousMagicSquare
            | FieldNumber::GaliousBoss
            | FieldNumber::LegendaryTree
            | FieldNumber::GaliousFanfare
            | FieldNumber::HellTemple1
            | FieldNumber::HellTemple2
            | FieldNumber::SurfaceNight
            | FieldNumber::LavaPit
            | FieldNumber::GaliousStagesSwitch
            | FieldNumber::Pr3
            | FieldNumber::MukimukiMemorial
            | FieldNumber::HellTemple3
            | FieldNumber::Swimsuit => None,
        }
    }
}

impl Ord for FieldNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_logic_number().cmp(&other.to_logic_number())
    }
}

impl PartialOrd for FieldNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for FieldNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

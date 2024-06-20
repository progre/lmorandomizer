use std::num::NonZero;

use crate::script::data::items;

#[derive(Clone, Debug)]
pub struct MainWeapon {
    pub name: String,
    pub number: items::MainWeapon,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct SubWeapon {
    pub name: String,
    pub number: items::SubWeapon,
    pub count: Option<NonZero<u8>>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Equipment {
    pub name: String,
    pub number: items::Equipment,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub name: String,
    pub number: items::Rom,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Seal {
    pub name: String,
    pub number: u8,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub enum Item {
    MainWeapon(MainWeapon),
    SubWeapon(SubWeapon),
    Equipment(Equipment),
    Rom(Rom),
    Seal(Seal),
}

impl Item {
    #[inline]
    fn initial_assert(number: i8, flag: u16, is_sub_weapon: bool) {
        debug_assert!(
            [494, 524].contains(&flag)
                || (684..=883).contains(&flag)
                || is_sub_weapon && flag == 65279,
            "invalid value: {flag} ({number})"
        );
    }
    pub fn main_weapon(name: String, number: items::MainWeapon, flag: u16) -> Self {
        Self::initial_assert(number as i8, flag, false);
        Self::MainWeapon(MainWeapon { name, number, flag })
    }
    pub fn sub_weapon(
        name: String,
        number: items::SubWeapon,
        count: Option<NonZero<u8>>,
        flag: u16,
    ) -> Self {
        Self::initial_assert(number as i8, flag, true);
        Self::SubWeapon(SubWeapon {
            name,
            number,
            count,
            flag,
        })
    }
    pub fn equipment(name: String, number: items::Equipment, flag: u16) -> Self {
        Self::initial_assert(number as i8, flag, false);
        Self::Equipment(Equipment { name, number, flag })
    }
    pub fn rom(name: String, number: items::Rom, flag: u16) -> Self {
        Self::initial_assert(number.0 as i8, flag, false);
        Self::Rom(Rom { name, number, flag })
    }
    pub fn seal(name: String, number: u8, flag: u16) -> Self {
        Self::initial_assert(number as i8, flag, false);
        Self::Seal(Seal { name, number, flag })
    }

    pub fn name(&self) -> &str {
        match self {
            Self::MainWeapon(x) => &x.name,
            Self::SubWeapon(x) => &x.name,
            Self::Equipment(x) => &x.name,
            Self::Rom(x) => &x.name,
            Self::Seal(x) => &x.name,
        }
    }

    pub fn flag(&self) -> u16 {
        match self {
            Self::MainWeapon(x) => x.flag,
            Self::SubWeapon(x) => x.flag,
            Self::Equipment(x) => x.flag,
            Self::Rom(x) => x.flag,
            Self::Seal(x) => x.flag,
        }
    }

    // chests -> equipments / rom
    // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
    // shops -> equipments / rom
    // shops <- subWeapon / subWeaponAmmo / equipments / rom
    pub fn can_display_in_shop(&self) -> bool {
        self.flag() % 256 != 0
            && match self {
                Self::MainWeapon(_) => false,
                Self::SubWeapon(x) => {
                    x.count.is_some()
                        || x.number == items::SubWeapon::Pistol
                        || x.number == items::SubWeapon::Buckler
                        || x.number == items::SubWeapon::HandScanner
                }
                Self::Equipment(x) => {
                    x.number != items::Equipment::Map && x.number != items::Equipment::SacredOrb
                }
                Self::Rom(_) => true,
                Self::Seal(_) => false,
            }
    }
}

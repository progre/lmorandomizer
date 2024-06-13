use crate::randomizer::items::{EquipmentNumber, SubWeaponNumber};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub name: String,
    /// 'mainWeapon' | 'subWeapon' | 'equipment' | 'rom' | 'seal'
    pub r#type: String,
    pub number: i8,
    pub count: u8,
    pub flag: i32,
}

impl Item {
    pub fn new(name: String, r#type: String, number: i8, count: u8, flag: i32) -> Self {
        debug_assert!(
            flag == -1
                || [494, 524].contains(&flag)
                || (684..=883).contains(&flag)
                || r#type == "subWeapon" && flag == 65279,
            "invalid value: {flag} ({number})"
        );
        Self {
            name,
            r#type,
            number,
            count,
            flag,
        }
    }

    // chests -> equipments / rom
    // chests <- subWeapon / subWeaponAmmo / equipments / rom / sign
    // shops -> equipments / rom
    // shops <- subWeapon / subWeaponAmmo / equipments / rom
    pub fn can_display_in_shop(&self) -> bool {
        self.flag % 256 != 0
            && (self.r#type == "equipment"
                && self.number != EquipmentNumber::Map as i8
                && self.number != EquipmentNumber::SacredOrb as i8
                || self.r#type == "rom"
                || self.r#type == "subWeapon"
                    && (self.count > 0
                        || self.number == SubWeaponNumber::Pistol as i8
                        || self.number == SubWeaponNumber::Buckler as i8
                        || self.number == SubWeaponNumber::HandScanner as i8))
    }
}

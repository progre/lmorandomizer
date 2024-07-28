use anyhow::{bail, Result};

use crate::dataset::{self, item::ItemSource, spot::FieldId};

use super::{items, object::Shop, script::Script, shop_items_data::ShopItem};

#[derive(Clone)]
pub struct MainWeapon {
    pub content: items::MainWeapon,
    pub flag: u16,
}

#[derive(Clone)]
pub struct SubWeapon {
    pub content: items::SubWeapon,
    pub amount: u8,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Equipment {
    pub content: items::Equipment,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub content: items::Rom,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub enum ChestItem {
    /// -1 is used only for "open from the fronts"
    None(i32),
    Equipment(Equipment),
    Rom(Rom),
}

impl ChestItem {
    pub fn flag(&self) -> i32 {
        match self {
            ChestItem::Equipment(item) => item.flag as i32,
            ChestItem::Rom(item) => item.flag as i32,
            ChestItem::None(flag) => *flag,
        }
    }
}

#[derive(Clone)]
pub struct Seal {
    pub content: items::Seal,
    pub flag: u16,
}

fn to_field_number(field_id: FieldId) -> u8 {
    match field_id {
        FieldId::Surface => 1,
        FieldId::GateOfGuidance => 0,
        FieldId::MausoleumOfTheGiants => 2,
        FieldId::TempleOfTheSun => 3,
        FieldId::SpringInTheSky => 4,
        FieldId::InfernoCavern => 5,
        FieldId::ChamberOfExtinction => 6,
        FieldId::TwinLabyrinthsLeft => 9,
        FieldId::EndlessCorridor => 7,
        FieldId::ShrineOfTheMother => 8,
        FieldId::GateOfIllusion => 11,
        FieldId::GraveyardOfTheGiants => 12,
        FieldId::TempleOfMoonlight => 14,
        FieldId::TowerOfTheGoddess => 13,
        FieldId::TowerOfRuin => 15,
        FieldId::ChamberOfBirth => 16,
        FieldId::TwinLabyrinthsRight => 10,
        FieldId::DimensionalCorridor => 17,
        FieldId::TrueShrineOfTheMother => 19,
    }
}

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

    pub fn from_dataset(item: &dataset::item::Item, script: &Script) -> Result<Self> {
        Ok(match &item.src {
            ItemSource::MainWeapon(main_weapon) => {
                let Some(item) = script
                    .main_weapons()
                    .find(|x| x.main_weapon().content == *main_weapon)
                else {
                    bail!("invalid main weapon: {}", main_weapon)
                };
                Self::MainWeapon(item.main_weapon().clone())
            }
            ItemSource::SubWeapon((field_id, sub_weapon)) => {
                let Some(item) = script
                    .field(to_field_number(*field_id))
                    .unwrap()
                    .sub_weapons()
                    .find(|x| x.sub_weapon().content == *sub_weapon)
                else {
                    bail!("sub weapon not found: {} {}", field_id, sub_weapon)
                };
                Self::SubWeapon(item.sub_weapon().clone())
            }
            ItemSource::Chest((field_id, dataset::item::ChestItem::Equipment(equipment))) => {
                let Some(item) = script
                    .field(to_field_number(*field_id))
                    .unwrap()
                    .chests()
                    .filter_map(|x| {
                        if let ChestItem::Equipment(x) = x.item() {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .find(|x| x.content == *equipment)
                else {
                    bail!("equipment not found: {} {}", field_id, equipment)
                };
                Self::Equipment(item.clone())
            }
            ItemSource::Chest((field_id, dataset::item::ChestItem::Rom(rom))) => {
                let Some(item) = script
                    .field(to_field_number(*field_id))
                    .unwrap()
                    .chests()
                    .filter_map(|x| {
                        if let ChestItem::Rom(x) = x.item() {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .find(|x| x.content == *rom)
                else {
                    bail!("rom not found: {} {}", field_id, rom)
                };
                Self::Rom(item.clone())
            }
            ItemSource::Seal(seal) => {
                let Some(obj) = script.seals().find(|x| x.seal().content == *seal) else {
                    bail!("invalid seal: {}", seal)
                };
                Self::Seal(obj.seal().clone())
            }
            ItemSource::Shop(shop_idx, item_idx) => {
                let Some(shop) = script
                    .shops()
                    .filter_map(|x| Shop::try_from_shop_object(x, &script.talks).transpose())
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .nth(*shop_idx)
                else {
                    bail!("invalid shop index: {}", shop_idx)
                };
                let item = match *item_idx {
                    0 => shop.items.0,
                    1 => shop.items.1,
                    2 => shop.items.2,
                    _ => bail!("invalid shop item index: {}", item_idx),
                };
                match item {
                    ShopItem::SubWeapon(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.flag, true);
                        Self::SubWeapon(item.item)
                    }
                    ShopItem::Equipment(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.flag, false);
                        Self::Equipment(item.item)
                    }
                    ShopItem::Rom(item) => {
                        Self::initial_assert(item.item.content as i8, item.item.flag, false);
                        Self::Rom(item.item)
                    }
                }
            }
            ItemSource::Rom(rom) => {
                let Some(rom) = script.roms().find(|x| x.rom().content == *rom) else {
                    bail!("invalid rom: {}", rom)
                };
                Self::Rom(rom.rom().clone())
            }
        })
    }

    pub fn flag(&self) -> u16 {
        match self {
            Self::MainWeapon(item) => item.flag,
            Self::SubWeapon(item) => item.flag,
            Self::Equipment(item) => item.flag,
            Self::Rom(item) => item.flag,
            Self::Seal(item) => item.flag,
        }
    }

    pub fn price(&self) -> Option<u16> {
        match self {
            Self::SubWeapon(item) => item.price,
            Self::Equipment(item) => item.price,
            Self::Rom(item) => item.price,
            _ => None,
        }
    }
}

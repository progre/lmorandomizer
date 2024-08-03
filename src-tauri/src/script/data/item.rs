use anyhow::{bail, Result};

use crate::{
    randomizer::storage::item::ItemSource,
    script::enums::{self, FieldNumber},
};

use super::{object::Shop, script::Script, shop_items_data::ShopItem};

#[derive(Clone)]
pub struct MainWeapon {
    pub content: enums::MainWeapon,
    pub flag: u16,
}

#[derive(Clone)]
pub struct SubWeapon {
    pub content: enums::SubWeapon,
    pub amount: u8,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Equipment {
    pub content: enums::Equipment,
    pub price: Option<u16>,
    pub flag: u16,
}

#[derive(Clone, Debug)]
pub struct Rom {
    pub content: enums::Rom,
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
    pub content: enums::Seal,
    pub flag: u16,
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

    fn main_weapon(script: &Script, main_weapon: enums::MainWeapon) -> Result<Self> {
        let Some(item) = script
            .main_weapons()
            .find(|x| x.main_weapon().content == main_weapon)
        else {
            bail!("invalid main weapon: {}", main_weapon)
        };
        Ok(Self::MainWeapon(item.main_weapon().clone()))
    }
    fn sub_weapon(
        script: &Script,
        field_number: FieldNumber,
        sub_weapon: enums::SubWeapon,
    ) -> Result<Self> {
        let Some(item) = script
            .field(field_number)
            .unwrap()
            .sub_weapons()
            .find(|x| x.sub_weapon().content == sub_weapon)
        else {
            bail!("sub weapon not found: {:?} {}", field_number, sub_weapon)
        };
        Ok(Self::SubWeapon(item.sub_weapon().clone()))
    }
    fn chest_equipment(
        script: &Script,
        field_number: FieldNumber,
        equipment: enums::Equipment,
    ) -> Result<Self> {
        let Some(item) = script
            .field(field_number)
            .unwrap()
            .chests()
            .filter_map(|x| {
                if let ChestItem::Equipment(x) = x.item() {
                    Some(x)
                } else {
                    None
                }
            })
            .find(|x| x.content == equipment)
        else {
            bail!("equipment not found: {:?} {}", field_number, equipment)
        };
        Ok(Self::Equipment(item.clone()))
    }
    fn chest_rom(script: &Script, field_number: FieldNumber, rom: enums::Rom) -> Result<Self> {
        let Some(item) = script
            .field(field_number)
            .unwrap()
            .chests()
            .filter_map(|x| {
                if let ChestItem::Rom(x) = x.item() {
                    Some(x)
                } else {
                    None
                }
            })
            .find(|x| x.content == rom)
        else {
            bail!("rom not found: {:?} {}", field_number, rom)
        };
        Ok(Self::Rom(item.clone()))
    }
    fn seal(script: &Script, seal: enums::Seal) -> Result<Self> {
        let Some(obj) = script.seals().find(|x| x.seal().content == seal) else {
            bail!("invalid seal: {}", seal)
        };
        Ok(Self::Seal(obj.seal().clone()))
    }
    fn roadside_rom(script: &Script, rom: enums::Rom) -> Result<Self> {
        let Some(rom) = script.roms().find(|x| x.rom().content == rom) else {
            bail!("invalid rom: {}", rom)
        };
        Ok(Self::Rom(rom.rom().clone()))
    }
    fn shop(script: &Script, items: [Option<enums::ShopItem>; 3], item_idx: usize) -> Result<Self> {
        let Some(shop) = script
            .shops()
            .filter_map(|x| Shop::try_from_shop_object(x, &script.talks).transpose())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .find(|x| {
                let old = ShopItem::to_spot_shop_items(&x.items);
                enums::ShopItem::matches_items(old, items)
            })
        else {
            bail!("invalid shop: {:?}", items)
        };
        let item = match item_idx {
            0 => shop.items.0,
            1 => shop.items.1,
            2 => shop.items.2,
            _ => bail!("invalid shop item index: {}", item_idx),
        };
        Ok(match item {
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
        })
    }

    pub fn new(item_src: &ItemSource, script: &Script) -> Result<Self> {
        match item_src {
            ItemSource::MainWeapon(main_weapon) => Self::main_weapon(script, *main_weapon),
            ItemSource::SubWeapon((field_number, sub_weapon)) => {
                Self::sub_weapon(script, *field_number, *sub_weapon)
            }
            ItemSource::Chest((field_number, enums::ChestItem::Equipment(equipment))) => {
                Self::chest_equipment(script, *field_number, *equipment)
            }
            ItemSource::Chest((field_number, enums::ChestItem::Rom(rom))) => {
                Self::chest_rom(script, *field_number, *rom)
            }
            ItemSource::Seal(seal) => Self::seal(script, *seal),
            ItemSource::Rom(rom) => Self::roadside_rom(script, *rom),
            ItemSource::Shop(items, item_idx) => Self::shop(script, *items, *item_idx),
        }
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

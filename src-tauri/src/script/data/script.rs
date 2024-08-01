use std::mem::take;

use anyhow::Result;

use crate::{
    randomizer::storage::Storage,
    script::file::scripttxtparser::{parse_script_txt, stringify_script_txt},
};

use super::{
    add_starting_items::add_starting_items,
    item::{self, ChestItem},
    items,
    object::{
        ChestObject, MainWeaponObject, Object, RomObject, SealObject, Shop, ShopObject,
        SubWeaponObject, UnknownObject,
    },
    scripteditor::replace_items,
    talks_editor::replace_shops,
};

#[derive(Clone)]
pub struct Map {
    pub attrs: (u8, u8, u8),
    pub up: (i8, i8, i8, i8),
    pub right: (i8, i8, i8, i8),
    pub down: (i8, i8, i8, i8),
    pub left: (i8, i8, i8, i8),
    pub objects: Vec<Object>,
}

#[derive(Clone)]
pub struct Field {
    pub attrs: (u8, u8, u8, u8, u8),
    pub chip_line: (u16, u16),
    pub hits: Vec<(i16, i16)>,
    pub animes: Vec<Vec<u16>>,
    pub objects: Vec<UnknownObject>,
    pub maps: Vec<Map>,
}

impl Field {
    pub fn number(&self) -> u8 {
        self.attrs.0
    }

    pub fn chests(&self) -> impl Iterator<Item = &ChestObject> {
        self.maps.iter().flat_map(|x| &x.objects).filter_map(|x| {
            let Object::Chest(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn sub_weapons(&self) -> impl Iterator<Item = &SubWeaponObject> {
        self.maps.iter().flat_map(|x| &x.objects).filter_map(|x| {
            let Object::SubWeapon(x) = x else {
                return None;
            };
            Some(x)
        })
    }
}

#[derive(Clone)]
pub struct World {
    pub number: u8,
    pub fields: Vec<Field>,
}

#[derive(Clone)]
pub struct Talk(String);

impl Talk {
    pub fn new(txt: String) -> Self {
        Self(txt)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct Script {
    pub talks: Vec<Talk>,
    pub worlds: Vec<World>,
}

impl Script {
    pub fn parse(txt: &str) -> Result<Script> {
        let (talks, worlds) = parse_script_txt(txt)?;
        let zelf = Self { talks, worlds };
        debug_assert_eq!(txt, zelf.stringify());
        Ok(zelf)
    }

    pub fn stringify(&self) -> String {
        stringify_script_txt(&self.talks, &self.worlds)
    }

    pub fn field(&self, number: u8) -> Option<&Field> {
        self.worlds
            .iter()
            .flat_map(|x| &x.fields)
            .find(|x| x.number() == number)
    }

    pub fn main_weapons(&self) -> impl Iterator<Item = &MainWeaponObject> {
        self.view_objects().filter_map(|x| {
            let Object::MainWeapon(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn sub_weapons(&self) -> impl Iterator<Item = &SubWeaponObject> {
        self.view_objects().filter_map(|x| {
            let Object::SubWeapon(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn chests(&self) -> impl Iterator<Item = &ChestObject> {
        self.view_objects()
            // without 2nd twinStatue
            .filter(|x| {
                !(x.number() == 1
                    && x.x() == 8192
                    && x.y() == 6144
                    && x.op1() == 420
                    && x.op2() == 14
                    && x.op3() == 766
                    && x.op4() == 0)
            })
            .filter_map(|x| {
                let Object::Chest(x) = x else {
                    return None;
                };
                Some(x)
            })
            .filter(|chest_obj| {
                !matches!(
                    chest_obj.item(),
                    ChestItem::None(_)
                        | ChestItem::Equipment(item::Equipment {
                            content: items::Equipment::SweetClothing,
                            ..
                        })
                )
            })
    }

    pub fn seals(&self) -> impl Iterator<Item = &SealObject> {
        self.view_objects().filter_map(|x| {
            let Object::Seal(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn shops(&self) -> impl Iterator<Item = &ShopObject> {
        self.view_objects().filter_map(|x| {
            let Object::Shop(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn roms(&self) -> impl Iterator<Item = &RomObject> {
        self.view_objects().filter_map(|x| {
            let Object::Rom(x) = x else {
                return None;
            };
            Some(x)
        })
    }

    pub fn replace_items(&mut self, script: &Script, shuffled: &Storage) -> Result<()> {
        let shops: Vec<_> = self
            .shops()
            .filter_map(|x| Shop::try_from_shop_object(x, &self.talks).transpose())
            .collect::<Result<_>>()?;
        replace_items(&mut self.worlds, script, shuffled)?;
        replace_shops(&mut self.talks, script, &shops, &shuffled.shops)?;
        Ok(())
    }

    pub fn add_starting_items(
        &mut self,
        equipment_list: &[items::Equipment],
        rom_list: &[items::Rom],
        sub_weapon_list: &[items::SubWeapon],
    ) {
        self.worlds = add_starting_items(
            take(&mut self.worlds),
            equipment_list,
            rom_list,
            sub_weapon_list,
        );
    }

    fn view_objects(&self) -> impl Iterator<Item = &Object> {
        self.worlds
            .iter()
            .flat_map(|x| &x.fields)
            .flat_map(|x| &x.maps)
            .flat_map(|y| &y.objects)
    }
}

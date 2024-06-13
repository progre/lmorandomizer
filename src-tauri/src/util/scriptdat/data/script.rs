use std::mem::take;

use anyhow::Result;

use crate::{
    dataset::storage::{self, Storage},
    randomizer::items::{EquipmentNumber, SubWeaponNumber},
    util::scriptdat::{
        data::scripteditor::replace_items,
        format::{
            scripttxtparser::{parse_script_txt, stringify_script_txt},
            shop_items_data::{self, ShopItemData},
        },
    },
};

use super::{
    add_starting_items::add_starting_items, lm_object::LMObject, scripteditor::replace_shops,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMChild {
    pub name: String,
    pub attrs: Vec<i32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMMap {
    pub attrs: (u8, u8, u8),
    pub children: Vec<LMChild>, // TODO: LMChild と LMObject は共に elements
    pub objects: Vec<LMObject>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMField {
    pub attrs: (u8, u8, u8, u8, u8),
    pub children: Vec<LMChild>, // TODO: LMChild, LMObject, LMMap は全て elements
    pub objects: Vec<LMObject>,
    pub maps: Vec<LMMap>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMWorld {
    pub value: u8,
    pub fields: Vec<LMField>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainWeapon {
    pub main_weapon_number: u8,
    pub flag: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubWeapon {
    pub sub_weapon_number: u8,
    pub count: u16,
    pub flag: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChestItem {
    pub chest_item_number: i16,
    pub open_flag: i32,
    pub flag: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Seal {
    pub seal_number: u8,
    pub flag: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shop {
    pub talk_number: i32,
    talking: String,
    pub items: (ShopItemData, ShopItemData, ShopItemData),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub talks: Vec<String>,
    pub worlds: Vec<LMWorld>,
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

    pub fn main_weapons(&self) -> Result<Vec<MainWeapon>> {
        self.view_objects()
            .iter()
            .filter(|x| x.number == 77)
            .map(|x| x.as_main_weapon())
            .collect()
    }

    pub fn sub_weapons(&self) -> Result<Vec<SubWeapon>> {
        self.view_objects()
            .iter()
            .filter(|x| x.number == 13)
            .map(|x| x.as_sub_weapon())
            .collect()
    }

    pub fn chests(&self) -> Result<Vec<ChestItem>> {
        Ok(self
            .view_objects()
            .iter()
            // without 2nd twinStatue
            .filter(|x| {
                !(x.number == 1
                    && x.x == 8192
                    && x.y == 6144
                    && x.op1 == 420
                    && x.op2 == 14
                    && x.op3 == 766
                    && x.op4 == 0)
            })
            .filter(|x| x.number == 1)
            .map(|x| x.as_chest_item())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(
                |ChestItem {
                     chest_item_number, ..
                 }| {
                    *chest_item_number != -1
                        && *chest_item_number != EquipmentNumber::SweetClothing as i16
                },
            )
            .collect())
    }

    pub fn seals(&self) -> Result<Vec<Seal>> {
        self.view_objects()
            .iter()
            .filter(|x| x.number == 71)
            .map(|x| x.as_seal())
            .collect()
    }

    pub fn shops(&self) -> Result<Vec<Shop>> {
        self.view_objects()
            .iter()
            .filter(|x| x.number == 14 && x.op1 <= 99)
            .map(|x| {
                Ok(Shop {
                    talk_number: x.op4,
                    talking: self.talks[usize::try_from(x.op3)?].clone(),
                    items: shop_items_data::parse(&self.talks[usize::try_from(x.op4)?]),
                })
            })
            .collect()
    }

    pub fn replace_shops(&mut self, shops: &[storage::Shop]) -> Result<()> {
        replace_shops(&mut self.talks, shops)
    }

    pub fn replace_items(&mut self, shuffled: &Storage) -> Result<()> {
        self.worlds = replace_items(take(&mut self.worlds), shuffled)?;
        Ok(())
    }

    pub fn add_starting_items(
        &mut self,
        equipment_list: &[EquipmentNumber],
        sub_weapon_list: &[SubWeaponNumber],
    ) {
        self.worlds = add_starting_items(take(&mut self.worlds), equipment_list, sub_weapon_list);
    }

    fn view_objects(&self) -> Vec<LMObject> {
        self.worlds
            .iter()
            .flat_map(|x| &x.fields)
            .flat_map(|x| &x.maps)
            .flat_map(|y| &y.objects)
            .cloned()
            .collect()
    }
}

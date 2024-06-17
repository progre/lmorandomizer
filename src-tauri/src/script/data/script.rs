use std::mem::take;

use anyhow::Result;

use crate::{
    dataset::storage::{self, Storage},
    script::{
        format::scripttxtparser::{parse_script_txt, stringify_script_txt},
        items,
    },
};

use super::{
    add_starting_items::add_starting_items,
    object::{ChestItem, MainWeapon, Object, Seal, Shop, SubWeapon},
    scripteditor::{replace_items, replace_shops},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Map {
    pub attrs: (u8, u8, u8),
    pub up: (i8, i8, i8, i8),
    pub right: (i8, i8, i8, i8),
    pub down: (i8, i8, i8, i8),
    pub left: (i8, i8, i8, i8),
    pub objects: Vec<Object>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Field {
    pub attrs: (u8, u8, u8, u8, u8),
    pub chip_line: (u16, u16),
    pub hits: Vec<(i16, i16)>,
    pub animes: Vec<Vec<u16>>,
    pub objects: Vec<Object>,
    pub maps: Vec<Map>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct World {
    pub number: u8,
    pub fields: Vec<Field>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub talks: Vec<String>,
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

    pub fn main_weapons(&self) -> Result<Vec<MainWeapon>> {
        self.view_objects()
            .iter()
            .filter_map(|x| x.to_main_weapon().transpose())
            .collect()
    }

    pub fn sub_weapons(&self) -> Result<Vec<SubWeapon>> {
        self.view_objects()
            .iter()
            .filter_map(|x| x.to_sub_weapon().transpose())
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
            .filter_map(|x| x.to_chest_item().transpose())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(
                |ChestItem {
                     chest_item_number, ..
                 }| {
                    *chest_item_number != -1
                        && *chest_item_number != items::Equipment::SweetClothing as i16
                },
            )
            .collect())
    }

    pub fn seals(&self) -> Result<Vec<Seal>> {
        self.view_objects()
            .iter()
            .filter_map(|x| x.to_seal().transpose())
            .collect()
    }

    pub fn shops(&self) -> Result<Vec<Shop>> {
        self.view_objects()
            .iter()
            .filter_map(|x| x.to_shop(&self.talks).transpose())
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
        equipment_list: &[items::Equipment],
        sub_weapon_list: &[items::SubWeapon],
    ) {
        self.worlds = add_starting_items(take(&mut self.worlds), equipment_list, sub_weapon_list);
    }

    fn view_objects(&self) -> Vec<Object> {
        self.worlds
            .iter()
            .flat_map(|x| &x.fields)
            .flat_map(|x| &x.maps)
            .flat_map(|y| &y.objects)
            .cloned()
            .collect()
    }
}

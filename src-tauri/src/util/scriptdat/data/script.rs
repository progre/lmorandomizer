use anyhow::Result;

use crate::util::scriptdat::format::scripttxtparser::{parse_script_txt, stringify_script_txt};

use super::lm_object::LMObject;

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
}

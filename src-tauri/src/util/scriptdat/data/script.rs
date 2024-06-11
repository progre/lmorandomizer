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

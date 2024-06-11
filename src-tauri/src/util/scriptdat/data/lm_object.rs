#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMStart {
    pub number: u32,
    pub value: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LMObject {
    pub number: u16,
    pub x: i32,
    pub y: i32,
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    // elements
    pub starts: Vec<LMStart>,
}

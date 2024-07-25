use super::{
    super::item::{ChestItem, Seal},
    Start,
};

#[derive(Clone)]
pub struct ChestObject {
    x: i32,
    y: i32,
    open_flag: u16,
    item: ChestItem,
    unused: i32,
    starts: Vec<Start>,
}

impl ChestObject {
    pub fn new(
        x: i32,
        y: i32,
        open_flag: u16,
        item: ChestItem,
        unused: i32,
        starts: Vec<Start>,
    ) -> Self {
        Self {
            x,
            y,
            open_flag,
            item,
            unused,
            starts,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn open_flag(&self) -> u16 {
        self.open_flag
    }
    pub fn item(&self) -> &ChestItem {
        &self.item
    }
    pub fn op4(&self) -> i32 {
        self.unused
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }
}

#[derive(Clone)]
pub struct SealObject {
    x: i32,
    y: i32,
    seal: Seal,
    starts: Vec<Start>,
}

impl SealObject {
    pub fn new(x: i32, y: i32, seal: Seal, starts: Vec<Start>) -> Self {
        Self { x, y, seal, starts }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn seal(&self) -> &Seal {
        &self.seal
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }
}

#[derive(Clone)]
pub struct UnknownObject {
    pub number: u16,
    pub x: i32,
    pub y: i32,
    pub op1: i32,
    pub op2: i32,
    pub op3: i32,
    pub op4: i32,
    pub starts: Vec<Start>,
}

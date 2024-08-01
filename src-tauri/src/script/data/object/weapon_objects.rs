use super::{
    super::item::{MainWeapon, SubWeapon},
    Start,
};

#[derive(Clone)]
pub struct SubWeaponObject {
    x: i32,
    y: i32,
    sub_weapon: SubWeapon,
    starts: Vec<Start>,
}

impl SubWeaponObject {
    pub fn new(x: i32, y: i32, sub_weapon: SubWeapon, starts: Vec<Start>) -> Self {
        Self {
            x,
            y,
            sub_weapon,
            starts,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn sub_weapon(&self) -> &SubWeapon {
        &self.sub_weapon
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }
}

#[derive(Clone)]
pub struct MainWeaponObject {
    x: i32,
    y: i32,
    main_weapon: MainWeapon,
    starts: Vec<Start>,
}

impl MainWeaponObject {
    pub fn new(x: i32, y: i32, main_weapon: MainWeapon, starts: Vec<Start>) -> Self {
        Self {
            x,
            y,
            main_weapon,
            starts,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn main_weapon(&self) -> &MainWeapon {
        &self.main_weapon
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }
}

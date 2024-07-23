use anyhow::{bail, Result};

const U16_MAX: i32 = u16::MAX as i32;

use crate::script::data::shop_items_data::ShopItem;

use super::{shop_items_data, Start};

pub struct Shop {
    pub talk_number: u16,
    pub items: (ShopItem, ShopItem, ShopItem),
}

#[derive(Clone)]
pub struct ShopObject {
    x: i32,
    y: i32,
    form: i32,
    music: i32,
    talk_number: u16,
    op4: i32,
    starts: Vec<Start>,
}

impl ShopObject {
    pub fn new(
        x: i32,
        y: i32,
        form: i32,
        music: i32,
        talk_number: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if form < 100 && !(matches!(op4, 0..=U16_MAX)) || 200 <= form && op4 != 0 {
            bail!("invalid parameters: op1={}, op4={}", form, op4);
        }
        Ok(Self {
            x,
            y,
            form,
            music,
            talk_number: u16::try_from(talk_number)?,
            op4,
            starts,
        })
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn form(&self) -> i32 {
        self.form
    }
    pub fn music(&self) -> i32 {
        self.music
    }
    pub fn op3(&self) -> i32 {
        self.talk_number as i32
    }
    pub fn op4(&self) -> i32 {
        self.op4
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }

    pub fn to_shop(&self, talks: &[String]) -> Result<Option<Shop>> {
        if self.form >= 100 {
            return Ok(None);
        }
        Ok(Some(Shop {
            talk_number: self.op4 as u16,
            items: shop_items_data::parse(&talks[self.op4 as usize])?,
        }))
    }
}

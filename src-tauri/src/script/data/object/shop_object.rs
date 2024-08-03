use anyhow::{bail, Result};

const U16_MAX: i32 = u16::MAX as i32;

use crate::script::data::{
    shop_items_data::ShopItem,
    talk::{read_u16, Talk},
};

use super::{shop_items_data, Start};

pub struct Storyteller {
    talk_number: u16,
    talk: Talk,
}

impl Storyteller {
    fn new(obj: &ShopObject, talks: &[Talk]) -> Result<Self> {
        let talk_number = obj.op3;
        let Some(talk) = talks.get(talk_number as usize).cloned() else {
            bail!("script broken: talk_number={}", talk_number)
        };
        Ok(Storyteller { talk_number, talk })
    }

    pub fn talk_number(&self) -> u16 {
        self.talk_number
    }
    pub fn into_talk(self) -> Talk {
        self.talk
    }
}

pub struct ItemShop {
    item_data_talk_number: u16,
    items: (ShopItem, ShopItem, ShopItem),
}

impl ItemShop {
    pub fn try_from_shop_object(obj: &ShopObject, talks: &[Talk]) -> Result<Option<Self>> {
        let shop = Shop::try_from_shop_object(obj, talks)?;
        match shop {
            Shop::ItemShop(item_shop) => Ok(Some(item_shop)),
            Shop::Storyteller(_) | Shop::Eldest(_) => Ok(None),
        }
    }

    fn new(obj: &ShopObject, talks: &[Talk]) -> Result<Self> {
        let Some(talk) = talks.get(obj.op4 as usize) else {
            bail!("script broken: talk_number={}", obj.op4)
        };
        Ok(ItemShop {
            item_data_talk_number: obj.op4 as u16,
            items: shop_items_data::parse(talk)?,
        })
    }

    pub fn item_data_talk_number(&self) -> u16 {
        self.item_data_talk_number
    }
    pub fn items(&self) -> &(ShopItem, ShopItem, ShopItem) {
        &self.items
    }
    pub fn into_items(self) -> (ShopItem, ShopItem, ShopItem) {
        self.items
    }
}

pub struct Eldest {
    important_talk_cond_talk_numbers: Vec<u16>,
    important_talk_numbers: Vec<u16>,
    important_talks: Vec<Talk>,
}

impl Eldest {
    pub fn new(obj: &ShopObject, talks: &[Talk]) -> Result<Self> {
        let Some(table_header) = talks.get(obj.op3 as usize).cloned() else {
            bail!("script broken: talk_number={}", obj.op3)
        };
        let bytes = table_header.as_bytes();
        if bytes.len() < 3 {
            bail!("invalid table header: {:?}", bytes);
        }
        let important_talk_cond_base_number = read_u16(bytes[0], bytes[1]);
        let len = bytes[2] as u16;
        if talks.len() < (important_talk_cond_base_number + len) as usize {
            bail!("invalid table header: {:?}", bytes);
        }
        let important_talk_cond_talk_numbers: Vec<_> =
            (important_talk_cond_base_number..(important_talk_cond_base_number + len)).collect();
        let important_talk_numbers: Vec<_> = important_talk_cond_talk_numbers
            .iter()
            .map(|&x| {
                let bytes = talks[x as usize].as_bytes();
                read_u16(bytes[0], bytes[1])
            })
            .collect();
        let important_talks = important_talk_numbers
            .iter()
            .map(|&x| talks[x as usize].clone())
            .collect();
        Ok(Eldest {
            important_talk_cond_talk_numbers,
            important_talk_numbers,
            important_talks,
        })
    }

    pub fn into_important_talk_cond_talk_numbers(self) -> Vec<u16> {
        self.important_talk_cond_talk_numbers
    }
    pub fn important_talk_numbers(&self) -> &[u16] {
        &self.important_talk_numbers
    }
    pub fn into_important_talk_numbers(self) -> Vec<u16> {
        self.important_talk_numbers
    }
    pub fn into_important_talks(self) -> Vec<Talk> {
        self.important_talks
    }
}

pub enum Shop {
    Storyteller(Storyteller),
    #[allow(clippy::enum_variant_names)]
    ItemShop(ItemShop),
    Eldest(Eldest),
}

impl Shop {
    pub fn try_from_shop_object(obj: &ShopObject, talks: &[Talk]) -> Result<Self> {
        Ok(match obj.form {
            0..=99 => Shop::ItemShop(ItemShop::new(obj, talks)?),
            100..=199 => Shop::Storyteller(Storyteller::new(obj, talks)?),
            _ => Shop::Eldest(Eldest::new(obj, talks)?),
        })
    }
}

#[derive(Clone)]
pub struct ShopObject {
    x: i32,
    y: i32,
    form: i32,
    music: i32,
    op3: u16,
    op4: i32,
    starts: Vec<Start>,
}

impl ShopObject {
    pub fn new(
        x: i32,
        y: i32,
        form: i32,
        music: i32,
        op3: i32,
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
            op3: u16::try_from(op3)?,
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
        self.op3 as i32
    }
    pub fn op4(&self) -> i32 {
        self.op4
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }
}

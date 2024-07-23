use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;

const U16_MAX: i32 = u16::MAX as i32;

use super::{
    item::{ChestItem, Equipment, MainWeapon, Rom, Seal, SubWeapon},
    items,
    shop_items_data::{self, ShopItem},
};

#[derive(Clone, Copy, PartialEq)]
pub enum ChestContent {
    Equipment(items::Equipment),
    Rom(items::Rom),
}

pub struct Shop {
    pub talk_number: u16,
    pub items: (ShopItem, ShopItem, ShopItem),
}

#[derive(Clone)]
pub struct Start {
    /// max 99999
    pub flag: u32,
    pub run_when_unset: bool,
}

#[derive(Clone)]
pub struct ChestObject {
    x: i32,
    y: i32,
    open_flag: i32,
    content: Option<ChestContent>,
    set_flag: i32,
    unused: i32,
    starts: Vec<Start>,
}

impl ChestObject {
    fn new(
        x: i32,
        y: i32,
        open_flag: i32,
        content: i32,
        set_flag: i32,
        unused: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if ![-1, 0].contains(&unused) {
            bail!("invalid parameter: op4={}", unused);
        }
        if content >= 0 && !matches!(set_flag, 0..=U16_MAX) {
            bail!("invalid parameters: op2={}, op3={}", content, set_flag);
        }
        Ok(Self {
            x,
            y,
            open_flag,
            content: match content {
                -1 => None,
                0..=99 => Some(ChestContent::Equipment(
                    items::Equipment::from_i32(content)
                        .ok_or_else(|| anyhow!("invalid parameter: op2={}", content))?,
                )),
                _ => Some(ChestContent::Rom(items::Rom(u8::try_from(content - 100)?))),
            },
            set_flag,
            unused,
            starts,
        })
    }

    pub fn open_flag(&self) -> i32 {
        self.open_flag
    }
    pub fn content(&self) -> Option<&ChestContent> {
        self.content.as_ref()
    }
    fn op2(&self) -> i32 {
        match self.content {
            Some(ChestContent::Equipment(content)) => content as i32,
            Some(ChestContent::Rom(content)) => content.0 as i32 + 100,
            None => -1,
        }
    }
    pub fn starts(&self) -> &[Start] {
        &self.starts
    }

    pub fn to_chest_item(&self) -> Option<ChestItem> {
        match self.content {
            Some(ChestContent::Equipment(content)) => Some(ChestItem::Equipment(Equipment {
                content,
                price: None,
                set_flag: self.set_flag as u16,
            })),
            Some(ChestContent::Rom(rom)) => Some(ChestItem::Rom(Rom {
                content: rom,
                price: None,
                set_flag: self.set_flag as u16,
            })),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct SubWeaponObject {
    x: i32,
    y: i32,
    content: items::SubWeapon,
    amount: u8,
    set_flag: u16,
    starts: Vec<Start>,
}

impl SubWeaponObject {
    fn new(
        x: i32,
        y: i32,
        content: i32,
        amount: i32,
        set_flag: i32,
        unused: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if unused != -1 {
            bail!("invalid parameter: op4={}", unused);
        }
        Ok(Self {
            x,
            y,
            content: items::SubWeapon::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: op1={}", content))?,
            amount: u8::try_from(amount)?,
            set_flag: u16::try_from(set_flag)?,
            starts,
        })
    }

    pub fn content(&self) -> items::SubWeapon {
        self.content
    }
    fn op1(&self) -> i32 {
        self.content as i32
    }
    fn op2(&self) -> i32 {
        self.amount as i32
    }
    pub fn set_flag(&self) -> u16 {
        self.set_flag
    }
    fn op3(&self) -> i32 {
        self.set_flag as i32
    }
    const fn op4() -> i32 {
        -1
    }

    pub fn to_sub_weapon(&self) -> SubWeapon {
        SubWeapon {
            content: self.content,
            amount: self.amount,
            price: None,
            set_flag: self.set_flag,
        }
    }
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
    fn new(
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

    fn op3(&self) -> i32 {
        self.talk_number as i32
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

#[derive(Clone)]
pub struct SealObject {
    x: i32,
    y: i32,
    content: items::Seal,
    set_flag: u16,
    starts: Vec<Start>,
}

impl SealObject {
    fn new(
        x: i32,
        y: i32,
        content: i32,
        set_flag: i32,
        op3: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if op3 != -1 || op4 != -1 {
            bail!("invalid parameters: op3={}, op4={}", op3, op4);
        }
        Ok(Self {
            x,
            y,
            content: items::Seal::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: content={}", content))?,
            set_flag: u16::try_from(set_flag)?,
            starts,
        })
    }

    fn op1(&self) -> i32 {
        self.content as i32
    }
    fn op2(&self) -> i32 {
        self.set_flag as i32
    }
    const fn op3() -> i32 {
        -1
    }
    const fn op4() -> i32 {
        -1
    }

    pub fn to_seal(&self) -> Seal {
        Seal {
            content: self.content,
            set_flag: self.set_flag,
        }
    }
}

#[derive(Clone)]
pub struct MainWeaponObject {
    x: i32,
    y: i32,
    content: items::MainWeapon,
    set_flag: u16,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
}

impl MainWeaponObject {
    fn new(
        x: i32,
        y: i32,
        content: i32,
        set_flag: i32,
        op3: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Result<Self> {
        if op3 != -1 || op4 != -1 {
            bail!("invalid parameters: op3={}, op4={}", op3, op4);
        }
        Ok(Self {
            x,
            y,
            content: items::MainWeapon::from_i32(content)
                .ok_or_else(|| anyhow!("invalid parameter: op1={}", content))?,
            set_flag: u16::try_from(set_flag)?,
            op3,
            op4,
            starts,
        })
    }

    fn op1(&self) -> i32 {
        self.content as i32
    }
    fn op2(&self) -> i32 {
        self.set_flag as i32
    }

    pub fn to_main_weapon(&self) -> MainWeapon {
        MainWeapon {
            content: self.content,
            set_flag: self.set_flag,
        }
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

#[derive(Clone)]
pub enum Object {
    Chest(ChestObject),
    SubWeapon(SubWeaponObject),
    Shop(ShopObject),
    Seal(SealObject),
    MainWeapon(MainWeaponObject),
    Unknown(UnknownObject),
}

impl Object {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        number: u16,
        x: i32,
        y: i32,
        op1: i32,
        op2: i32,
        op3: i32,
        op4: i32,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(match number {
            1 => Object::Chest(ChestObject::new(x, y, op1, op2, op3, op4, starts)?),
            13 => Object::SubWeapon(SubWeaponObject::new(x, y, op1, op2, op3, op4, starts)?),
            14 => Object::Shop(ShopObject::new(x, y, op1, op2, op3, op4, starts)?),
            71 => Object::Seal(SealObject::new(x, y, op1, op2, op3, op4, starts)?),
            77 => Object::MainWeapon(MainWeaponObject::new(x, y, op1, op2, op3, op4, starts)?),
            _ => Object::Unknown(UnknownObject {
                number,
                x,
                y,
                op1,
                op2,
                op3,
                op4,
                starts,
            }),
        })
    }

    pub fn chest(
        pos: &Object,
        op1: i32,
        number: i16,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::Chest(ChestObject::new(
            pos.x(),
            pos.y(),
            op1,
            number as i32,
            set_flag as i32,
            -1,
            starts,
        )?))
    }

    pub fn sub_weapon(
        pos: &Object,
        content: items::SubWeapon,
        count: u8,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        if cfg!(debug_assertions) {
            if count == 0 {
                debug_assert!(content != items::SubWeapon::AnkhJewel);
            } else {
                debug_assert!(content != items::SubWeapon::AnkhJewel || count == 1);
            }
        }
        Ok(Object::SubWeapon(SubWeaponObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            count as i32,
            set_flag as i32,
            -1,
            starts,
        )?))
    }

    pub fn seal(
        pos: &Object,
        content: items::Seal,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::Seal(SealObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            set_flag as i32,
            -1,
            -1,
            starts,
        )?))
    }

    pub fn main_weapon(
        pos: &Object,
        content: items::MainWeapon,
        set_flag: u16,
        starts: Vec<Start>,
    ) -> Result<Object> {
        Ok(Object::MainWeapon(MainWeaponObject::new(
            pos.x(),
            pos.y(),
            content as i32,
            set_flag as i32,
            -1,
            -1,
            starts,
        )?))
    }

    pub fn number(&self) -> u16 {
        match self {
            Self::Chest(_) => 1,
            Self::SubWeapon(_) => 13,
            Self::Shop(_) => 14,
            Self::Seal(_) => 71,
            Self::MainWeapon(_) => 77,
            Self::Unknown(obj) => obj.number,
        }
    }
    pub fn x(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.x,
            Self::SubWeapon(obj) => obj.x,
            Self::Shop(obj) => obj.x,
            Self::Seal(obj) => obj.x,
            Self::MainWeapon(obj) => obj.x,
            Self::Unknown(obj) => obj.x,
        }
    }
    pub fn y(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.y,
            Self::SubWeapon(obj) => obj.y,
            Self::Shop(obj) => obj.y,
            Self::Seal(obj) => obj.y,
            Self::MainWeapon(obj) => obj.y,
            Self::Unknown(obj) => obj.y,
        }
    }
    pub fn op1(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.open_flag,
            Self::SubWeapon(obj) => obj.op1(),
            Self::Shop(obj) => obj.form,
            Self::Seal(obj) => obj.op1(),
            Self::MainWeapon(obj) => obj.op1(),
            Self::Unknown(obj) => obj.op1,
        }
    }
    pub fn op2(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.op2(),
            Self::SubWeapon(obj) => obj.op2(),
            Self::Shop(obj) => obj.music,
            Self::Seal(obj) => obj.op2(),
            Self::MainWeapon(obj) => obj.op2(),
            Self::Unknown(obj) => obj.op2,
        }
    }
    pub fn op3(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.set_flag,
            Self::SubWeapon(obj) => obj.op3(),
            Self::Shop(obj) => obj.op3(),
            Self::Seal(_) => SealObject::op3(),
            Self::MainWeapon(obj) => obj.op3,
            Self::Unknown(obj) => obj.op3,
        }
    }
    pub fn op4(&self) -> i32 {
        match self {
            Self::Chest(obj) => obj.unused,
            Self::SubWeapon(_) => SubWeaponObject::op4(),
            Self::Shop(obj) => obj.op4,
            Self::Seal(_) => SealObject::op4(),
            Self::MainWeapon(obj) => obj.op4,
            Self::Unknown(obj) => obj.op4,
        }
    }
    pub fn starts(&self) -> &[Start] {
        match self {
            Self::Chest(obj) => &obj.starts,
            Self::SubWeapon(obj) => &obj.starts,
            Self::Shop(obj) => &obj.starts,
            Self::Seal(obj) => &obj.starts,
            Self::MainWeapon(obj) => &obj.starts,
            Self::Unknown(obj) => &obj.starts,
        }
    }

    pub fn get_item_flag(&self) -> Result<u16> {
        Ok(match self {
            Self::Chest(obj) => u16::try_from(obj.set_flag)?,
            Self::SubWeapon(obj) => obj.set_flag,
            Self::Seal(obj) => obj.set_flag,
            Self::MainWeapon(obj) => obj.set_flag,
            Self::Shop(_) => bail!("shop has no item flag"),
            Self::Unknown(UnknownObject { number, .. }) => bail!("invalid number: {}", number),
        })
    }
}

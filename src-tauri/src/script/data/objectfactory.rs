use anyhow::Result;

use crate::dataset::{self, item::Item};

use super::{
    items::SubWeapon,
    object::{Object, Start},
};

pub fn to_object_for_shutter(old_obj: &Object, start_flag: u16, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => create_main_weapon(old_obj, item)?,
        Item::SubWeapon(item) => create_sub_weapon(old_obj, item)?,
        Item::Equipment(item) => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: item.number as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: starts_that_hide_when_startup(old_obj, start_flag)?,
        },
        Item::Rom(item) => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: 100 + item.number.0 as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: starts_that_hide_when_startup(old_obj, start_flag)?,
        },
        Item::Seal(item) => create_seal(old_obj, item)?,
    })
}

pub fn to_object_for_special_chest(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => create_main_weapon(old_obj, item)?,
        Item::SubWeapon(item) => {
            debug_assert!(
                !(item.number == SubWeapon::AnkhJewel && item.count.map_or(0, |x| x.get()) > 1)
            );
            create_sub_weapon(old_obj, item)?
        }
        Item::Equipment(item) => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: item.number as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: get_starts_without_old_flag(old_obj)?,
        },
        Item::Rom(item) => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: 100 + item.number.0 as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: get_starts_without_old_flag(old_obj)?,
        },
        Item::Seal(item) => create_seal(old_obj, item)?,
    })
}

pub fn to_objects_for_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(match item {
        Item::MainWeapon(item) => create_main_weapon_chest(old_obj, item)?,
        Item::SubWeapon(item) => {
            debug_assert!(
                !(item.number == SubWeapon::AnkhJewel && item.count.map_or(0, |x| x.get()) > 1)
            );
            create_sub_weapon_chest(old_obj, item)?
        }
        Item::Equipment(item) => vec![Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: old_obj.op1,
            op2: item.number as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: starts_as_is(old_obj, item.flag)?,
        }],
        Item::Rom(item) => vec![Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: old_obj.op1,
            op2: 100 + item.number.0 as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: starts_as_is(old_obj, item.flag)?,
        }],
        Item::Seal(item) => create_seal_chest(old_obj, item)?,
    })
}

fn create_main_weapon(old_obj: &Object, item: &dataset::item::MainWeapon) -> Result<Object> {
    Ok(Object {
        number: 77,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.flag as i32,
        op3: -1,
        op4: -1,
        starts: starts_as_is(old_obj, item.flag)?,
    })
}

fn create_main_weapon_chest(
    old_obj: &Object,
    item: &dataset::item::MainWeapon,
) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item.flag)?,
        Object {
            number: 77,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.flag as i32,
            op3: -1,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item.flag)?,
        },
    ])
}

fn create_sub_weapon(old_obj: &Object, item: &dataset::item::SubWeapon) -> Result<Object> {
    Ok(Object {
        number: 13,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.count.map_or(0, |x| x.get()) as i32,
        op3: item.flag as i32,
        op4: -1,
        starts: starts_as_is(old_obj, item.flag)?,
    })
}

fn create_sub_weapon_chest(
    old_obj: &Object,
    item: &dataset::item::SubWeapon,
) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item.flag)?,
        Object {
            number: 13,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.count.map_or(0, |x| x.get()) as i32,
            op3: item.flag as i32,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item.flag)?,
        },
    ])
}

fn create_seal(old_obj: &Object, item: &dataset::item::Seal) -> Result<Object> {
    Ok(Object {
        number: 71,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.flag as i32,
        op3: -1,
        op4: -1,
        starts: starts_as_is(old_obj, item.flag)?,
    })
}

fn create_seal_chest(old_obj: &Object, item: &dataset::item::Seal) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item.flag)?,
        Object {
            number: 71,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.flag as i32,
            op3: -1,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item.flag)?,
        },
    ])
}

fn create_empty_chest(old_obj: &Object, flag: u16) -> Result<Object> {
    Ok(Object {
        number: 1,
        x: old_obj.x,
        y: old_obj.y,
        op1: old_obj.op1,
        op2: -1,
        op3: old_obj.op1,
        op4: -1,
        starts: starts_as_is(old_obj, flag)?,
    })
}

fn starts_that_hide_when_startup_and_taken(
    old_chest_obj: &Object,
    flag: u16,
) -> Result<Vec<Start>> {
    debug_assert_eq!(old_chest_obj.number, 1);
    let mut vec = vec![
        Start {
            number: 99999,
            run_when_unset: true,
        },
        Start {
            number: u32::try_from(old_chest_obj.to_chest_item()?.unwrap().open_flag)?,
            run_when_unset: true,
        },
        Start {
            number: flag as u32,
            run_when_unset: false,
        },
    ];
    vec.append(
        &mut get_starts_without_old_flag(old_chest_obj)?
            .into_iter()
            .filter(|x| x.number != 99999)
            .collect::<Vec<_>>(),
    );
    Ok(vec)
}

fn starts_that_hide_when_startup(old_obj: &Object, start_flag: u16) -> Result<Vec<Start>> {
    let mut vec = vec![
        Start {
            number: 99999,
            run_when_unset: true,
        },
        Start {
            number: start_flag as u32,
            run_when_unset: true,
        },
    ];
    vec.append(
        &mut get_starts_without_old_flag(old_obj)?
            .into_iter()
            .filter(|x| x.number != 99999)
            .collect(),
    );
    Ok(vec)
}

fn starts_as_is(old_obj: &Object, flag: u16) -> Result<Vec<Start>> {
    let mut vec = get_starts_without_old_flag(old_obj)?;
    let item_flag = old_obj.get_item_flag()?;
    if old_obj.starts.iter().any(|x| x.number == item_flag as u32) {
        vec.push(Start {
            number: flag as u32,
            run_when_unset: false,
        });
    }
    Ok(vec)
}

fn get_starts_without_old_flag(old_obj: &Object) -> Result<Vec<Start>> {
    let item_flag = old_obj.get_item_flag()?;
    Ok(old_obj
        .starts
        .iter()
        .filter(|x| x.number != item_flag as u32)
        .map(|x| Start {
            number: x.number,
            run_when_unset: x.run_when_unset,
        })
        .collect())
}

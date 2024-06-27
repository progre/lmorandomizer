use anyhow::Result;

use crate::dataset::item::Item;

use super::object::{Object, Start};

pub fn to_object_for_shutter(old_obj: &Object, start_flag: u16, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::main_weapon(old_obj, item.content, item.flag, starts)
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::sub_weapon_body(old_obj, item.content, item.flag, starts)
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::sub_weapon_ammo(old_obj, item.content, item.count, item.flag, starts)
        }
        Item::Equipment(item) => {
            let starts = starts_that_hide_when_startup(old_obj, start_flag)?;
            Object::chest(old_obj, 40, item.content as i16, item.flag, starts)
        }
        Item::Rom(item) => {
            let starts = starts_that_hide_when_startup(old_obj, start_flag)?;
            Object::chest(old_obj, 40, 100 + item.content.0 as i16, item.flag, starts)
        }
        Item::Seal(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::seal(old_obj, item.content, item.flag, starts)
        }
    })
}

pub fn to_object_for_special_chest(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::main_weapon(old_obj, item.content, item.flag, starts)
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::sub_weapon_body(old_obj, item.content, item.flag, starts)
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::sub_weapon_ammo(old_obj, item.content, item.count, item.flag, starts)
        }
        Item::Equipment(item) => {
            let starts = get_starts_without_old_flag(old_obj)?;
            Object::chest(old_obj, 40, item.content as i16, item.flag, starts)
        }
        Item::Rom(item) => {
            let starts = get_starts_without_old_flag(old_obj)?;
            Object::chest(old_obj, 40, 100 + item.content.0 as i16, item.flag, starts)
        }
        Item::Seal(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            Object::seal(old_obj, item.content, item.flag, starts)
        }
    })
}

pub fn to_objects_for_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag)?;
            vec![
                create_empty_chest(old_obj, item.flag)?,
                Object::main_weapon(old_obj, item.content, item.flag, starts),
            ]
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag)?;
            vec![
                create_empty_chest(old_obj, item.flag)?,
                Object::sub_weapon_body(old_obj, item.content, item.flag, starts),
            ]
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag)?;
            vec![
                create_empty_chest(old_obj, item.flag)?,
                Object::sub_weapon_ammo(old_obj, item.content, item.count, item.flag, starts),
            ]
        }
        Item::Equipment(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            let chest = Object::chest(old_obj, old_obj.op1, item.content as i16, item.flag, starts);
            vec![chest]
        }
        Item::Rom(item) => {
            let starts = starts_as_is(old_obj, item.flag)?;
            let number = 100 + item.content.0 as i16;
            let chest = Object::chest(old_obj, old_obj.op1, number, item.flag, starts);
            vec![chest]
        }
        Item::Seal(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag)?;
            vec![
                create_empty_chest(old_obj, item.flag)?,
                Object::seal(old_obj, item.content, item.flag, starts),
            ]
        }
    })
}

fn create_empty_chest(old_obj: &Object, flag: u16) -> Result<Object> {
    let starts = starts_as_is(old_obj, flag)?;
    let obj = Object::chest(old_obj, old_obj.op1, -1, old_obj.op1 as u16, starts);
    Ok(obj)
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
        .cloned()
        .collect())
}

use anyhow::Result;

use super::{
    item::Item,
    object::{Object, Start},
};

pub fn to_object_for_shutter(old_obj: &Object, start_flag: u16, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::main_weapon(old_obj, item.content, item.set_flag, starts)
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::sub_weapon_body(old_obj, item.content, item.set_flag, starts)
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::sub_weapon_ammo(old_obj, item.content, item.amount, item.set_flag, starts)
        }
        Item::Equipment(item) => {
            let starts = starts_that_hide_when_startup(old_obj, start_flag)?;
            Object::chest(old_obj, 40, item.content as i16, item.set_flag, starts)
        }
        Item::Rom(item) => {
            let starts = starts_that_hide_when_startup(old_obj, start_flag)?;
            let number = 100 + item.content.0 as i16;
            Object::chest(old_obj, 40, number, item.set_flag, starts)
        }
        Item::Seal(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::seal(old_obj, item.content, item.set_flag, starts)
        }
    })
}

pub fn to_object_for_special_chest(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::main_weapon(old_obj, item.content, item.set_flag, starts)
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::sub_weapon_body(old_obj, item.content, item.set_flag, starts)
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::sub_weapon_ammo(old_obj, item.content, item.amount, item.set_flag, starts)
        }
        Item::Equipment(item) => {
            let starts = starts_without_old_flag(old_obj)?;
            Object::chest(old_obj, 40, item.content as i16, item.set_flag, starts)
        }
        Item::Rom(item) => {
            let starts = starts_without_old_flag(old_obj)?;
            let number = 100 + item.content.0 as i16;
            Object::chest(old_obj, 40, number, item.set_flag, starts)
        }
        Item::Seal(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            Object::seal(old_obj, item.content, item.set_flag, starts)
        }
    })
}

pub fn to_objects_for_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(match item {
        Item::MainWeapon(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag)?;
            vec![
                create_empty_chest(old_obj, item.set_flag)?,
                Object::main_weapon(old_obj, item.content, item.set_flag, starts),
            ]
        }
        Item::SubWeaponBody(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag)?;
            vec![
                create_empty_chest(old_obj, item.set_flag)?,
                Object::sub_weapon_body(old_obj, item.content, item.set_flag, starts),
            ]
        }
        Item::SubWeaponAmmo(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag)?;
            vec![
                create_empty_chest(old_obj, item.set_flag)?,
                Object::sub_weapon_ammo(old_obj, item.content, item.amount, item.set_flag, starts),
            ]
        }
        Item::Equipment(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            let number = item.content as i16;
            let chest = Object::chest(old_obj, old_obj.op1, number, item.set_flag, starts);
            vec![chest]
        }
        Item::Rom(item) => {
            let starts = starts_as_is(old_obj, item.set_flag)?;
            let number = 100 + item.content.0 as i16;
            let chest = Object::chest(old_obj, old_obj.op1, number, item.set_flag, starts);
            vec![chest]
        }
        Item::Seal(item) => {
            let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag)?;
            vec![
                create_empty_chest(old_obj, item.set_flag)?,
                Object::seal(old_obj, item.content, item.set_flag, starts),
            ]
        }
    })
}

fn create_empty_chest(old_obj: &Object, flag: u16) -> Result<Object> {
    let starts = starts_as_is(old_obj, flag)?;
    let obj = Object::chest(old_obj, old_obj.op1, -1, old_obj.op1 as u16, starts);
    Ok(obj)
}

fn starts_that_hide_when_startup_and_taken(old_obj: &Object, flag: u16) -> Result<Vec<Start>> {
    debug_assert_eq!(old_obj.number, 1);
    let old_item_flag = old_obj.get_item_flag()?;
    Ok([
        Start {
            flag: 99999,
            run_when_unset: true,
        },
        Start {
            flag: u32::try_from(old_obj.to_chest_item()?.unwrap().open_flag)?,
            run_when_unset: true,
        },
        Start {
            flag: flag as u32,
            run_when_unset: false,
        },
    ]
    .into_iter()
    .chain(
        old_obj
            .starts
            .iter()
            .filter(|x| ![99999, old_item_flag as u32].contains(&x.flag))
            .cloned(),
    )
    .collect())
}

fn starts_that_hide_when_startup(old_obj: &Object, start_flag: u16) -> Result<Vec<Start>> {
    let old_item_flag = old_obj.get_item_flag()?;
    Ok([
        Start {
            flag: 99999,
            run_when_unset: true,
        },
        Start {
            flag: start_flag as u32,
            run_when_unset: true,
        },
    ]
    .into_iter()
    .chain(
        old_obj
            .starts
            .iter()
            .filter(|x| ![99999, old_item_flag as u32].contains(&x.flag))
            .cloned(),
    )
    .collect())
}

fn starts_as_is(old_obj: &Object, flag: u16) -> Result<Vec<Start>> {
    let mut vec = starts_without_old_flag(old_obj)?;
    let old_item_flag = old_obj.get_item_flag()?;
    if old_obj
        .starts
        .iter()
        .any(|x| x.flag == old_item_flag as u32)
    {
        vec.push(Start {
            flag: flag as u32,
            run_when_unset: false, // TODO: bug? 元の run_when_unset を維持すべきの筈
        });
    }
    Ok(vec)
}

fn starts_without_old_flag(old_obj: &Object) -> Result<Vec<Start>> {
    let old_item_flag = old_obj.get_item_flag()?;
    Ok(old_obj
        .starts
        .iter()
        .filter(|x| x.flag != old_item_flag as u32)
        .cloned()
        .collect())
}

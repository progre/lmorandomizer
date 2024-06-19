use anyhow::Result;

use crate::dataset::item::Item;

use super::{
    items::SubWeapon,
    object::{Start, Object},
};

pub fn to_object_for_shutter(old_obj: &Object, start_flag: i32, item: &Item) -> Result<Object> {
    Ok(match item.r#type.as_ref() {
        "mainWeapon" => create_main_weapon(old_obj, item)?,
        "subWeapon" => create_sub_weapon(old_obj, item)?,
        "equipment" => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: starts_that_hide_when_startup(old_obj, start_flag)?,
        },
        "rom" => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: 100 + item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: starts_that_hide_when_startup(old_obj, start_flag)?,
        },
        "seal" => create_seal(old_obj, item)?,
        _ => unreachable!(),
    })
}

pub fn to_object_for_special_chest(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(match item.r#type.as_ref() {
        "mainWeapon" => create_main_weapon(old_obj, item)?,
        "subWeapon" => {
            debug_assert!(!(item.number == SubWeapon::AnkhJewel as i8 && item.count > 1));
            create_sub_weapon(old_obj, item)?
        }
        "equipment" => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: get_starts_without_old_flag(old_obj)?,
        },
        "rom" => Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: 40,
            op2: 100 + item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: get_starts_without_old_flag(old_obj)?,
        },
        "seal" => create_seal(old_obj, item)?,
        _ => unreachable!(),
    })
}

pub fn to_objects_for_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(match item.r#type.as_ref() {
        "mainWeapon" => {
            debug_assert!(!(item.number == SubWeapon::AnkhJewel as i8 && item.count > 1));
            create_main_weapon_chest(old_obj, item)?
        }
        "subWeapon" => {
            debug_assert!(!(item.number == SubWeapon::AnkhJewel as i8 && item.count > 1));
            create_sub_weapon_chest(old_obj, item)?
        }
        "equipment" => vec![Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: old_obj.op1,
            op2: item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: starts_as_is(old_obj, item)?,
        }],
        "rom" => vec![Object {
            number: 1,
            x: old_obj.x,
            y: old_obj.y,
            op1: old_obj.op1,
            op2: 100 + item.number as i32,
            op3: item.flag,
            op4: -1,
            starts: starts_as_is(old_obj, item)?,
        }],
        "seal" => create_seal_chest(old_obj, item)?,
        _ => unreachable!(),
    })
}

fn create_main_weapon(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(Object {
        number: 77,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.flag,
        op3: -1,
        op4: -1,
        starts: starts_as_is(old_obj, item)?,
    })
}

fn create_main_weapon_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item)?,
        Object {
            number: 77,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.flag,
            op3: -1,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item)?,
        },
    ])
}

fn create_sub_weapon(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(Object {
        number: 13,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.count as i32,
        op3: item.flag,
        op4: -1,
        starts: starts_as_is(old_obj, item)?,
    })
}

fn create_sub_weapon_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item)?,
        Object {
            number: 13,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.count as i32,
            op3: item.flag,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item)?,
        },
    ])
}

fn create_seal(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(Object {
        number: 71,
        x: old_obj.x,
        y: old_obj.y,
        op1: item.number as i32,
        op2: item.flag,
        op3: -1,
        op4: -1,
        starts: starts_as_is(old_obj, item)?,
    })
}

fn create_seal_chest(old_obj: &Object, item: &Item) -> Result<Vec<Object>> {
    Ok(vec![
        create_empty_chest(old_obj, item)?,
        Object {
            number: 71,
            x: old_obj.x,
            y: old_obj.y,
            op1: item.number as i32,
            op2: item.flag,
            op3: -1,
            op4: -1,
            starts: starts_that_hide_when_startup_and_taken(old_obj, item)?,
        },
    ])
}

fn create_empty_chest(old_obj: &Object, item: &Item) -> Result<Object> {
    Ok(Object {
        number: 1,
        x: old_obj.x,
        y: old_obj.y,
        op1: old_obj.op1,
        op2: -1,
        op3: old_obj.op1,
        op4: -1,
        starts: starts_as_is(old_obj, item)?,
    })
}

fn starts_that_hide_when_startup_and_taken(
    old_chest_obj: &Object,
    item: &Item,
) -> Result<Vec<Start>> {
    debug_assert_eq!(old_chest_obj.number, 1);
    let mut vec = vec![
        Start {
            number: 99999,
            run_when_unset: true,
        },
        Start {
            number: old_chest_obj.to_chest_item()?.unwrap().open_flag,
            run_when_unset: true,
        },
        Start {
            number: item.flag,
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

fn starts_that_hide_when_startup(old_obj: &Object, start_flag: i32) -> Result<Vec<Start>> {
    let mut vec = vec![
        Start {
            number: 99999,
            run_when_unset: true,
        },
        Start {
            number: start_flag,
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

fn starts_as_is(old_obj: &Object, item: &Item) -> Result<Vec<Start>> {
    let mut vec = get_starts_without_old_flag(old_obj)?;
    let item_flag = old_obj.get_item_flag()?;
    if old_obj.starts.iter().any(|x| x.number == item_flag) {
        vec.push(Start {
            number: item.flag,
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
        .filter(|x| x.number != item_flag)
        .map(|x| Start {
            number: x.number,
            run_when_unset: x.run_when_unset,
        })
        .collect())
}

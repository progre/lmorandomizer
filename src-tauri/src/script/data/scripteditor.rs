use anyhow::{anyhow, Result};

use crate::{
    dataset::{
        item::Item,
        storage::{Shop, Storage},
        supplements::{
            NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT,
            TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT,
        },
    },
    script::{
        format::shop_items_data::{self, ShopItemData},
        items::{Equipment, SubWeapon},
    },
};

use super::{
    object::{LMStart, Object},
    objectfactory::{to_object_for_shutter, to_object_for_special_chest, to_objects_for_chest},
    script::{Field, Map, World},
};

pub fn replace_shops(talks: &mut [String], shops: &[Shop]) -> Result<()> {
    for (i, shop_str) in talks.iter_mut().enumerate() {
        let talk_number = u16::try_from(i)?;
        let Some(new_shop) = shops
            .iter()
            .find(|x| x.spot.talk_number == Some(talk_number))
        else {
            continue;
        };
        let old = shop_items_data::parse(shop_str);
        let mut replaced = [old.0, old.1, old.2]
            .into_iter()
            .enumerate()
            .map(|(j, item)| {
                let new_shop_item = [&new_shop.items.0, &new_shop.items.1, &new_shop.items.2][j];
                ShopItemData {
                    r#type: to_integer_item_type(&new_shop_item.r#type),
                    number: new_shop_item.number,
                    price: item.price,
                    count: new_shop_item.count,
                    flag: new_shop_item.flag,
                }
            });
        *shop_str = shop_items_data::stringify((
            replaced.next().unwrap(),
            replaced.next().unwrap(),
            replaced.next().unwrap(),
        ))?;
    }
    Ok(())
}

fn to_integer_item_type(string_item_type: &str) -> u8 {
    match string_item_type {
        "mainWeapon" => unreachable!(),
        "subWeapon" => 0,
        "equipment" => 1,
        "rom" => 2,
        "seal" => unreachable!(),
        _ => unreachable!(),
    }
}

pub fn replace_items(worlds: Vec<World>, shuffled: &Storage) -> Result<Vec<World>> {
    let mut main_weapon_spot_idx = 0;
    let mut sub_weapon_spot_idx = 0;
    let mut chest_idx = 0;
    let mut seal_chest_idx = 0;

    worlds
        .into_iter()
        .map(|world| {
            Ok(World {
                fields: world
                    .fields
                    .into_iter()
                    .map(|field| {
                        Ok(Field {
                            maps: field
                                .maps
                                .into_iter()
                                .map(|map| {
                                    Ok(Map {
                                        objects: map
                                            .objects
                                            .iter()
                                            .enumerate()
                                            .map(|(i, obj)| match obj.number {
                                                77 => {
                                                    let item = &shuffled.main_weapon_shutters()
                                                        [main_weapon_spot_idx]
                                                        .item;
                                                    main_weapon_spot_idx += 1;
                                                    let next_shutter_check_flag =
                                                        get_next_shutter_check_flag(
                                                            &map.objects[i + 1..],
                                                        )
                                                        .ok_or(anyhow!(
                                                            "next_shutter_check_flag not found"
                                                        ))?;
                                                    Ok(vec![to_object_for_shutter(
                                                        obj,
                                                        next_shutter_check_flag,
                                                        item,
                                                    )?])
                                                }
                                                13 => {
                                                    // TODO: nightSurface
                                                    if sub_weapon_spot_idx
                                                        >= shuffled.sub_weapon_shutters().len()
                                                    {
                                                        let sum =
                                                            shuffled.sub_weapon_shutters().len()
                                                                + NIGHT_SURFACE_SUB_WEAPON_COUNT;
                                                        debug_assert!(sub_weapon_spot_idx < sum);
                                                        sub_weapon_spot_idx += 1;
                                                        return Ok(vec![obj.clone()]);
                                                    }
                                                    let item = &shuffled.sub_weapon_shutters()
                                                        [sub_weapon_spot_idx]
                                                        .item;
                                                    sub_weapon_spot_idx += 1;
                                                    if obj.op1 == SubWeapon::AnkhJewel as i32 {
                                                        if obj.op3 != 743 {
                                                            return Ok(vec![
                                                                to_object_for_special_chest(
                                                                    obj, item,
                                                                )?,
                                                            ]);
                                                        }
                                                        // gateOfGuidance ankhJewel
                                                        let wall_check_flag =
                                                            get_next_wall_check_flag(
                                                                &map.objects[i + 1..],
                                                            )
                                                            .ok_or(anyhow!(
                                                                "wall_check_flag not found"
                                                            ))?;
                                                        return Ok(vec![to_object_for_shutter(
                                                            obj,
                                                            wall_check_flag,
                                                            item,
                                                        )?]);
                                                    }
                                                    let next_shutter_check_flag =
                                                        if obj.op1 == SubWeapon::Pistol as i32 {
                                                            get_next_breakable_wall_check_flag(
                                                                &map.objects[i + 1..],
                                                            )
                                                        } else {
                                                            get_next_shutter_check_flag(
                                                                &map.objects[i + 1..],
                                                            )
                                                        }
                                                        .ok_or(anyhow!(
                                                            "next_shutter_check_flag not found"
                                                        ))?;
                                                    Ok(vec![to_object_for_shutter(
                                                        obj,
                                                        next_shutter_check_flag,
                                                        item,
                                                    )?])
                                                }
                                                1 => {
                                                    if obj.op2 == -1
                                                        || obj.op2
                                                            == Equipment::SweetClothing as i32
                                                    {
                                                        return Ok(vec![obj.clone()]);
                                                    }
                                                    // TODO: nightSurface
                                                    if chest_idx >= shuffled.chests().len() {
                                                        let sum = shuffled.chests().len()
                                                            + NIGHT_SURFACE_CHEST_COUNT;
                                                        debug_assert!(chest_idx < sum);
                                                        chest_idx += 1;
                                                        return Ok(vec![obj.clone()]);
                                                    }
                                                    let item: &Item;
                                                    // twinStatue
                                                    if obj.op1 == 420 {
                                                        item =
                                                            &shuffled.chests()[chest_idx - 1].item;
                                                    } else {
                                                        item = &shuffled.chests()[chest_idx].item;
                                                        chest_idx += 1;
                                                    }
                                                    to_objects_for_chest(obj, item)
                                                }
                                                71 => {
                                                    // TODO: trueShrineOfTheMother
                                                    // TODO: nightSurface
                                                    if seal_chest_idx
                                                        >= shuffled.seal_chests().len()
                                                    {
                                                        let sum = shuffled.seal_chests().len()
                                                            + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT
                                                            + NIGHT_SURFACE_SEAL_COUNT;
                                                        debug_assert!(seal_chest_idx < sum);
                                                        seal_chest_idx += 1;
                                                        return Ok(vec![obj.clone()]);
                                                    }
                                                    let item = &shuffled.seal_chests()
                                                        [seal_chest_idx]
                                                        .item;
                                                    seal_chest_idx += 1;
                                                    Ok(vec![to_object_for_special_chest(
                                                        obj, item,
                                                    )?])
                                                }
                                                140 => {
                                                    // mausoleumOfTheGiants ankhJewel
                                                    if obj.x == 49152 && obj.y == 16384 {
                                                        return Ok(vec![Object {
                                                            number: obj.number,
                                                            x: obj.x,
                                                            y: obj.y,
                                                            op1: shuffled.sub_weapon_shutters()
                                                                [sub_weapon_spot_idx - 1]
                                                                .item
                                                                .flag,
                                                            op2: obj.op2,
                                                            op3: obj.op3,
                                                            op4: obj.op4,
                                                            starts: obj.starts.clone(),
                                                        }]);
                                                    }
                                                    Ok(vec![obj.clone()])
                                                }
                                                186 => {
                                                    if obj.starts.len() == 1
                                                        && obj.starts[0].number == 788
                                                    {
                                                        return Ok(vec![Object {
                                                            number: obj.number,
                                                            x: obj.x,
                                                            y: obj.y,
                                                            op1: obj.op1,
                                                            op2: obj.op2,
                                                            op3: obj.op3,
                                                            op4: obj.op4,
                                                            starts: vec![LMStart {
                                                                number: 891,
                                                                value: obj.starts[0].value,
                                                            }],
                                                        }]);
                                                    }
                                                    Ok(vec![obj.clone()])
                                                }
                                                _ => Ok(vec![obj.clone()]),
                                            })
                                            .collect::<Result<Vec<_>>>()?
                                            .into_iter()
                                            .flatten()
                                            .collect(),
                                        ..map
                                    })
                                })
                                .collect::<Result<_>>()?,
                            ..field
                        })
                    })
                    .collect::<Result<_>>()?,
                ..world
            })
        })
        .collect::<Result<_>>()
}

fn get_next_shutter_check_flag(objs: &[Object]) -> Option<i32> {
    Some(objs.iter().find(|x| x.number == 20)?.op1)
}

fn get_next_wall_check_flag(objs: &[Object]) -> Option<i32> {
    Some(objs.iter().find(|x| x.number == 59)?.op3)
}

fn get_next_breakable_wall_check_flag(objs: &[Object]) -> Option<i32> {
    let data = objs.iter().find(|x| x.number == 70)?.op4;
    Some((data - (data / 10000) * 10000) / 10)
}

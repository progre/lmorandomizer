use crate::{
    dataset::{
        item::Item,
        storage::{Shop, Storage},
        supplements::{
            NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT,
            TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT, WARE_NO_MISE_COUNT,
        },
    },
    script::data::shop_items_data::{self, ShopItem},
};
use anyhow::{anyhow, Result};

use super::{
    items::{Equipment, SubWeapon},
    object::{Object, Start},
    objectfactory::{to_object_for_shutter, to_object_for_special_chest, to_objects_for_chest},
    script::World,
};

pub fn replace_shops(
    talks: &mut [String],
    script_shop: &[super::object::Shop],
    shops: &[Shop],
) -> Result<()> {
    assert_eq!(script_shop.len(), shops.len() + WARE_NO_MISE_COUNT);
    for (i, shop_str) in talks.iter_mut().enumerate() {
        let talk_number = u16::try_from(i)?;
        let Some(idx) = script_shop
            .iter()
            .position(|x| x.talk_number == talk_number)
        else {
            continue;
        };
        let Some(new_shop) = shops.get(idx) else {
            continue;
        };
        let old = shop_items_data::parse(shop_str)?;
        let mut replaced = [old.0, old.1, old.2]
            .into_iter()
            .enumerate()
            .map(|(j, old_item)| {
                let new_item = [&new_shop.items.0, &new_shop.items.1, &new_shop.items.2][j];
                to_shop_item(&old_item, new_item)
            })
            .collect::<Vec<_>>()
            .into_iter();
        *shop_str = shop_items_data::stringify((
            replaced.next().unwrap(),
            replaced.next().unwrap(),
            replaced.next().unwrap(),
        ))?;
    }
    Ok(())
}

fn to_shop_item(old_item: &ShopItem, new_item: &Item) -> ShopItem {
    let price = old_item.price();
    match new_item {
        Item::MainWeapon(_) => unreachable!(),
        Item::SubWeapon(new_item) => {
            let set_flag = new_item.flag;
            ShopItem::sub_weapon(new_item.number, price, new_item.count, set_flag)
        }
        Item::Equipment(new_item) => {
            let set_flag = new_item.flag;
            ShopItem::equipment(new_item.number, price, set_flag)
        }
        Item::Rom(new_item) => {
            let set_flag = new_item.flag;
            ShopItem::rom(new_item.number, price, set_flag)
        }
        Item::Seal(_) => unreachable!(),
    }
}

fn fix_trap_of_mausoleum_of_the_giants(obj: &mut Object, prev_sub_weapon_shutter_item: &Item) {
    obj.op1 = prev_sub_weapon_shutter_item.flag() as i32;
}

fn new_objs(
    obj: &Object,
    next_objs: &[Object],
    shuffled: &Storage,
    main_weapon_spot_idx: &mut usize,
    sub_weapon_spot_idx: &mut usize,
    chest_idx: &mut usize,
    seal_chest_idx: &mut usize,
) -> Result<Vec<Object>> {
    match obj.number {
        // Main weapons
        77 => {
            let item = &shuffled.main_weapon_shutters()[*main_weapon_spot_idx].item;
            *main_weapon_spot_idx += 1;
            let next_shutter_check_flag = get_next_shutter_check_flag(next_objs)?
                .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(
                obj,
                next_shutter_check_flag,
                item,
            )?])
        }
        // Sub weapons
        13 => {
            // TODO: nightSurface
            if *sub_weapon_spot_idx >= shuffled.sub_weapon_shutters().len() {
                let sum = shuffled.sub_weapon_shutters().len() + NIGHT_SURFACE_SUB_WEAPON_COUNT;
                debug_assert!(*sub_weapon_spot_idx < sum);
                *sub_weapon_spot_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // Ankh Jewel
            let item = &shuffled.sub_weapon_shutters()[*sub_weapon_spot_idx].item;
            *sub_weapon_spot_idx += 1;
            if obj.op1 == SubWeapon::AnkhJewel as i32 {
                // Gate of Guidance
                if obj.op3 == 743 {
                    let wall_check_flag = get_next_wall_check_flag(next_objs)
                        .ok_or(anyhow!("wall_check_flag not found"))?;
                    return Ok(vec![to_object_for_shutter(
                        obj,
                        u16::try_from(wall_check_flag)?,
                        item,
                    )?]);
                }
                return Ok(vec![to_object_for_special_chest(obj, item)?]);
            }
            let next_shutter_check_flag = if obj.op1 == SubWeapon::Pistol as i32 {
                get_next_breakable_wall_check_flag(next_objs)?
            } else {
                get_next_shutter_check_flag(next_objs)?
            }
            .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(
                obj,
                next_shutter_check_flag,
                item,
            )?])
        }
        // Chests
        1 => {
            // Skip the empty and Sweet Clothing
            if [-1, Equipment::SweetClothing as i32].contains(&obj.op2) {
                return Ok(vec![obj.clone()]);
            }
            // TODO: nightSurface
            if *chest_idx >= shuffled.chests().len() {
                let sum = shuffled.chests().len() + NIGHT_SURFACE_CHEST_COUNT;
                debug_assert!(*chest_idx < sum);
                *chest_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // twinStatue
            if obj.op1 == 420 {
                let item = &shuffled.chests()[*chest_idx - 1].item;
                return to_objects_for_chest(obj, item);
            }
            let item = &shuffled.chests()[*chest_idx].item;
            *chest_idx += 1;
            to_objects_for_chest(obj, item)
        }
        // Seal chests
        // TODO: trueShrineOfTheMother
        // TODO: nightSurface
        71 => {
            if *seal_chest_idx >= shuffled.seal_chests().len() {
                let sum = shuffled.seal_chests().len()
                    + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT
                    + NIGHT_SURFACE_SEAL_COUNT;
                debug_assert!(*seal_chest_idx < sum);
                *seal_chest_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            let item = &shuffled.seal_chests()[*seal_chest_idx].item;
            *seal_chest_idx += 1;
            Ok(vec![to_object_for_special_chest(obj, item)?])
        }
        // Trap object for the Ankh Jewel Treasure Chest in Mausoleum of the Giants.
        // It is made to work correctly when acquiring items.
        140 if obj.x == 49152 && obj.y == 16384 => {
            let mut obj = obj.clone();
            let prev_sub_weapon_shutter_item =
                &shuffled.sub_weapon_shutters()[*sub_weapon_spot_idx - 1].item;
            fix_trap_of_mausoleum_of_the_giants(&mut obj, prev_sub_weapon_shutter_item);
            Ok(vec![obj])
        }
        // ヴィマーナは飛行機模型を取得したら出現しないようになっている。
        // 飛行機模型の宝箱を開けた段階で出現しないようにする。
        // NOTE: 不要では？
        186 if obj.starts.len() == 1 && obj.starts[0].number == 788 => Ok(vec![Object {
            number: obj.number,
            x: obj.x,
            y: obj.y,
            op1: obj.op1,
            op2: obj.op2,
            op3: obj.op3,
            op4: obj.op4,
            starts: vec![Start {
                number: 891,
                run_when_unset: obj.starts[0].run_when_unset,
            }],
        }]),
        _ => Ok(vec![obj.clone()]),
    }
}

pub fn replace_items(worlds: &mut Vec<World>, shuffled: &Storage) -> Result<()> {
    let mut main_weapon_spot_idx = 0;
    let mut sub_weapon_spot_idx = 0;
    let mut chest_idx = 0;
    let mut seal_chest_idx = 0;

    for world in worlds {
        for field in &mut world.fields {
            for map in &mut field.maps {
                let mut objects = Vec::new();
                for i in 0..map.objects.len() {
                    objects.append(&mut new_objs(
                        &map.objects[i],
                        &map.objects[i + 1..],
                        shuffled,
                        &mut main_weapon_spot_idx,
                        &mut sub_weapon_spot_idx,
                        &mut chest_idx,
                        &mut seal_chest_idx,
                    )?);
                }
                map.objects = objects;
            }
        }
    }
    Ok(())
}

fn get_next_shutter_check_flag(objs: &[Object]) -> Result<Option<u16>> {
    Ok(objs
        .iter()
        .find(|x| x.number == 20)
        .map(|x| u16::try_from(x.op1))
        .transpose()?)
}

fn get_next_wall_check_flag(objs: &[Object]) -> Option<i32> {
    Some(objs.iter().find(|x| x.number == 59)?.op3)
}

fn get_next_breakable_wall_check_flag(objs: &[Object]) -> Result<Option<u16>> {
    objs.iter()
        .find(|x| x.number == 70)
        .map(|x| {
            let data = x.op4;
            Ok(u16::try_from((data - (data / 10000) * 10000) / 10)?)
        })
        .transpose()
}

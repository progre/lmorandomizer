use crate::{
    dataset::{
        storage::{Shop, Storage, StorageIndices},
        NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT,
        TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT, WARE_NO_MISE_COUNT,
    },
    script::data::shop_items_data::{self, ShopItem},
};
use anyhow::{anyhow, Result};

use super::{
    item::Item,
    items::{Equipment, SubWeapon},
    object::{Object, Start},
    objectfactory::{to_object_for_shutter, to_object_for_special_chest, to_objects_for_chest},
    script::{Script, World},
};

pub fn replace_shops(
    talks: &mut [String],
    script: &Script,
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
                let is_consumable = new_item.name.is_consumable();
                let new_item = Item::from_dataset(new_item, script)?;
                let price = if is_consumable {
                    new_item.price().unwrap()
                } else {
                    old_item.price()
                };
                Ok(ShopItem::from_item(new_item, price))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter();
        *shop_str = shop_items_data::stringify((
            replaced.next().unwrap(),
            replaced.next().unwrap(),
            replaced.next().unwrap(),
        ))?;
    }
    Ok(())
}

fn fix_trap_of_mausoleum_of_the_giants(obj: &mut Object, prev_sub_weapon_shutter_item: &Item) {
    obj.op1 = prev_sub_weapon_shutter_item.set_flag() as i32;
}

fn new_objs(
    obj: &Object,
    next_objs: &[Object],
    script: &Script,
    shuffled: &Storage,
    indices: &mut StorageIndices,
) -> Result<Vec<Object>> {
    match obj.number {
        // Main weapons
        77 => {
            let item = &shuffled.main_weapons()[indices.main_weapon_spot_idx].item;
            let item = &Item::from_dataset(item, script)?;
            indices.main_weapon_spot_idx += 1;
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
            if indices.sub_weapon_spot_idx >= shuffled.sub_weapons().len() {
                let sum = shuffled.sub_weapons().len() + NIGHT_SURFACE_SUB_WEAPON_COUNT;
                debug_assert!(indices.sub_weapon_spot_idx < sum);
                indices.sub_weapon_spot_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // Ankh Jewel
            let item = &shuffled.sub_weapons()[indices.sub_weapon_spot_idx].item;
            let item = &Item::from_dataset(item, script)?;
            indices.sub_weapon_spot_idx += 1;
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
            if indices.chest_idx >= shuffled.chests().len() {
                let sum = shuffled.chests().len() + NIGHT_SURFACE_CHEST_COUNT;
                debug_assert!(indices.chest_idx < sum);
                indices.chest_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // twinStatue
            if obj.op1 == 420 {
                let item = &shuffled.chests()[indices.chest_idx - 1].item;
                let item = &Item::from_dataset(item, script)?;
                return to_objects_for_chest(obj, item);
            }
            let item = &shuffled.chests()[indices.chest_idx].item;
            let item = &Item::from_dataset(item, script)?;
            indices.chest_idx += 1;
            to_objects_for_chest(obj, item)
        }
        // Seal chests
        // TODO: trueShrineOfTheMother
        // TODO: nightSurface
        71 => {
            if indices.seal_chest_idx >= shuffled.seals().len() {
                let sum = shuffled.seals().len()
                    + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT
                    + NIGHT_SURFACE_SEAL_COUNT;
                debug_assert!(indices.seal_chest_idx < sum);
                indices.seal_chest_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            let item = &shuffled.seals()[indices.seal_chest_idx].item;
            let item = &Item::from_dataset(item, script)?;
            indices.seal_chest_idx += 1;
            Ok(vec![to_object_for_special_chest(obj, item)?])
        }
        // Trap object for the Ankh Jewel Treasure Chest in Mausoleum of the Giants.
        // It is made to work correctly when acquiring items.
        140 if obj.x == 49152 && obj.y == 16384 => {
            let mut obj = obj.clone();
            let prev_sub_weapon_shutter_item =
                &shuffled.sub_weapons()[indices.sub_weapon_spot_idx - 1].item;
            let prev_sub_weapon_shutter_item =
                &Item::from_dataset(prev_sub_weapon_shutter_item, script)?;
            fix_trap_of_mausoleum_of_the_giants(&mut obj, prev_sub_weapon_shutter_item);
            Ok(vec![obj])
        }
        // ヴィマーナは飛行機模型を取得したら出現しないようになっている。
        // 飛行機模型取得後に飛行機模型の宝箱を開けられるように、飛行機模型出現のフラグに変更する。
        186 if obj.starts.len() == 1 && obj.starts[0].flag == 788 => Ok(vec![Object {
            starts: vec![Start {
                flag: 891,
                run_when_unset: obj.starts[0].run_when_unset,
            }],
            ..obj.clone()
        }]),
        _ => Ok(vec![obj.clone()]),
    }
}

pub fn replace_items(worlds: &mut [World], script: &Script, shuffled: &Storage) -> Result<()> {
    let mut indices = StorageIndices::default();

    for world in worlds {
        for field in &mut world.fields {
            for map in &mut field.maps {
                let mut objects = Vec::new();
                for i in 0..map.objects.len() {
                    objects.append(&mut new_objs(
                        &map.objects[i],
                        &map.objects[i + 1..],
                        script,
                        shuffled,
                        &mut indices,
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

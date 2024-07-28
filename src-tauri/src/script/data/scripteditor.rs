use crate::{
    dataset::{
        storage::{Shop, Storage, StorageIndices},
        NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT, WARE_NO_MISE_COUNT,
    },
    script::data::shop_items_data::{self, ShopItem},
};
use anyhow::{anyhow, bail, Result};

use super::{
    item::{self, ChestItem, Item},
    items::{self, SubWeapon},
    object::{Object, Start, UnknownObject},
    objectfactory::{
        to_object_for_shutter, to_object_for_special_chest, to_objects_for_chest,
        to_objects_for_hand_scanner,
    },
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

fn fix_trap_of_mausoleum_of_the_giants(
    obj: &mut UnknownObject,
    prev_sub_weapon_shutter_item: &Item,
) {
    obj.op1 = prev_sub_weapon_shutter_item.flag() as i32;
}

fn replace_all_flags(starts: &[Start], script: &Script, shuffled: &Storage) -> Result<Vec<Start>> {
    starts
        .iter()
        .map(|start| {
            let Some(old_rom) = script.roms().find(|x| x.rom().flag as u32 == start.flag) else {
                return Ok(start.clone());
            };
            let Some(new_rom) = shuffled
                .roms
                .values()
                .find(|new_rom| new_rom.spot.rom() == old_rom.rom().content)
            else {
                log::warn!("unsupported rom: {}", old_rom.rom().content);
                return Ok(start.clone());
            };
            let item = Item::from_dataset(&new_rom.item, script)?;
            Ok(Start {
                flag: item.flag() as u32,
                run_when: start.run_when,
            })
        })
        .collect()
}

fn new_objs(
    obj: &Object,
    next_objs: &[Object],
    script: &Script,
    shuffled: &Storage,
    indices: &mut StorageIndices,
) -> Result<Vec<Object>> {
    match obj {
        Object::Chest(chest_obj) => {
            // Skip the empty and Sweet Clothing
            if matches!(
                chest_obj.item(),
                ChestItem::None(_)
                    | ChestItem::Equipment(item::Equipment {
                        content: items::Equipment::SweetClothing,
                        ..
                    })
            ) {
                return Ok(vec![obj.clone()]);
            }
            // TODO: nightSurface
            if indices.chest_idx >= shuffled.chests.len() {
                let sum = shuffled.chests.len() + NIGHT_SURFACE_CHEST_COUNT;
                debug_assert!(indices.chest_idx < sum);
                indices.chest_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // twinStatue
            if chest_obj.open_flag() == 420 {
                let item = &shuffled.chests[indices.chest_idx - 1].item;
                let item = Item::from_dataset(item, script)?;
                return Ok(to_objects_for_chest(chest_obj, item));
            }
            let item = &shuffled.chests[indices.chest_idx].item;
            let item = Item::from_dataset(item, script)?;
            indices.chest_idx += 1;
            Ok(to_objects_for_chest(chest_obj, item))
        }
        Object::SubWeapon(sub_weapon_obj) => {
            // TODO: nightSurface
            if indices.sub_weapon_spot_idx >= shuffled.sub_weapons.len() {
                let sum = shuffled.sub_weapons.len() + NIGHT_SURFACE_SUB_WEAPON_COUNT;
                debug_assert!(indices.sub_weapon_spot_idx < sum);
                indices.sub_weapon_spot_idx += 1;
                return Ok(vec![obj.clone()]);
            }
            // Ankh Jewel
            let item = &shuffled.sub_weapons[indices.sub_weapon_spot_idx].item;
            let item = Item::from_dataset(item, script)?;
            indices.sub_weapon_spot_idx += 1;
            if sub_weapon_obj.sub_weapon().content == SubWeapon::AnkhJewel {
                // Gate of Guidance
                if sub_weapon_obj.sub_weapon().flag == 743 {
                    let wall_check_flag = get_next_wall_check_flag(next_objs)
                        .ok_or(anyhow!("wall_check_flag not found"))?;
                    let open_flag = u16::try_from(wall_check_flag)?;
                    return Ok(vec![to_object_for_shutter(obj, open_flag, item)]);
                }
                return Ok(vec![to_object_for_special_chest(obj, item)]);
            }
            let open_flag = if sub_weapon_obj.sub_weapon().content == SubWeapon::Pistol {
                get_next_breakable_wall_check_flag(next_objs)?
            } else {
                get_next_shutter_check_flag(next_objs)?
            }
            .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(obj, open_flag, item)])
        }
        Object::Shop(_) => Ok(vec![obj.clone()]),
        Object::Rom(obj) => {
            let Some(rom) = shuffled.roms.get(&obj.rom().content) else {
                bail!("rom not found: {}", obj.rom().content)
            };
            let item = Item::from_dataset(&rom.item, script)?;
            Ok(to_objects_for_hand_scanner(obj, item))
        }
        Object::Seal(seal_obj) => {
            let Some(seal) = shuffled.seals.get(&seal_obj.seal().content) else {
                bail!("seal not found: {}", seal_obj.seal().content)
            };
            let item = Item::from_dataset(&seal.item, script)?;
            Ok(vec![to_object_for_special_chest(obj, item)])
        }
        Object::MainWeapon(main_weapon_obj) => {
            let Some(main_weapon) = &shuffled
                .main_weapons
                .get(&main_weapon_obj.main_weapon().content)
            else {
                let content = main_weapon_obj.main_weapon().content;
                bail!("main_weapon not found: {}", content)
            };
            let item = Item::from_dataset(&main_weapon.item, script)?;
            let open_flag = get_next_shutter_check_flag(next_objs)?
                .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(obj, open_flag, item)])
        }
        Object::Unknown(unknown_obj) => {
            match unknown_obj.number {
                // Chests | Sub weapons | Shops | Roms | Seals | Main weapons
                1 | 13 | 14 | 32 | 71 | 77 => unreachable!(),
                // Trap object for the Ankh Jewel Treasure Chest in Mausoleum of the Giants.
                // It is made to work correctly when acquiring items.
                140 if unknown_obj.x == 49152 && unknown_obj.y == 16384 => {
                    let mut obj = unknown_obj.clone();
                    let prev_sub_weapon_shutter_item =
                        &shuffled.sub_weapons[indices.sub_weapon_spot_idx - 1].item;
                    let prev_sub_weapon_shutter_item =
                        &Item::from_dataset(prev_sub_weapon_shutter_item, script)?;
                    fix_trap_of_mausoleum_of_the_giants(&mut obj, prev_sub_weapon_shutter_item);
                    Ok(vec![Object::Unknown(obj)])
                }
                // ヴィマーナは飛行機模型を取得したら出現しないようになっている。
                // 飛行機模型取得後に飛行機模型の宝箱を開けられるように、飛行機模型出現のフラグに変更する。
                186 if unknown_obj.starts.len() == 1 && unknown_obj.starts[0].flag == 788 => {
                    Ok(vec![Object::Unknown(UnknownObject {
                        starts: vec![Start {
                            flag: 891,
                            run_when: unknown_obj.starts[0].run_when,
                        }],
                        ..unknown_obj.clone()
                    })])
                }
                _ => Ok(vec![Object::Unknown(UnknownObject {
                    number: obj.number(),
                    x: obj.x(),
                    y: obj.y(),
                    op1: obj.op1(),
                    op2: obj.op2(),
                    op3: obj.op3(),
                    op4: obj.op4(),
                    starts: replace_all_flags(obj.starts(), script, shuffled)?,
                })]),
            }
        }
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
        .find(|x| x.number() == 20)
        .map(|x| u16::try_from(x.op1()))
        .transpose()?)
}

fn get_next_wall_check_flag(objs: &[Object]) -> Option<i32> {
    Some(objs.iter().find(|x| x.number() == 59)?.op3())
}

fn get_next_breakable_wall_check_flag(objs: &[Object]) -> Result<Option<u16>> {
    objs.iter()
        .find(|x| x.number() == 70)
        .map(|x| {
            let data = x.op4();
            Ok(u16::try_from((data - (data / 10000) * 10000) / 10)?)
        })
        .transpose()
}

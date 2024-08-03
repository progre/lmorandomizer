use anyhow::{anyhow, bail, Result};
use log::debug;

use crate::{
    randomizer::storage::Storage,
    script::{
        data::{
            item::{ChestItem, Equipment, Item, Rom},
            object::{Object, Start, UnknownObject},
            script::{Script, World},
        },
        enums::{self, FieldNumber},
    },
};

use super::objects_factory::{
    to_object_for_shutter, to_object_for_special_chest, to_objects_for_chest,
    to_objects_for_hand_scanner,
};

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
                debug!("rom not found: {}", old_rom.rom().content);
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
    mut field_number: FieldNumber,
    next_objs: &[Object],
    script: &Script,
    shuffled: &Storage,
) -> Result<Vec<Object>> {
    if field_number == FieldNumber::SurfaceNight {
        field_number = FieldNumber::Surface;
    }
    match obj {
        Object::Chest(chest_obj) => {
            let chest_item = match chest_obj.item() {
                // Skip the empty or Sweet Clothing
                ChestItem::None(_)
                | ChestItem::Equipment(Equipment {
                    content: enums::Equipment::SweetClothing,
                    ..
                }) => {
                    return Ok(vec![obj.clone()]);
                }
                ChestItem::Equipment(Equipment { content, .. }) => {
                    enums::ChestItem::Equipment(*content)
                }
                ChestItem::Rom(Rom { content, .. }) => enums::ChestItem::Rom(*content),
            };
            let Some(chest) = shuffled.chests.get(&(field_number, chest_item)) else {
                bail!("chest not found: {} {:?}", field_number, chest_obj.item())
            };
            let item = Item::from_dataset(&chest.item, script)?;
            Ok(to_objects_for_chest(chest_obj, item))
        }
        Object::SubWeapon(sub_weapon_obj) => {
            let key = (field_number, sub_weapon_obj.sub_weapon().content);
            let Some(sub_weapon) = shuffled.sub_weapons.get(&key) else {
                let content = sub_weapon_obj.sub_weapon().content;
                bail!("sub_weapon not found: {}", content)
            };
            let item = Item::from_dataset(&sub_weapon.item, script)?;

            if sub_weapon_obj.sub_weapon().content == enums::SubWeapon::AnkhJewel {
                // Gate of Guidance
                if sub_weapon_obj.sub_weapon().flag == 743 {
                    let wall_check_flag = get_next_wall_check_flag(next_objs)
                        .ok_or(anyhow!("wall_check_flag not found"))?;
                    let open_flag = u16::try_from(wall_check_flag)?;
                    return Ok(vec![to_object_for_shutter(obj, open_flag, item)]);
                }
                return Ok(vec![to_object_for_special_chest(obj, item)]);
            }
            let open_flag = if sub_weapon_obj.sub_weapon().content == enums::SubWeapon::Pistol {
                get_next_breakable_wall_check_flag(next_objs)?
            } else {
                get_next_shutter_check_flag(next_objs)?
            }
            .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(obj, open_flag, item)])
        }
        Object::Shop(_) => Ok(vec![obj.clone()]),
        Object::Rom(rom_obj) => {
            let Some(rom) = shuffled.roms.get(&rom_obj.rom().content) else {
                debug!("rom not found: {}", rom_obj.rom().content);
                return Ok(vec![obj.clone()]);
            };
            let item = Item::from_dataset(&rom.item, script)?;
            Ok(to_objects_for_hand_scanner(rom_obj, item))
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
                    let key = (field_number, enums::SubWeapon::AnkhJewel);
                    let Some(sub_weapon) = shuffled.sub_weapons.get(&key) else {
                        bail!("sub_weapon not found")
                    };
                    let mut obj = unknown_obj.clone();
                    let prev_sub_weapon_shutter_item =
                        &Item::from_dataset(&sub_weapon.item, script)?;
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
    for world in worlds {
        for field in &mut world.fields {
            let field_number = field.number();
            for map in &mut field.maps {
                let mut objects = Vec::new();
                for i in 0..map.objects.len() {
                    objects.append(&mut new_objs(
                        &map.objects[i],
                        field_number,
                        &map.objects[i + 1..],
                        script,
                        shuffled,
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

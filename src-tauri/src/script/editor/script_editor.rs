use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use log::debug;

use crate::{
    randomizer::storage::Storage,
    script::{
        data::{
            item::{ChestItem, Equipment, Item, Rom},
            object::{Object, Shop, Start, UnknownObject},
            script::{Script, World},
        },
        enums,
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

pub fn find_item_set_flag(script: &Script, talk_item: enums::TalkItem) -> Result<Option<u16>> {
    Ok(script
        .shops()
        .map(|shop_obj| Shop::try_from_shop_object(shop_obj, &script.talks))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|x| match x {
            Shop::Storyteller(x) => Some(vec![x.talk_number()]),
            Shop::ItemShop(_) => None,
            Shop::Eldest(x) => Some(x.into_important_talk_numbers()),
        })
        .flatten()
        .filter_map(|talk_number| script.talks[talk_number as usize].item().transpose())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .find(|(item, _set_flag)| item == &talk_item)
        .map(|(_item, set_flag)| set_flag))
}

/// ROMs do not alter the environment. Therefore, the flags of ROMs can be replaced in a batch.
fn replace_flag_map(shuffled: &Storage, script: &Script) -> Result<HashMap<u16, u16>> {
    shuffled
        .roms
        .values()
        .map(|rom| {
            let old_rom_obj = script
                .roms()
                .find(|x| x.rom().content == rom.spot.rom())
                .ok_or_else(|| anyhow!("rom not found: {}", rom.spot.rom()))?;
            let new_item_flag = Item::new(&rom.item.src, script).unwrap().flag();
            Ok((old_rom_obj.rom().flag, new_item_flag))
        })
        .chain(
            shuffled
                .talks
                .iter()
                .filter(|talk| match talk.spot.item() {
                    enums::TalkItem::Equipment(_) => false,
                    enums::TalkItem::Rom(_) => true,
                })
                .map(|talk| {
                    let old_flag = find_item_set_flag(script, talk.spot.item())?
                        .ok_or_else(|| anyhow!("talk not found: {:?}", talk.spot.item()))?;
                    let new_flag = Item::new(&talk.item.src, script).unwrap().flag();
                    Ok((old_flag, new_flag))
                }),
        )
        .collect()
}

fn replace_all_flags(starts: &[Start], replace_flag_map: &HashMap<u16, u16>) -> Vec<Start> {
    starts
        .iter()
        .map(|start| {
            let Ok(flag) = u16::try_from(start.flag) else {
                return start.clone();
            };
            let Some(&new_flag) = replace_flag_map.get(&flag) else {
                return start.clone();
            };
            let flag = new_flag as u32;
            let run_when = start.run_when;
            Start { flag, run_when }
        })
        .collect()
}

fn new_objs(
    obj: &Object,
    mut field_number: enums::FieldNumber,
    next_objs: &[Object],
    script: &Script,
    shuffled: &Storage,
    replace_flag_map: &HashMap<u16, u16>,
) -> Result<Vec<Object>> {
    if field_number == enums::FieldNumber::SurfaceNight {
        field_number = enums::FieldNumber::Surface;
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
            let item = Item::new(&chest.item.src, script)?;
            Ok(to_objects_for_chest(chest_obj, item))
        }
        Object::SubWeapon(sub_weapon_obj) => {
            let key = (field_number, sub_weapon_obj.sub_weapon().content);
            let Some(sub_weapon) = shuffled.sub_weapons.get(&key) else {
                let content = sub_weapon_obj.sub_weapon().content;
                bail!("sub_weapon not found: {}", content)
            };
            let item = Item::new(&sub_weapon.item.src, script)?;

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
        Object::Shop(_) => {
            let starts = if field_number == enums::FieldNumber::GateOfIllusion {
                // NOTE: Do not rewrite flags elsewhere, as the item must be effective.
                let mut replace_flag_map = replace_flag_map.clone();

                let pepper = enums::TalkItem::Equipment(enums::Equipment::Pepper);
                let old_flag = find_item_set_flag(script, pepper)?
                    .ok_or_else(|| anyhow!("talk not found: {:?}", pepper))?;
                let new_flag = Item::talk(script, pepper)?.flag();
                replace_flag_map.insert(old_flag, new_flag);
                let anchor = enums::TalkItem::Equipment(enums::Equipment::Anchor);
                let old_flag = find_item_set_flag(script, anchor)?
                    .ok_or_else(|| anyhow!("talk not found: {:?}", anchor))?;
                let new_flag = Item::talk(script, anchor)?.flag();
                replace_flag_map.insert(old_flag, new_flag);

                let mini_doll = enums::TalkItem::Equipment(enums::Equipment::MiniDoll);
                let old_flag = find_item_set_flag(script, mini_doll)?
                    .ok_or_else(|| anyhow!("talk not found: {:?}", mini_doll))?;
                let new_flag = Item::talk(script, mini_doll)?.flag();
                replace_flag_map.insert(old_flag, new_flag);

                replace_all_flags(obj.starts(), &replace_flag_map)
            } else {
                replace_all_flags(obj.starts(), replace_flag_map)
            };
            Ok(vec![Object::Unknown(UnknownObject {
                number: obj.number(),
                x: obj.x(),
                y: obj.y(),
                op1: obj.op1(),
                op2: obj.op2(),
                op3: obj.op3(),
                op4: obj.op4(),
                starts,
            })])
        }
        Object::Rom(rom_obj) => {
            let Some(rom) = shuffled.roms.get(&rom_obj.rom().content) else {
                debug!("rom not found: {}", rom_obj.rom().content);
                return Ok(vec![obj.clone()]);
            };
            let item = Item::new(&rom.item.src, script)?;
            Ok(to_objects_for_hand_scanner(rom_obj, item))
        }
        Object::Seal(seal_obj) => {
            let Some(seal) = shuffled.seals.get(&seal_obj.seal().content) else {
                bail!("seal not found: {}", seal_obj.seal().content)
            };
            let item = Item::new(&seal.item.src, script)?;
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
            let item = Item::new(&main_weapon.item.src, script)?;
            let open_flag = get_next_shutter_check_flag(next_objs)?
                .ok_or(anyhow!("next_shutter_check_flag not found"))?;
            Ok(vec![to_object_for_shutter(obj, open_flag, item)])
        }
        Object::Unknown(unknown_obj) => {
            match unknown_obj.number {
                // Chests | Sub weapons | Shops | Roms | Seals | Main weapons
                1 | 13 | 14 | 32 | 71 | 77 => unreachable!(),
                // 59: Map rewrite
                // apply ROMs replacement
                59 => Ok(vec![Object::Unknown(UnknownObject {
                    number: obj.number(),
                    x: obj.x(),
                    y: obj.y(),
                    op1: obj.op1(),
                    op2: obj.op2(),
                    op3: replace_flag_map
                        .get(&u16::try_from(obj.op3())?)
                        .copied()
                        .unwrap_or(obj.op3() as u16) as i32,
                    op4: obj.op4(),
                    starts: obj.starts().to_vec(),
                })]),
                // Trap object for the Ankh Jewel Treasure Chest in Mausoleum of the Giants.
                // It is made to work correctly when acquiring items.
                140 if unknown_obj.x == 49152 && unknown_obj.y == 16384 => {
                    let key = (field_number, enums::SubWeapon::AnkhJewel);
                    let Some(sub_weapon) = shuffled.sub_weapons.get(&key) else {
                        bail!("sub_weapon not found")
                    };
                    let mut obj = unknown_obj.clone();
                    let prev_sub_weapon_shutter_item = &Item::new(&sub_weapon.item.src, script)?;
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
                    starts: replace_all_flags(obj.starts(), replace_flag_map),
                })]),
            }
        }
    }
}

pub fn replace_items(worlds: &mut [World], script: &Script, shuffled: &Storage) -> Result<()> {
    let replace_flag_map = replace_flag_map(shuffled, script)?;
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
                        &replace_flag_map,
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

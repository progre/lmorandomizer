use crate::{
    dataset::{
        self,
        spot::FieldId,
        storage::{Shop, Storage},
        WARE_NO_MISE_COUNT,
    },
    script::data::shop_items_data::{self, ShopItem},
};
use anyhow::{anyhow, bail, Result};

use super::{
    item::{ChestItem, Equipment, Item, Rom},
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

fn to_field_id(field_number: u8) -> Option<FieldId> {
    match field_number {
        0 => Some(FieldId::GateOfGuidance),
        1 => Some(FieldId::Surface),
        2 => Some(FieldId::MausoleumOfTheGiants),
        3 => Some(FieldId::TempleOfTheSun),
        4 => Some(FieldId::SpringInTheSky),
        5 => Some(FieldId::InfernoCavern),
        6 => Some(FieldId::ChamberOfExtinction),
        7 => Some(FieldId::EndlessCorridor),
        8 => Some(FieldId::ShrineOfTheMother),
        9 => Some(FieldId::TwinLabyrinthsLeft),
        10 => Some(FieldId::TwinLabyrinthsRight),
        11 => Some(FieldId::GateOfIllusion),
        12 => Some(FieldId::GraveyardOfTheGiants),
        13 => Some(FieldId::TowerOfTheGoddess),
        14 => Some(FieldId::TempleOfMoonlight),
        15 => Some(FieldId::TowerOfRuin),
        16 => Some(FieldId::ChamberOfBirth),
        17 => Some(FieldId::DimensionalCorridor),
        19 => Some(FieldId::TrueShrineOfTheMother),
        28 => Some(FieldId::Surface),
        _ => None,
    }
}

fn new_objs(
    obj: &Object,
    field_number: u8,
    next_objs: &[Object],
    script: &Script,
    shuffled: &Storage,
) -> Result<Vec<Object>> {
    match obj {
        Object::Chest(chest_obj) => {
            let chest_item = match chest_obj.item() {
                // Skip the empty or Sweet Clothing
                ChestItem::None(_)
                | ChestItem::Equipment(Equipment {
                    content: items::Equipment::SweetClothing,
                    ..
                }) => {
                    return Ok(vec![obj.clone()]);
                }
                ChestItem::Equipment(Equipment { content, .. }) => {
                    dataset::item::ChestItem::Equipment(*content)
                }
                ChestItem::Rom(Rom { content, .. }) => dataset::item::ChestItem::Rom(*content),
            };
            let field_id = to_field_id(field_number)
                .ok_or_else(|| anyhow!("field_id not found: {}", field_number))?;
            let Some(chest) = shuffled.chests.get(&(field_id, chest_item)) else {
                bail!("chest not found: {:?}", chest_obj.item())
            };
            let item = Item::from_dataset(&chest.item, script)?;
            Ok(to_objects_for_chest(chest_obj, item))
        }
        Object::SubWeapon(sub_weapon_obj) => {
            let field_id = to_field_id(field_number)
                .ok_or_else(|| anyhow!("field_id not found: {}", field_number))?;
            let key = (field_id, sub_weapon_obj.sub_weapon().content);
            let Some(sub_weapon) = shuffled.sub_weapons.get(&key) else {
                let content = sub_weapon_obj.sub_weapon().content;
                bail!("sub_weapon not found: {}", content)
            };
            let item = Item::from_dataset(&sub_weapon.item, script)?;

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
                    let key = (to_field_id(field_number).unwrap(), SubWeapon::AnkhJewel);
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

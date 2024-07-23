use crate::script::data::object::starts::starts_that_hide_when_startup_and_taken;

use super::{
    item::{Equipment, Item, MainWeapon, Rom, Seal, SubWeapon},
    object::{
        starts::{starts_as_is, starts_that_hide_when_startup, starts_without_old_flag},
        ChestObject, MainWeaponObject, Object, SealObject, SubWeaponObject,
    },
};

fn hidden_equipment_chest(old_obj: &Object, item: &Equipment, open_flag: u16) -> ChestObject {
    let op2 = item.content as i32;
    let set_flag = item.set_flag as i32;
    let starts = starts_that_hide_when_startup(old_obj, open_flag).unwrap();
    ChestObject::new(old_obj.x(), old_obj.y(), 40, op2, set_flag, -1, starts).unwrap()
}

fn hidden_rom_chest(old_obj: &Object, item: &Rom, open_flag: u16) -> ChestObject {
    let op2 = 100 + item.content.0 as i32;
    let set_flag = item.set_flag as i32;
    let starts = starts_that_hide_when_startup(old_obj, open_flag).unwrap();
    ChestObject::new(old_obj.x(), old_obj.y(), 40, op2, set_flag, -1, starts).unwrap()
}

fn equipment_chest_from_ankh_jewel_or_seal(old_obj: &Object, item: &Equipment) -> ChestObject {
    let op2 = item.content as i32;
    let set_flag = item.set_flag as i32;
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.get_item_flag().unwrap());
    ChestObject::new(old_obj.x(), old_obj.y(), 40, op2, set_flag, -1, starts).unwrap()
}

fn rom_chset_from_ankh_jewel_or_seal(old_obj: &Object, item: &Rom) -> ChestObject {
    let op2 = 100 + item.content.0 as i32;
    let set_flag = item.set_flag as i32;
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.get_item_flag().unwrap());
    ChestObject::new(old_obj.x(), old_obj.y(), 40, op2, set_flag, -1, starts).unwrap()
}

fn equipment_chest(old_obj: &ChestObject, item: &Equipment) -> ChestObject {
    let op1 = old_obj.open_flag();
    let op2 = item.content as i32;
    let op3 = item.set_flag as i32;
    let old_set_flag = u16::try_from(old_obj.set_flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.set_flag);
    ChestObject::new(old_obj.x(), old_obj.y(), op1, op2, op3, -1, starts).unwrap()
}

fn rom_chest(old_obj: &ChestObject, item: &Rom) -> ChestObject {
    let op1 = old_obj.open_flag();
    let op2 = 100 + item.content.0 as i32;
    let op3 = item.set_flag as i32;
    let old_set_flag = u16::try_from(old_obj.set_flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.set_flag);
    ChestObject::new(old_obj.x(), old_obj.y(), op1, op2, op3, -1, starts).unwrap()
}

fn empty_chest(old_obj: &ChestObject, set_flag: u16) -> ChestObject {
    let x = old_obj.x();
    let y = old_obj.y();
    let open_flag = old_obj.open_flag();
    let old_set_flag = u16::try_from(old_obj.set_flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, set_flag);
    ChestObject::new(x, y, open_flag, -1, open_flag, -1, starts).unwrap()
}

fn sub_weapon(old_obj: &Object, item: &SubWeapon) -> SubWeaponObject {
    let op1 = item.content as i32;
    let op2 = item.amount as i32;
    let op3 = item.set_flag as i32;
    let old_set_flag = old_obj.get_item_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.set_flag);
    SubWeaponObject::new(old_obj.x(), old_obj.y(), op1, op2, op3, -1, starts).unwrap()
}

fn hidden_sub_weapon(old_obj: &ChestObject, item: &SubWeapon) -> SubWeaponObject {
    let op1 = item.content as i32;
    let op2 = item.amount as i32;
    let op3 = item.set_flag as i32;
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag).unwrap();
    SubWeaponObject::new(old_obj.x(), old_obj.y(), op1, op2, op3, -1, starts).unwrap()
}

fn seal(old_obj: &Object, item: &Seal) -> SealObject {
    let op1 = item.content as i32;
    let op2 = item.set_flag as i32;
    let old_set_flag = old_obj.get_item_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.set_flag);
    SealObject::new(old_obj.x(), old_obj.y(), op1, op2, -1, -1, starts).unwrap()
}

fn hidden_seal(old_obj: &ChestObject, item: &Seal) -> SealObject {
    let op1 = item.content as i32;
    let op2 = item.set_flag as i32;
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag).unwrap();
    SealObject::new(old_obj.x(), old_obj.y(), op1, op2, -1, -1, starts).unwrap()
}

fn main_weapon(old_obj: &Object, item: &MainWeapon) -> MainWeaponObject {
    let op1 = item.content as i32;
    let op2 = item.set_flag as i32;
    let old_set_flag = old_obj.get_item_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.set_flag);
    MainWeaponObject::new(old_obj.x(), old_obj.y(), op1, op2, -1, -1, starts).unwrap()
}

fn hidden_main_weapon(old_obj: &ChestObject, item: &MainWeapon) -> MainWeaponObject {
    let op1 = item.content as i32;
    let op2 = item.set_flag as i32;
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.set_flag).unwrap();
    MainWeaponObject::new(old_obj.x(), old_obj.y(), op1, op2, -1, -1, starts).unwrap()
}

pub fn to_object_for_shutter(old_obj: &Object, open_flag: u16, item: &Item) -> Object {
    match item {
        Item::MainWeapon(item) => Object::MainWeapon(main_weapon(old_obj, item)),
        Item::SubWeapon(item) => Object::SubWeapon(sub_weapon(old_obj, item)),
        Item::Equipment(item) => Object::Chest(hidden_equipment_chest(old_obj, item, open_flag)),
        Item::Rom(item) => Object::Chest(hidden_rom_chest(old_obj, item, open_flag)),
        Item::Seal(item) => Object::Seal(seal(old_obj, item)),
    }
}

pub fn to_object_for_special_chest(old_obj: &Object, item: &Item) -> Object {
    match item {
        Item::MainWeapon(item) => Object::MainWeapon(main_weapon(old_obj, item)),
        Item::SubWeapon(item) => Object::SubWeapon(sub_weapon(old_obj, item)),
        Item::Equipment(item) => {
            Object::Chest(equipment_chest_from_ankh_jewel_or_seal(old_obj, item))
        }
        Item::Rom(item) => Object::Chest(rom_chset_from_ankh_jewel_or_seal(old_obj, item)),
        Item::Seal(item) => Object::Seal(seal(old_obj, item)),
    }
}

pub fn to_objects_for_chest(old_obj: &ChestObject, item: &Item) -> Vec<Object> {
    match item {
        Item::MainWeapon(item) => vec![
            Object::Chest(empty_chest(old_obj, item.set_flag)),
            Object::MainWeapon(hidden_main_weapon(old_obj, item)),
        ],
        Item::SubWeapon(item) => vec![
            Object::Chest(empty_chest(old_obj, item.set_flag)),
            Object::SubWeapon(hidden_sub_weapon(old_obj, item)),
        ],
        Item::Equipment(item) => vec![
            Object::Chest(equipment_chest(old_obj, item)), //
        ],
        Item::Rom(item) => vec![
            Object::Chest(rom_chest(old_obj, item)), //
        ],
        Item::Seal(item) => vec![
            Object::Chest(empty_chest(old_obj, item.set_flag)),
            Object::Seal(hidden_seal(old_obj, item)),
        ],
    }
}

use log::warn;

use crate::script::data::object::starts::starts_that_hide_when_startup_and_taken;

use super::{
    consts::{BLANK_TALK_NUMBER, MEMO_FLAG_BASE_NO},
    item::{ChestItem, Equipment, Item, MainWeapon, Rom, Seal, SubWeapon},
    object::{
        starts::{
            starts_as_is, starts_that_hide_when_startup, starts_with_open_and_remove_flags,
            starts_with_replaced_flag, starts_without_old_flag,
        },
        ChestObject, MainWeaponObject, Object, RomObject, SealObject, Start, SubWeaponObject,
        UnknownObject,
    },
};

const ALWAYS_ON_FLAG: u16 = 40;

fn hidden_equipment_chest(old_obj: &Object, item: Equipment, open_flag: u16) -> ChestObject {
    let item = ChestItem::Equipment(item);
    let starts = starts_that_hide_when_startup(old_obj, open_flag).unwrap();
    ChestObject::new(old_obj.x(), old_obj.y(), ALWAYS_ON_FLAG, item, -1, starts)
}

fn hidden_rom_chest(old_obj: &Object, item: Rom, open_flag: u16) -> ChestObject {
    let item = ChestItem::Rom(item);
    let starts = starts_that_hide_when_startup(old_obj, open_flag).unwrap();
    ChestObject::new(old_obj.x(), old_obj.y(), ALWAYS_ON_FLAG, item, -1, starts)
}

fn invisible_chest(x: i32, y: i32, open_flag: u16, item: ChestItem) -> ChestObject {
    let starts = starts_with_open_and_remove_flags(open_flag, u16::try_from(item.flag()).unwrap());
    ChestObject::new(x, y, open_flag, item, -1, starts.to_vec())
}

fn equipment_chest_from_ankh_jewel_or_seal(old_obj: &Object, item: Equipment) -> ChestObject {
    let item = ChestItem::Equipment(item);
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.set_flag().unwrap());
    ChestObject::new(old_obj.x(), old_obj.y(), ALWAYS_ON_FLAG, item, -1, starts)
}

fn rom_chset_from_ankh_jewel_or_seal(old_obj: &Object, item: Rom) -> ChestObject {
    let item = ChestItem::Rom(item);
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.set_flag().unwrap());
    ChestObject::new(old_obj.x(), old_obj.y(), ALWAYS_ON_FLAG, item, -1, starts)
}

fn equipment_chest(old_obj: &ChestObject, item: Equipment) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, item.flag);
    let item = ChestItem::Equipment(item);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

fn rom_chest(old_obj: &ChestObject, item: Rom) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, item.flag);
    let item = ChestItem::Rom(item);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

fn empty_chest(old_obj: &ChestObject, set_flag: u16) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let item = ChestItem::None(open_flag as i32);
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, set_flag);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

fn simple_sub_weapon(x: i32, y: i32, open_flag: u16, item: SubWeapon) -> SubWeaponObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    SubWeaponObject::new(x, y, item, starts.to_vec())
}

fn sub_weapon(old_obj: &Object, item: SubWeapon) -> SubWeaponObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    SubWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

fn hidden_sub_weapon(old_obj: &ChestObject, item: SubWeapon) -> SubWeaponObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    SubWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

fn rom(old_obj: &RomObject, new_rom: Rom) -> RomObject {
    let starts = starts_with_replaced_flag(old_obj.starts(), old_obj.rom().flag, new_rom.flag);
    RomObject::new(old_obj.x(), old_obj.y(), new_rom, starts)
}

fn simple_seal(x: i32, y: i32, open_flag: u16, item: Seal) -> SealObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    SealObject::new(x, y, item, starts.to_vec())
}

fn seal(old_obj: &Object, item: Seal) -> SealObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    SealObject::new(old_obj.x(), old_obj.y(), item, starts)
}

fn hidden_seal(old_obj: &ChestObject, item: Seal) -> SealObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    SealObject::new(old_obj.x(), old_obj.y(), item, starts)
}

fn simple_main_weapon(x: i32, y: i32, open_flag: u16, item: MainWeapon) -> MainWeaponObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    MainWeaponObject::new(x, y, item, starts.to_vec())
}

fn main_weapon(old_obj: &Object, item: MainWeapon) -> MainWeaponObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    MainWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

fn hidden_main_weapon(old_obj: &ChestObject, item: MainWeapon) -> MainWeaponObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    MainWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn to_object_for_shutter(old_obj: &Object, open_flag: u16, item: Item) -> Object {
    match item {
        Item::MainWeapon(item) => Object::MainWeapon(main_weapon(old_obj, item)),
        Item::SubWeapon(item) => Object::SubWeapon(sub_weapon(old_obj, item)),
        Item::Equipment(item) => Object::Chest(hidden_equipment_chest(old_obj, item, open_flag)),
        Item::Rom(item) => Object::Chest(hidden_rom_chest(old_obj, item, open_flag)),
        Item::Seal(item) => Object::Seal(seal(old_obj, item)),
    }
}

pub fn to_object_for_special_chest(old_obj: &Object, item: Item) -> Object {
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

pub fn to_objects_for_chest(old_obj: &ChestObject, item: Item) -> Vec<Object> {
    match item {
        Item::MainWeapon(item) => vec![
            Object::Chest(empty_chest(old_obj, item.flag)),
            Object::MainWeapon(hidden_main_weapon(old_obj, item)),
        ],
        Item::SubWeapon(item) => vec![
            Object::Chest(empty_chest(old_obj, item.flag)),
            Object::SubWeapon(hidden_sub_weapon(old_obj, item)),
        ],
        Item::Equipment(item) => vec![
            Object::Chest(equipment_chest(old_obj, item)), //
        ],
        Item::Rom(item) => vec![
            Object::Chest(rom_chest(old_obj, item)), //
        ],
        Item::Seal(item) => vec![
            Object::Chest(empty_chest(old_obj, item.flag)),
            Object::Seal(hidden_seal(old_obj, item)),
        ],
    }
}

fn memo(old_obj: &RomObject, set_flag: u16) -> UnknownObject {
    let shadow_event = Start {
        flag: 99999,
        run_when: true,
    };
    let starts: Vec<_> = [shadow_event]
        .into_iter()
        .chain(
            old_obj
                .starts()
                .iter()
                .filter(|start| start.flag != 99999)
                .map(|x| {
                    if x.flag == old_obj.rom().flag as u32 {
                        Start {
                            flag: set_flag as u32,
                            run_when: x.run_when,
                        }
                    } else {
                        x.clone()
                    }
                }),
        )
        .collect();
    if starts.len() > 4 {
        warn!(
            "too many starts: spot={:?}, len={}",
            old_obj.rom().content,
            starts.len()
        );
    }
    UnknownObject {
        number: 37,
        x: old_obj.x(),
        y: old_obj.y(),
        op1: BLANK_TALK_NUMBER,
        op2: set_flag as i32,
        op3: -1,
        op4: -1,
        starts,
    }
}

pub fn to_objects_for_hand_scanner(old_obj: &RomObject, item: Item) -> Vec<Object> {
    if let Item::Rom(item) = item {
        return vec![Object::Rom(rom(old_obj, item))];
    }

    let open_flag = MEMO_FLAG_BASE_NO + old_obj.rom().content as u16;
    let memo = memo(old_obj, open_flag);
    let x = old_obj.x();
    // Raise it by half a square,
    // because the location of the ROMs is half a square lower than the normal items.
    let y = old_obj.y() - 2048;
    match item {
        Item::Equipment(item) => vec![
            Object::Unknown(memo),
            Object::Chest(invisible_chest(x, y, open_flag, ChestItem::Equipment(item))),
        ],
        Item::SubWeapon(item) => vec![
            Object::Unknown(memo),
            Object::SubWeapon(simple_sub_weapon(x, y, open_flag, item)),
        ],
        Item::Rom(_) => unreachable!(),
        Item::Seal(item) => vec![
            Object::Unknown(memo),
            Object::Seal(simple_seal(x, y, open_flag, item)),
        ],
        Item::MainWeapon(item) => vec![
            Object::Unknown(memo),
            Object::MainWeapon(simple_main_weapon(x, y, open_flag, item)),
        ],
    }
}

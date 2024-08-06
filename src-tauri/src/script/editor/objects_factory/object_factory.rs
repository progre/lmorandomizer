use log::warn;

use crate::script::{
    consts::{ALWAYS_ON_FLAG_NO, BLANK_TALK_NUMBER},
    data::{
        item::{ChestItem, Equipment, MainWeapon, Rom, Seal, SubWeapon},
        object::{
            starts::{
                starts_as_is, starts_that_hide_when_startup_and_taken,
                starts_with_open_and_remove_flags, starts_with_replaced_flag,
                starts_without_old_flag,
            },
            ChestObject, MainWeaponObject, Object, RomObject, SealObject, Start, SubWeaponObject,
            UnknownObject,
        },
    },
};

pub fn invisible_chest(x: i32, y: i32, open_flag: u16, item: ChestItem) -> ChestObject {
    let starts = starts_with_open_and_remove_flags(open_flag, u16::try_from(item.flag()).unwrap());
    ChestObject::new(x, y, open_flag, item, -1, starts.to_vec())
}

pub fn equipment_chest_from_ankh_jewel_or_seal(old_obj: &Object, item: Equipment) -> ChestObject {
    let item = ChestItem::Equipment(item);
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.set_flag().unwrap());
    ChestObject::new(
        old_obj.x(),
        old_obj.y(),
        ALWAYS_ON_FLAG_NO,
        item,
        -1,
        starts,
    )
}

pub fn rom_chset_from_ankh_jewel_or_seal(old_obj: &Object, item: Rom) -> ChestObject {
    let item = ChestItem::Rom(item);
    let starts = starts_without_old_flag(old_obj.starts(), old_obj.set_flag().unwrap());
    ChestObject::new(
        old_obj.x(),
        old_obj.y(),
        ALWAYS_ON_FLAG_NO,
        item,
        -1,
        starts,
    )
}

pub fn equipment_chest(old_obj: &ChestObject, item: Equipment) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, item.flag);
    let item = ChestItem::Equipment(item);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

pub fn rom_chest(old_obj: &ChestObject, item: Rom) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, item.flag);
    let item = ChestItem::Rom(item);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

pub fn empty_chest(old_obj: &ChestObject, set_flag: u16) -> ChestObject {
    let open_flag = old_obj.open_flag();
    let item = ChestItem::None(open_flag as i32);
    let old_item_flag = u16::try_from(old_obj.item().flag()).unwrap();
    let starts = starts_as_is(old_obj.starts(), old_item_flag, set_flag);
    ChestObject::new(old_obj.x(), old_obj.y(), open_flag, item, -1, starts)
}

pub fn simple_sub_weapon(x: i32, y: i32, open_flag: u16, item: SubWeapon) -> SubWeaponObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    SubWeaponObject::new(x, y, item, starts.to_vec())
}

pub fn sub_weapon(old_obj: &Object, item: SubWeapon) -> SubWeaponObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    SubWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn hidden_sub_weapon(old_obj: &ChestObject, item: SubWeapon) -> SubWeaponObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    SubWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn rom(old_obj: &RomObject, new_rom: Rom) -> RomObject {
    let starts = starts_with_replaced_flag(old_obj.starts(), old_obj.rom().flag, new_rom.flag);
    RomObject::new(old_obj.x(), old_obj.y(), new_rom, starts)
}

pub fn simple_seal(x: i32, y: i32, open_flag: u16, item: Seal) -> SealObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    SealObject::new(x, y, item, starts.to_vec())
}

pub fn seal(old_obj: &Object, item: Seal) -> SealObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    SealObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn hidden_seal(old_obj: &ChestObject, item: Seal) -> SealObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    SealObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn simple_main_weapon(x: i32, y: i32, open_flag: u16, item: MainWeapon) -> MainWeaponObject {
    let starts = starts_with_open_and_remove_flags(open_flag, item.flag);
    MainWeaponObject::new(x, y, item, starts.to_vec())
}

pub fn main_weapon(old_obj: &Object, item: MainWeapon) -> MainWeaponObject {
    let old_set_flag = old_obj.set_flag().unwrap();
    let starts = starts_as_is(old_obj.starts(), old_set_flag, item.flag);
    MainWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn hidden_main_weapon(old_obj: &ChestObject, item: MainWeapon) -> MainWeaponObject {
    let starts = starts_that_hide_when_startup_and_taken(old_obj, item.flag).unwrap();
    MainWeaponObject::new(old_obj.x(), old_obj.y(), item, starts)
}

pub fn memo(old_obj: &RomObject, set_flag: u16, done_flag: u16) -> UnknownObject {
    let shadow_event = Start {
        flag: 99999,
        run_when: true,
    };
    debug_assert!(!old_obj.starts().iter().any(|start| {
        start
            == &Start {
                flag: old_obj.rom().flag as u32,
                run_when: true,
            }
    }));
    let starts: Vec<_> = [shadow_event]
        .into_iter()
        .chain(
            old_obj
                .starts()
                .iter()
                .filter(|start| start.flag != 99999)
                .filter(|start| start.flag != old_obj.rom().flag as u32)
                .cloned(),
        )
        .chain([Start {
            flag: done_flag as u32,
            run_when: false,
        }])
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

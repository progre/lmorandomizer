mod object_factory;

use object_factory::{
    empty_chest, equipment_chest, equipment_chest_from_ankh_jewel_or_seal, hidden_main_weapon,
    hidden_seal, hidden_sub_weapon, invisible_chest, main_weapon, memo, rom, rom_chest,
    rom_chset_from_ankh_jewel_or_seal, seal, simple_main_weapon, simple_seal, simple_sub_weapon,
    sub_weapon,
};

use crate::script::{
    consts::MEMO_FLAG_BASE_NO,
    data::{
        item::{ChestItem, Item},
        object::{ChestObject, Object, RomObject},
    },
};

pub fn to_object_for_shutter(old_obj: &Object, open_flag: u16, item: Item) -> Object {
    match item {
        Item::MainWeapon(item) => Object::MainWeapon(main_weapon(old_obj, item)),
        Item::SubWeapon(item) => Object::SubWeapon(sub_weapon(old_obj, item)),
        Item::Equipment(item) => {
            let item = ChestItem::Equipment(item);
            Object::Chest(invisible_chest(old_obj.x(), old_obj.y(), open_flag, item))
        }
        Item::Rom(item) => {
            let item = ChestItem::Rom(item);
            Object::Chest(invisible_chest(old_obj.x(), old_obj.y(), open_flag, item))
        }
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

pub fn to_objects_for_hand_scanner(old_obj: &RomObject, item: Item) -> Vec<Object> {
    if let Item::Rom(item) = item {
        return vec![Object::Rom(rom(old_obj, item))];
    }

    let open_flag = MEMO_FLAG_BASE_NO + old_obj.rom().content as u16;
    let memo = memo(old_obj, open_flag, item.flag());
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

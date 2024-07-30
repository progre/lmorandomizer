use super::{
    item,
    items::{Equipment, Rom, SubWeapon},
    object::{ChestObject, Object, Start, UnknownObject},
    script::{Field, Map, World},
};

pub fn add_starting_items(
    worlds: Vec<World>,
    equipment_list: &[Equipment],
    rom_list: &[Rom],
    sub_weapon_list: &[SubWeapon],
) -> Vec<World> {
    let unused_one_time_flag_no = 7400;
    let unused_save_flag_no = 6000;
    let x = 26624;
    let y = 14336;
    let starting_items: Vec<_> = [
        // // money
        // Object::Unknown(UnknownObject {
        //     number: 7,
        //     x: 43008,
        //     y: 22528,
        //     op1: 7,
        //     op2: 999,
        //     op3: -1,
        //     op4: -1,
        //     starts: vec![],
        // }),
        // // weights
        // Object::Unknown(UnknownObject {
        //     number: 7,
        //     x: 43008,
        //     y: 22528,
        //     op1: 6,
        //     op2: 999,
        //     op3: -1,
        //     op4: -1,
        //     starts: vec![],
        // }),
        Object::Unknown(UnknownObject {
            number: 22,
            x: 26624,
            y: 10240,
            op1: 2,
            op2: 2,
            op3: unused_one_time_flag_no,
            op4: -1,
            starts: vec![],
        }),
    ]
    .into_iter()
    .chain(sub_weapon_list.iter().flat_map(|sub_weapon| {
        [
            Object::Unknown(UnknownObject {
                number: 13,
                x,
                y,
                op1: *sub_weapon as i32,
                op2: 0,
                op3: unused_save_flag_no as i32,
                op4: -1,
                starts: vec![Start {
                    flag: unused_save_flag_no,
                    run_when: false,
                }],
            }),
            Object::Unknown(UnknownObject {
                number: 13,
                x,
                y,
                op1: *sub_weapon as i32,
                op2: 255,
                op3: unused_save_flag_no as i32,
                op4: -1,
                starts: vec![Start {
                    flag: unused_save_flag_no,
                    run_when: false,
                }],
            }),
        ]
    }))
    .chain(equipment_list.iter().map(|equipment| {
        Object::Chest(ChestObject::new(
            x,
            y,
            unused_one_time_flag_no as u16,
            item::ChestItem::Equipment(item::Equipment {
                content: *equipment,
                price: None,
                flag: unused_save_flag_no as u16,
            }),
            -1,
            vec![],
        ))
    }))
    .chain(rom_list.iter().map(|rom| {
        Object::Chest(ChestObject::new(
            x,
            y,
            unused_one_time_flag_no as u16,
            item::ChestItem::Rom(item::Rom {
                content: *rom,
                price: None,
                flag: unused_save_flag_no as u16,
            }),
            -1,
            vec![],
        ))
    }))
    .collect();
    worlds
        .into_iter()
        .map(|world| World {
            number: world.number,
            fields: world
                .fields
                .into_iter()
                .map(|field| {
                    if field.attrs.0 != 1 {
                        field
                    } else {
                        Field {
                            maps: field
                                .maps
                                .into_iter()
                                .map(|map| {
                                    if !(map.attrs.0 == 3 && map.attrs.1 == 1) {
                                        map
                                    } else {
                                        Map {
                                            objects: map
                                                .objects
                                                .into_iter()
                                                .chain(starting_items.iter().cloned())
                                                .collect(),
                                            ..map
                                        }
                                    }
                                })
                                .collect(),
                            ..field
                        }
                    }
                })
                .collect(),
        })
        .collect()
}

use super::{
    items::{Equipment, SubWeapon},
    object::{Object, Start},
    script::{Field, Map, World},
};

pub fn add_starting_items(
    worlds: Vec<World>,
    equipment_list: &[Equipment],
    sub_weapon_list: &[SubWeapon],
) -> Vec<World> {
    let unused_one_time_flag_no = 7400;
    let unused_save_flag_no = 6000;
    let starting_items: Vec<_> = [
        // create_object(7, 43008, 22528, 7, 999, -1, -1, vec![]), // money
        // create_object(7, 43008, 22528, 6, 999, -1, -1, vec![]), // weights
        Object {
            number: 22,
            x: 26624,
            y: 10240,
            op1: 2,
            op2: 2,
            op3: unused_one_time_flag_no,
            op4: -1,
            starts: vec![],
        },
    ]
    .into_iter()
    .chain(sub_weapon_list.iter().flat_map(|x| {
        [
            Object {
                number: 13,
                x: 26624,
                y: 10240,
                op1: *x as i32,
                op2: 0,
                op3: unused_save_flag_no as i32,
                op4: -1,
                starts: vec![Start {
                    flag: unused_save_flag_no,
                    run_when_unset: false,
                }],
            },
            Object {
                number: 13,
                x: 26624,
                y: 10240,
                op1: *x as i32,
                op2: 255,
                op3: unused_save_flag_no as i32,
                op4: -1,
                starts: vec![Start {
                    flag: unused_save_flag_no,
                    run_when_unset: false,
                }],
            },
        ]
    }))
    .chain(equipment_list.iter().map(|x| Object {
        number: 1,
        x: 26624,
        y: 14336,
        op1: unused_one_time_flag_no,
        op2: *x as i32,
        op3: unused_save_flag_no as i32,
        op4: -1,
        starts: vec![],
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

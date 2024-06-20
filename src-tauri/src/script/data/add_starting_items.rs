use super::{
    items::{Equipment, SubWeapon},
    object::{Object, Start},
    script::{Field, Map, World},
};

#[allow(clippy::too_many_arguments)]
fn create_object(
    number: u16,
    x: i32,
    y: i32,
    op1: i32,
    op2: i32,
    op3: i32,
    op4: i32,
    starts: Vec<Start>,
) -> Object {
    Object {
        number,
        x,
        y,
        op1,
        op2,
        op3,
        op4,
        starts,
    }
}

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
        create_object(22, 26624, 10240, 2, 2, unused_one_time_flag_no, -1, vec![]),
    ]
    .into_iter()
    .chain(sub_weapon_list.iter().flat_map(|x| {
        [
            create_object(
                13,
                26624,
                10240,
                *x as i32,
                0,
                unused_save_flag_no as i32,
                -1,
                vec![Start {
                    number: unused_save_flag_no,
                    run_when_unset: false,
                }],
            ),
            create_object(
                13,
                26624,
                10240,
                *x as i32,
                255,
                unused_save_flag_no as i32,
                -1,
                vec![Start {
                    number: unused_save_flag_no,
                    run_when_unset: false,
                }],
            ),
        ]
    }))
    .chain(equipment_list.iter().map(|x| {
        create_object(
            1,
            26624,
            14336,
            unused_one_time_flag_no,
            *x as i32,
            unused_save_flag_no as i32,
            -1,
            vec![],
        )
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

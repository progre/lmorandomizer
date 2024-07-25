use anyhow::Result;

use super::{ChestObject, Object, Start};

pub fn starts_that_hide_when_startup_and_taken(
    old_obj: &ChestObject,
    flag: u16,
) -> Result<Vec<Start>> {
    let old_item_flag = old_obj.set_flag();
    Ok([
        Start {
            flag: 99999,
            run_when: true,
        },
        Start {
            flag: u32::try_from(old_obj.open_flag())?,
            run_when: true,
        },
        Start {
            flag: flag as u32,
            run_when: false,
        },
    ]
    .into_iter()
    .chain(
        old_obj
            .starts()
            .iter()
            .filter(|x| ![99999, old_item_flag as u32].contains(&x.flag))
            .cloned(),
    )
    .collect())
}

pub fn starts_that_hide_when_startup(old_obj: &Object, start_flag: u16) -> Result<Vec<Start>> {
    let old_item_flag = old_obj.get_item_flag()?;
    Ok([
        Start {
            flag: 99999,
            run_when: true,
        },
        Start {
            flag: start_flag as u32,
            run_when: true,
        },
    ]
    .into_iter()
    .chain(
        old_obj
            .starts()
            .iter()
            .filter(|x| ![99999, old_item_flag as u32].contains(&x.flag))
            .cloned(),
    )
    .collect())
}

pub fn starts_as_is(old_starts: &[Start], old_set_flag: u16, flag: u16) -> Vec<Start> {
    let mut vec = starts_without_old_flag(old_starts, old_set_flag);
    if old_starts.iter().any(|x| x.flag == old_set_flag as u32) {
        vec.push(Start {
            flag: flag as u32,
            run_when: false, // TODO: bug? 元の run_when を維持すべきの筈
        });
    }
    vec
}

pub fn starts_without_old_flag(starts: &[Start], set_flag: u16) -> Vec<Start> {
    starts
        .iter()
        .filter(|x| x.flag != set_flag as u32)
        .cloned()
        .collect()
}

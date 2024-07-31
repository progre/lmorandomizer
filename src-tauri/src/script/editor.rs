pub mod add_starting_items;
mod consts;
mod objects_factory;
mod script_editor;
mod talks_editor;

use std::ops::Deref;

use anyhow::Result;

use crate::randomizer::storage::Storage;

use super::data::{object::ItemShop, script::Script};

use {script_editor::replace_items, talks_editor::replace_shops};

pub fn apply_storage(script: &mut Script, shuffled: &Storage) -> Result<()> {
    let mut worlds = script.worlds.clone();
    replace_items(&mut worlds, script.deref(), shuffled)?;

    let shops: Vec<_> = script
        .shops()
        .filter_map(|x| ItemShop::try_from_shop_object(x, &script.talks).transpose())
        .collect::<Result<_>>()?;
    let mut talks = script.talks.clone();
    replace_shops(&mut talks, script.deref(), &shops, &shuffled.shops)?;
    script.worlds = worlds;
    script.talks = talks;
    Ok(())
}

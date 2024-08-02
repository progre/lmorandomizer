mod randomize_items;
mod spoiler;
mod spoiler_log;
pub mod storage;

use std::mem::take;

use anyhow::Result;
use log::trace;
use randomize_items::randomize_items;
pub use spoiler_log::SpoilerLog;
use storage::{create_source::create_source, Storage};

use crate::{
    dataset::{
        game_structure::{GameStructure, GameStructureFiles},
        NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT,
        TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT, WARE_NO_MISE_COUNT,
    },
    script::{
        data::{object::Shop, script::Script},
        editor::add_starting_items::add_starting_items,
        enums::Rom,
        file::scriptconverter::{build_script_dat, read_script_dat},
    },
};

pub fn assert_eq_elem_count(source: &Storage, script: &Script) {
    let main_weapon_count = source.main_weapons.len();
    debug_assert_eq!(main_weapon_count, script.main_weapons().count());
    let sub_weapon_count = source.sub_weapons.len() + NIGHT_SURFACE_SUB_WEAPON_COUNT;
    debug_assert_eq!(sub_weapon_count, script.sub_weapons().count());
    let chest_count = source.chests.len() + NIGHT_SURFACE_CHEST_COUNT;
    debug_assert_eq!(chest_count, script.chests().count());
    let seal_count =
        source.seals.len() + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT + NIGHT_SURFACE_SEAL_COUNT;
    debug_assert_eq!(seal_count, script.seals().count());
    let shop_count = source.shops.len() + WARE_NO_MISE_COUNT;
    debug_assert_eq!(
        shop_count,
        script
            .shops()
            .filter_map(|x| Shop::try_from_shop_object(x, &script.talks).transpose())
            .collect::<Result<Vec<_>>>()
            .unwrap()
            .len()
    );
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomizeOptions {
    pub seed: String,
    pub shuffle_secret_roms: bool,
    pub need_glitches: bool,
    pub absolutely_shuffle: bool,
}

pub fn randomize(
    script_dat: &[u8],
    game_structure_files: GameStructureFiles,
    options: &RandomizeOptions,
) -> Result<(Vec<u8>, SpoilerLog)> {
    let start = std::time::Instant::now();
    let mut script = read_script_dat(script_dat)?;
    trace!("Read script.dat in {:?}", start.elapsed());

    let game_structure = GameStructure::new(game_structure_files)?;
    let source = create_source(&game_structure, options)?;

    if cfg!(debug_assertions) {
        let start = std::time::Instant::now();
        assert_eq_elem_count(&source, &script);
        trace!("assert_eq_elem_count {:?}", start.elapsed());
    }

    let start = std::time::Instant::now();
    let spoiler_log = randomize_items(&mut script, &source, options)?;
    if false {
        let worlds = take(&mut script.worlds);
        script.worlds = add_starting_items(
            worlds,
            &[
                // crate::script::data::items::Equipment::Boots,
                // crate::script::data::items::Equipment::Feather,
                // crate::script::data::items::Equipment::Glove,
                // crate::script::data::items::Equipment::SacredOrb,
                // crate::script::data::items::Equipment::SacredOrb,
            ],
            &[Rom::GameMaster],
            &[
                // crate::script::data::items::SubWeapon::Pistol
            ],
        );
    }
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let dat = build_script_dat(&script);
    trace!("Built script.dat in {:?}", start.elapsed());

    Ok((dat, spoiler_log.to_owned()))
}

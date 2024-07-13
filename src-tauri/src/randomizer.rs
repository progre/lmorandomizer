mod pickup_items;
mod randomize_items;
mod sphere;
mod spoiler;
mod spoiler_log;

use anyhow::Result;
use log::trace;
use randomize_items::randomize_items;
pub use spoiler_log::SpoilerLog;

use crate::{
    dataset::{
        create_source::create_source, storage::Storage, supplements::SupplementFiles,
        NIGHT_SURFACE_CHEST_COUNT, NIGHT_SURFACE_SEAL_COUNT, NIGHT_SURFACE_SUB_WEAPON_COUNT,
        TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT, WARE_NO_MISE_COUNT,
    },
    script::{
        data::{items::Equipment, script::Script},
        file::scriptconverter::{build_script_dat, read_script_dat},
    },
};

pub fn assert_eq_elem_count(source: &Storage, script: &Script) {
    let main_weapon_count = source.main_weapons().len();
    debug_assert_eq!(main_weapon_count, script.main_weapons().unwrap().len());
    let sub_weapon_count = source.sub_weapons().len() + NIGHT_SURFACE_SUB_WEAPON_COUNT;
    debug_assert_eq!(sub_weapon_count, script.sub_weapons().unwrap().len(),);
    let chest_count = source.chests().len() + NIGHT_SURFACE_CHEST_COUNT;
    debug_assert_eq!(chest_count, script.chests().unwrap().len(),);
    let seal_count =
        source.seals().len() + TRUE_SHRINE_OF_THE_MOTHER_SEAL_COUNT + NIGHT_SURFACE_SEAL_COUNT;
    debug_assert_eq!(seal_count, script.seals().unwrap().len());
    let shop_count = source.shops().len() + WARE_NO_MISE_COUNT;
    debug_assert_eq!(shop_count, script.shops().unwrap().len());
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomizeOptions {
    seed: String,
    easy_mode: bool,
}

pub fn randomize(
    script_dat: &[u8],
    supplement_files: &SupplementFiles,
    options: &RandomizeOptions,
) -> Result<(Vec<u8>, SpoilerLog)> {
    let start = std::time::Instant::now();
    let mut script = read_script_dat(script_dat)?;
    trace!("Read script.dat in {:?}", start.elapsed());

    let start = std::time::Instant::now();

    trace!("Loaded supplements in {:?}", start.elapsed());

    let source = create_source(supplement_files);

    let start = std::time::Instant::now();
    if cfg!(debug_assertions) {
        assert_eq_elem_count(&source, &script);
    }
    trace!("assert_eq_elem_count {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let spoiler_log = randomize_items(&mut script, &source, &options.seed)?;
    if options.easy_mode {
        script.add_starting_items(&[Equipment::GameMaster], &[]);
    }
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let dat = build_script_dat(&script);
    trace!("Built script.dat in {:?}", start.elapsed());

    Ok((dat, spoiler_log))
}

use anyhow::Result;
use items::EquipmentNumber;
use randomize_items::randomize_items;

use crate::{
    dataset::supplements::{SupplementFiles, Supplements},
    util::scriptdat::format::scriptconverter::{build_script_dat, read_script_dat},
};

pub mod items;
pub mod randomize_items;
pub mod shuffle_utils;
pub mod validate;

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
) -> Result<Vec<u8>> {
    let mut script = read_script_dat(script_dat)?;
    let supplements = Supplements::new(supplement_files);
    randomize_items(&mut script, &supplements, &options.seed)?;
    if options.easy_mode {
        script.add_starting_items(&[EquipmentNumber::GameMaster], &[]);
    }
    Ok(build_script_dat(&script))
}

mod randomize_items;
mod shuffle_utils;
mod validate;

use anyhow::Result;
use log::trace;
use randomize_items::randomize_items;

use crate::{
    dataset::{create_source::create_source, supplements::SupplementFiles},
    script::{
        data::items::Equipment,
        file::scriptconverter::{build_script_dat, read_script_dat},
    },
};

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
    let start = std::time::Instant::now();
    let mut script = read_script_dat(script_dat)?;
    trace!("Read script.dat in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let source = create_source(&script, supplement_files)?;
    randomize_items(&mut script, &source, &options.seed)?;
    if options.easy_mode {
        script.add_starting_items(&[Equipment::GameMaster], &[]);
    }
    trace!("Randomized items in {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let dat = build_script_dat(&script);
    trace!("Built script.dat in {:?}", start.elapsed());

    Ok(dat)
}

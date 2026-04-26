use anyhow::Result;
use futures::future::join_all;
use log::info;
use semver::Version;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager, path::BaseDirectory};
use tokio::{
    fs::{File, read_to_string},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::{dataset::game_structure::GameStructureFiles, randomizer::SpoilerLog};

pub async fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)
        .await
        .inspect_err(|_| log::trace!("open {:?}", path))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .inspect_err(|_| log::trace!("read"))?;
    Ok(contents)
}

pub async fn write_file(path: &Path, contents: &[u8]) -> io::Result<()> {
    info!("Writing file: {:?}", path);
    let mut file = File::create(path).await?;
    file.write_all(contents).await?;
    Ok(())
}

async fn read_game_structure_files_internal(
    resolve_path: impl Fn(&str) -> Result<PathBuf>,
) -> anyhow::Result<GameStructureFiles> {
    let file_paths = [
        "res/00_Surface.yml",
        "res/01_Gate_of_Guidance.yml",
        "res/02_Mausoleum_of_the_Giants.yml",
        "res/03_Temple_of_the_Sun.yml",
        "res/04_Spring_in_the_Sky.yml",
        "res/05_Inferno_Cavern.yml",
        "res/06_Chamber_of_Extinction.yml",
        "res/07_Twin_Labyrinths_Left.yml",
        "res/08_Endless_Corridor.yml",
        "res/09_Shrine_of_the_Mother.yml",
        "res/11_Gate_of_Illusion.yml",
        "res/12_Graveyard_of_the_Giants.yml",
        "res/13_Temple_of_Moonlight.yml",
        "res/14_Tower_of_the_Goddess.yml",
        "res/15_Tower_of_Ruin.yml",
        "res/16_Chamber_of_Birth.yml",
        "res/17_Twin_Labyrinths_Right.yml",
        "res/18_Dimensional_Corridor.yml",
        "res/19_True_Shrine_of_the_Mother.yml",
    ];
    #[allow(clippy::redundant_closure)]
    let futures: Vec<_> = file_paths
        .map(|file_path| resolve_path(file_path))
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(read_to_string)
        .collect();
    let fields: BTreeMap<_, _> = join_all(futures)
        .await
        .into_iter()
        .collect::<io::Result<Vec<_>>>()?
        .into_iter()
        .zip(file_paths)
        .map(|(contents, file_path)| (file_path[4..6].parse::<u8>().unwrap(), contents))
        .collect();
    let events = read_to_string(resolve_path("res/events.yml")?).await?;

    GameStructureFiles::new(fields, events)
}

pub async fn read_game_structure_files(handle: &AppHandle) -> Result<GameStructureFiles> {
    let path = handle.path();
    read_game_structure_files_internal(|file_path| {
        Ok(path.resolve(file_path, BaseDirectory::Resource)?)
    })
    .await
}

#[cfg(test)]
pub async fn read_game_structure_files_debug() -> Result<GameStructureFiles> {
    read_game_structure_files_internal(|file_path| Ok(PathBuf::from(file_path))).await
}

pub async fn write_spoiler_log(
    path: &Path,
    version: &Version,
    seed: &str,
    spoiler_log: &SpoilerLog,
) -> io::Result<()> {
    info!("Writing file: {:?}", path);
    let header = format!("version = v{version}\nseed = {seed}\n\n");
    let mut file = File::create(path).await?;
    file.write_all(header.as_bytes()).await?;
    file.write_all(spoiler_log.to_string().as_bytes()).await?;
    Ok(())
}

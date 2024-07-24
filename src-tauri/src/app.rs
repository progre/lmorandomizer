use anyhow::Result;
use futures::future::join_all;
use log::{error, info};
use num_traits::FromPrimitive;
use serde_json::json;
use std::{collections::BTreeMap, path::PathBuf};
use tauri::{path::BaseDirectory, AppHandle, Manager, State, Wry};
use tauri_plugin_store::{with_store, Store, StoreCollection};
use tokio::{
    fs::{read_to_string, File},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::{
    dataset::{game_structure::GameStructureFiles, spot::FieldId},
    randomizer::{randomize, RandomizeOptions, SpoilerLog},
    script::file::scriptconverter::is_valid_script_dat,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialData {
    seed: String,
    install_directory: String,
    easy_mode: bool,
}

impl InitialData {
    pub fn read(store: &Store<Wry>) -> Self {
        Self {
            seed: store
                .get("seed")
                .and_then(|obj| obj.as_str())
                .unwrap_or("")
                .to_owned(),
            install_directory: store
                .get("install_directory")
                .and_then(|obj| obj.as_str())
                .unwrap_or("")
                .to_owned(),
            easy_mode: store
                .get("easy_mode")
                .and_then(|obj| obj.as_bool())
                .unwrap_or(false),
        }
    }

    pub fn write(&self, store: &mut Store<Wry>) -> Result<(), tauri_plugin_store::Error> {
        let InitialData {
            seed,
            install_directory,
            easy_mode,
        } = &self;
        store.insert("seed".to_owned(), json!(seed))?;
        store.insert("install_directory".to_owned(), json!(install_directory))?;
        store.insert("easy_mode".to_owned(), json!(easy_mode))?;
        Ok(())
    }
}

#[tauri::command]
pub fn initial_data(app_handle: AppHandle, stores: State<StoreCollection<Wry>>) -> InitialData {
    with_store(app_handle, stores, PathBuf::from("store.json"), |store| {
        Ok(InitialData::read(store))
    })
    .unwrap()
}

#[tauri::command]
pub fn ready(app_handle: AppHandle) {
    app_handle
        .get_webview_window("main")
        .unwrap()
        .show()
        .unwrap();
}

fn set_initial_data_value<T>(
    app_handle: AppHandle,
    stores: State<StoreCollection<Wry>>,
    callback: impl FnOnce(&mut InitialData) -> T,
) where
    T: serde::Serialize,
{
    with_store(app_handle, stores, PathBuf::from("store.json"), |store| {
        let mut data = InitialData::read(store);
        callback(&mut data);
        data.write(store)?;
        store.save()?;
        Ok(())
    })
    .unwrap();
}

#[tauri::command]
pub fn set_seed(app_handle: AppHandle, stores: State<StoreCollection<Wry>>, value: String) {
    set_initial_data_value(app_handle, stores, |data| data.seed = value);
}

#[tauri::command]
pub fn set_install_directory(
    app_handle: AppHandle,
    stores: State<StoreCollection<Wry>>,
    value: String,
) {
    set_initial_data_value(app_handle, stores, |data| data.install_directory = value);
}

#[tauri::command]
pub fn set_easy_mode(app_handle: AppHandle, stores: State<StoreCollection<Wry>>, value: bool) {
    set_initial_data_value(app_handle, stores, |data| data.easy_mode = value);
}

async fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)
        .await
        .inspect_err(|_| log::trace!("open {}", path))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .inspect_err(|_| log::trace!("read"))?;
    Ok(contents)
}

async fn write_file(path: &str, contents: &[u8]) -> io::Result<()> {
    info!("Writing file: {}", path);
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
        .map(|(contents, file_path)| {
            (
                FieldId::from_u8(file_path[4..6].parse::<u8>().unwrap()).unwrap(),
                contents,
            )
        })
        .collect();
    let events = read_to_string(resolve_path("res/events.yml")?).await?;

    GameStructureFiles::new(fields, events)
}

async fn read_game_structure_files(handle: AppHandle) -> Result<GameStructureFiles> {
    let path = handle.path();
    read_game_structure_files_internal(|file_path| {
        Ok(path.resolve(file_path, BaseDirectory::Resource)?)
    })
    .await
}

#[allow(unused)]
pub async fn read_game_structure_files_debug() -> Result<GameStructureFiles> {
    read_game_structure_files_internal(|file_path| Ok(PathBuf::from(file_path))).await
}

#[tauri::command]
pub async fn apply(
    handle: AppHandle,
    install_directory: String,
    options: RandomizeOptions,
) -> String {
    log::trace!("{}", install_directory);
    let target_file_path = format!("{}/data/script.dat", install_directory);
    let backup_file_path = format!("{}/data/script.dat.bak", install_directory);

    let working = if let Some(working) = read_valid_file_or_null(&backup_file_path).await {
        working
    } else {
        let Some(working) = read_file(&target_file_path).await.ok() else {
            return "Unable to find La-Mulana install directory.".to_owned();
        };
        if !is_valid_script_dat(&working) {
            return "Valid script is not found. Please re-install La-Mulana.".to_owned();
        }
        if let Err(err) = write_valid_script_dat(&backup_file_path, &working).await {
            return format!("Failed to backup script.dat: {}", err);
        }
        working
    };
    let game_structure = match read_game_structure_files(handle).await {
        Ok(ok) => ok,
        Err(err) => return format!("Failed to read game structure files: {}", err),
    };

    let (randomized, spoiler_log) = match randomize(&working, game_structure, &options) {
        Ok(randomized) => randomized,
        Err(e) => {
            error!("{:?}", e);
            return format!("Randomization failed: {}", e);
        }
    };

    if let Err(err) = write_file(&target_file_path, &randomized).await {
        return format!("Failed to write randomized script.dat: {}", err);
    }
    let spoiler_log_file_path = format!("{}/data/spoilerlog.txt", install_directory);
    if let Err(err) = write_spoiler_log(&spoiler_log_file_path, &spoiler_log).await {
        return format!("Failed to write spoiler log: {}", err);
    }
    "Succeeded.".to_owned()
}

#[tauri::command]
pub async fn restore(install_directory: String) -> String {
    let target_file_path = format!("{}/data/script.dat", install_directory);
    let backup_file_path = format!("{}/data/script.dat.bak", install_directory);

    if let Ok(target_file) = read_file(&target_file_path).await {
        if is_valid_script_dat(&target_file) {
            return "Already clean.".to_owned();
        }
    }
    let Some(working) = read_valid_file_or_null(&backup_file_path).await else {
        return "Backup is broken. Please re-install La-Mulana.".to_owned();
    };
    if let Err(err) = write_valid_script_dat(&target_file_path, &working).await {
        return format!("Failed to restore script.dat: {}", err);
    }
    "Succeeded.".to_owned()
}

async fn read_valid_file_or_null(path: &str) -> Option<Vec<u8>> {
    let Ok(working) = read_file(path).await else {
        return None;
    };
    if !is_valid_script_dat(&working) {
        return None;
    }
    Some(working)
}

async fn write_valid_script_dat(path: &str, script_dat: &[u8]) -> io::Result<()> {
    write_file(path, script_dat).await?;
    if cfg!(debug_assertions) {
        let Ok(outputed) = read_file(path).await else {
            panic!();
        };
        if !is_valid_script_dat(&outputed) {
            panic!();
        }
    }
    Ok(())
}

async fn write_spoiler_log(path: &str, spoiler_log: &SpoilerLog) -> io::Result<()> {
    write_file(path, spoiler_log.to_string().as_bytes()).await
}

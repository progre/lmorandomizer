mod file;
mod initial_data;
mod launch;

use log::error;
use smol::io;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;

use crate::{
    app::{
        file::{read_file, read_game_structure_files, write_file, write_spoiler_log},
        initial_data::InitialData,
    },
    randomizer::{RandomizeOptions, randomize},
    script::file::scriptconverter::is_valid_script_dat,
};

#[cfg(test)]
pub use file::read_game_structure_files_debug;

#[tauri::command]
pub fn initial_data(app_handle: AppHandle) -> InitialData {
    let store = app_handle.store(PathBuf::from("store.json")).unwrap();
    InitialData::read(&store)
}

#[tauri::command]
pub fn ready(app_handle: AppHandle) {
    app_handle
        .get_webview_window("main")
        .unwrap()
        .show()
        .unwrap();
}

fn set_initial_data_value<T>(app_handle: AppHandle, callback: impl FnOnce(&mut InitialData) -> T)
where
    T: serde::Serialize,
{
    let store = app_handle.store(PathBuf::from("store.json")).unwrap();
    let mut data = InitialData::read(&store);
    callback(&mut data);
    data.write(&store);
    store.save().unwrap();
}

#[tauri::command]
pub fn set_seed(app_handle: AppHandle, value: String) {
    set_initial_data_value(app_handle, |data| data.seed = value);
}

#[tauri::command]
pub fn set_install_directory(app_handle: AppHandle, value: String) {
    set_initial_data_value(app_handle, |data| data.install_directory = value);
}

#[tauri::command]
pub fn set_easy_mode(app_handle: AppHandle, value: bool) {
    set_initial_data_value(app_handle, |data| data.easy_mode = value);
}

#[tauri::command]
pub fn set_shuffle_secret_roms(app_handle: AppHandle, value: bool) {
    set_initial_data_value(app_handle, |data| data.shuffle_secret_roms = value);
}

#[tauri::command]
pub fn set_need_glitches(app_handle: AppHandle, value: bool) {
    set_initial_data_value(app_handle, |data| data.need_glitches = value);
}

#[tauri::command]
pub fn set_absolutely_shuffle(app_handle: AppHandle, value: bool) {
    set_initial_data_value(app_handle, |data| data.absolutely_shuffle = value);
}

#[tauri::command]
pub async fn launch(
    handle: AppHandle,
    install_directory: String,
    options: RandomizeOptions,
) -> String {
    match launch::launch(handle, install_directory, options).await {
        Ok(()) => "Succeeded.".to_owned(),
        Err(err) => format!("{err}"),
    }
}

#[tauri::command]
pub async fn open_folder(app: AppHandle) {
    let dir = app.path().app_data_dir().unwrap();
    app.opener()
        .open_path(dir.to_string_lossy(), None::<&str>)
        .unwrap();
}

#[tauri::command]
pub async fn apply(
    handle: AppHandle,
    install_directory: String,
    options: RandomizeOptions,
) -> String {
    log::trace!("{}", install_directory);
    let target_file_path = PathBuf::from(format!("{}/data/script.dat", install_directory));
    let backup_file_path = PathBuf::from(format!("{}/data/script.dat.bak", install_directory));

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
    let game_structure = match read_game_structure_files(&handle).await {
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
    let spoiler_log_file_path = PathBuf::from(format!("{}/data/spoilerlog.txt", install_directory));
    let version = &handle.package_info().version;
    if let Err(err) =
        write_spoiler_log(&spoiler_log_file_path, version, &options.seed, &spoiler_log).await
    {
        return format!("Failed to write spoiler log: {}", err);
    }
    "Succeeded.".to_owned()
}

#[tauri::command]
pub async fn restore(install_directory: String) -> String {
    let target_file_path = PathBuf::from(format!("{}/data/script.dat", install_directory));
    let backup_file_path = PathBuf::from(format!("{}/data/script.dat.bak", install_directory));

    if let Ok(target_file) = read_file(&target_file_path).await
        && is_valid_script_dat(&target_file)
    {
        return "Already clean.".to_owned();
    }
    let Some(working) = read_valid_file_or_null(&backup_file_path).await else {
        return "Backup is broken. Please re-install La-Mulana.".to_owned();
    };
    if let Err(err) = write_valid_script_dat(&target_file_path, &working).await {
        return format!("Failed to restore script.dat: {}", err);
    }
    "Succeeded.".to_owned()
}

async fn read_valid_file_or_null(path: &Path) -> Option<Vec<u8>> {
    let Ok(working) = read_file(path).await else {
        return None;
    };
    if !is_valid_script_dat(&working) {
        return None;
    }
    Some(working)
}

async fn write_valid_script_dat(path: &Path, script_dat: &[u8]) -> io::Result<()> {
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

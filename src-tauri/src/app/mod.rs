mod initial_data;
mod launch;

use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;

use crate::{app::initial_data::InitialData, randomizer::RandomizeOptions};

#[cfg(test)]
pub use launch::read_game_structure_files_debug;

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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dataset;
mod randomizer;
mod util;

use std::path::PathBuf;

use log::{info, LevelFilter};
use serde_json::json;
use tauri::{AppHandle, Config, Manager, State, Wry};
use tauri_plugin_store::{with_store, Store, StoreCollection};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct InitialData {
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
fn initial_data(app_handle: AppHandle, stores: State<StoreCollection<Wry>>) -> InitialData {
    with_store(app_handle, stores, PathBuf::from("store.json"), |store| {
        Ok(InitialData::read(store))
    })
    .unwrap()
}

#[tauri::command]
fn ready(app_handle: AppHandle) {
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
fn set_seed(app_handle: AppHandle, stores: State<StoreCollection<Wry>>, value: String) {
    set_initial_data_value(app_handle, stores, |data| data.seed = value);
}

#[tauri::command]
fn set_install_directory(
    app_handle: AppHandle,
    stores: State<StoreCollection<Wry>>,
    value: String,
) {
    set_initial_data_value(app_handle, stores, |data| data.install_directory = value);
}

#[tauri::command]
fn set_easy_mode(app_handle: AppHandle, stores: State<StoreCollection<Wry>>, value: bool) {
    set_initial_data_value(app_handle, stores, |data| data.easy_mode = value);
}

#[tauri::command]
async fn read_file(path: String) -> Option<Vec<u8>> {
    let mut file = File::open(path).await.ok()?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.ok()?;
    Some(contents)
}

#[tauri::command]
async fn write_file(path: String, contents: Vec<u8>) -> bool {
    info!("Writing file: {}", path);
    let Ok(mut file) = File::create(path).await else {
        return false;
    };
    let Ok(()) = file.write_all(&contents).await else {
        return false;
    };
    true
}

#[tauri::command]
fn read_script_dat(file: Vec<u8>) -> util::scriptdat::data::script::Script {
    util::scriptdat::format::scriptconverter::read_script_dat(&file).unwrap()
}

#[tauri::command]
fn build_script_dat(script: util::scriptdat::data::script::Script) -> Vec<u8> {
    util::scriptdat::format::scriptconverter::build_script_dat(&script)
}

#[tauri::command]
fn is_valid_script_dat(file: Vec<u8>) -> bool {
    util::scriptdat::format::scriptconverter::is_valid_script_dat(&file)
}

#[tauri::command]
fn create_supplements(
    supplement_files: dataset::supplements::SupplementFiles,
) -> dataset::supplements::Supplements {
    dataset::supplements::Supplements::new(supplement_files)
}

#[tauri::command]
fn script_add_starting_items(
    mut this: util::scriptdat::data::script::Script,
    equipment_list: Vec<randomizer::items::EquipmentNumber>,
    sub_weapon_list: Vec<randomizer::items::SubWeaponNumber>,
) -> util::scriptdat::data::script::Script {
    util::scriptdat::data::script::Script::add_starting_items(
        &mut this,
        &equipment_list,
        &sub_weapon_list,
    );
    this
}

#[tauri::command]
fn randomize_items(
    mut script: util::scriptdat::data::script::Script,
    supplements: dataset::supplements::Supplements,
    seed: String,
) -> util::scriptdat::data::script::Script {
    randomizer::randomize_items::randomize_items(&mut script, &supplements, &seed)
        .map_err(|e| e.to_string())
        .unwrap();
    script
}

fn main() {
    let mut context = tauri::generate_context!();
    let Config { app, version, .. } = context.config_mut();
    let version = version.as_ref().unwrap();
    app.windows[0].title = format!("La-Mulana Original Randomizer v{version}",);
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level_for("html5ever", LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            initial_data,
            ready,
            set_seed,
            set_install_directory,
            set_easy_mode,
            read_file,
            write_file,
            read_script_dat,
            build_script_dat,
            is_valid_script_dat,
            create_supplements,
            script_add_starting_items,
            randomize_items,
        ])
        .run(context)
        .expect("error while running tauri application");
}

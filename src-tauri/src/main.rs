// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod dataset;
pub mod randomizer;
mod script;

use log::LevelFilter;
use tauri::Config;

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
            app::initial_data,
            app::ready,
            app::set_seed,
            app::set_install_directory,
            app::set_easy_mode,
            app::set_shuffle_secret_roms,
            app::set_need_glitches,
            app::set_absolutely_shuffle,
            app::apply,
            app::restore,
        ])
        .run(context)
        .expect("error while running tauri application");
}

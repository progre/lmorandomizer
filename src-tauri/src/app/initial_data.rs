use serde_json::json;
use tauri::Wry;
use tauri_plugin_store::Store;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialData {
    pub seed: String,
    pub install_directory: String,
    pub easy_mode: bool,
    pub shuffle_secret_roms: bool,
    pub need_glitches: bool,
    pub absolutely_shuffle: bool,
}

impl InitialData {
    pub fn read(store: &Store<Wry>) -> Self {
        Self {
            seed: store
                .get("seed")
                .and_then(|x| x.as_str().map(|x| x.to_owned()))
                .unwrap_or_default(),
            install_directory: store
                .get("install_directory")
                .and_then(|x| x.as_str().map(|x| x.to_owned()))
                .unwrap_or_default(),
            easy_mode: store
                .get("easy_mode")
                .and_then(|obj| obj.as_bool())
                .unwrap_or(false),
            shuffle_secret_roms: store
                .get("shuffle_secret_roms")
                .and_then(|obj| obj.as_bool())
                .unwrap_or(true),
            need_glitches: store
                .get("need_glitches")
                .and_then(|obj| obj.as_bool())
                .unwrap_or(false),
            absolutely_shuffle: store
                .get("absolutely_shuffle")
                .and_then(|obj| obj.as_bool())
                .unwrap_or(false),
        }
    }

    pub fn write(&self, store: &Store<Wry>) {
        let InitialData {
            seed,
            install_directory,
            easy_mode,
            shuffle_secret_roms,
            need_glitches,
            absolutely_shuffle,
        } = &self;
        store.set("seed".to_owned(), json!(seed));
        store.set("install_directory".to_owned(), json!(install_directory));
        store.set("easy_mode".to_owned(), json!(easy_mode));
        store.set(
            "shuffle_secret_roms".to_owned(),
            json!(*shuffle_secret_roms),
        );
        store.set("need_glitches".to_owned(), json!(*need_glitches));
        store.set("absolutely_shuffle".to_owned(), json!(*absolutely_shuffle));
    }
}

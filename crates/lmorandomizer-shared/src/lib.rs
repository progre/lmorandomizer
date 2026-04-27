#[cfg(target_os = "windows")]
pub mod ipc;
pub mod lmo;

use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    Init(PathBuf),
}

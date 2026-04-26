use anyhow::{Context, Result, bail};
use log::error;
use semver::Version;
use smol::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tokio::io::{self};

use crate::{
    app::file::{read_file, read_game_structure_files, write_file, write_spoiler_log},
    launcher,
    randomizer::{RandomizeOptions, randomize},
    script::file::scriptconverter::is_valid_script_dat,
};

pub async fn launch(
    handle: AppHandle,
    install_directory: String,
    options: RandomizeOptions,
) -> Result<()> {
    let install_directory = PathBuf::from(install_directory);
    log::trace!("{:?}", install_directory);

    let version = &handle.package_info().version;
    let dst_dir_path = {
        let mut path = handle.path().app_data_dir().unwrap();
        path.push("worlds");
        path.push(to_dir_name(version, &options));
        path
    };
    let dst_file_path = dst_dir_path.join("script.dat");
    let spoiler_log_file_path = dst_dir_path.join("spoilerlog.txt");

    let found = match fs::metadata(&dst_file_path).await {
        Ok(_) => true,
        Err(err) if err.kind() == io::ErrorKind::NotFound => false,
        Err(err) => return Err(err.into()),
    };
    if !found {
        let _ = fs::create_dir_all(&dst_dir_path).await;

        create_randomized_script_dat(
            &handle,
            install_directory.clone(),
            options,
            version,
            &dst_file_path,
            &spoiler_log_file_path,
        )
        .await?;
    }

    match launcher::launch(&install_directory, "lamulana.exe", dst_dir_path) {
        Ok(_) => (),
        Err(err) => bail!("Failed to launch the game: {err}"),
    }

    Ok(())
}

async fn create_randomized_script_dat(
    handle: &AppHandle,
    mut install_directory: PathBuf,
    options: RandomizeOptions,
    version: &Version,
    dst_file_path: &Path,
    spoiler_log_file_path: &Path,
) -> Result<()> {
    let src_file_path = {
        install_directory.extend(["data", "script.dat"]);
        install_directory
    };
    let working = read_valid_file(&src_file_path).await?;
    let game_structure = match read_game_structure_files(handle).await {
        Ok(ok) => ok,
        Err(err) => bail!("Failed to read game structure files: {}", err),
    };

    let (randomized, spoiler_log) = match randomize(&working, game_structure, &options) {
        Ok(randomized) => randomized,
        Err(e) => {
            error!("{:?}", e);
            bail!("Randomization failed: {}", e);
        }
    };

    if let Err(err) = write_file(dst_file_path, &randomized).await {
        bail!("Failed to write randomized script.dat: {err}");
    }
    if let Err(err) =
        write_spoiler_log(spoiler_log_file_path, version, &options.seed, &spoiler_log).await
    {
        bail!("Failed to write spoiler log: {err}");
    }
    Ok(())
}

async fn read_valid_file(src_file_path: &Path) -> Result<Vec<u8>> {
    let working = read_file(src_file_path)
        .await
        .context("Unable to open script.dat.")?;
    if !is_valid_script_dat(&working) {
        bail!("Valid script.dat is not found. Please re-install La-Mulana.");
    }
    Ok(working)
}

fn to_dir_name(version: &Version, options: &RandomizeOptions) -> String {
    let seed = options
        .seed
        .chars()
        .map(|c| {
            if c.is_control()
                || matches!(
                    c,
                    '\0' | '"' | '%' | '*' | ',' | '.' | '/' | ':' | '<' | '>' | '?' | '\\' | '|'
                )
            {
                c.to_string()
                    .bytes()
                    .map(|x| format!("%{x:02x}"))
                    .collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();
    format!(
        "{},{},{}{}{}",
        version,
        seed,
        options.absolutely_shuffle as u8,
        options.need_glitches as u8,
        options.shuffle_secret_roms as u8,
    )
}

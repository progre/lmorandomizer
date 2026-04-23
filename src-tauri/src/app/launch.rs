use anyhow::{Context, Result, bail};
use futures::future::join_all;
use log::{error, info};
use semver::Version;
use smol::fs;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager, path::BaseDirectory};
use tokio::{
    fs::{File, read_to_string},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::{
    dataset::game_structure::GameStructureFiles,
    launcher,
    randomizer::{RandomizeOptions, SpoilerLog, randomize},
    script::file::scriptconverter::is_valid_script_dat,
};

async fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)
        .await
        .inspect_err(|_| log::trace!("open {:?}", path))?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .inspect_err(|_| log::trace!("read"))?;
    Ok(contents)
}

async fn write_file(path: &Path, contents: &[u8]) -> io::Result<()> {
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

async fn read_game_structure_files(handle: &AppHandle) -> Result<GameStructureFiles> {
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

async fn write_spoiler_log(
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

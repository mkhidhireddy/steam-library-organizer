use crate::{
    domain::Game,
    steam::{
        api::{fetch_owned_games, fetch_store_metadata},
        collections::cloud::parse_user_collections,
        export::apply_collection_update,
    },
};
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportPreview {
    pub cloud_file: String,
    pub collection_name: String,
    pub game_count: usize,
    pub existing_count: usize,
}

#[tauri::command]
pub async fn sync_library(steam_id: String, api_key: String) -> Result<Vec<Game>, String> {
    if steam_id.parse::<u64>().is_err() || api_key.trim().is_empty() {
        return Err("Enter a valid SteamID64 and API key".into());
    }
    fetch_owned_games(&steam_id, &api_key).await
}

#[tauri::command]
pub async fn refresh_metadata(mut games: Vec<Game>) -> Result<Vec<Game>, String> {
    for game in &mut games {
        if let Ok(tags) = fetch_store_metadata(game.app_id).await {
            game.tags = tags;
        }
    }
    Ok(games)
}

#[cfg(windows)]
fn steam_root() -> Result<PathBuf, String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Valve\\Steam")
        .map_err(|_| "Steam is not installed for this Windows user")?;
    let path: String = key
        .get_value("SteamPath")
        .map_err(|_| "Steam installation path is unavailable")?;
    Ok(PathBuf::from(path))
}

#[cfg(not(windows))]
fn steam_root() -> Result<PathBuf, String> {
    Err("The MVP supports Windows only".into())
}

fn find_cloud_file() -> Result<PathBuf, String> {
    let userdata = steam_root()?.join("userdata");
    let mut matches = Vec::new();
    for account in fs::read_dir(userdata).map_err(|error| error.to_string())? {
        let path = account
            .map_err(|error| error.to_string())?
            .path()
            .join("config")
            .join("cloudstorage")
            .join("cloud-storage-namespace-1.json");
        if path.is_file() {
            matches.push(path);
        }
    }
    match matches.len() {
        1 => Ok(matches.remove(0)),
        0 => Err("No Steam cloud collection file was found".into()),
        _ => Err("Multiple Steam accounts were found; account selection is required".into()),
    }
}

fn steam_is_running() -> bool {
    Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq steam.exe", "/NH"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .to_ascii_lowercase()
                .contains("steam.exe")
        })
        .unwrap_or(false)
}

#[tauri::command]
pub fn preview_export(name: String, app_ids: Vec<u32>) -> Result<ExportPreview, String> {
    let path = find_cloud_file()?;
    let source = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let existing_count = parse_user_collections(&source)?
        .iter()
        .find(|collection| collection.name == name)
        .map(|collection| collection.added.len())
        .unwrap_or(0);
    Ok(ExportPreview {
        cloud_file: path.display().to_string(),
        collection_name: name,
        game_count: app_ids.len(),
        existing_count,
    })
}

#[tauri::command]
pub fn apply_export(name: String, app_ids: Vec<u32>) -> Result<String, String> {
    if steam_is_running() {
        return Err("Close Steam completely before exporting collections".into());
    }
    let path = find_cloud_file()?;
    let id = format!("slo-{:x}", stable_hash(&name));
    let backup = apply_collection_update(Path::new(&path), &id, &name, &app_ids)?;
    Ok(backup.display().to_string())
}

fn stable_hash(value: &str) -> u64 {
    value.bytes().fold(1469598103934665603u64, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(1099511628211)
    })
}

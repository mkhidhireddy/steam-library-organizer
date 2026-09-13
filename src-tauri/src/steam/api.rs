use crate::domain::Game;
use serde::Deserialize;

#[derive(Deserialize)]
struct Envelope {
    response: Response,
}

#[derive(Deserialize)]
struct Response {
    games: Option<Vec<OwnedGame>>,
}

#[derive(Deserialize)]
struct OwnedGame {
    appid: u32,
    name: String,
    #[serde(default)]
    playtime_forever: u32,
}

pub fn parse_owned_games(source: &str) -> Result<Vec<Game>, String> {
    let envelope: Envelope = serde_json::from_str(source).map_err(|error| error.to_string())?;
    let games = envelope
        .response
        .games
        .ok_or("Steam response did not include owned games")?;
    Ok(games
        .into_iter()
        .map(|game| Game {
            app_id: game.appid,
            name: game.name,
            playtime_minutes: game.playtime_forever,
            installed: false,
            tags: Vec::new(),
            approved_ip: None,
        })
        .collect())
}

pub async fn fetch_owned_games(steam_id: &str, api_key: &str) -> Result<Vec<Game>, String> {
    let response = reqwest::Client::new()
        .get("https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/")
        .query(&[
            ("key", api_key),
            ("steamid", steam_id),
            ("include_appinfo", "true"),
            ("include_played_free_games", "true"),
        ])
        .send()
        .await
        .map_err(|_| "Could not reach Steam".to_owned())?;
    if !response.status().is_success() {
        return Err(format!("Steam returned status {}", response.status()));
    }
    parse_owned_games(
        &response
            .text()
            .await
            .map_err(|_| "Steam returned unreadable data".to_owned())?,
    )
}

pub fn parse_store_metadata(app_id: u32, source: &str) -> Result<Vec<String>, String> {
    let root: serde_json::Value =
        serde_json::from_str(source).map_err(|error| error.to_string())?;
    let entry = root
        .get(app_id.to_string())
        .ok_or("store response missing app")?;
    if entry.get("success").and_then(|value| value.as_bool()) != Some(true) {
        return Ok(Vec::new());
    }
    let data = entry.get("data").ok_or("store response missing data")?;
    let mut tags = Vec::new();
    for key in ["genres", "categories"] {
        if let Some(values) = data.get(key).and_then(|value| value.as_array()) {
            for value in values {
                if let Some(tag) = value.get("description").and_then(|value| value.as_str()) {
                    if !tags
                        .iter()
                        .any(|existing: &String| existing.eq_ignore_ascii_case(tag))
                    {
                        tags.push(tag.to_owned());
                    }
                }
            }
        }
    }
    Ok(tags)
}

pub async fn fetch_store_metadata(app_id: u32) -> Result<Vec<String>, String> {
    let response = reqwest::get(format!(
        "https://store.steampowered.com/api/appdetails?appids={app_id}&l=english"
    ))
    .await
    .map_err(|_| "Could not reach the Steam store".to_owned())?;
    parse_store_metadata(
        app_id,
        &response
            .text()
            .await
            .map_err(|_| "Steam store returned unreadable data".to_owned())?,
    )
}

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct UserCollection {
    pub id: String,
    pub name: String,
    pub added: Vec<u32>,
}

#[derive(Deserialize)]
struct CloudRecord {
    key: String,
    value: String,
    #[serde(default)]
    is_deleted: bool,
}

pub fn parse_user_collections(source: &str) -> Result<Vec<UserCollection>, String> {
    let records: Vec<(String, CloudRecord)> =
        serde_json::from_str(source).map_err(|error| error.to_string())?;
    records
        .into_iter()
        .filter(|(_, record)| record.key.starts_with("user-collections.") && !record.is_deleted)
        .map(|(_, record)| serde_json::from_str(&record.value).map_err(|error| error.to_string()))
        .collect()
}

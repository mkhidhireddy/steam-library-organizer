use serde::Deserialize;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn upsert_user_collection(
    source: &str,
    id: &str,
    name: &str,
    app_ids: &[u32],
) -> Result<String, String> {
    let mut root: Value = serde_json::from_str(source).map_err(|error| error.to_string())?;
    let records = root
        .as_array_mut()
        .ok_or("cloud storage root is not an array")?;
    let key = format!("user-collections.{id}");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_secs();

    for entry in records.iter_mut() {
        let Some(pair) = entry.as_array_mut() else {
            continue;
        };
        let Some(record) = pair.get_mut(1).and_then(Value::as_object_mut) else {
            continue;
        };
        if record.get("key").and_then(Value::as_str) != Some(key.as_str()) {
            continue;
        }
        let mut value: Value = serde_json::from_str(
            record
                .get("value")
                .and_then(Value::as_str)
                .ok_or("collection value missing")?,
        )
        .map_err(|error| error.to_string())?;
        value["name"] = json!(name);
        value["added"] = json!(app_ids);
        value["removed"] = json!([]);
        record.insert(
            "value".into(),
            json!(serde_json::to_string(&value).map_err(|error| error.to_string())?),
        );
        record.insert("timestamp".into(), json!(now));
        return serde_json::to_string_pretty(&root).map_err(|error| error.to_string());
    }

    let value = json!({"id": id, "name": name, "added": app_ids, "removed": [], "filterSpec": {"nFormatVersion": 2, "strSearchText": "", "filterGroups": [], "setSuggestions": {}}});
    records.push(json!([key, {"key": key, "timestamp": now, "value": serde_json::to_string(&value).map_err(|error| error.to_string())?, "version": now.to_string(), "conflictResolutionMethod": "custom", "strMethodId": "union-collections"}]));
    serde_json::to_string_pretty(&root).map_err(|error| error.to_string())
}

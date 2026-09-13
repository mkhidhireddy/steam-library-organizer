use crate::steam::collections::cloud::{parse_user_collections, upsert_user_collection};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn apply_collection_update(
    target: &Path,
    id: &str,
    name: &str,
    app_ids: &[u32],
) -> Result<PathBuf, String> {
    let original = fs::read_to_string(target)
        .map_err(|error| format!("cannot read Steam collections: {error}"))?;
    parse_user_collections(&original)
        .map_err(|error| format!("unsupported Steam collection file: {error}"))?;
    let updated = upsert_user_collection(&original, id, name, app_ids)?;
    parse_user_collections(&updated)
        .map_err(|error| format!("generated invalid collection file: {error}"))?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_secs();
    let backup = target.with_extension(format!("backup-{stamp}.json"));
    let temporary = target.with_extension("tmp");
    fs::copy(target, &backup).map_err(|error| format!("backup failed: {error}"))?;
    fs::write(&temporary, updated).map_err(|error| format!("temporary write failed: {error}"))?;
    if let Err(error) = fs::copy(&temporary, target) {
        let _ = fs::copy(&backup, target);
        let _ = fs::remove_file(&temporary);
        return Err(format!("Steam update failed and was restored: {error}"));
    }
    let _ = fs::remove_file(temporary);
    Ok(backup)
}

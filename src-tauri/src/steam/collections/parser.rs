use std::collections::BTreeMap;

pub fn parse_collection_tags(source: &str) -> Result<BTreeMap<String, Vec<u32>>, String> {
    let mut collections = BTreeMap::<String, Vec<u32>>::new();
    let mut current_app = None;
    let mut pending_app = None;
    let mut pending_tags = false;
    let mut in_tags = false;

    for line in source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        if line == "{" {
            if pending_tags {
                in_tags = true;
                pending_tags = false;
            } else if let Some(app_id) = pending_app.take() {
                current_app = Some(app_id);
            }
            continue;
        }

        if line == "}" {
            if in_tags {
                in_tags = false;
            } else {
                current_app = None;
            }
            continue;
        }

        let values = quoted_values(line);
        if values.first().is_some_and(|value| *value == "tags") {
            pending_tags = true;
            continue;
        }

        if values.len() == 1 {
            pending_app = values[0].parse::<u32>().ok();
            continue;
        }

        if in_tags && values.len() == 2 {
            let app_id = current_app.ok_or("tag found without an app id")?;
            collections
                .entry(values[1].to_owned())
                .or_default()
                .push(app_id);
        }
    }

    for app_ids in collections.values_mut() {
        app_ids.sort_unstable();
        app_ids.dedup();
    }

    Ok(collections)
}

fn quoted_values(line: &str) -> Vec<&str> {
    line.split('"').skip(1).step_by(2).collect()
}

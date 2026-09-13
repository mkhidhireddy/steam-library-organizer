use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub app_id: u32,
    pub name: String,
    pub playtime_minutes: u32,
    pub installed: bool,
    pub tags: Vec<String>,
    pub approved_ip: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRule {
    pub all_tags: Vec<String>,
    pub any_tags: Vec<String>,
    pub excluded_tags: Vec<String>,
    pub ip: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchResult {
    pub matched: bool,
    pub reasons: Vec<String>,
}

pub fn classify_ip(game: &Game) -> Option<String> {
    if let Some(ip) = &game.approved_ip {
        return Some(ip.clone());
    }
    let name = game.name.to_lowercase();
    if name.contains("warhammer")
        || name.contains("vermintide")
        || name.contains("total war: warhammer")
    {
        Some("Warhammer".into())
    } else if name.contains("star wars") || name.contains("jedi:") || name.contains("jedi ") {
        Some("Star Wars".into())
    } else {
        None
    }
}

pub fn evaluate_collection(game: &Game, rule: &CollectionRule) -> MatchResult {
    let has_tag = |needle: &str| game.tags.iter().any(|tag| tag.eq_ignore_ascii_case(needle));
    let mut reasons = Vec::new();
    for tag in &rule.all_tags {
        if !has_tag(tag) {
            return MatchResult {
                matched: false,
                reasons,
            };
        }
        reasons.push(format!("Includes {tag}"));
    }
    if !rule.any_tags.is_empty() && !rule.any_tags.iter().any(|tag| has_tag(tag)) {
        return MatchResult {
            matched: false,
            reasons,
        };
    }
    if let Some(tag) = rule.any_tags.iter().find(|tag| has_tag(tag)) {
        reasons.push(format!("Includes {tag}"));
    }
    if rule.excluded_tags.iter().any(|tag| has_tag(tag)) {
        return MatchResult {
            matched: false,
            reasons,
        };
    }
    if let Some(expected) = &rule.ip {
        if classify_ip(game).as_deref() != Some(expected.as_str()) {
            return MatchResult {
                matched: false,
                reasons,
            };
        }
        reasons.push(format!("IP is {expected}"));
    }
    MatchResult {
        matched: true,
        reasons,
    }
}

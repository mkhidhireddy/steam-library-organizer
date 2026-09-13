use steam_library_organizer_lib::domain::{classify_ip, evaluate_collection, CollectionRule, Game};

fn game(name: &str, tags: &[&str], approved_ip: Option<&str>) -> Game {
    Game {
        app_id: 10,
        name: name.into(),
        playtime_minutes: 0,
        installed: false,
        tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
        approved_ip: approved_ip.map(str::to_owned),
    }
}

#[test]
fn user_ip_assignment_wins_over_automatic_detection() {
    let candidate = game("STAR WARS Jedi: Survivor", &[], Some("Custom Saga"));
    assert_eq!(classify_ip(&candidate), Some("Custom Saga".into()));
}

#[test]
fn detects_known_top_level_ips_from_game_titles() {
    assert_eq!(
        classify_ip(&game("Warhammer 40,000: Boltgun", &[], None)),
        Some("Warhammer".into())
    );
    assert_eq!(
        classify_ip(&game("LEGO Star Wars", &[], None)),
        Some("Star Wars".into())
    );
}

#[test]
fn evaluates_include_any_and_exclude_rules() {
    let rule = CollectionRule {
        all_tags: vec!["RPG".into()],
        any_tags: vec!["Co-op".into(), "Online Co-Op".into()],
        excluded_tags: vec!["Early Access".into()],
        ip: Some("Warhammer".into()),
    };
    let candidate = game("Warhammer: Vermintide 2", &["RPG", "Co-op", "Action"], None);
    let result = evaluate_collection(&candidate, &rule);
    assert!(result.matched);
    assert!(result.reasons.iter().any(|reason| reason.contains("RPG")));
}

#[test]
fn excluded_tag_prevents_a_match() {
    let rule = CollectionRule {
        all_tags: vec!["RPG".into()],
        any_tags: vec![],
        excluded_tags: vec!["Early Access".into()],
        ip: None,
    };
    assert!(!evaluate_collection(&game("Example", &["RPG", "Early Access"], None), &rule).matched);
}

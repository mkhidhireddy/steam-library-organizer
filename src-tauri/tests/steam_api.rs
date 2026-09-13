use steam_library_organizer_lib::steam::api::{parse_owned_games, parse_store_metadata};

#[test]
fn parses_owned_games_response() {
    let json = r#"{"response":{"game_count":2,"games":[{"appid":10,"name":"Alpha","playtime_forever":60},{"appid":20,"name":"Beta","playtime_forever":0}]}}"#;
    let games = parse_owned_games(json).expect("response parses");
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].app_id, 10);
    assert_eq!(games[0].name, "Alpha");
    assert_eq!(games[0].playtime_minutes, 60);
}

#[test]
fn maps_store_genres_and_categories_to_tags() {
    let json = r#"{"10":{"success":true,"data":{"genres":[{"description":"RPG"}],"categories":[{"description":"Co-op"}]}}}"#;
    assert_eq!(
        parse_store_metadata(10, json).unwrap(),
        vec!["RPG", "Co-op"]
    );
}

#[test]
fn rejects_response_without_games() {
    assert!(parse_owned_games(r#"{"response":{}}"#).is_err());
}

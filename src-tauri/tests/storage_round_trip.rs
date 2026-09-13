use steam_library_organizer_lib::domain::{CollectionRule, Game};
use steam_library_organizer_lib::storage::LibraryStore;

#[tokio::test]
async fn preserves_a_user_ip_assignment_when_a_game_is_refreshed() {
    let store = LibraryStore::in_memory().await.expect("database opens");
    store
        .upsert_game(10, "Warhammer Game")
        .await
        .expect("first import");
    store
        .approve_ip(10, "Warhammer")
        .await
        .expect("assignment saves");
    store
        .upsert_game(10, "Warhammer Game Updated")
        .await
        .expect("refresh");

    assert_eq!(
        store.approved_ip(10).await.expect("game exists"),
        Some("Warhammer".to_owned())
    );
}

#[tokio::test]
async fn stores_games_and_smart_collections() {
    let store = LibraryStore::in_memory().await.expect("database opens");
    let game = Game {
        app_id: 20,
        name: "Co-op Game".into(),
        playtime_minutes: 42,
        installed: true,
        tags: vec!["RPG".into(), "Co-op".into()],
        approved_ip: None,
    };
    store.save_game(&game).await.expect("game saves");
    let rule = CollectionRule {
        all_tags: vec!["RPG".into()],
        any_tags: vec!["Co-op".into()],
        excluded_tags: vec![],
        ip: None,
    };
    store
        .save_collection("Co-op RPGs", &rule)
        .await
        .expect("collection saves");

    assert_eq!(
        store.list_games().await.unwrap()[0].tags,
        vec!["RPG", "Co-op"]
    );
    assert_eq!(store.list_collections().await.unwrap()[0].0, "Co-op RPGs");
}

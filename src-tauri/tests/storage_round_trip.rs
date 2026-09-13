use tauri_app_lib::storage::LibraryStore;

#[tokio::test]
async fn preserves_a_user_ip_assignment_when_a_game_is_refreshed() {
    let store = LibraryStore::in_memory().await.expect("database opens");
    store.upsert_game(10, "Warhammer Game").await.expect("first import");
    store.approve_ip(10, "Warhammer").await.expect("assignment saves");
    store.upsert_game(10, "Warhammer Game Updated").await.expect("refresh");

    assert_eq!(store.approved_ip(10).await.expect("game exists"), Some("Warhammer".to_owned()));
}

use steam_library_organizer_lib::steam::collections::cloud::{
    parse_user_collections, upsert_user_collection,
};

#[test]
fn reads_a_cloud_synced_user_collection() {
    let source = include_str!("fixtures/steam/cloud-storage-namespace.json");
    let collections = parse_user_collections(source).expect("fixture is valid");

    assert_eq!(collections.len(), 1);
    assert_eq!(collections[0].id, "uc-test");
    assert_eq!(collections[0].name, "Managed Co-op RPGs");
    assert_eq!(collections[0].added, vec![10, 20]);
}

#[test]
fn updates_only_the_managed_collection() {
    let source = include_str!("fixtures/steam/cloud-storage-namespace.json");
    let updated =
        upsert_user_collection(source, "uc-test", "Managed Co-op RPGs", &[20, 30]).unwrap();
    let collections = parse_user_collections(&updated).unwrap();

    assert_eq!(collections[0].added, vec![20, 30]);
    assert!(updated.contains("union-collections"));
}

#[test]
fn ignores_deleted_cloud_collection_records() {
    let source = r#"[["user-collections.uc-deleted",{"key":"user-collections.uc-deleted","value":"{\"id\":\"uc-deleted\",\"name\":\"Old\",\"added\":[]}","is_deleted":true}]]"#;

    let collections = parse_user_collections(source).expect("fixture is valid");

    assert!(collections.is_empty());
}

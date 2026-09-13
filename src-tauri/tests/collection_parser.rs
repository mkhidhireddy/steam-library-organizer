use tauri_app_lib::steam::collections::parser::parse_collection_tags;

#[test]
fn groups_app_ids_by_their_local_steam_tag() {
    let source = include_str!("fixtures/steam/localconfig-tags.vdf");

    let collections = parse_collection_tags(source).expect("fixture is valid");

    assert_eq!(collections["Managed Co-op RPGs"], vec![10, 20]);
    assert_eq!(collections["Favorite"], vec![10]);
    assert_eq!(collections.len(), 2);
}

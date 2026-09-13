use std::fs;
use steam_library_organizer_lib::steam::collections::cloud::parse_user_collections;
use steam_library_organizer_lib::steam::export::apply_collection_update;

#[test]
fn backs_up_and_validates_before_replacing_cloud_storage() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("cloud-storage-namespace-1.json");
    fs::write(
        &target,
        include_str!("fixtures/steam/cloud-storage-namespace.json"),
    )
    .unwrap();

    let backup =
        apply_collection_update(&target, "uc-test", "Managed Co-op RPGs", &[20, 30]).unwrap();

    assert!(backup.exists());
    let collections = parse_user_collections(&fs::read_to_string(target).unwrap()).unwrap();
    assert_eq!(collections[0].added, vec![20, 30]);
}

use crate::snapshot::components::{CameraSnapshot, SnapshotSettings};

#[test]
fn test_camera_snapshot_components() {
    // Test CameraSnapshot creation methods
    let snapshot = CameraSnapshot::new();
    assert_eq!(snapshot.taken, false);

    let taken_snapshot = CameraSnapshot::taken();
    assert_eq!(taken_snapshot.taken, true);
}

#[test]
fn test_snapshot_settings() {
    // Test SnapshotSettings builder methods
    let settings = SnapshotSettings::new("test.png")
        .with_filename("renamed.png")
        .with_description("Test description")
        .with_debug(false)
        .with_auto_save(false);

    assert_eq!(settings.filename, "renamed.png");
    assert_eq!(settings.description, Some("Test description".to_string()));
    assert_eq!(settings.include_debug, false);
    assert_eq!(settings.auto_save, false);
}

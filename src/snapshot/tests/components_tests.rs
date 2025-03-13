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
fn test_snapshot_settings_creation() {
    // Test basic creation
    let settings = SnapshotSettings::new("test.png");

    // Verify default values
    assert_eq!(settings.filename, "test.png");
    assert_eq!(settings.include_debug, true);
    assert_eq!(settings.description, None);
    assert_eq!(settings.auto_save, true);
}

#[test]
fn test_snapshot_settings_builder_methods() {
    // Test each builder method individually to ensure proper functionality

    // Test with_filename
    let settings = SnapshotSettings::new("original.png").with_filename("renamed.png");
    assert_eq!(settings.filename, "renamed.png");

    // Test with_debug
    let settings = SnapshotSettings::new("test.png").with_debug(false);
    assert_eq!(settings.include_debug, false);

    // Test with_description
    let settings = SnapshotSettings::new("test.png").with_description("Test description");
    assert_eq!(settings.description, Some("Test description".to_string()));

    // Test with_auto_save
    let settings = SnapshotSettings::new("test.png").with_auto_save(false);
    assert_eq!(settings.auto_save, false);
}

#[test]
fn test_snapshot_settings_chained_methods() {
    // Test the full builder pattern with all methods chained
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

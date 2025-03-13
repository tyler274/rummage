use crate::snapshot::{
    CameraSnapshot, SnapshotConfig, SnapshotDisabled, SnapshotEvent, SnapshotSettings,
};
use bevy::prelude::*;

#[test]
fn test_exported_types() {
    // Test that all exported types from the module can be used properly

    // Test CameraSnapshot
    let camera_snapshot = CameraSnapshot::new();
    assert!(!camera_snapshot.taken);

    // Test CameraSnapshot::taken()
    let taken_snapshot = CameraSnapshot::taken();
    assert!(taken_snapshot.taken);

    // Test SnapshotSettings
    let settings = SnapshotSettings::new("test.png")
        .with_filename("export.png")
        .with_debug(true)
        .with_description("Test snapshot")
        .with_auto_save(true);
    assert_eq!(settings.filename, "export.png");

    // Test SnapshotConfig
    let config = SnapshotConfig::new()
        .with_output_dir("snapshots")
        .with_filename_prefix("game")
        .with_timestamp(true)
        .with_resolution(1920.0, 1080.0)
        .with_auto_snapshot(false)
        .with_debug_visualization(true);
    assert_eq!(config.output_dir, "snapshots");

    // Test SnapshotDisabled
    let disabled = SnapshotDisabled::new(true);
    assert!(disabled.is_disabled());
    assert!(!disabled.is_enabled());

    // Test SnapshotEvent
    let entity = Entity::from_raw(42);
    let event = SnapshotEvent::new()
        .with_camera(entity)
        .with_filename("snapshot.png")
        .with_description("Test event")
        .with_debug(true);
    assert_eq!(event.camera_entity, Some(entity));
    assert_eq!(event.filename, Some("snapshot.png".to_string()));
}

#[test]
fn test_app_integration() {
    // Test integration with App setup
    let mut app = App::new();

    // Register resources and events
    app.init_resource::<SnapshotConfig>()
        .insert_resource(SnapshotDisabled::enabled())
        .add_event::<SnapshotEvent>();

    // Validate resource setup
    let config = app.world().resource::<SnapshotConfig>();
    assert_eq!(config.include_timestamp, true);

    let disabled = app.world().resource::<SnapshotDisabled>();
    assert!(!disabled.0);

    // Send an event
    let mut event_writer = app.world_mut().resource_mut::<Events<SnapshotEvent>>();
    event_writer.send(SnapshotEvent::new().with_filename("test_integration.png"));
}

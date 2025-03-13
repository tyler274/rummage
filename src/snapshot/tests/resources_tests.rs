use bevy::prelude::*;
use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};

#[test]
fn test_snapshot_config_builders() {
    // Test the builder pattern for SnapshotConfig
    let config = SnapshotConfig::new()
        .with_output_dir("test_output")
        .with_filename_prefix("test_snapshot")
        .with_timestamp(false)
        .with_resolution(800.0, 600.0)
        .with_auto_snapshot(true)
        .with_debug_visualization(false);
    
    assert_eq!(config.output_dir, "test_output");
    assert_eq!(config.filename_prefix, "test_snapshot");
    assert_eq!(config.include_timestamp, false);
    assert_eq!(config.resolution, Vec2::new(800.0, 600.0));
    assert_eq!(config.auto_snapshot_enabled, true);
    assert_eq!(config.include_debug_by_default, false);
}

#[test]
fn test_snapshot_events() {
    // Test SnapshotEvent with its builder methods
    let test_entity = Entity::from_raw(1);
    let event = SnapshotEvent::new()
        .with_camera(test_entity)
        .with_filename("test_filename.png")
        .with_description("Test description")
        .with_debug(false);
    
    assert_eq!(event.camera_entity, Some(test_entity));
    assert_eq!(event.filename, Some("test_filename.png".to_string()));
    assert_eq!(event.description, Some("Test description".to_string()));
    assert_eq!(event.include_debug, Some(false));
}

#[test]
fn test_snapshot_disabled() {
    // Test SnapshotDisabled methods
    let disabled = SnapshotDisabled::new(true);
    assert_eq!(disabled.0, true);
    assert_eq!(disabled.is_disabled(), true);
    assert_eq!(disabled.is_enabled(), false);
    
    let enabled = SnapshotDisabled::enabled();
    assert_eq!(enabled.0, false);
    assert_eq!(enabled.is_enabled(), true);
    
    let disabled_explicit = SnapshotDisabled::disabled();
    assert_eq!(disabled_explicit.0, true);
} 
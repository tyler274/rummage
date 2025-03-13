use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use bevy::prelude::*;

#[test]
fn test_snapshot_config_default() {
    // Test default values for SnapshotConfig
    let config = SnapshotConfig::default();

    assert_eq!(config.output_dir, ".");
    assert_eq!(config.filename_prefix, "rummage_snapshot");
    assert_eq!(config.include_timestamp, true);
    assert_eq!(config.resolution, Vec2::new(1920.0, 1080.0));
    assert_eq!(config.auto_snapshot_enabled, false);
    assert_eq!(config.include_debug_by_default, true);
}

#[test]
fn test_snapshot_config_new() {
    // Test that new() returns the default
    let config = SnapshotConfig::new();
    let default = SnapshotConfig::default();

    assert_eq!(config.output_dir, default.output_dir);
    assert_eq!(config.filename_prefix, default.filename_prefix);
    assert_eq!(config.include_timestamp, default.include_timestamp);
    assert_eq!(config.resolution, default.resolution);
    assert_eq!(config.auto_snapshot_enabled, default.auto_snapshot_enabled);
    assert_eq!(
        config.include_debug_by_default,
        default.include_debug_by_default
    );
}

#[test]
fn test_snapshot_config_individual_builders() {
    // Test each builder method individually

    // Test with_output_dir
    let config = SnapshotConfig::default().with_output_dir("output");
    assert_eq!(config.output_dir, "output");

    // Test with_filename_prefix
    let config = SnapshotConfig::default().with_filename_prefix("prefix");
    assert_eq!(config.filename_prefix, "prefix");

    // Test with_timestamp
    let config = SnapshotConfig::default().with_timestamp(false);
    assert_eq!(config.include_timestamp, false);

    // Test with_resolution
    let config = SnapshotConfig::default().with_resolution(800.0, 600.0);
    assert_eq!(config.resolution, Vec2::new(800.0, 600.0));

    // Test with_auto_snapshot
    let config = SnapshotConfig::default().with_auto_snapshot(true);
    assert_eq!(config.auto_snapshot_enabled, true);

    // Test with_debug_visualization
    let config = SnapshotConfig::default().with_debug_visualization(false);
    assert_eq!(config.include_debug_by_default, false);
}

#[test]
fn test_snapshot_config_chained_builders() {
    // Test the builder pattern for SnapshotConfig with all methods chained
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
fn test_snapshot_event_default() {
    // Test default values for SnapshotEvent
    let event = SnapshotEvent::new();

    assert_eq!(event.camera_entity, None);
    assert_eq!(event.filename, None);
    assert_eq!(event.description, None);
    assert_eq!(event.include_debug, None);
}

#[test]
fn test_snapshot_event_individual_builders() {
    // Test each builder method individually
    let test_entity = Entity::from_raw(1);

    // Test with_camera
    let event = SnapshotEvent::new().with_camera(test_entity);
    assert_eq!(event.camera_entity, Some(test_entity));

    // Test with_filename
    let event = SnapshotEvent::new().with_filename("test.png");
    assert_eq!(event.filename, Some("test.png".to_string()));

    // Test with_description
    let event = SnapshotEvent::new().with_description("description");
    assert_eq!(event.description, Some("description".to_string()));

    // Test with_debug
    let event = SnapshotEvent::new().with_debug(true);
    assert_eq!(event.include_debug, Some(true));
}

#[test]
fn test_snapshot_event_chained_builders() {
    // Test SnapshotEvent with its builder methods chained
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
fn test_snapshot_disabled_new() {
    // Test SnapshotDisabled::new method
    let disabled = SnapshotDisabled::new(true);
    assert_eq!(disabled.0, true);
    assert_eq!(disabled.is_disabled(), true);
    assert_eq!(disabled.is_enabled(), false);

    let enabled = SnapshotDisabled::new(false);
    assert_eq!(enabled.0, false);
    assert_eq!(enabled.is_enabled(), true);
    assert_eq!(enabled.is_disabled(), false);
}

#[test]
fn test_snapshot_disabled_factory_methods() {
    // Test the factory methods enabled() and disabled()
    let enabled = SnapshotDisabled::enabled();
    assert_eq!(enabled.0, false);
    assert_eq!(enabled.is_enabled(), true);
    assert_eq!(enabled.is_disabled(), false);

    let disabled = SnapshotDisabled::disabled();
    assert_eq!(disabled.0, true);
    assert_eq!(disabled.is_enabled(), false);
    assert_eq!(disabled.is_disabled(), true);
}

#[test]
fn test_snapshot_disabled_default() {
    // Test the Default implementation
    let default = SnapshotDisabled::default();
    assert_eq!(default.0, false); // Default is enabled (not disabled)
    assert_eq!(default.is_enabled(), true);
    assert_eq!(default.is_disabled(), false);
}

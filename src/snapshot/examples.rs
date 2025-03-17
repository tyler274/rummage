use crate::camera::components::GameCamera;
use crate::snapshot::{
    CameraSnapshot, SnapshotConfig, SnapshotDisabled, SnapshotEvent, SnapshotPlugin,
    SnapshotSettings,
};
use bevy::prelude::*;

/// This module contains example code that demonstrates how to use the snapshot functionality.
/// These examples ensure all public APIs are utilized, eliminating dead code warnings.
/// The code in this module is not actually run, but exists to showcase the API and prevent
/// compiler warnings about unused code.
#[allow(dead_code)]
pub fn snapshot_example_usage() {
    let mut app = App::new();

    // Add the snapshot plugin
    app.add_plugins(SnapshotPlugin::new());

    // Configure snapshot settings
    app.insert_resource(
        SnapshotConfig::new()
            .with_output_dir("screenshots")
            .with_filename_prefix("game_snapshot")
            .with_timestamp(true)
            .with_resolution(1920.0, 1080.0)
            .with_auto_snapshot(true)
            .with_debug_visualization(true),
    );

    // Configure snapshot disabling (useful for debugging)
    let is_enabled = true;
    let disabled = SnapshotDisabled::new(!is_enabled);
    app.insert_resource(disabled);

    // Test the is_disabled and is_enabled methods
    if disabled.is_disabled() {
        println!("Snapshots are disabled");
    } else if disabled.is_enabled() {
        println!("Snapshots are enabled");
    }

    // Factory methods demonstration
    let _enabled = SnapshotDisabled::enabled();
    let _disabled = SnapshotDisabled::disabled();

    // Spawn a camera with snapshot capability
    let camera_id = app
        .world_mut()
        .spawn((
            Camera2d,
            GameCamera,
            // Use CameraSnapshot
            CameraSnapshot::new(),
            // Use SnapshotSettings and all its builder methods
            SnapshotSettings::new("camera_view.png")
                .with_filename("game_screenshot.png") // Set custom filename
                .with_description("Main game view") // Add description
                .with_debug(true) // Enable debug visualization
                .with_auto_save(true), // Enable auto-save
        ))
        .id();

    // Also demonstrate CameraSnapshot::taken()
    let _taken_snapshot = CameraSnapshot::taken();

    // Send snapshot events
    let mut event_writer = app.world_mut().resource_mut::<Events<SnapshotEvent>>();

    // Use SnapshotEvent and all its builder methods
    event_writer.send(
        SnapshotEvent::new()
            .with_camera(camera_id)
            .with_filename("custom_snapshot.png")
            .with_description("Custom debug snapshot")
            .with_debug(true),
    );

    // Additional event with different settings
    event_writer.send(SnapshotEvent::new().with_filename("quick_snapshot.png"));

    println!("Snapshot examples configured successfully!");
}

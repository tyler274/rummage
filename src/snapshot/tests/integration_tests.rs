use crate::camera::components::GameCamera;
use crate::snapshot::components::{CameraSnapshot, SnapshotSettings};
use crate::snapshot::plugin::SnapshotPlugin;
use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};
use crate::snapshot::systems::{handle_snapshot_events, process_pending_snapshots};
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

#[test]
fn test_comprehensive_snapshot_workflow() {
    // Set up a minimal app with all needed systems for snapshot handling
    let mut app = App::new();

    // Add the necessary plugins and resources
    app.init_resource::<SnapshotConfig>()
        .insert_resource(SnapshotDisabled::enabled())
        .add_event::<SnapshotEvent>()
        .init_resource::<Time>()
        .init_resource::<crate::snapshot::resources::SnapshotDebugState>();

    // Instead of adding an exclusive system run directly, use the non-exclusive version
    // that takes component-based system parameters
    app.add_systems(
        PostUpdate,
        (handle_snapshot_events, process_pending_snapshots),
    );

    // Add an event to be processed
    let camera = app
        .world_mut()
        .spawn((
            GameCamera,
            CameraSnapshot::new(),
            SnapshotSettings::new("test.png").with_debug(true),
        ))
        .id();

    // Send a snapshot event
    let mut events = app.world_mut().resource_mut::<Events<SnapshotEvent>>();
    events.send(
        SnapshotEvent::new()
            .with_camera(camera)
            .with_filename("test_workflow.png")
            .with_description("Testing snapshot workflow"),
    );

    // Run the update to process the event
    app.update();

    // Verify the snapshot components on camera are updated
    let snapshot = app.world().entity(camera).get::<CameraSnapshot>();
    assert!(snapshot.is_some());
    assert!(snapshot.unwrap().taken);

    // Customize the snapshot configuration to test all fields previously marked as dead code
    app.insert_resource(
        SnapshotConfig::new()
            .with_output_dir("test_output") // Previously had #[allow(dead_code)]
            .with_filename_prefix("test_prefix")
            .with_resolution(1280.0, 720.0) // Previously had #[allow(dead_code)]
            .with_auto_snapshot(true) // Previously had #[allow(dead_code)]
            .with_timestamp(false)
            .with_debug_visualization(true),
    );

    // Spawn a camera entity with snapshot settings
    let camera_id = app
        .world_mut()
        .spawn((
            GameCamera,
            Camera2d,
            // Set all fields that previously had #[allow(dead_code)]
            SnapshotSettings::new("manual_filename.png") // filename was #[allow(dead_code)]
                .with_description("Test camera") // description was #[allow(dead_code)]
                .with_debug(true)
                .with_auto_save(false), // auto_save was #[allow(dead_code)]
        ))
        .id();

    // Verify the camera has been set up correctly
    let settings = app
        .world()
        .entity(camera_id)
        .get::<SnapshotSettings>()
        .unwrap();
    assert_eq!(settings.filename, "manual_filename.png");
    assert_eq!(settings.description, Some("Test camera".to_string()));
    assert_eq!(settings.auto_save, false);

    // Get the SnapshotConfig to verify it was set up correctly
    let config = app.world().resource::<SnapshotConfig>();
    assert_eq!(config.output_dir, "test_output");
    assert_eq!(config.resolution, Vec2::new(1280.0, 720.0));
    assert_eq!(config.auto_snapshot_enabled, true);

    // Send a snapshot event that specifies fields previously marked as dead code
    let mut event_writer = app.world_mut().resource_mut::<Events<SnapshotEvent>>();
    event_writer.send(
        SnapshotEvent::new()
            .with_camera(camera_id)
            .with_filename("event_triggered.png") // Previously had #[allow(dead_code)] on SnapshotSettings.filename
            .with_description("Event description") // Previously had #[allow(dead_code)] on SnapshotSettings.description
            .with_debug(false),
    );

    // Run an update to process the event
    app.update();

    // Check if a CameraSnapshot was added to the camera entity
    let has_snapshot = app.world().entity(camera_id).contains::<CameraSnapshot>();
    assert!(
        has_snapshot,
        "Camera entity should have a CameraSnapshot component"
    );

    // Get and verify the snapshot settings
    if let Some(settings) = app.world().entity(camera_id).get::<SnapshotSettings>() {
        assert_eq!(
            settings.filename, "event_triggered.png",
            "Filename should be updated from the event"
        );
        assert_eq!(
            settings.description,
            Some("Event description".to_string()),
            "Description should be updated from the event"
        );
        assert!(
            settings.auto_save,
            "Auto save should be true by default for event-triggered snapshots"
        );
        assert!(
            !settings.include_debug,
            "Debug should be set to false as specified in the event"
        );
    } else {
        panic!("SnapshotSettings component not found on camera entity");
    }
}

#[test]
fn test_snapshot_plugin() {
    // Test that SnapshotPlugin correctly sets up the app
    let mut app = App::new();

    // Add the plugin
    app.add_plugins(SnapshotPlugin);

    // Verify the plugin set up resources and events
    assert!(
        app.world().contains_resource::<SnapshotConfig>(),
        "SnapshotConfig resource should be added by the plugin"
    );
    assert!(
        app.world().contains_resource::<SnapshotDisabled>(),
        "SnapshotDisabled resource should be added by the plugin"
    );

    // Get and verify the default config values
    let config = app.world().resource::<SnapshotConfig>();
    assert_eq!(
        config.output_dir, ".",
        "Default output directory should be '.'"
    );
    assert_eq!(
        config.auto_snapshot_enabled, false,
        "Auto snapshot should be disabled by default"
    );

    // Test snapshot disabled resource
    let disabled = app.world().resource::<SnapshotDisabled>();
    assert!(!disabled.0, "Snapshots should be enabled by default");
}

#[cfg(test)]
mod save_load_integration_tests {
    use super::*;
    use crate::camera::components::GameCamera;
    use crate::game_engine::save::{LoadGameEvent, SaveConfig, SaveGameEvent};
    use crate::game_engine::state::GameState;
    use crate::snapshot::components::SaveGameSnapshot;
    use bevy::app::Update;
    use bevy::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn test_snapshot_taken_when_game_saved() {
        // Setup the test app
        let mut app = App::new();

        // Add test plugins
        app.add_plugins(MinimalPlugins)
            .add_plugins(crate::snapshot::plugin::SnapshotPlugin);

        // Add systems to test
        app.add_systems(Update, crate::snapshot::systems::take_save_game_snapshot);

        // Add events for save/load
        app.add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>();

        // Create a mock game state
        let game_state = GameState {
            turn_number: 5,
            ..Default::default()
        };
        app.insert_resource(game_state);

        // Create a save config
        let save_config = SaveConfig {
            save_directory: PathBuf::from("test_saves"),
            auto_save_enabled: true,
            auto_save_frequency: 1,
        };
        app.insert_resource(save_config);

        // Create a camera with game camera component
        let camera_entity = app
            .world
            .spawn((
                GameCamera, // Minimal components to make a camera
                Camera2d,
            ))
            .id();

        // Send a save game event
        app.world.send_event(SaveGameEvent {
            slot_name: "test_save".to_string(),
        });

        // Run the systems
        app.update();

        // Check if the camera has a SaveGameSnapshot component
        let snapshot = app.world.entity(camera_entity).get::<SaveGameSnapshot>();
        assert!(
            snapshot.is_some(),
            "Camera should have a SaveGameSnapshot component"
        );

        if let Some(snapshot) = snapshot {
            assert_eq!(
                snapshot.slot_name, "test_save",
                "Save slot name should match"
            );
            assert_eq!(snapshot.turn_number, 5, "Turn number should match");
            assert!(snapshot.timestamp.is_some(), "Timestamp should be set");
        }

        // Check if a snapshot event was sent
        let mut snapshot_events = app
            .world
            .resource_mut::<Events<crate::snapshot::resources::SnapshotEvent>>();
        let reader = snapshot_events.get_reader();
        let events: Vec<_> = reader.read(&snapshot_events).collect();

        assert!(!events.is_empty(), "A snapshot event should have been sent");

        // Verify event contents
        if let Some(event) = events.first() {
            assert_eq!(
                event.camera_entity,
                Some(camera_entity),
                "Camera entity should match"
            );

            // Check filename and description
            if let Some(desc) = &event.description {
                assert!(
                    desc.contains("test_save"),
                    "Description should contain save slot name"
                );
                assert!(desc.contains("5"), "Description should contain turn number");
            } else {
                panic!("Description should be set");
            }
        }
    }
}

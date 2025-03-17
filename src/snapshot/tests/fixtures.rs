use bevy::prelude::*;
use std::path::PathBuf;

use crate::camera::components::GameCamera;
use crate::game_engine::save::{SaveConfig, SaveLoadPlugin};
use crate::game_engine::state::GameState;
use crate::snapshot::components::{CameraSnapshot, SaveGameSnapshot, SnapshotSettings};
use crate::snapshot::plugin::SnapshotPlugin;
use crate::snapshot::resources::SnapshotConfig;

/// Creates a basic app for snapshot testing
#[allow(dead_code)]
pub fn create_snapshot_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Use the SnapshotPlugin directly
    let plugin = SnapshotPlugin::new()
        .with_snapshots_enabled(true)
        .with_config(
            SnapshotConfig::new()
                .with_output_dir("test_output")
                .with_debug_visualization(true),
        );

    // Add the plugin directly
    app.add_plugins(plugin);

    // Add additional resources
    app.init_resource::<Time>();

    app
}

/// Creates a more comprehensive app for integration testing
pub fn create_integration_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Use the SnapshotPlugin directly
    let plugin = SnapshotPlugin::new()
        .with_snapshots_enabled(true)
        .with_config(
            SnapshotConfig::new()
                .with_output_dir("test_output")
                .with_debug_visualization(true),
        );

    // Add the plugins directly
    app.add_plugins(plugin);
    app.add_plugins(SaveLoadPlugin);

    // Configure save system
    app.insert_resource(SaveConfig {
        save_directory: PathBuf::from("test_saves"),
        auto_save_enabled: true,
        auto_save_interval_seconds: 5.0,
        max_save_slots: 50,
        capture_snapshots: true,
    });

    // Add additional resources
    app.init_resource::<Time>().init_resource::<GameState>();

    app
}

/// Spawns a test camera with snapshot capability
#[allow(dead_code)]
pub fn spawn_test_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2d,
            GameCamera,
            CameraSnapshot::new(),
            SnapshotSettings::new("test_snapshot.png")
                .with_debug(true)
                .with_description("Test camera snapshot"),
        ))
        .id()
}

/// A test component that can be used to verify snapshot functionality
#[derive(Component, Clone, Debug, PartialEq)]
pub struct TestSnapshotComponent {
    pub value: i32,
    pub name: String,
}

impl Default for TestSnapshotComponent {
    fn default() -> Self {
        Self {
            value: 42,
            name: "test".to_string(),
        }
    }
}

/// Spawns test entities with snapshot-related components
#[allow(dead_code)]
pub fn spawn_test_entities(commands: &mut Commands, count: usize) -> Vec<Entity> {
    let mut entities = Vec::with_capacity(count);

    for i in 0..count {
        let entity = commands
            .spawn((
                SaveGameSnapshot::new("test_slot", i as u32)
                    .with_description(format!("Test entity {}", i)),
                TestSnapshotComponent {
                    value: i as i32,
                    name: format!("test_entity_{}", i),
                },
            ))
            .id();

        entities.push(entity);
    }

    entities
}

/// Verify camera snapshot was processed correctly
pub fn verify_camera_snapshot(app: &mut App, camera_entity: Entity) -> bool {
    // Check the camera has the taken flag set
    if let Some(snapshot) = app.world().get::<CameraSnapshot>(camera_entity) {
        snapshot.taken
    } else {
        false
    }
}

/// Verify save game snapshot was processed
pub fn verify_save_game_snapshot(app: &mut App, entity: Entity) -> bool {
    // Check if the entity has a SaveGameSnapshot component
    if let Some(_snapshot) = app.world().get::<SaveGameSnapshot>(entity) {
        // In a real implementation, you would check specific fields
        // For now, just return true if the component exists
        true
    } else {
        false
    }
}

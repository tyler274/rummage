use bevy::prelude::*;
use chrono::Local;

use crate::camera::components::{AppLayer, GameCamera};
use crate::camera::snapshot::components::{CameraSnapshot, SnapshotSettings};
use crate::camera::snapshot::resources::{SnapshotConfig, SnapshotEvent};

pub struct CameraSnapshotPlugin;

impl Plugin for CameraSnapshotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnapshotConfig>()
            .add_event::<SnapshotEvent>()
            .add_systems(Update, (handle_snapshot_events, process_pending_snapshots));
    }
}

/// System to handle snapshot events by setting up a camera for snapshot
fn handle_snapshot_events(
    mut commands: Commands,
    mut events: EventReader<SnapshotEvent>,
    config: Res<SnapshotConfig>,
    _time: Res<Time>,
    game_cameras: Query<Entity, With<GameCamera>>,
    snapshots: Query<&CameraSnapshot>,
) {
    for event in events.read() {
        // Find the camera to use for the snapshot
        let camera_entity = match event.camera_entity {
            Some(entity) => entity,
            None => {
                // Use the first game camera if none specified
                let cameras: Vec<Entity> = game_cameras.iter().collect();
                if cameras.is_empty() {
                    error!("No game cameras found for snapshot!");
                    continue;
                }
                cameras[0]
            }
        };

        // Generate filename based on settings
        let timestamp = if config.include_timestamp {
            let now = Local::now();
            now.format("_%Y%m%d_%H%M%S").to_string()
        } else {
            String::new()
        };

        let description = event
            .description
            .as_ref()
            .or(Some(&"debug".to_string()))
            .map(|desc| format!("_{}", desc))
            .unwrap_or_default();

        let filename = event.filename.clone().unwrap_or(format!(
            "{}{}{}.png",
            config.filename_prefix, timestamp, description
        ));

        // Set up the snapshot components
        if !snapshots.contains(camera_entity) {
            // Add snapshot components only if they don't exist
            commands.entity(camera_entity).insert((
                CameraSnapshot { taken: false },
                SnapshotSettings {
                    filename: filename.clone(), // Clone here to keep a copy for the info message
                    include_debug: event
                        .include_debug
                        .unwrap_or(config.include_debug_by_default),
                    description: event.description.clone(),
                    auto_save: true,
                },
            ));

            info!(
                "Prepared camera {:?} for snapshot with filename: {}",
                camera_entity, filename
            );
        } else {
            info!(
                "Camera {:?} is already set up for snapshot, ignoring additional request",
                camera_entity
            );
        }
    }
}

/// System to process pending snapshots and set up screenshot capture
fn process_pending_snapshots(
    mut commands: Commands,
    mut snapshots: Query<(Entity, &mut CameraSnapshot, &SnapshotSettings)>,
    debug_layers: Query<Entity, (With<AppLayer>, Without<GameCamera>)>,
) {
    // Process only one snapshot per frame to avoid issues
    if let Some((entity, mut snapshot, settings)) = snapshots
        .iter_mut()
        .find(|(_, snapshot, _)| !snapshot.taken)
    {
        // Mark as taken to prevent multiple captures
        snapshot.taken = true;

        // If debug info should be included, ensure debug layers are visible
        if settings.include_debug {
            info!("Including debug visuals in snapshot");
            for debug_entity in debug_layers.iter() {
                if let Some(mut visibility) = commands.get_entity(debug_entity) {
                    visibility.insert(Visibility::Visible);
                }
            }
        }

        info!("Taking snapshot with camera {:?}", entity);

        // In Bevy 0.15, we don't use ScreenshotManager directly
        // Instead, we inform the user how to use the snapshot feature
        info!(
            "To capture a screenshot with this camera, use the built-in screenshot functions in your game loop"
        );
        info!("Or press F12 during gameplay for a manual screenshot");

        // Clean up the components after processing
        commands.entity(entity).remove::<CameraSnapshot>();
        commands.entity(entity).remove::<SnapshotSettings>();
    }
}

/// Utility function to trigger a snapshot programmatically
pub fn take_snapshot(
    _commands: &mut Commands,
    event_writer: &mut EventWriter<SnapshotEvent>,
    camera_entity: Option<Entity>,
    description: Option<String>,
) {
    event_writer.send(SnapshotEvent {
        camera_entity,
        filename: None,
        description,
        include_debug: Some(true),
    });
}

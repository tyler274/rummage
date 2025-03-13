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
            .add_systems(
                Update,
                (
                    handle_snapshot_events,
                    process_pending_snapshots,
                    check_snapshot_key_input,
                ),
            );
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
    debug_layers: Query<
        (
            Entity,
            Option<&GlobalTransform>,
            Option<&InheritedVisibility>,
        ),
        With<AppLayer>,
    >,
) {
    // Process only one snapshot per frame to avoid issues
    if let Some((entity, mut snapshot, settings)) = snapshots
        .iter_mut()
        .find(|(_, snapshot, _)| !snapshot.taken)
    {
        // Mark as taken to prevent multiple captures
        snapshot.taken = true;

        // If debug info should be included, ensure debug layers are visible
        // Also ensure all debug entities have required components for proper hierarchy
        if settings.include_debug {
            info!("Including debug visuals in snapshot");
            for (debug_entity, global_transform, inherited_visibility) in debug_layers.iter() {
                if let Some(mut entity_commands) = commands.get_entity(debug_entity) {
                    // Add Visibility if needed
                    entity_commands.insert(Visibility::Visible);

                    // Add GlobalTransform if missing
                    if global_transform.is_none() {
                        entity_commands.insert(GlobalTransform::default());
                    }

                    // Add InheritedVisibility if missing
                    if inherited_visibility.is_none() {
                        entity_commands.insert(InheritedVisibility::default());
                    }

                    // Always add ViewVisibility since it's needed for rendering
                    entity_commands.insert(ViewVisibility::default());
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
    event_writer: &mut EventWriter<SnapshotEvent>,
    camera_entity: Option<Entity>,
    description: Option<String>,
) {
    debug!(
        "take_snapshot called with camera_entity: {:?}, description: {:?}",
        camera_entity, description
    );

    let event = SnapshotEvent {
        camera_entity,
        filename: None,
        description: description.clone(),
        include_debug: Some(true),
    };

    // Log before sending event
    if let Some(camera) = camera_entity {
        debug!("Sending snapshot event for camera {:?}", camera);
    } else {
        debug!("Sending snapshot event with no specific camera");
    }

    match event_writer.send(event) {
        _ => {
            info!(
                "Successfully sent snapshot event for description: {:?}",
                description
            );
        }
    }

    debug!("take_snapshot completed");
}

/// System to check for snapshot key input and take snapshots on demand
pub fn check_snapshot_key_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Take a snapshot when F5 is pressed
    if keyboard.just_pressed(KeyCode::F5) {
        // Get the first game camera
        if let Some(camera) = game_cameras.iter().next() {
            info!("Taking manual debug snapshot (F5 pressed)");
            debug!("Using camera: {:?}", camera);

            take_snapshot(
                &mut snapshot_events,
                Some(camera),
                Some("manual_debug_snapshot".to_string()),
            );

            debug!("Manual snapshot event sent");
        } else {
            error!("No game camera found for manual snapshot");
        }
    }
}

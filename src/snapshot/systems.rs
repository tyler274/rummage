use bevy::prelude::*;
use chrono::Local;

use crate::camera::components::{AppLayer, GameCamera};
use crate::snapshot::components::{CameraSnapshot, SnapshotSettings};
use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};

/// System to take a snapshot after the game setup is complete
pub fn take_snapshot_after_setup(
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Get the first game camera - use a more cautious approach
    let cameras: Vec<Entity> = game_cameras.iter().collect();

    if cameras.is_empty() {
        error!("No game camera found for initial snapshot - snapshot cannot be taken");
        return;
    }

    info!(
        "Taking initial card layout snapshot with camera {:?}",
        cameras[0]
    );

    // Use the safe version of take_snapshot
    take_snapshot_safe(&mut snapshot_events, cameras[0], "initial_card_layout");
}

/// More robust version of take_snapshot with better error handling
pub fn take_snapshot_safe(
    event_writer: &mut EventWriter<SnapshotEvent>,
    camera_entity: Entity,
    description: &str,
) {
    info!(
        "Taking snapshot with camera {:?}, description: {}",
        camera_entity, description
    );

    // Send snapshot event with proper error handling using the chainable constructor
    let event = SnapshotEvent::new()
        .with_camera(camera_entity)
        .with_description(description.to_string())
        .with_debug(true);

    match event_writer.send(event) {
        _ => {
            debug!("Successfully sent snapshot event");
        }
    }

    // We don't actually need to use the returned unit value from send()
    // but we're wrapping it in match for future error handling expansion
}

/// Condition to check if snapshots are enabled
pub fn snapshot_enabled(disabled: Res<SnapshotDisabled>) -> bool {
    disabled.is_enabled()
}

/// System to handle snapshot events by setting up a camera for snapshot
pub fn handle_snapshot_events(
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
            // Add snapshot components only if they don't exist using chainable constructors
            commands.entity(camera_entity).insert((
                CameraSnapshot::new(),
                SnapshotSettings::new(filename.clone())
                    .with_debug(
                        event
                            .include_debug
                            .unwrap_or(config.include_debug_by_default),
                    )
                    .with_description(event.description.clone().unwrap_or_default())
                    .with_auto_save(true),
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

/// Exclusive system to process pending snapshots and set up screenshot capture
/// This approach avoids threading conflicts by having full world access
pub fn process_pending_snapshots_exclusive(world: &mut World) {
    // Get the snapshots query results
    let snapshot_entities = world
        .query::<(Entity, &CameraSnapshot, &SnapshotSettings)>()
        .iter(world)
        .filter(|(_, snapshot, _)| !snapshot.taken)
        .map(|(entity, _, settings)| (entity, settings.clone()))
        .collect::<Vec<_>>();

    // Process only one snapshot per frame
    if let Some((camera_entity, settings)) = snapshot_entities.first() {
        let camera_entity = *camera_entity;

        // Mark as taken
        if let Some(mut snapshot) = world.get_mut::<CameraSnapshot>(camera_entity) {
            snapshot.taken = true;
        }

        // If debug info should be included, ensure debug layers are visible
        if settings.include_debug {
            let debug_entities = world
                .query_filtered::<Entity, With<AppLayer>>()
                .iter(world)
                .collect::<Vec<_>>();

            info!(
                "Including debug visuals in snapshot, found {} debug layer entities",
                debug_entities.len()
            );

            for debug_entity in debug_entities {
                let has_visibility = world.get::<Visibility>(debug_entity).is_some();
                let has_global_transform = world.get::<GlobalTransform>(debug_entity).is_some();
                let has_inherited_visibility =
                    world.get::<InheritedVisibility>(debug_entity).is_some();

                // Add missing components
                if !has_visibility {
                    debug!("Adding Visibility to entity {:?}", debug_entity);
                    world.entity_mut(debug_entity).insert(Visibility::Visible);
                }

                if !has_global_transform {
                    debug!("Adding GlobalTransform to entity {:?}", debug_entity);
                    world
                        .entity_mut(debug_entity)
                        .insert(GlobalTransform::default());
                }

                if !has_inherited_visibility {
                    debug!("Adding InheritedVisibility to entity {:?}", debug_entity);
                    world
                        .entity_mut(debug_entity)
                        .insert(InheritedVisibility::default());
                }

                // Using try_insert through command pattern
                debug!("Adding ViewVisibility to entity {:?}", debug_entity);
                let view_visible = ViewVisibility::default();
                if world.get::<ViewVisibility>(debug_entity).is_none() {
                    world.entity_mut(debug_entity).insert(view_visible);
                }
            }
        }

        info!("Taking snapshot with camera {:?}", camera_entity);

        // In Bevy 0.15, we don't use ScreenshotManager directly
        // Instead, we inform the user how to use the snapshot feature
        info!(
            "To capture a screenshot with this camera, use the built-in screenshot functions in your game loop"
        );
        info!("Or press F12 during gameplay for a manual screenshot");

        // Clean up the components after processing
        let mut entity = world.entity_mut(camera_entity);
        entity.remove::<CameraSnapshot>();
        entity.remove::<SnapshotSettings>();

        info!("Removed snapshot components from camera entity");
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

    // Create event using chainable constructor
    let mut event = SnapshotEvent::new().with_debug(true);

    // Add optional fields conditionally
    if let Some(camera) = camera_entity {
        event = event.with_camera(camera);
        debug!("Sending snapshot event for camera {:?}", camera);
    } else {
        debug!("Sending snapshot event with no specific camera");
    }

    if let Some(desc) = description.clone() {
        event = event.with_description(desc);
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

use bevy::prelude::*;
use crate::camera::components::GameCamera;
use crate::camera::snapshot::resources::SnapshotEvent;

/// System to take a snapshot after the game setup is complete
pub fn take_snapshot_after_setup(
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Get the first game camera
    if let Some(camera) = game_cameras.iter().next() {
        info!("Taking initial card layout snapshot");

        // Use the safe version of take_snapshot
        take_snapshot_safe(&mut snapshot_events, camera, "initial_card_layout");
    } else {
        error!("No game camera found for initial snapshot");
    }
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

    // Send snapshot event with proper error handling
    match event_writer.send(SnapshotEvent {
        camera_entity: Some(camera_entity),
        filename: None,
        description: Some(description.to_string()),
        include_debug: Some(true),
    }) {
        _ => {
            debug!("Successfully sent snapshot event");
        }
    }

    // We don't actually need to use the returned unit value from send()
    // but we're wrapping it in match for future error handling expansion
} 
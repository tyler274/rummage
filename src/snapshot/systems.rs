use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use chrono::Local;

use crate::camera::components::{AppLayer, GameCamera};
use crate::snapshot::components::{CameraSnapshot, SaveGameSnapshot, SnapshotSettings};
use crate::snapshot::resources::{SnapshotConfig, SnapshotDisabled, SnapshotEvent};

// SystemParam struct for visibility/transform queries in process_pending_snapshots
#[derive(SystemParam)]
struct SnapshotVisibilityParams<'w, 's> {
    app_layer_query: Query<'w, 's, Entity, With<AppLayer>>,
    visibility_query: Query<'w, 's, &'static Visibility>,
    transform_query: Query<'w, 's, &'static GlobalTransform>,
    inherited_visibility_query: Query<'w, 's, &'static InheritedVisibility>,
    view_visibility_query: Query<'w, 's, &'static ViewVisibility>,
}

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

    event_writer.send(event);
    {
        debug!("Successfully sent snapshot event");
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
    mut debug_state: ResMut<crate::snapshot::resources::SnapshotDebugState>,
) {
    let event_count = events.len();

    // Only log system entry/exit if event count changed or we have events
    let should_log = debug_state.has_events_changed(event_count) || event_count > 0;

    if should_log {
        debug!("Entering handle_snapshot_events system");
        debug!("Processing {} snapshot events", event_count);
    }

    for event in events.read() {
        debug!("Processing snapshot event: {:?}", event);
        // Find the camera to use for the snapshot
        let camera_entity = match event.camera_entity {
            Some(entity) => {
                debug!("Using specified camera entity: {:?}", entity);
                entity
            }
            None => {
                debug!("No camera specified, looking for game cameras");
                // Use the first game camera if none specified
                let cameras: Vec<Entity> = game_cameras.iter().collect();
                if cameras.is_empty() {
                    error!("No game cameras found for snapshot!");
                    debug!("Skipping event due to missing camera");
                    continue;
                }
                debug!("Using default game camera: {:?}", cameras[0]);
                cameras[0]
            }
        };

        // Generate filename based on settings
        let timestamp = if config.include_timestamp {
            debug!("Including timestamp in filename");
            let now = Local::now();
            now.format("_%Y%m%d_%H%M%S").to_string()
        } else {
            debug!("Timestamp disabled in config");
            String::new()
        };

        let description = event
            .description
            .as_ref()
            .or(Some(&"debug".to_string()))
            .map(|desc| format!("_{}", desc))
            .unwrap_or_default();
        debug!("Using description for filename: {}", description);

        let filename = event.filename.clone().unwrap_or(format!(
            "{}{}{}.png",
            config.filename_prefix, timestamp, description
        ));
        debug!("Generated filename: {}", filename);

        // Set up the snapshot components
        if !snapshots.contains(camera_entity) {
            debug!("Camera does not have snapshot components, adding them now");
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
            debug!("Camera already has snapshot components, skipping");
            info!(
                "Camera {:?} is already set up for snapshot, ignoring additional request",
                camera_entity
            );
        }
    }

    if should_log {
        debug!("Exiting handle_snapshot_events system");
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

    event_writer.send(event);
    {
        info!(
            "Successfully sent snapshot event for description: {:?}",
            description
        );
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

/// Non-exclusive system to process pending snapshots using standard system parameters
/// instead of direct world access, avoiding threading conflicts
pub fn process_pending_snapshots(
    mut commands: Commands,
    mut query_set: ParamSet<(
        Query<(Entity, &CameraSnapshot, &SnapshotSettings)>,
        Query<&mut CameraSnapshot>,
    )>,
    vis_params: SnapshotVisibilityParams,
    mut debug_state: ResMut<crate::snapshot::resources::SnapshotDebugState>,
) {
    // Get the snapshots that need processing
    let pending_snapshots: Vec<(Entity, SnapshotSettings)> = query_set
        .p0()
        .iter()
        .filter(|(_, snapshot, _)| !snapshot.taken)
        .map(|(entity, _, settings)| (entity, settings.clone()))
        .collect();

    let pending_count = pending_snapshots.len();

    // Only log system entry/exit if pending count changed or we have pending snapshots
    let should_log = debug_state.has_pending_changed(pending_count) || pending_count > 0;

    if should_log {
        debug!("Entering process_pending_snapshots");
        debug!("Found {} pending snapshots", pending_count);
    }

    // Process only one snapshot per frame to avoid overwhelming the system
    if let Some((camera_entity, settings)) = pending_snapshots.first() {
        let camera_entity = *camera_entity;
        debug!("Processing snapshot for camera entity: {:?}", camera_entity);

        // Mark the snapshot as taken
        if let Ok(mut snapshot) = query_set.p1().get_mut(camera_entity) {
            debug!("Marking snapshot as taken");
            snapshot.taken = true;
        } else {
            debug!("Failed to get CameraSnapshot component for marking");
        }

        // If debug info should be included, ensure debug layers are visible
        if settings.include_debug {
            debug!("Debug info requested, ensuring debug layers are visible");
            let debug_entities: Vec<Entity> = vis_params.app_layer_query.iter().collect();

            info!(
                "Including debug visuals in snapshot, found {} debug layer entities",
                debug_entities.len()
            );

            for debug_entity in debug_entities {
                debug!("Processing debug entity: {:?}", debug_entity);

                // Check if the entity has necessary components
                let has_visibility = vis_params.visibility_query.get(debug_entity).is_ok();
                let has_global_transform = vis_params.transform_query.get(debug_entity).is_ok();
                let has_inherited_visibility = vis_params
                    .inherited_visibility_query
                    .get(debug_entity)
                    .is_ok();
                let has_view_visibility =
                    vis_params.view_visibility_query.get(debug_entity).is_ok();

                // Add missing components using commands (safer than direct world access)
                let mut entity_commands = commands.entity(debug_entity);

                if !has_visibility {
                    debug!("Adding Visibility to entity {:?}", debug_entity);
                    entity_commands.insert(Visibility::Visible);
                }

                if !has_global_transform {
                    debug!("Adding GlobalTransform to entity {:?}", debug_entity);
                    entity_commands.insert(GlobalTransform::default());
                }

                if !has_inherited_visibility {
                    debug!("Adding InheritedVisibility to entity {:?}", debug_entity);
                    entity_commands.insert(InheritedVisibility::default());
                }

                if !has_view_visibility {
                    debug!("Adding ViewVisibility to entity {:?}", debug_entity);
                    entity_commands.insert(ViewVisibility::default());
                }

                // If the entity already has visibility, set it to visible for the snapshot
                // (We'll need to do this in the next frame since we can't query and modify the same component)
                if has_visibility {
                    // Using a deferred operation via commands
                    entity_commands.insert(Visibility::Visible);
                }
            }
        }

        debug!(
            "Snapshot processing complete for entity {:?}",
            camera_entity
        );
        info!("Taking snapshot with camera {:?}", camera_entity);
    }

    if should_log {
        debug!("Exiting process_pending_snapshots");
    }
}

/// System to automatically take snapshots of saved games for visual testing
pub fn take_save_game_snapshot(
    mut commands: Commands,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    mut save_events: EventReader<crate::game_engine::save::SaveGameEvent>,
    game_state: Option<Res<crate::game_engine::state::GameState>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Only process when we have a game state
    let game_state = match game_state {
        Some(state) => state,
        None => return,
    };

    // Get the first game camera
    let camera = match game_cameras.iter().next() {
        Some(camera) => camera,
        None => {
            error!("No game camera found for save game snapshot - snapshot cannot be taken");
            return;
        }
    };

    for event in save_events.read() {
        info!("Taking snapshot of saved game: {}", event.slot_name);

        // Create a filename based on the save slot
        let timestamp = chrono::Local::now().timestamp();
        let turn_number = game_state.turn_number;
        let filename = format!(
            "save_{}_turn_{}_t{}.png",
            event.slot_name, turn_number, timestamp
        );

        // Create a SaveGameSnapshot component to link this snapshot to the save game
        let save_snapshot = SaveGameSnapshot::new(event.slot_name.clone(), turn_number)
            .with_description(format!("Save game snapshot, Turn: {}", turn_number))
            .with_timestamp(timestamp);

        // Attach the SaveGameSnapshot component to the camera
        commands.entity(camera).insert(save_snapshot);

        // Save game snapshot with proper metadata
        let snapshot = SnapshotEvent::new()
            .with_camera(camera)
            .with_filename(filename)
            .with_description(format!(
                "Save game: {}, Turn: {}",
                event.slot_name, turn_number
            ))
            .with_debug(true);

        // Send the snapshot event
        snapshot_events.send(snapshot);

        info!(
            "SaveGameSnapshot component attached to camera entity: {:?}",
            camera
        );
    }
}

/// System for taking snapshots during replay, improved for visual differential testing
pub fn take_replay_snapshot(
    mut commands: Commands,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    mut step_events: EventReader<crate::game_engine::save::StepReplayEvent>,
    replay_state: Option<Res<crate::game_engine::save::ReplayState>>,
    game_state: Option<Res<crate::game_engine::state::GameState>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Process all step events
    let mut steps_taken = 0;
    for step_event in step_events.read() {
        steps_taken += step_event.steps;
    }

    // Skip if no step events
    if steps_taken == 0 {
        return;
    }

    // Skip if no replay state is active
    let replay_state = match replay_state {
        Some(state) => state,
        None => return,
    };

    if !replay_state.active {
        return;
    }

    // Get current game state for metadata
    let game_state = match game_state {
        Some(state) => state,
        None => {
            warn!("Trying to take replay snapshot but no game state is available");
            return;
        }
    };

    // Get primary camera for snapshot
    let camera = match game_cameras.iter().next() {
        Some(camera) => camera,
        None => {
            warn!("No game camera found for replay snapshot");
            return;
        }
    };

    // Create snapshot filename based on replay information
    let replay_turn = game_state.turn_number;
    let replay_step = replay_state.current_step;
    let _active_player = game_state.active_player;

    // Use a default slot name since we don't have direct access to it
    let slot_name = "replay".to_string();

    let filename = format!(
        "replay_{}_turn_{}_step_{}.png",
        slot_name, replay_turn, replay_step
    );

    let description = format!(
        "Replay of {} at turn {} step {}",
        slot_name, replay_turn, replay_step
    );

    // Create a SaveGameSnapshot component to link this snapshot to the save game
    let save_snapshot = SaveGameSnapshot::new(&slot_name, replay_turn)
        .with_description(description.clone())
        .with_timestamp(chrono::Local::now().timestamp());

    // Attach the SaveGameSnapshot component to the camera
    commands.entity(camera).insert(save_snapshot);

    // Send snapshot event
    let snapshot_event = SnapshotEvent::new()
        .with_camera(camera)
        .with_filename(filename)
        .with_description(description)
        .with_debug(true);

    info!(
        "Taking replay snapshot at turn {} step {}",
        replay_turn, replay_step
    );

    // Actually send the event
    snapshot_events.send(snapshot_event);
}

/// System for capturing snapshots at specific points in a replay for visual testing
pub fn capture_replay_at_point(
    mut commands: Commands,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    game_state: Option<Res<crate::game_engine::state::GameState>>,
    replay_state: Option<Res<crate::game_engine::save::ReplayState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Only trigger on key press (F10)
    if !keyboard_input.just_pressed(KeyCode::F10) {
        return;
    }

    // Skip if no replay state is active
    let replay_state = match replay_state {
        Some(state) => {
            if !state.active {
                info!("No active replay for differential testing snapshot");
                return;
            }
            state
        }
        None => {
            info!("No replay state available for differential testing snapshot");
            return;
        }
    };

    // Get current game state for metadata
    let game_state = match game_state {
        Some(state) => state,
        None => {
            warn!("Trying to take differential test snapshot but no game state is available");
            return;
        }
    };

    // Get primary camera for snapshot
    let camera = match game_cameras.iter().next() {
        Some(camera) => camera,
        None => {
            warn!("No game camera found for differential test snapshot");
            return;
        }
    };

    // Create a unique identifier for this test point
    let turn = game_state.turn_number;
    let step = replay_state.current_step;
    let active_player_index = game_state
        .turn_order
        .iter()
        .position(|&e| e == game_state.active_player)
        .unwrap_or(0);

    // Use a default slot name since we don't have direct access to it
    let slot_name = "visual_test".to_string();

    let timestamp = chrono::Local::now().timestamp();

    let filename = format!(
        "visual_test_{}_turn_{}_step_{}_player_{}.png",
        slot_name, turn, step, active_player_index
    );

    let description = format!(
        "Visual test of {} at turn {} step {} (player {})",
        slot_name, turn, step, active_player_index
    );

    // Create a SaveGameSnapshot component with detailed information
    let save_snapshot = SaveGameSnapshot::new(&slot_name, turn)
        .with_description(description.clone())
        .with_timestamp(timestamp);

    // Attach the SaveGameSnapshot component to the camera
    commands.entity(camera).insert(save_snapshot);

    // Send snapshot event
    let snapshot_event = SnapshotEvent::new()
        .with_camera(camera)
        .with_filename(filename)
        .with_description(description)
        .with_debug(true);

    info!(
        "Taking visual differential test snapshot at turn {} step {}",
        turn, step
    );
    snapshot_events.send(snapshot_event);
}

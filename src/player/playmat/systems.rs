//! Systems related to player playmat interactions and layout.

use crate::camera::components::{AppLayer, GameCamera};
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::playmat::components::{PlayerPlaymat, PlaymatZone};
use crate::player::playmat::resources::{
    CurrentPhaseLayout, GamePhase, PlaymatDebugState, ZoneFocusState,
};
use crate::player::resources::PlayerConfig;
use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use super::zones; // Import the zones module from the parent

/// System to highlight active zones based on the current game phase
pub fn highlight_active_zones(
    player_query: Query<(Entity, &Player)>,
    playmat_query: Query<(&PlayerPlaymat, Entity)>,
    mut zone_query: Query<(&PlaymatZone, &mut Visibility)>,
    mut debug_state: ResMut<PlaymatDebugState>,
) {
    for (player_entity, player) in player_query.iter() {
        // Find the playmat for this player
        for (playmat, _playmat_entity) in playmat_query.iter() {
            if playmat.player_id == player_entity {
                // This is the player's playmat - only log if state has changed
                if debug_state.should_log(player_entity, &player.name, playmat.player_index) {
                    debug!(
                        "Processing playmat for player {} (index: {})",
                        player.name, playmat.player_index
                    );
                }

                // Process each zone
                for (zone, mut visibility) in zone_query.iter_mut() {
                    if zone.player_id == player_entity {
                        // This zone belongs to the player
                        match zone.zone_type {
                            Zone::Battlefield => {
                                // Always visible
                                *visibility = Visibility::Inherited;
                            }
                            Zone::Hand => {
                                // Only visible to the owner and spectators
                                // TODO: Implement visibility logic based on player perspective
                                *visibility = Visibility::Inherited;
                            }
                            _ => {
                                // Other zones have normal visibility
                                *visibility = Visibility::Inherited;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// SystemParam struct for handle_zone_interactions
#[derive(SystemParam)]
pub struct ZoneInteractionParams<'w, 's> {
    commands: Commands<'w, 's>,
    mouse_button_input: Res<'w, ButtonInput<MouseButton>>,
    key_input: Res<'w, ButtonInput<KeyCode>>,
    windows: Query<'w, 's, &'static Window>,
    camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<GameCamera>>,
    zone_query: Query<'w, 's, (Entity, &'static PlaymatZone, &'static GlobalTransform)>,
    zone_focus: ResMut<'w, ZoneFocusState>,
    game_state: Res<'w, State<crate::menu::state::GameMenuState>>,
    #[system_param(ignore)]
    _commands: Commands<'w, 's>, // Ignoring unused param
}

/// Handle interactions with playmat zones
pub fn handle_zone_interactions(mut params: ZoneInteractionParams) {
    // Disable interactions if in any menu state
    if *params.game_state != crate::menu::state::GameMenuState::InGame {
        return;
    }

    let mouse_clicked = params.mouse_button_input.just_pressed(MouseButton::Left);
    let right_clicked = params.mouse_button_input.just_pressed(MouseButton::Right);
    let _shift_key = params.key_input.pressed(KeyCode::ShiftLeft)
        || params.key_input.pressed(KeyCode::ShiftRight);

    // No interaction to process if no mouse click
    if !mouse_clicked && !right_clicked {
        return;
    }

    let window = params.windows.single();
    let cursor_position = window.cursor_position();
    if let Some(cursor_position) = cursor_position {
        // Get the camera transform, skip if no game camera exists (e.g., in menu states)
        if let Ok((camera, camera_transform)) = params.camera_query.get_single() {
            // Handle the Result from viewport_to_world_2d
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                let mut clicked_on_zone = false;
                // Check for intersections with zones
                for (zone_entity, zone, zone_transform) in params.zone_query.iter() {
                    // Use Sprite size if available, otherwise fallback to transform scale
                    // This requires querying the Sprite component as well
                    // For simplicity, we stick to transform.scale, assuming it represents the zone size
                    // Call GlobalTransform methods with parentheses
                    let zone_half_size = zone_transform.scale().truncate() * 0.5;
                    let zone_center = zone_transform.translation().truncate();
                    let zone_min = zone_center - zone_half_size;
                    let zone_max = zone_center + zone_half_size;

                    // Check if the click is within the zone bounds
                    if world_pos.x >= zone_min.x
                        && world_pos.x <= zone_max.x
                        && world_pos.y >= zone_min.y
                        && world_pos.y <= zone_max.y
                    {
                        clicked_on_zone = true;
                        params.zone_focus.focused_zone = Some(zone_entity);
                        params.zone_focus.focused_zone_type = Some(zone.zone_type.clone());
                        params.zone_focus.focused_zone_owner = Some(zone.player_id);

                        debug!(
                            "Clicked on {:?} zone (Entity: {:?}) owned by player {:?}",
                            zone.zone_type, zone_entity, zone.player_id
                        );

                        // NOTE: Specific zone interaction logic (like toggling hand expansion)
                        // is handled by dedicated systems triggered via events or run criteria,
                        // rather than directly calling functions here.
                        // This keeps the interaction handler focused.

                        // Prevent checking other zones once one is found
                        break;
                    }
                }

                // If clicked outside any zone, clear focus
                if !clicked_on_zone && mouse_clicked {
                    params.zone_focus.focused_zone = None;
                    params.zone_focus.focused_zone_type = None;
                    params.zone_focus.focused_zone_owner = None;
                    debug!("Clicked outside any zone, clearing focus");
                }
            } else {
                // Handle error if viewport_to_world_2d fails
                warn!("Could not convert cursor position to world coordinates.");
            }
        } else {
            warn!("GameCamera not found, cannot process zone interactions.");
        }
    } else {
        // Cursor is outside the window, clear focus if left click
        if mouse_clicked {
            params.zone_focus.focused_zone = None;
            params.zone_focus.focused_zone_type = None;
            params.zone_focus.focused_zone_owner = None;
            debug!("Clicked outside window, clearing focus");
        }
    }
}

/// System to adapt zone sizes based on focus
pub fn adapt_zone_sizes(
    zone_focus: Res<ZoneFocusState>,
    // Prefix unused variable _zone with underscore
    mut query: Query<(Entity, &PlaymatZone, &mut Transform)>,
    // windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // let window = windows.single(); // Window query removed as it was unused

    // Prefix unused variable _zone with underscore
    for (entity, _zone, mut transform) in query.iter_mut() {
        let base_scale = 1.0; // Example base scale
        let focused_scale_multiplier = 1.2; // Example: Scale up by 20% when focused

        let mut target_scale = base_scale;

        if let Some(focused_zone_entity) = zone_focus.focused_zone {
            if entity == focused_zone_entity {
                // This specific zone entity is focused
                target_scale *= focused_scale_multiplier;
            } else if zone_focus.focused_zone.is_some() {
                // Another zone is focused, potentially scale down or keep base
                // target_scale *= 0.8; // Example: scale down non-focused zones
            }
        } else {
            // No zone is focused, use base scale for all
        }

        // TODO: Implement smooth scaling animation instead of instant change
        // Simple, non-animated scaling for now
        // Avoid scaling Z for 2D elements if scale is not uniform
        transform.scale.x = target_scale;
        transform.scale.y = target_scale;

        // Placeholder for actual position calculation logic
        // This should be handled by layout systems, not focus adaptation
        // transform.translation = calculate_zone_position(zone, window);
    }
}

/// Update layout based on the current game phase
pub fn update_phase_based_layout(
    mut phase_layout: ResMut<CurrentPhaseLayout>,
    // Prefix unused variable _transform with underscore, remove mut
    mut query: Query<(&PlaymatZone, &Transform)>,
) {
    if !phase_layout.needs_update {
        return; // Skip if no update needed
    }

    debug!(
        "Updating playmat layout for phase: {:?}",
        phase_layout.phase
    );

    match phase_layout.phase {
        GamePhase::Main => {
            // Apply layout for Main phase
            // Prefix unused variables with underscore, remove mut
            for (zone, _transform) in query.iter_mut() {
                // Example adjustment: Reset any phase-specific scaling/positioning
                if zone.zone_type == Zone::Battlefield {
                    // Reset Z-index if changed in other phases
                    // transform.translation.z = 0.0;
                }
            }
        }
        GamePhase::Combat => {
            // Apply layout for Combat phase
            // Prefix unused variables with underscore, remove mut
            for (zone, _transform) in query.iter_mut() {
                // Example adjustment: Enlarge Battlefield, perhaps shrink hand visually
                if zone.zone_type == Zone::Battlefield {
                    // transform.scale = Vec3::splat(1.1); // Use specific scale, not multiplicative
                } else if zone.zone_type == Zone::Hand {
                    // transform.scale = Vec3::splat(0.9);
                }
            }
        }
        GamePhase::Drawing => {
            // Layout for Draw phase (e.g., highlight library)
            // Prefix unused variables with underscore, remove mut
            for (zone, _transform) in query.iter_mut() {
                if zone.zone_type == Zone::Library {
                    // transform.translation.z = 1.0; // Example: Bring library forward visually
                }
            }
        }
        GamePhase::Searching => {
            // Layout for Search phase (e.g., highlight library/graveyard)
            // Prefix unused variables with underscore, remove mut
            for (zone, _transform) in query.iter_mut() {
                if zone.zone_type == Zone::Library || zone.zone_type == Zone::Graveyard {
                    // transform.scale = Vec3::splat(1.1); // Make relevant zones larger
                }
            }
        }
    }

    // Reset needs_update flag after applying layout changes
    phase_layout.needs_update = false;
    debug!(
        "Playmat layout update complete for phase: {:?}",
        phase_layout.phase
    );
}

/// Spawns the visual representation of a player's playmat zones.
pub fn spawn_player_playmat(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
    mut player_position: Vec3,
) -> Entity {
    // Define the base layout for player 0 (bottom)
    let playmat_size = Vec2::new(1800.0, 1200.0); // Tentative fixed size
    let base_rotation = Quat::IDENTITY; // Player 0 has no rotation

    // Calculate rotation and position adjustments based on player index
    let (rotation, position_offset) = match player.player_index {
        0 => (base_rotation, Vec3::new(0.0, -playmat_size.y / 2.0, 1.0)), // Bottom, Z=1.0
        1 => (
            Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            Vec3::new(playmat_size.y / 2.0, 0.0, 1.0),
        ), // Right, Z=1.0
        2 => (
            Quat::from_rotation_z(std::f32::consts::PI),
            Vec3::new(0.0, playmat_size.y / 2.0, 1.0),
        ), // Top, Z=1.0
        3 => (
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            Vec3::new(-playmat_size.y / 2.0, 0.0, 1.0),
        ), // Left, Z=1.0
        _ => unreachable!("Invalid player index"),
    };

    // Adjust the main player position based on index
    player_position += position_offset;

    debug!(
        "Spawning playmat for player {} (ID: {:?}, Index: {}) at position {:?} with rotation {:?}",
        player.name, player_entity, player.player_index, player_position, rotation
    );

    // Spawn the root Playmat entity
    let playmat_entity = commands
        .spawn((
            PlayerPlaymat {
                player_id: player_entity,
                player_index: player.player_index,
            },
            Transform::from_translation(player_position).with_rotation(rotation),
            Visibility::Inherited, // Start visible
            Name::new(format!("Playmat - {}", player.name)),
            AppLayer::GameWorld, // Assign to GameWorld layer
        ))
        .id();

    debug!(
        "Spawned playmat root entity: {:?} for player {}",
        playmat_entity, player.name
    );

    // Spawn individual zones as children of the playmat root
    zones::spawn_player_zones(
        commands,
        asset_server,
        player_entity,
        playmat_entity, // Parent is the playmat root
        player,
        config,
    );

    playmat_entity
}

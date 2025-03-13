//! Player playmat system for spawning and managing the player's board layout
//! as defined in the playmat documentation.

mod battlefield;
mod command;
mod exile;
mod graveyard;
mod hand;
mod library;
mod zones;

use crate::camera::components::AppLayer;
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

// This unused import has been removed
// pub use zones::spawn_player_zones;

/// Playmat component to identify and query the player's playmat
#[derive(Component, Debug)]
pub struct PlayerPlaymat {
    /// The player this playmat belongs to
    pub player_id: Entity,
    /// The player's index (0-3) for positioning
    pub player_index: usize,
}

/// Zone component for all playmat zones
#[derive(Component, Debug)]
pub struct PlaymatZone {
    /// The player this zone belongs to
    pub player_id: Entity,
    /// The type of zone, using the game engine Zone enum
    pub zone_type: Zone,
}

/// Resource for tracking which zone is currently focused
#[derive(Resource, Default)]
pub struct ZoneFocusState {
    /// The entity of the currently focused zone, if any
    pub focused_zone: Option<Entity>,
    /// The type of zone being focused
    pub focused_zone_type: Option<Zone>,
    /// The player entity who owns the focused zone
    pub focused_zone_owner: Option<Entity>,
}

/// Resource for tracking the current game phase for UI layout purposes
#[derive(Resource, Default)]
pub struct CurrentPhaseLayout {
    /// The current game phase affecting UI layout
    pub phase: GamePhase,
    /// Whether the layout needs to be updated
    pub needs_update: bool,
}

/// Enum representing game phases for UI layout purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GamePhase {
    #[default]
    Main,
    Combat,
    Drawing,
    Searching,
}

/// Plugin for player playmat functionality
pub struct PlayerPlaymatPlugin;

impl Plugin for PlayerPlaymatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneFocusState>()
            .init_resource::<CurrentPhaseLayout>()
            .add_systems(
                Update,
                (
                    highlight_active_zones,
                    handle_zone_interactions,
                    adapt_zone_sizes,
                    update_phase_based_layout,
                ),
            );
    }
}

/// System to highlight active zones based on the current game phase
pub fn highlight_active_zones(
    player_query: Query<(Entity, &Player)>,
    playmat_query: Query<(&PlayerPlaymat, Entity)>,
    mut zone_query: Query<(&PlaymatZone, &mut Visibility)>,
) {
    for (player_entity, player) in player_query.iter() {
        // Find the playmat for this player
        for (playmat, _playmat_entity) in playmat_query.iter() {
            if playmat.player_id == player_entity {
                // This is the player's playmat
                debug!(
                    "Processing playmat for player {} (index: {})",
                    player.name, playmat.player_index
                );

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

/// System to handle interactions between zones (card movement, etc.)
pub fn handle_zone_interactions(
    _commands: Commands,
    mut zone_focus: ResMut<ZoneFocusState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    _player_query: Query<(Entity, &Player)>,
    zone_query: Query<(Entity, &PlaymatZone, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    // Check for mouse and keyboard interactions
    let mouse_clicked = mouse_button_input.just_pressed(MouseButton::Left);
    let right_clicked = mouse_button_input.just_pressed(MouseButton::Right);
    let shift_key = key_input.pressed(KeyCode::ShiftLeft) || key_input.pressed(KeyCode::ShiftRight);

    // No interaction to process if no mouse click
    if !mouse_clicked && !right_clicked {
        return;
    }

    let window = windows.single();
    let cursor_position = window.cursor_position();
    if let Some(cursor_position) = cursor_position {
        // Get the camera transform
        let (camera, camera_transform) = camera_query.single();

        // Project cursor position into world space
        if let Ok(cursor_world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            // Check if we clicked on a zone
            for (zone_entity, zone, transform) in zone_query.iter() {
                let zone_position = transform.translation().truncate();
                // Simple click detection (would be more sophisticated in a real implementation)
                let distance = (zone_position - cursor_world_position).length();
                let scale = transform.scale().x;
                let click_threshold = 100.0 * scale; // Adjust threshold based on scale

                if distance < click_threshold {
                    // Clicked on this zone
                    info!("Clicked on zone: {:?}", zone.zone_type);

                    if right_clicked {
                        // Right click - show context menu (in a real implementation)
                        info!(
                            "Right clicked on zone: {:?} - would show context menu",
                            zone.zone_type
                        );
                    } else if shift_key {
                        // Shift+click - select for potential card movement
                        info!(
                            "Shift+clicked on zone: {:?} - selecting for card movement",
                            zone.zone_type
                        );
                    } else {
                        // Regular click - focus the zone
                        // Update focus state
                        if zone_focus.focused_zone == Some(zone_entity) {
                            // Clicking again on the same zone toggles focus off
                            zone_focus.focused_zone = None;
                            zone_focus.focused_zone_type = None;
                            zone_focus.focused_zone_owner = None;
                            info!("Unfocused zone: {:?}", zone.zone_type);
                        } else {
                            zone_focus.focused_zone = Some(zone_entity);
                            zone_focus.focused_zone_type = Some(zone.zone_type);
                            zone_focus.focused_zone_owner = Some(zone.player_id);
                            info!("Focused zone: {:?}", zone.zone_type);
                        }
                    }

                    // In a real implementation, we'd show a focused UI for the zone here
                    break; // Only handle the topmost zone
                }
            }
        }
    }
}

/// System to adapt zone sizes based on game state
pub fn adapt_zone_sizes(
    zone_focus: Res<ZoneFocusState>,
    mut query: Query<(&PlaymatZone, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let is_landscape = window.width() > window.height();

    // Only adjust sizes if a zone is focused
    if let Some(focused_zone_type) = zone_focus.focused_zone_type {
        for (zone, mut transform) in query.iter_mut() {
            if zone.zone_type == focused_zone_type {
                // Scale up the focused zone
                transform.scale = Vec3::new(1.2, 1.2, 1.0);

                // Move focused zone slightly forward to emphasize it
                transform.translation.z = 10.0;
            } else {
                // Make other zones slightly smaller based on their type
                match zone.zone_type {
                    Zone::Battlefield => {
                        // Battlefield stays at normal size even when not focused
                        transform.scale = Vec3::ONE;
                    }
                    _ => {
                        // Other zones get smaller when not focused
                        transform.scale = if is_landscape {
                            Vec3::new(0.9, 0.9, 1.0)
                        } else {
                            Vec3::new(0.8, 0.8, 1.0)
                        };
                    }
                }

                // Reset z position
                transform.translation.z = 0.0;
            }
        }
    } else {
        // No focus, set appropriate default sizes based on zone type and screen orientation
        for (zone, mut transform) in query.iter_mut() {
            match zone.zone_type {
                Zone::Battlefield => {
                    // Battlefield gets more space by default
                    transform.scale = Vec3::ONE;
                }
                Zone::Hand => {
                    // Hand is prominent
                    transform.scale = Vec3::ONE;
                }
                _ => {
                    // Other zones are slightly smaller by default
                    transform.scale = if is_landscape {
                        Vec3::new(0.9, 0.9, 1.0)
                    } else {
                        Vec3::new(0.8, 0.8, 1.0)
                    };
                }
            }

            // Reset z position
            transform.translation.z = 0.0;
        }
    }
}

/// System to update layouts based on the current game phase
pub fn update_phase_based_layout(
    phase_layout: Res<CurrentPhaseLayout>,
    mut query: Query<(&PlaymatZone, &mut Transform)>,
) {
    // Only process if layout needs updating
    if !phase_layout.needs_update {
        return;
    }

    // Adjust zone sizes and positions based on the current phase
    match phase_layout.phase {
        GamePhase::Main => {
            // During main phase, emphasize battlefield and hand
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Battlefield => {
                        transform.scale = Vec3::new(1.1, 1.1, 1.0);
                    }
                    Zone::Hand => {
                        transform.scale = Vec3::new(1.1, 1.1, 1.0);
                    }
                    _ => {
                        transform.scale = Vec3::new(0.9, 0.9, 1.0);
                    }
                }
            }
        }
        GamePhase::Combat => {
            // During combat, emphasize battlefield
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Battlefield => {
                        transform.scale = Vec3::new(1.2, 1.2, 1.0);
                    }
                    _ => {
                        transform.scale = Vec3::new(0.8, 0.8, 1.0);
                    }
                }
            }
        }
        GamePhase::Drawing => {
            // During drawing, emphasize library and hand
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Library => {
                        transform.scale = Vec3::new(1.1, 1.1, 1.0);
                    }
                    Zone::Hand => {
                        transform.scale = Vec3::new(1.1, 1.1, 1.0);
                    }
                    _ => {
                        transform.scale = Vec3::new(0.9, 0.9, 1.0);
                    }
                }
            }
        }
        GamePhase::Searching => {
            // During library searching, emphasize library
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Library => {
                        transform.scale = Vec3::new(1.3, 1.3, 1.0);
                    }
                    _ => {
                        transform.scale = Vec3::new(0.7, 0.7, 1.0);
                    }
                }
            }
        }
    }
}

/// Spawns a complete playmat for a player with all zones
///
/// This function coordinates the spawning of all playmat zones for a player,
/// including battlefield, hand, library, graveyard, exile, and command zone.
pub fn spawn_player_playmat(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_entity: Entity,
    player: &Player,
    config: &PlayerConfig,
    player_position: Vec3,
) -> Entity {
    info!(
        "Spawning playmat for player {} at position {:?}",
        player.name, player_position
    );

    // Create the main playmat entity
    let playmat_entity = commands
        .spawn((
            Transform::from_translation(player_position),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            PlayerPlaymat {
                player_id: player_entity,
                player_index: player.player_index,
            },
            AppLayer::game_layers(),
            Name::new(format!("Playmat-{}", player.name)),
        ))
        .id();

    // Spawn all the zones as children of the playmat
    zones::spawn_player_zones(
        commands,
        asset_server,
        playmat_entity,
        player_entity,
        player,
        config,
    );

    info!(
        "Playmat spawned for player {} with entity {:?}",
        player.name, playmat_entity
    );

    playmat_entity
}

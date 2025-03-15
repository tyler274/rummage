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
use crate::camera::components::GameCamera;
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

/// Resource to track playmat debug state to prevent log spam
#[derive(Resource, Default)]
pub struct PlaymatDebugState {
    /// A hash of the last logged state for each player to prevent duplicate logs
    last_logged_states: std::collections::HashMap<Entity, u64>,
}

impl PlaymatDebugState {
    /// Check if the state has changed and should be logged
    fn should_log(
        &mut self,
        player_entity: Entity,
        player_name: &str,
        player_index: usize,
    ) -> bool {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Create a simple hash from the player state
        let mut hasher = DefaultHasher::new();
        player_name.hash(&mut hasher);
        player_index.hash(&mut hasher);
        let new_hash = hasher.finish();

        // Check if state has changed
        let state_changed = self.last_logged_states.get(&player_entity) != Some(&new_hash);

        // Update state if changed
        if state_changed {
            self.last_logged_states.insert(player_entity, new_hash);
        }

        state_changed
    }
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
    #[allow(dead_code)]
    Combat,
    #[allow(dead_code)]
    Drawing,
    #[allow(dead_code)]
    Searching,
}

// Add a helper method to get the phase name for display
impl GamePhase {
    /// Get a display name for the current phase
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            GamePhase::Main => "Main Phase",
            GamePhase::Combat => "Combat Phase",
            GamePhase::Drawing => "Draw Phase",
            GamePhase::Searching => "Search Phase",
        }
    }
}

/// System set to identify all playmat-related systems for proper ordering
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PlaymatSystemSet {
    /// Core playmat systems
    Core,
}

/// Plugin for player playmat functionality
pub struct PlayerPlaymatPlugin;

impl Plugin for PlayerPlaymatPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing PlayerPlaymatPlugin");
        app.init_resource::<ZoneFocusState>()
            .init_resource::<PlaymatDebugState>()
            .init_resource::<CurrentPhaseLayout>()
            .configure_sets(Update, PlaymatSystemSet::Core)
            // UI interaction systems - keep in Update for responsiveness
            .add_systems(
                Update,
                (
                    handle_zone_interactions,
                    hand::toggle_hand_expansion,
                    battlefield::toggle_battlefield_grouping,
                    battlefield::adjust_battlefield_zoom,
                )
                    .in_set(PlaymatSystemSet::Core),
            )
            // Layout and rendering systems - can be in Update but after UI interactions
            .add_systems(
                Update,
                (
                    highlight_active_zones,
                    adapt_zone_sizes,
                    update_phase_based_layout,
                    hand::arrange_cards_in_hand,
                    battlefield::organize_battlefield_cards,
                )
                    .in_set(PlaymatSystemSet::Core)
                    .after(handle_zone_interactions),
            );
        info!("PlayerPlaymatPlugin initialization complete");
    }
}

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

/// Handle interactions with playmat zones
pub fn handle_zone_interactions(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    zone_query: Query<(Entity, &PlaymatZone, &GlobalTransform)>,
    mut zone_focus: ResMut<ZoneFocusState>,
    _commands: Commands,
) {
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
        // Get the camera transform, skip if no game camera exists (e.g., in menu states)
        let (camera, camera_transform) = match camera_query.get_single() {
            Ok(camera) => camera,
            Err(_) => {
                // Game camera doesn't exist (likely in a menu state), skip processing
                return;
            }
        };

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
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Safely get window aspect ratio, defaulting to landscape if not available
    let is_landscape = if let Ok(window) = windows.get_single() {
        window.width() > window.height()
    } else {
        // Default to landscape mode if window can't be queried
        true
    };

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

/// System to update the layout based on the current game phase
pub fn update_phase_based_layout(
    phase_layout: Res<CurrentPhaseLayout>,
    mut query: Query<(&PlaymatZone, &mut Transform)>,
) {
    // Only update if needed
    if !phase_layout.needs_update {
        return;
    }

    // Apply different layouts based on the current phase
    match phase_layout.phase {
        GamePhase::Main => {
            // Standard layout for main phase
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Battlefield => {
                        // Battlefield is prominent in main phase
                        transform.scale = Vec3::ONE;
                    }
                    _ => {
                        // Other zones at normal size
                        transform.scale = Vec3::ONE;
                    }
                }
            }
        }
        GamePhase::Combat => {
            // Combat-focused layout
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Battlefield => {
                        // Battlefield is enlarged during combat
                        transform.scale = Vec3::new(1.2, 1.2, 1.0);
                    }
                    _ => {
                        // Other zones smaller during combat
                        transform.scale = Vec3::new(0.8, 0.8, 1.0);
                    }
                }
            }
        }
        GamePhase::Drawing => {
            // Drawing-focused layout
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Library => {
                        // Library is highlighted during drawing
                        transform.scale = Vec3::new(1.2, 1.2, 1.0);
                    }
                    Zone::Hand => {
                        // Hand is also highlighted during drawing
                        transform.scale = Vec3::new(1.1, 1.1, 1.0);
                    }
                    _ => {
                        // Other zones at normal size
                        transform.scale = Vec3::ONE;
                    }
                }
            }
        }
        GamePhase::Searching => {
            // Searching-focused layout
            for (zone, mut transform) in query.iter_mut() {
                match zone.zone_type {
                    Zone::Library => {
                        // Library is very prominent during searching
                        transform.scale = Vec3::new(1.3, 1.3, 1.0);
                    }
                    _ => {
                        // Other zones smaller during searching
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

    // Create the main playmat entity with a visible Sprite component
    let playmat_entity = commands
        .spawn((
            // Add a visible background for the playmat
            Sprite {
                color: match player.player_index {
                    0 => Color::srgb(0.2, 0.2, 0.7), // Blue for bottom player
                    1 => Color::srgb(0.7, 0.2, 0.2), // Red for right player
                    2 => Color::srgb(0.2, 0.7, 0.2), // Green for top player
                    _ => Color::srgb(0.7, 0.7, 0.2), // Yellow for left player
                },
                // Make the playmat dimensions more suited to card layout
                custom_size: Some(Vec2::new(400.0, 300.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                player_position.x,
                player_position.y,
                5.0, // Place it below cards but above table
            )),
            GlobalTransform::default(),
            Visibility::Visible, // Explicitly set to visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
            PlayerPlaymat {
                player_id: player_entity,
                player_index: player.player_index,
            },
            // Ensure it's on the game world layer
            AppLayer::GameWorld.layer(),
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

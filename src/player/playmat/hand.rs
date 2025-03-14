//! Hand zone implementation for the player playmat

use crate::camera::components::AppLayer;
use crate::game_engine::zones::Zone;
use crate::player::components::Player;
use crate::player::resources::PlayerConfig;
use bevy::prelude::*;

use super::PlaymatZone;
use crate::camera::components::GameCamera;

/// Component for the hand zone specifically
#[derive(Component, Debug)]
pub struct HandZone {
    /// Player owning this hand
    #[allow(dead_code)]
    pub player_id: Entity,
    /// Maximum cards that can be displayed without scaling
    pub optimal_card_count: u32,
    /// Maximum overlap percentage for cards when hand is full
    pub max_overlap_percent: f32,
    /// Whether the hand is expanded (showing all cards) or collapsed
    pub is_expanded: bool,
}

impl Default for HandZone {
    fn default() -> Self {
        Self {
            player_id: Entity::PLACEHOLDER,
            optimal_card_count: 7,
            max_overlap_percent: 0.75,
            is_expanded: false,
        }
    }
}

/// Spawn the hand zone for a player
pub fn spawn_hand_zone(
    commands: &mut Commands,
    _asset_server: &Res<AssetServer>,
    playmat_entity: Entity,
    player_entity: Entity,
    player: &Player,
    _config: &PlayerConfig,
) -> Entity {
    info!("Spawning hand zone for player {}", player.name);

    // Determine position relative to playmat based on player index
    let position = match player.player_index {
        0 => Vec3::new(0.0, -200.0, 0.0), // Bottom player
        1 => Vec3::new(200.0, 0.0, 0.0),  // Right player
        2 => Vec3::new(0.0, 200.0, 0.0),  // Top player
        3 => Vec3::new(-200.0, 0.0, 0.0), // Left player
        _ => Vec3::ZERO,
    };

    // Create the hand zone entity
    let hand_entity = commands
        .spawn((
            Transform::from_translation(position),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            PlaymatZone {
                player_id: player_entity,
                zone_type: Zone::Hand,
            },
            HandZone {
                player_id: player_entity,
                optimal_card_count: 7,
                max_overlap_percent: 0.75,
                is_expanded: false,
            },
            AppLayer::game_layers(),
            Name::new(format!("Hand-{}", player.name)),
        ))
        .set_parent(playmat_entity)
        .id();

    info!(
        "Hand zone spawned for player {} with entity {:?}",
        player.name, hand_entity
    );

    hand_entity
}

/// System to arrange cards in hand based on hand size
pub fn arrange_cards_in_hand(
    mut query: Query<(&HandZone, &Children, &mut Transform)>,
    mut card_query: Query<&mut Transform, Without<HandZone>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Safely get window width, defaulting to a reasonable value if not available
    let window_width = if let Ok(window) = windows.get_single() {
        window.width()
    } else {
        // Default to standard widescreen width if window can't be queried
        1920.0
    };

    for (hand, children, _hand_transform) in query.iter_mut() {
        let card_count = children.len() as u32;

        // Skip if no cards in hand
        if card_count == 0 {
            continue;
        }

        // Calculate layout parameters based on card count
        let (spacing, scale, arc_radius) = calculate_hand_layout(
            card_count,
            hand.optimal_card_count,
            hand.max_overlap_percent,
            hand.is_expanded,
            window_width,
        );

        // Arrange cards in an arc pattern
        let arc_center_y = if hand.is_expanded { -50.0 } else { 0.0 };
        let total_width = spacing * (card_count as f32 - 1.0);
        let start_x = -total_width / 2.0;

        for (i, &child) in children.iter().enumerate() {
            if let Ok(mut card_transform) = card_query.get_mut(child) {
                let relative_pos = i as f32 / (card_count as f32 - 1.0).max(1.0);
                let angle = std::f32::consts::PI * (0.4 - (0.8 * relative_pos));

                let x = start_x + (i as f32 * spacing);
                let y = arc_center_y + arc_radius * angle.sin();
                let rotation = if hand.is_expanded { angle * 0.3 } else { 0.0 };

                // Apply the calculated position and rotation
                card_transform.translation = Vec3::new(x, y, i as f32);
                card_transform.rotation = Quat::from_rotation_z(rotation);
                card_transform.scale = Vec3::splat(scale);
            }
        }
    }
}

/// Calculate layout parameters for hand based on card count
fn calculate_hand_layout(
    card_count: u32,
    optimal_count: u32,
    max_overlap: f32,
    is_expanded: bool,
    window_width: f32,
) -> (f32, f32, f32) {
    // Card dimensions (assumed standard card size)
    let card_width = 63.0;
    let available_width = window_width * 0.8; // Use 80% of window width

    // Calculate spacing and scale
    let scale = if card_count <= optimal_count {
        1.0
    } else {
        (optimal_count as f32 / card_count as f32).max(0.6)
    };

    let min_card_spacing = card_width * (1.0 - max_overlap) * scale;

    // Calculate actual spacing
    let spacing = if is_expanded {
        card_width * scale * 0.8
    } else {
        let total_space_needed = min_card_spacing * (card_count as f32 - 1.0);
        if total_space_needed > available_width {
            available_width / (card_count as f32 - 1.0)
        } else {
            min_card_spacing
        }
    };

    // Arc radius increases with card count to create nicer curve
    let arc_radius = if is_expanded {
        400.0 + (card_count as f32 * 10.0).min(200.0)
    } else {
        200.0 + (card_count as f32 * 5.0).min(100.0)
    };

    (spacing, scale, arc_radius)
}

/// Toggle the expansion state of a hand zone when clicked
pub fn toggle_hand_expansion(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut hand_query: Query<(&mut HandZone, &GlobalTransform)>,
) {
    // Only process on mouse click
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Get cursor position
    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        // Get camera, skip if no game camera exists (e.g., in menu states)
        let (camera, camera_transform) = match camera_query.get_single() {
            Ok(camera) => camera,
            Err(_) => {
                // Game camera doesn't exist (likely in a menu state), skip processing
                return;
            }
        };

        // Convert cursor to world position
        if let Ok(cursor_world_position) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .map(|ray| ray.origin.truncate())
        {
            // Check if cursor is over any hand zones
            for (mut hand, transform) in hand_query.iter_mut() {
                let hand_position = transform.translation().truncate();
                let distance = (hand_position - cursor_world_position).length();

                // Simple distance-based click detection - would be improved with actual colliders
                if distance < 150.0 {
                    // Toggle expanded state
                    hand.is_expanded = !hand.is_expanded;
                    info!("Hand expansion toggled: {}", hand.is_expanded);
                    break;
                }
            }
        }
    }
}

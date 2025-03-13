use crate::player::components::Player;
use bevy::prelude::*;
use std::collections::HashMap;

/// Resource to track the previous positions of players
#[derive(Resource, Default)]
pub struct PlayerPositionTracker {
    /// Maps player entity to their last logged position
    previous_positions: HashMap<Entity, Vec2>,
}

/// System to visualize player positions for debugging
///
/// This system draws visual indicators (circles and labels) at player positions
/// to help debug the spatial layout of the game.
pub fn debug_draw_player_positions(
    mut gizmos: Gizmos,
    player_query: Query<(Entity, &Transform, &Player)>,
    mut position_tracker: ResMut<PlayerPositionTracker>,
) {
    for (entity, transform, player) in player_query.iter() {
        // Draw a circle at the player position
        let position = transform.translation.truncate();

        // Choose color based on player index (Player 1 = blue, Player 2 = red, etc.)
        let color = match player.player_index {
            0 => Color::srgb(0.0, 0.5, 1.0), // Blue for Player 1
            1 => Color::srgb(1.0, 0.2, 0.2), // Red for Player 2
            _ => Color::srgb(0.5, 0.5, 0.5), // Gray for other players
        };

        // Draw a filled circle to represent the player
        gizmos.circle_2d(position, 1.5, color);

        // Draw an outline circle for better visibility
        gizmos.circle_2d(position, 1.7, Color::WHITE);

        // Use a semi-transparent version of the color for larger elements
        let alpha_color = match player.player_index {
            0 => Color::srgba(0.0, 0.5, 1.0, 0.2), // Blue with alpha
            1 => Color::srgba(1.0, 0.2, 0.2, 0.2), // Red with alpha
            _ => Color::srgba(0.5, 0.5, 0.5, 0.2), // Gray with alpha
        };
        gizmos.circle_2d(position, 5.0, alpha_color);

        // Draw a line from the player to where their cards would spawn
        let card_y_pos = if player.player_index == 0 {
            -150.0 // Updated to match new Player 1 card position
        } else {
            150.0 // Updated to match new Player 2 card position
        };

        let card_center = Vec2::new(position.x, card_y_pos);

        // Draw connection line for both players
        let line_color = match player.player_index {
            0 => Color::srgba(0.0, 0.5, 1.0, 0.5), // Blue with alpha
            1 => Color::srgba(1.0, 0.2, 0.2, 0.5), // Red with alpha
            _ => Color::srgba(0.5, 0.5, 0.5, 0.5), // Gray with alpha
        };
        gizmos.line_2d(position, card_center, line_color);

        // Draw a rectangle representing the card area for both players
        let card_width = 10.0; // Estimated total width of card area
        let card_height = 3.0;

        // Draw rectangle representing card area
        // rect_2d requires position, half_size, and color
        let rect_color = match player.player_index {
            0 => Color::srgba(0.0, 0.5, 1.0, 0.2), // Blue with alpha
            1 => Color::srgba(1.0, 0.2, 0.2, 0.2), // Red with alpha
            _ => Color::srgba(0.5, 0.5, 0.5, 0.2), // Gray with alpha
        };
        gizmos.rect_2d(
            Vec2::new(position.x, card_y_pos),              // Center position
            Vec2::new(card_width / 2.0, card_height / 2.0), // Half-size
            rect_color,                                     // Color with transparency
        );

        // Only log player positions when they change
        let previous_position = position_tracker.previous_positions.get(&entity).cloned();
        if previous_position.is_none() || previous_position.unwrap() != position {
            info!("Player {} position: {:?}", player.name, position);
            position_tracker.previous_positions.insert(entity, position);
        }
    }
}

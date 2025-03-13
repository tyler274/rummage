use crate::player::components::Player;
use bevy::prelude::*;

/// System to visualize player positions for debugging
///
/// This system draws visual indicators (circles and labels) at player positions
/// to help debug the spatial layout of the game.
pub fn debug_draw_player_positions(mut gizmos: Gizmos, player_query: Query<(&Transform, &Player)>) {
    for (transform, player) in player_query.iter() {
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
        let card_y_pos = -30.0; // This should match the y-position in spawn_visual_cards
        let card_center = Vec2::new(position.x, card_y_pos);

        if player.player_index == 0 {
            gizmos.line_2d(position, card_center, Color::srgba(0.0, 0.5, 1.0, 0.5));

            // Draw a rectangle representing the card area
            let card_width = 10.0; // Estimated total width of card area
            let card_height = 3.0;

            // Draw rectangle representing card area
            // rect_2d requires position, half_size, and color
            gizmos.rect_2d(
                Vec2::new(position.x, card_y_pos),              // Center position
                Vec2::new(card_width / 2.0, card_height / 2.0), // Half-size
                Color::srgba(0.0, 0.5, 1.0, 0.2),               // Color
            );
        }

        // Log player positions for debugging
        info!("Player {} position: {:?}", player.name, position);
    }
}

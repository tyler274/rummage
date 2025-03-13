use bevy::prelude::*;
use std::f32::consts::PI;

/// Handles the layout of players around a table
///
/// This struct calculates positions for any number of players
/// arranged in a circular pattern around a central point
#[derive(Debug, Clone)]
pub struct TableLayout {
    /// Number of players at the table
    pub player_count: usize,
    /// Distance from center for player positioning
    pub distance: f32,
    /// Distance from center for card positioning
    pub card_distance: f32,
}

impl TableLayout {
    /// Creates a new table layout with the given number of players and distance
    pub fn new(player_count: usize, distance: f32) -> Self {
        Self {
            player_count,
            distance,
            card_distance: distance * 1.5, // Default card distance is 150% of player distance
        }
    }

    /// Sets the card distance from center
    #[allow(dead_code)]
    pub fn with_card_distance(mut self, distance: f32) -> Self {
        self.card_distance = distance;
        self
    }

    /// Calculate position for a player based on index
    ///
    /// Players are positioned in a circle around the table
    pub fn get_player_position(&self, player_index: usize) -> Transform {
        let angle = self.get_player_angle(player_index);
        let position = Vec3::new(
            self.distance * angle.sin(),
            self.distance * angle.cos(),
            0.0,
        );

        Transform::from_translation(position)
    }

    /// Get the angle (in radians) for a player's position
    pub fn get_player_angle(&self, player_index: usize) -> f32 {
        // Calculate angle based on player index and total count
        // Start at the bottom (PI) and go counter-clockwise
        PI + 2.0 * PI * (player_index as f32 / self.player_count as f32)
    }

    /// Get a descriptive position name based on player index
    pub fn get_position_name(&self, player_index: usize) -> &'static str {
        // For a 4-player game, use the standard positions
        if self.player_count == 4 {
            match player_index % 4 {
                0 => "bottom", // Main player
                1 => "right",
                2 => "top",
                3 => "left",
                _ => "unknown", // Fallback (shouldn't happen)
            }
        } else {
            // For other player counts, use position numbers
            match player_index {
                0 => "bottom", // Main player is always at the bottom
                _ => "seat",   // Other players are just numbered seats
            }
        }
    }

    /// Calculate the card offset for a player based on their position
    pub fn get_card_offset(&self, player_index: usize) -> Vec3 {
        if self.player_count == 4 {
            // Standard 4-player layout
            match player_index % 4 {
                0 => Vec3::new(0.0, -1200.0, 0.0), // Bottom: below player
                1 => Vec3::new(4500.0, 0.0, 0.0),  // Right: to the right (increased from 3000)
                2 => Vec3::new(0.0, 1200.0, 0.0),  // Top: above player
                3 => Vec3::new(-4500.0, 0.0, 0.0), // Left: to the left (increased from -3000)
                _ => Vec3::ZERO,                   // Fallback
            }
        } else {
            // For other player counts, calculate based on angle
            let angle = self.get_player_angle(player_index);
            // Position cards in front of players, offset toward the center
            Vec3::new(
                -self.card_distance * 0.5 * angle.sin(),
                -self.card_distance * 0.5 * angle.cos(),
                0.0,
            )
        }
    }

    /// Returns if a player's cards should be laid out horizontally
    pub fn is_horizontal_layout(&self, player_index: usize) -> bool {
        if self.player_count == 4 {
            // For 4 players: horizontal for top and bottom, vertical for left and right
            player_index % 4 == 0 || player_index % 4 == 2
        } else {
            // For other player counts, determine based on angle
            let angle = self.get_player_angle(player_index);
            angle.sin().abs() < 0.5 // Horizontal if mostly vertical position
        }
    }
}

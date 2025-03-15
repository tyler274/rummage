use bevy::prelude::*;
use std::f32::consts::PI;

/// Handles the layout of players around a table
///
/// This struct calculates positions for any number of players
/// arranged in a polygon pattern around a central point
#[derive(Debug, Clone)]
pub struct TableLayout {
    /// Number of players at the table
    pub player_count: usize,
    /// Distance from center for player positioning
    #[allow(dead_code)]
    pub distance: f32,
    /// Distance from center for card positioning
    pub card_distance: f32,
    /// Standard dimensions for a playmat
    pub playmat_size: Vec2,
}

impl TableLayout {
    /// Creates a new table layout with the given number of players and distance
    pub fn new(player_count: usize, distance: f32) -> Self {
        Self {
            player_count,
            distance,
            card_distance: distance * 1.5, // Default card distance is 150% of player distance
            playmat_size: Vec2::new(430.0, 330.0), // Increased playmat size for larger cards
        }
    }

    /// Sets the card distance from center
    #[allow(dead_code)]
    pub fn with_card_distance(mut self, distance: f32) -> Self {
        self.card_distance = distance;
        self
    }

    /// Sets the playmat size
    pub fn with_playmat_size(mut self, size: Vec2) -> Self {
        self.playmat_size = size;
        self
    }

    /// Get the optimal playmat distance for an N-sided polygon
    fn get_polygon_distance(&self) -> f32 {
        if self.player_count == 2 {
            // For 2 players, position them directly opposite each other
            // Calculate distance based on playmat size to ensure they don't overlap
            self.playmat_size.y * 1.2
        } else {
            // For a regular N-sided polygon where playmats touch ONLY at corners:
            // 1. Calculate the diagonal distance from center to corner
            // 2. Position playmats so only the outer corners touch

            let angle_between_players = 2.0 * PI / self.player_count as f32;

            // Calculate half-width and half-height of playmat
            let half_width = self.playmat_size.x / 2.0;
            let half_height = self.playmat_size.y / 2.0;

            // Distance from center of playmat to its corner
            let to_corner = (half_width.powi(2) + half_height.powi(2)).sqrt();

            // Calculate the distance needed for corners to barely touch
            // We need to position the playmats so that adjacent corners just touch
            // This requires a distance that's slightly larger than if they were overlapping

            // Sine of half the angle between players gives us the ratio we need
            let half_angle = angle_between_players / 2.0;

            // The distance from center to each playmat where corners just touch
            // Formula derived from geometry: distance = to_corner / sin(half_angle)
            let distance = to_corner / half_angle.sin();

            // Add a small buffer (2%) to ensure corners only touch and don't overlap
            distance * 1.02
        }
    }

    /// Calculate position for a player based on index
    ///
    /// Players are positioned in a polygon around the table
    pub fn get_player_position(&self, player_index: usize) -> Transform {
        let angle = self.get_player_angle(player_index);

        // Calculate polygon distance
        let polygon_distance = self.get_polygon_distance();

        // Calculate position based on angle and distance
        let position = Vec3::new(
            polygon_distance * angle.sin(),
            polygon_distance * angle.cos(),
            0.0,
        );

        // Create transform with appropriate rotation to face center
        let mut transform = Transform::from_translation(position);

        // Rotate playmat to face center and adjust orientation
        if self.player_count == 2 {
            // For 2 players, use special case horizontal arrangement
            transform.rotation = if player_index == 0 {
                Quat::from_rotation_z(0.0) // Bottom player
            } else {
                Quat::from_rotation_z(PI) // Top player
            };
        } else {
            // Point the playmat toward the center
            // For corner-to-corner placement, we rotate toward the center
            transform.rotation = Quat::from_rotation_z(angle + PI);
        }

        transform
    }

    /// Get the angle (in radians) for a player's position
    pub fn get_player_angle(&self, player_index: usize) -> f32 {
        if self.player_count == 2 {
            // For 2 players, position at bottom and top
            match player_index {
                0 => PI,  // Bottom
                _ => 0.0, // Top
            }
        } else {
            // Calculate angle based on player index and total count
            // Start at the bottom (PI) and go counter-clockwise
            PI + 2.0 * PI * (player_index as f32 / self.player_count as f32)
        }
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
        } else if self.player_count == 2 {
            match player_index {
                0 => "bottom", // Main player
                _ => "top",    // Opponent directly across
            }
        } else {
            // For other player counts, use position numbers
            match player_index {
                0 => "bottom", // Main player is always at the bottom
                _ => "seat",   // Other players are numbered seats
            }
        }
    }

    /// Calculate the card offset for a player based on their position
    pub fn get_card_offset(&self, player_index: usize) -> Vec3 {
        if self.player_count == 4 {
            // Standard 4-player layout with cards positioned better on playmats
            match player_index % 4 {
                0 => Vec3::new(0.0, -100.0, 0.0), // Bottom: offset downward on playmat
                1 => Vec3::new(100.0, 0.0, 0.0),  // Right: offset rightward on playmat
                2 => Vec3::new(0.0, 100.0, 0.0),  // Top: offset upward on playmat
                3 => Vec3::new(-100.0, 0.0, 0.0), // Left: offset leftward on playmat
                _ => Vec3::ZERO,                  // Fallback
            }
        } else if self.player_count == 2 {
            // For 2 players, just offset toward opponent
            match player_index {
                0 => Vec3::new(0.0, -100.0, 0.0), // Bottom: offset downward on playmat
                _ => Vec3::new(0.0, 100.0, 0.0),  // Top: offset upward on playmat
            }
        } else {
            // For other player counts, calculate based on angle with minimal distance
            let angle = self.get_player_angle(player_index);
            // Position cards closer to their respective playmats, oriented toward center
            Vec3::new(
                -75.0 * angle.sin(), // Reduced offset to keep cards on playmat
                -75.0 * angle.cos(), // Reduced offset to keep cards on playmat
                0.0,
            )
        }
    }

    /// Returns if a player's cards should be laid out horizontally
    pub fn is_horizontal_layout(&self, player_index: usize) -> bool {
        if self.player_count == 2 {
            // For 2 players: always horizontal
            true
        } else if self.player_count == 4 {
            // For 4 players: horizontal for top and bottom, vertical for left and right
            player_index % 4 == 0 || player_index % 4 == 2
        } else {
            // For other player counts, determine based on angle
            let angle = self.get_player_angle(player_index);
            angle.sin().abs() < 0.5 // Horizontal if mostly vertical position
        }
    }
}

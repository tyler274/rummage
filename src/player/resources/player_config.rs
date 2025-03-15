use bevy::prelude::*;

/// Configuration resource for player spawning and setup
#[derive(Resource, Debug, Clone)]
pub struct PlayerConfig {
    /// Number of players to spawn (typically 4 for Commander format)
    pub player_count: usize,

    /// Whether to spawn cards for all players or just player 1
    pub spawn_all_cards: bool,

    /// Starting life total for each player (default: 40 for Commander)
    pub starting_life: i32,

    /// Card size for rendering
    pub card_size: Vec2,

    /// Card spacing multiplier
    pub card_spacing_multiplier: f32,

    /// Distance from center for positioning player cards
    pub player_card_distance: f32,

    /// Vertical offsets for each player's cards based on their position
    pub player_card_offsets: [f32; 4],
}

impl PlayerConfig {
    /// Creates a new PlayerConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of players
    pub fn with_player_count(mut self, count: usize) -> Self {
        self.player_count = count;
        self
    }

    /// Sets whether to spawn cards for all players
    pub fn with_spawn_all_cards(mut self, spawn_all: bool) -> Self {
        self.spawn_all_cards = spawn_all;
        self
    }

    /// Sets the starting life total
    pub fn with_starting_life(mut self, life: i32) -> Self {
        self.starting_life = life;
        self
    }

    /// Sets the card size
    #[allow(dead_code)]
    pub fn with_card_size(mut self, size: Vec2) -> Self {
        self.card_size = size;
        self
    }

    /// Sets the card spacing multiplier
    #[allow(dead_code)]
    pub fn with_card_spacing_multiplier(mut self, multiplier: f32) -> Self {
        self.card_spacing_multiplier = multiplier;
        self
    }

    /// Sets the distance from center for player cards
    pub fn with_player_card_distance(mut self, distance: f32) -> Self {
        self.player_card_distance = distance;
        self
    }

    /// Sets a specific player card offset
    pub fn with_player_card_offset(mut self, player_index: usize, offset: f32) -> Self {
        if player_index < 4 {
            self.player_card_offsets[player_index] = offset;
        }
        self
    }

    /// Calculate position for a player's cards based on player index (0-based)
    #[allow(dead_code)]
    pub fn calculate_player_position(&self, player_index: usize) -> Vec3 {
        // Position players around a table
        match player_index % 4 {
            0 => Vec3::new(0.0, -self.player_card_distance, 0.0), // bottom (player's perspective)
            1 => Vec3::new(self.player_card_distance, 0.0, 0.0),  // right
            2 => Vec3::new(0.0, self.player_card_distance, 0.0),  // top
            3 => Vec3::new(-self.player_card_distance, 0.0, 0.0), // left
            _ => Vec3::ZERO,                                      // fallback (shouldn't happen)
        }
    }

    /// Get the Y offset for a player's cards
    pub fn get_player_card_y_offset(&self, player_index: usize) -> f32 {
        if player_index < 4 {
            self.player_card_offsets[player_index]
        } else {
            0.0 // fallback
        }
    }

    /// Get a descriptive position name based on player index
    pub fn get_position_name(&self, player_index: usize) -> &'static str {
        match player_index % 4 {
            0 => "bottom", // Main player
            1 => "right",
            2 => "top",
            3 => "left",
            _ => "unknown", // fallback (shouldn't happen)
        }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            player_count: 4, // Default to 4 players for Commander format
            spawn_all_cards: false,
            starting_life: 40,                   // Commander starting life total
            card_size: Vec2::new(896.0, 1248.0), // Increased from 672x936 to match the multiplier increase
            card_spacing_multiplier: 1.2,        // Increased from 1.1 for better spacing
            player_card_distance: 1200.0, // Increased from 950.0 to further eliminate playmat overlap
            player_card_offsets: [-1500.0, 0.0, 1500.0, 0.0], // Increased Y offsets for cards relative to player position
        }
    }
}

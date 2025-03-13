use bevy::prelude::*;

/// Configuration resource for player spawning and setup
#[derive(Resource, Debug, Clone)]
pub struct PlayerConfig {
    /// Number of players to spawn
    pub player_count: usize,
    
    /// Whether to spawn cards for all players or just player 1
    pub spawn_all_cards: bool,
    
    /// Starting life total for each player (default: 20)
    pub starting_life: i32,
    
    /// Card size for rendering
    pub card_size: Vec2,
    
    /// Card spacing multiplier
    pub card_spacing_multiplier: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            player_count: 1,
            spawn_all_cards: false,
            starting_life: 20,
            card_size: Vec2::new(672.0, 936.0),
            card_spacing_multiplier: 1.1,
        }
    }
} 
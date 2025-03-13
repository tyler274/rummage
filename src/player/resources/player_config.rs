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

    /// Vertical offset for Player 1's cards (bottom player)
    pub player1_card_y_offset: f32,

    /// Vertical offset for Player 2's cards (top player)
    pub player2_card_y_offset: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            player_count: 1,
            spawn_all_cards: false,
            starting_life: 20,
            card_size: Vec2::new(672.0, 936.0),
            card_spacing_multiplier: 1.1,
            player1_card_y_offset: -1200.0,
            player2_card_y_offset: 1200.0,
        }
    }
}

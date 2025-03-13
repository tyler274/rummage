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
    pub fn with_card_size(mut self, size: Vec2) -> Self {
        self.card_size = size;
        self
    }

    /// Sets the card spacing multiplier
    pub fn with_card_spacing_multiplier(mut self, multiplier: f32) -> Self {
        self.card_spacing_multiplier = multiplier;
        self
    }

    /// Sets the vertical offset for Player 1's cards
    pub fn with_player1_card_y_offset(mut self, offset: f32) -> Self {
        self.player1_card_y_offset = offset;
        self
    }

    /// Sets the vertical offset for Player 2's cards
    pub fn with_player2_card_y_offset(mut self, offset: f32) -> Self {
        self.player2_card_y_offset = offset;
        self
    }
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

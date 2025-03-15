use crate::mana::ManaPool;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a player in the game with their associated state
#[derive(Component, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Player {
    /// Player's display name
    pub name: String,
    /// Current life total
    pub life: i32,
    /// Available mana pool
    pub mana_pool: ManaPool,
    /// Player index (0-based) for positioning and identification
    pub player_index: usize,
}

impl Player {
    /// Creates a new player with default values
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            life: 40, // Default life total for Commander format
            mana_pool: ManaPool::default(),
            player_index: 0,
        }
    }

    /// Sets the player's life total
    pub fn with_life(mut self, life: i32) -> Self {
        self.life = life;
        self
    }

    /// Sets the player's mana pool
    pub fn with_mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = mana_pool;
        self
    }

    /// Sets the player's index
    pub fn with_player_index(mut self, index: usize) -> Self {
        self.player_index = index;
        self
    }
}

use crate::mana::ManaPool;
use serde::{Deserialize, Serialize};

/// Serializable player data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: usize,
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub player_index: usize,
}

/// Builder for PlayerData
#[derive(Default)]
pub struct PlayerDataBuilder {
    id: usize,
    name: String,
    life: i32,
    mana_pool: ManaPool,
    player_index: usize,
}

impl PlayerDataBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            life: 40, // Default life total
            mana_pool: ManaPool::default(),
            player_index: 0,
        }
    }

    /// Set the player ID
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    /// Set the player name
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Set the player's life total
    pub fn life(mut self, life: i32) -> Self {
        self.life = life;
        self
    }

    /// Set the player's mana pool
    pub fn mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = mana_pool;
        self
    }

    /// Set the player's index
    pub fn player_index(mut self, player_index: usize) -> Self {
        self.player_index = player_index;
        self
    }

    /// Build the PlayerData instance
    pub fn build(self) -> PlayerData {
        PlayerData {
            id: self.id,
            name: self.name,
            life: self.life,
            mana_pool: self.mana_pool,
            player_index: self.player_index,
        }
    }
}

impl PlayerData {
    /// Create a new builder for PlayerData
    pub fn builder() -> PlayerDataBuilder {
        PlayerDataBuilder::new()
    }
}

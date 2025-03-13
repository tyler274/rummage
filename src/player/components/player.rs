use crate::card::Card;
use crate::deck::Deck;
use crate::mana::ManaPool;
use bevy::prelude::*;

use super::builder::PlayerBuilder;

/// Represents a player in the game with their associated state
#[derive(Component, Default, Debug, Clone)]
pub struct Player {
    /// Player's display name
    pub name: String,
    /// Current life total
    pub life: i32,
    /// Available mana pool
    #[allow(dead_code)]
    pub mana_pool: ManaPool,
    /// Cards in the player's possession
    #[allow(dead_code)]
    pub cards: Vec<Card>,
    /// Player's deck
    #[allow(dead_code)]
    pub deck: Option<Deck>,
    /// Player index (0-based) for positioning and identification
    pub player_index: usize,
}

impl Player {
    /// Creates a new Player builder to use a chainable API for constructing a Player
    pub fn builder(name: &str) -> PlayerBuilder {
        PlayerBuilder::new(name)
    }
}

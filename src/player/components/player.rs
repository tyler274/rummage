use crate::card::Card;
use crate::deck::Deck;
use crate::mana::ManaPool;
use bevy::prelude::*;

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
    /// Creates a new player with default values
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            life: 40, // Default life total for Commander format
            mana_pool: ManaPool::default(),
            cards: Vec::new(),
            deck: None,
            player_index: 0,
        }
    }

    /// Sets the player's life total
    pub fn with_life(mut self, life: i32) -> Self {
        self.life = life;
        self
    }

    /// Sets the player's mana pool
    #[allow(dead_code)]
    pub fn with_mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = mana_pool;
        self
    }

    /// Sets the player's cards
    pub fn with_cards(mut self, cards: Vec<Card>) -> Self {
        self.cards = cards;
        self
    }

    /// Sets the player's deck
    pub fn with_deck(mut self, deck: Deck) -> Self {
        self.deck = Some(deck);
        self
    }

    /// Sets the player's index
    pub fn with_player_index(mut self, index: usize) -> Self {
        self.player_index = index;
        self
    }
}

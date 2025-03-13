use crate::card::Card;
use crate::deck::Deck;
use crate::mana::ManaPool;

use super::player::Player;

/// Builder for Player instances with a chainable API
#[derive(Debug, Clone)]
pub struct PlayerBuilder {
    name: String,
    life: i32,
    mana_pool: ManaPool,
    cards: Vec<Card>,
    deck: Option<Deck>,
    player_index: usize,
}

impl PlayerBuilder {
    /// Creates a new PlayerBuilder with the given name and default values
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            life: 20, // Default starting life
            mana_pool: ManaPool::default(),
            cards: Vec::new(),
            deck: None,
            player_index: 0,
        }
    }

    /// Sets the player's starting life total
    pub fn life(mut self, life: i32) -> Self {
        self.life = life;
        self
    }

    /// Sets the player's mana pool
    #[allow(dead_code)]
    pub fn mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = mana_pool;
        self
    }

    /// Sets the player's cards
    pub fn cards(mut self, cards: Vec<Card>) -> Self {
        self.cards = cards;
        self
    }

    /// Sets the player's deck
    pub fn deck(mut self, deck: Deck) -> Self {
        self.deck = Some(deck);
        self
    }

    /// Sets the player's index
    pub fn player_index(mut self, index: usize) -> Self {
        self.player_index = index;
        self
    }

    /// Builds the Player instance
    pub fn build(self) -> Player {
        Player {
            name: self.name,
            life: self.life,
            mana_pool: self.mana_pool,
            cards: self.cards,
            deck: self.deck,
            player_index: self.player_index,
        }
    }
}

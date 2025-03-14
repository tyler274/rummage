use crate::cards::Card;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a deck of Magic cards
#[derive(Debug, Clone)]
pub struct Deck {
    /// Name of the deck
    #[allow(dead_code)]
    pub name: String,
    /// Type of the deck (Commander, Standard, etc.)
    #[allow(dead_code)]
    pub deck_type: DeckType,
    /// Cards in the deck
    pub cards: Vec<Card>,
    /// Commander card ID if this is a Commander deck
    #[allow(dead_code)]
    pub commander: Option<Entity>,
    /// Partner commander card ID if applicable
    #[allow(dead_code)]
    pub partner: Option<Entity>,
    /// Owner of the deck
    #[allow(dead_code)]
    pub owner: Option<Entity>,
}

/// Component to track a player's deck
/// This allows decks to be a proper ECS component that can be independently queried and modified
#[derive(Component, Debug, Clone)]
pub struct PlayerDeck {
    /// The actual deck data
    pub deck: Deck,
}

impl PlayerDeck {
    /// Create a new player deck component
    pub fn new(deck: Deck) -> Self {
        Self { deck }
    }

    /// Draw a card from the top of the deck
    #[allow(dead_code)]
    pub fn draw(&mut self) -> Option<Card> {
        self.deck.draw()
    }

    /// Draw multiple cards from the top of the deck
    #[allow(dead_code)]
    pub fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        self.deck.draw_multiple(count)
    }
}

/// Represents different types of Magic decks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeckType {
    /// Standard format deck (60 card minimum)
    Standard,
    /// Commander/EDH format deck (100 card singleton with Commander)
    Commander,
    /// Modern format deck
    Modern,
    /// Legacy format deck
    Legacy,
    /// Vintage format deck
    Vintage,
    /// Pauper format deck
    Pauper,
    /// Pioneer format deck
    Pioneer,
    /// Limited format deck (40 card minimum)
    Limited,
    /// Brawl format deck
    Brawl,
    /// Custom format with special rules
    Custom(String),
}

/// Errors that can occur during deck validation
#[derive(Debug)]
#[allow(dead_code)]
pub enum DeckValidationError {
    /// Deck doesn't have enough cards
    TooFewCards { required: usize, actual: usize },
    /// Deck has illegal cards (e.g., banned cards)
    IllegalCards(Vec<String>),
    /// Deck has too many copies of a card
    TooManyCopies {
        card_name: String,
        max_allowed: usize,
        actual: usize,
    },
    /// Deck has cards outside the Commander's color identity
    ColorIdentityViolation(Vec<String>),
    /// Commander is missing
    MissingCommander,
    /// Other validation errors
    OtherError(String),
}

impl Deck {
    /// Create a new deck
    pub fn new(name: String, deck_type: DeckType, cards: Vec<Card>) -> Self {
        Self {
            name,
            deck_type,
            cards,
            commander: None,
            partner: None,
            owner: None,
        }
    }

    /// Set the owner of this deck
    #[allow(dead_code)]
    pub fn set_owner(&mut self, owner: Entity) {
        self.owner = Some(owner);
    }

    /// Set the commander for this deck
    #[allow(dead_code)]
    pub fn set_commander(&mut self, commander: Entity) {
        self.commander = Some(commander);
    }

    /// Set the partner commander for this deck
    #[allow(dead_code)]
    pub fn set_partner(&mut self, partner: Entity) {
        self.partner = Some(partner);
    }

    /// Get the number of cards in the deck
    #[allow(dead_code)]
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// Validate the deck against format rules
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), Vec<DeckValidationError>> {
        let mut errors = Vec::new();

        // Check minimum deck size
        let min_size = match self.deck_type {
            DeckType::Standard
            | DeckType::Modern
            | DeckType::Legacy
            | DeckType::Vintage
            | DeckType::Pioneer => 60,
            DeckType::Commander | DeckType::Brawl => 100,
            DeckType::Limited => 40,
            DeckType::Custom(_) => 0, // Custom formats may have different requirements
            DeckType::Pauper => 60,
        };

        if self.cards.len() < min_size {
            errors.push(DeckValidationError::TooFewCards {
                required: min_size,
                actual: self.cards.len(),
            });
        }

        // Check for Commander if this is a Commander deck
        if self.deck_type == DeckType::Commander && self.commander.is_none() {
            errors.push(DeckValidationError::MissingCommander);
        }

        // Check for too many copies of a card
        if self.deck_type != DeckType::Limited {
            let mut card_counts: HashMap<String, usize> = HashMap::new();
            for card in &self.cards {
                *card_counts.entry(card.name.name.clone()).or_insert(0) += 1;
            }

            // Check for max copies (4 in most formats, 1 in Commander/Brawl)
            let max_copies = match self.deck_type {
                DeckType::Commander | DeckType::Brawl => 1,
                _ => 4,
            };

            for (card_name, count) in card_counts {
                if count > max_copies {
                    errors.push(DeckValidationError::TooManyCopies {
                        card_name,
                        max_allowed: max_copies,
                        actual: count,
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        // Create a fresh random number generator with a unique seed
        // This ensures each shuffle is independent
        let seed = rand::random::<u64>();
        let mut rng = StdRng::seed_from_u64(seed);

        // Shuffle using our independent RNG
        self.cards.shuffle(&mut rng);
    }

    /// Draw a card from the top of the deck
    #[allow(dead_code)]
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Draw multiple cards from the top of the deck
    #[allow(dead_code)]
    pub fn draw_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.draw() {
                drawn.push(card);
            } else {
                break;
            }
        }
        drawn
    }

    /// Add a card to the top of the deck
    #[allow(dead_code)]
    pub fn add_top(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Add a card to the bottom of the deck
    #[allow(dead_code)]
    pub fn add_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    /// Search for cards by name
    #[allow(dead_code)]
    pub fn search(&self, name: &str) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|card| card.name.contains(name))
            .collect()
    }
}

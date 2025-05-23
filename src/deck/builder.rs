use super::types::{Deck, DeckType};
use crate::cards::Card;
use bevy::prelude::*;

/// Builder for creating decks
#[derive(Default)]
#[allow(dead_code)]
pub struct DeckBuilder {
    name: Option<String>,
    deck_type: Option<DeckType>,
    cards: Vec<Card>,
    commander: Option<Entity>,
    partner: Option<Entity>,
    owner: Option<Entity>,
}

impl DeckBuilder {
    /// Create a new empty deck builder
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the name of the deck
    #[allow(dead_code)]
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set the type of the deck
    #[allow(dead_code)]
    pub fn with_type(mut self, deck_type: DeckType) -> Self {
        self.deck_type = Some(deck_type);
        self
    }

    /// Add multiple cards at once
    #[allow(dead_code)]
    pub fn with_cards(mut self, cards: Vec<Card>) -> Self {
        self.cards.extend(cards);
        self
    }

    /// Add a single card
    #[allow(dead_code)]
    pub fn add_card(mut self, card: Card) -> Self {
        self.cards.push(card);
        self
    }

    /// Add multiple copies of a card
    #[allow(dead_code)]
    pub fn add_copies(mut self, card: Card, count: usize) -> Self {
        for _ in 0..count {
            self.cards.push(card.clone());
        }
        self
    }

    /// Set the commander (for Commander format)
    #[allow(dead_code)]
    pub fn with_commander(mut self, commander: Entity) -> Self {
        self.commander = Some(commander);
        self
    }

    /// Set the partner commander (for Commander format)
    #[allow(dead_code)]
    pub fn with_partner(mut self, partner: Entity) -> Self {
        self.partner = Some(partner);
        self
    }

    /// Set the owner of the deck
    #[allow(dead_code)]
    pub fn with_owner(mut self, owner: Entity) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Build the final deck
    #[allow(dead_code)]
    pub fn build(self) -> Result<Deck, String> {
        let name = self.name.unwrap_or_else(|| "Untitled Deck".to_string());
        let deck_type = self.deck_type.unwrap_or(DeckType::Standard);

        let mut deck = Deck::new(name, deck_type, self.cards);

        if let Some(commander) = self.commander {
            deck.set_commander(commander);
        }

        if let Some(partner) = self.partner {
            deck.set_partner(partner);
        }

        if let Some(owner) = self.owner {
            deck.set_owner(owner);
        }

        Ok(deck)
    }

    /// Build a shuffled deck
    #[allow(dead_code)]
    pub fn build_shuffled(self) -> Result<Deck, String> {
        let mut deck = self.build()?;
        deck.shuffle();
        Ok(deck)
    }
}

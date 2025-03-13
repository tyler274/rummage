use super::{Card, details::CardDetails, keywords::KeywordAbilities, types::CardTypes};
use crate::mana::Mana;

/// Builder for creating Card instances using a chainable API
pub struct CardBuilder {
    name: String,
    cost: Option<Mana>,
    types: Option<CardTypes>,
    card_details: Option<CardDetails>,
    rules_text: Option<String>,
}

impl CardBuilder {
    /// Create a new card builder with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cost: None,
            types: None,
            card_details: None,
            rules_text: None,
        }
    }

    /// Set the mana cost for the card
    pub fn cost(mut self, cost: Mana) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Set the card types
    pub fn types(mut self, types: CardTypes) -> Self {
        self.types = Some(types);
        self
    }

    /// Set the card details
    pub fn details(mut self, details: CardDetails) -> Self {
        self.card_details = Some(details);
        self
    }

    /// Set the rules text
    pub fn rules_text(mut self, text: &str) -> Self {
        self.rules_text = Some(text.to_string());
        self
    }

    /// Build the final Card
    pub fn build(self) -> Result<Card, String> {
        let cost = self
            .cost
            .ok_or_else(|| "Card must have a mana cost".to_string())?;
        let types = self
            .types
            .ok_or_else(|| "Card must have types".to_string())?;
        let card_details = self
            .card_details
            .ok_or_else(|| "Card must have details".to_string())?;
        let rules_text = self.rules_text.unwrap_or_default();

        // Initialize keywords from rules text
        let keywords = KeywordAbilities::from_rules_text(&rules_text);

        Ok(Card {
            name: self.name,
            cost,
            types,
            card_details,
            rules_text,
            keywords,
        })
    }

    /// Build the card or panic if any required fields are missing.
    /// This is useful for tests or when you're sure all fields are set.
    pub fn build_or_panic(self) -> Card {
        self.build()
            .expect("Failed to build card: missing required fields")
    }
}

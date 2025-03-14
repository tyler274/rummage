// Card struct definition and implementation

use crate::mana::Mana;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cards::components::{
    CardCost, CardDetailsComponent, CardKeywords, CardName, CardRulesText, CardTypeInfo,
};
use crate::cards::details::CardDetails;
use crate::cards::keywords::KeywordAbilities;
use crate::cards::types::{CardTypes, format_type_line};

#[cfg(test)]
use crate::cards::keywords::KeywordAbility;

/// Bundle for Magic: The Gathering cards
///
/// This bundle contains all the components that make up a card entity.
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Card {
    pub name: CardName,
    pub cost: CardCost,
    #[reflect(ignore)]
    pub type_info: CardTypeInfo,
    pub details: CardDetailsComponent,
    pub rules_text: CardRulesText,
    pub keywords: CardKeywords,
}

impl Card {
    /// Create a new Card from its component parts
    pub fn new(
        name: &str,
        cost: Mana,
        types: CardTypes,
        details: CardDetails,
        rules_text: &str,
    ) -> Self {
        // Initialize keywords from rules text
        let keywords = KeywordAbilities::from_rules_text(rules_text);

        Self {
            name: CardName {
                name: name.to_string(),
            },
            cost: CardCost { cost },
            type_info: CardTypeInfo { types },
            details: CardDetailsComponent { details },
            rules_text: CardRulesText {
                rules_text: rules_text.to_string(),
            },
            keywords: CardKeywords { keywords },
        }
    }

    /// Get a builder for this card type
    pub fn builder(name: &str) -> crate::cards::builder::CardBuilder {
        crate::cards::builder::CardBuilder::new(name)
    }

    /// Extract all individual components from a Card to match the old API
    /// This is for backward compatibility with code expecting separate components
    pub fn get_components(
        self,
    ) -> (
        Card,
        CardName,
        CardCost,
        CardTypeInfo,
        CardDetailsComponent,
        CardRulesText,
        CardKeywords,
    ) {
        let Card {
            name,
            cost,
            type_info,
            details,
            rules_text,
            keywords,
        } = self.clone();

        // Return a new card with the same data, plus the individual components
        (self, name, cost, type_info, details, rules_text, keywords)
    }

    /// Get the card's type line for display
    pub fn type_line_from_components(types: &CardTypes) -> String {
        format_type_line(types, &CardDetails::Other) // Default to Other when no details are provided
    }
}

// Test-only Card methods to avoid dead code warnings
#[cfg(test)]
impl Card {
    /// Helper method to spawn a card directly without using the builder
    pub fn spawn(
        commands: &mut Commands,
        name: &str,
        cost: Mana,
        types: CardTypes,
        details: CardDetails,
        rules_text: &str,
    ) -> Entity {
        commands
            .spawn(Self::new(name, cost, types, details, rules_text))
            .id()
    }

    /// Helper method to get a card's types from a query
    pub fn get_types<'a>(card: &'a Self) -> &'a CardTypes {
        &card.type_info.types
    }

    /// Helper method to get a card's cost from a query
    pub fn get_cost<'a>(card: &'a Self) -> &'a Mana {
        &card.cost.cost
    }

    /// Helper method to get a card's name from a query
    pub fn get_name<'a>(card: &'a Self) -> &'a str {
        &card.name.name
    }

    /// Helper method to get a card's rules text from a query
    pub fn get_rules_text<'a>(card: &'a Self) -> &'a str {
        &card.rules_text.rules_text
    }

    /// Helper method to get a card's details from a query
    pub fn get_details<'a>(card: &'a Self) -> &'a CardDetails {
        &card.details.details
    }

    /// Helper method to check if a card has a specific type
    pub fn has_type(card: &Self, card_type: CardTypes) -> bool {
        card.type_info.types.contains(card_type)
    }

    /// Add a keyword ability to a card
    pub fn add_keyword(card: &mut Self, keyword: KeywordAbility) {
        card.keywords.keywords.abilities.insert(keyword);
    }

    /// Add a keyword ability with a value to a card
    pub fn add_keyword_with_value(card: &mut Self, keyword: KeywordAbility, value: &str) {
        card.keywords.keywords.abilities.insert(keyword);
        card.keywords
            .keywords
            .ability_values
            .insert(keyword, value.to_string());
    }

    /// Check if a card has a specific keyword ability
    pub fn has_keyword(card: &Self, keyword: KeywordAbility) -> bool {
        card.keywords.keywords.abilities.contains(&keyword)
    }

    /// Get the value associated with a keyword ability
    pub fn get_keyword_value(card: &Self, keyword: KeywordAbility) -> Option<&str> {
        card.keywords
            .keywords
            .ability_values
            .get(&keyword)
            .map(|s| s.as_str())
    }

    /// Helper function to get card type line
    pub fn type_line(card: &Self) -> String {
        format_type_line(&card.type_info.types, &CardDetails::Other)
    }
}

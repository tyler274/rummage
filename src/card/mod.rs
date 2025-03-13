// External modules
pub mod artifacts;
pub mod black;
pub mod blue;
pub mod green;
pub mod hdr;
pub mod mtgjson;
pub mod penacony;
pub mod red;
pub mod white;

// Internal modules
mod builder;
mod components;
mod counters;
mod details;
mod keywords;
mod systems;
mod types;

// Re-export types for external use
pub use builder::CardBuilder;
pub use components::{Draggable, NoUntapCondition, NoUntapEffect, PermanentState};
pub use details::{
    ArtifactCard, CardDetails, CreatureCard, CreatureOnField, EnchantmentCard, LandCard, SpellCard,
    SpellType,
};
pub use keywords::{KeywordAbilities, KeywordAbility};
pub use systems::{debug_render_text_positions, handle_card_dragging};
pub use types::{CardTypes, CreatureType, format_type_line};

// Import external crates
use crate::mana::Mana;
use crate::menu::GameMenuState;
use bevy::prelude::*;

/// Represents a Magic: The Gathering card with all its properties
#[derive(Component, Debug, Clone)]
pub struct Card {
    pub name: String,
    pub cost: Mana,
    pub types: CardTypes,
    pub card_details: CardDetails,
    pub rules_text: String,
    /// Keyword abilities the card has
    #[allow(dead_code)]
    pub keywords: KeywordAbilities,
}

impl Card {
    pub fn new(
        name: &str,
        cost: Mana,
        types: CardTypes,
        details: CardDetails,
        rules_text: &str,
    ) -> Self {
        // Initialize keywords from rules text
        let keywords = KeywordAbilities::from_rules_text(rules_text);

        Card {
            name: name.to_string(),
            cost,
            types,
            card_details: details,
            rules_text: rules_text.to_string(),
            keywords,
        }
    }

    /// Create a new card builder with the given name.
    /// This is the entry point for the builder pattern.
    pub fn builder(name: &str) -> CardBuilder {
        CardBuilder::new(name)
    }

    pub fn type_line(&self) -> String {
        format_type_line(&self.types, &self.card_details)
    }

    /// Add a keyword ability to the card
    #[allow(dead_code)]
    pub fn add_keyword(&mut self, keyword: KeywordAbility) {
        self.keywords.abilities.insert(keyword);
    }

    /// Add a keyword ability with a value (like "Protection from black")
    #[allow(dead_code)]
    pub fn add_keyword_with_value(&mut self, keyword: KeywordAbility, value: &str) {
        self.keywords.abilities.insert(keyword);
        self.keywords
            .ability_values
            .insert(keyword, value.to_string());
    }

    /// Check if the card has a specific keyword
    #[allow(dead_code)]
    pub fn has_keyword(&self, keyword: KeywordAbility) -> bool {
        self.keywords.abilities.contains(&keyword)
    }

    /// Get the value for a keyword if it exists (e.g., "black" for "Protection from black")
    #[allow(dead_code)]
    pub fn get_keyword_value(&self, keyword: KeywordAbility) -> Option<&str> {
        self.keywords
            .ability_values
            .get(&keyword)
            .map(|s| s.as_str())
    }
}

/// A plugin that registers all card-related systems and resources with Bevy
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_card_dragging, debug_render_text_positions)
                .run_if(in_state(GameMenuState::InGame)),
        );
    }
}

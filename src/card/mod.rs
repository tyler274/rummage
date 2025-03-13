// Card module - Handles all card-related functionality and data structures

// Core card functionality
mod abilities;
mod builder;
mod components;
mod counters;
mod details;
mod keywords;
mod state;
mod systems;
mod types;

// Card data sources and specialized sets
pub mod hdr; // Historic Definition Records
pub mod mtgjson; // MTG JSON import functionality
pub mod penacony; // Specific set implementation
pub mod sets; // General set management
pub mod text; // Card text handling

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

/// Card rarity in Magic: The Gathering
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    MythicRare,
    Special,
    Bonus,
    Promo,
}

impl From<&str> for Rarity {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "common" => Rarity::Common,
            "uncommon" => Rarity::Uncommon,
            "rare" => Rarity::Rare,
            "mythic" | "mythic rare" => Rarity::MythicRare,
            "special" => Rarity::Special,
            "bonus" => Rarity::Bonus,
            "promo" => Rarity::Promo,
            _ => Rarity::Common, // Default to common
        }
    }
}

/// Component that identifies which set a card belongs to
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct CardSet {
    /// Set code (e.g., "MID" for Innistrad: Midnight Hunt)
    pub code: String,
    /// Full name of the set
    pub name: String,
    /// Release date of the set
    pub release_date: String,
}

/// Represents a Magic: The Gathering card with all its properties
#[derive(Component, Debug, Clone, Reflect)]
pub struct Card {
    pub name: String,
    #[reflect(ignore)]
    pub cost: Mana,
    #[reflect(ignore)]
    pub types: CardTypes,
    #[reflect(ignore)]
    pub card_details: CardDetails,
    pub rules_text: String,
    /// Keyword abilities the card has
    #[allow(dead_code)]
    #[reflect(ignore)]
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
        app
            // Register components for ECS queries
            .register_type::<Card>()
            .register_type::<CardSet>()
            .register_type::<Rarity>()
            // Initialize the card registry
            .add_systems(Startup, sets::systems::init_card_registry)
            // Register card when added
            .add_systems(Update, sets::systems::register_card)
            // Register systems
            .add_systems(
                Update,
                (handle_card_dragging, debug_render_text_positions)
                    .run_if(in_state(GameMenuState::InGame)),
            );
    }
}

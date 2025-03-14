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
pub use components::{
    CardCost, CardDetailsComponent, CardKeywords, CardName, CardRulesText, CardTypeInfo, Draggable,
    NoUntapCondition, NoUntapEffect, PermanentState,
};
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
            _ => Rarity::Common,
        }
    }
}

/// Information about a card set
#[derive(Component, Debug, Clone, Reflect)]
pub struct CardSet {
    /// Set code (e.g., "MID" for Innistrad: Midnight Hunt)
    pub code: String,
    /// Full name of the set
    pub name: String,
    /// Release date of the set
    pub release_date: String,
}

/// Bundle for Magic: The Gathering cards
///
/// This bundle contains all the components that make up a card entity.
#[derive(Component, Debug, Clone, Reflect)]
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
    /// Create a new Card with all required components
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

    /// Create a card builder for fluent configuration
    pub fn builder(name: &str) -> CardBuilder {
        CardBuilder::new(name)
    }

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

/// Plugin for registering card-related systems and components
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Card>()
            .register_type::<CardName>()
            .register_type::<CardCost>()
            // CardTypeInfo contains bitflags which don't fully implement reflection
            // .register_type::<CardTypeInfo>()
            .register_type::<CardDetailsComponent>()
            .register_type::<CardRulesText>()
            .register_type::<CardKeywords>()
            .register_type::<PermanentState>()
            .register_type::<CardSet>()
            .register_type::<Rarity>()
            .register_type::<CardDetails>()
            .register_type::<CreatureCard>()
            // These types use bitflags which don't fully implement reflection
            // .register_type::<CardTypes>()
            // .register_type::<CreatureType>()
            .register_type::<KeywordAbility>()
            .register_type::<KeywordAbilities>()
            .register_type::<SpellType>()
            .register_type::<SpellCard>()
            .register_type::<EnchantmentCard>()
            .register_type::<ArtifactCard>()
            .register_type::<LandCard>()
            .register_type::<NoUntapEffect>()
            .register_type::<NoUntapCondition>()
            .register_type::<Draggable>()
            .register_type::<crate::mana::Mana>()
            // Color uses bitflags which don't fully implement reflection
            // .register_type::<crate::mana::Color>()
            .register_type::<std::collections::HashSet<KeywordAbility>>()
            .register_type::<std::collections::HashMap<KeywordAbility, String>>()
            .add_systems(Update, handle_card_dragging)
            .add_systems(Update, debug_render_text_positions);
    }
}

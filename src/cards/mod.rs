// Card module - Handles all card-related functionality and data structures

// Core card functionality
mod abilities;
mod builder;
mod card;
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
pub use card::Card;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::Color;
    use bevy::prelude::*;

    /// Test that demonstrates using the various Card accessor methods
    #[test]
    fn test_card_accessors() {
        // Create a test card
        let mut card = Card::new(
            "Test Card",
            Mana::new(1, 0, 0, 0, 0, 0),
            CardTypes::new_creature(vec!["Wizard".to_string()]),
            CardDetails::new_creature(2, 2),
            "Flying, Haste (This creature can attack as soon as it comes under your control.)",
        );

        // Test get_name
        assert_eq!(Card::get_name(&card), "Test Card");

        // Test get_cost
        let cost = Card::get_cost(&card);
        assert_eq!(cost.converted_mana_cost(), 1);
        assert_eq!(cost.colored_mana_cost(Color::Colorless), 1);

        // Test get_types
        let types = Card::get_types(&card);
        assert!(types.is_creature());
        assert_eq!(types.get_creature_types().len(), 1);
        assert_eq!(types.get_creature_types()[0], "Wizard");

        // Test get_rules_text
        assert!(Card::get_rules_text(&card).contains("Flying, Haste"));

        // Test get_details
        let details = Card::get_details(&card);
        if let CardDetails::Creature {
            power, toughness, ..
        } = details
        {
            assert_eq!(*power, 2);
            assert_eq!(*toughness, 2);
        } else {
            panic!("Expected creature details");
        }

        // Test has_type
        assert!(Card::has_type(&card, CardTypes::TYPE_CREATURE));

        // Test keyword methods
        assert!(Card::has_keyword(&card, KeywordAbility::Flying));
        assert!(Card::has_keyword(&card, KeywordAbility::Haste));

        // Add a keyword with a value
        Card::add_keyword_with_value(&mut card, KeywordAbility::Protection, "from black");
        assert!(Card::has_keyword(&card, KeywordAbility::Protection));
        assert_eq!(
            Card::get_keyword_value(&card, KeywordAbility::Protection),
            Some("from black")
        );

        // Add a simple keyword
        Card::add_keyword(&mut card, KeywordAbility::Vigilance);
        assert!(Card::has_keyword(&card, KeywordAbility::Vigilance));

        // Test type_line
        let type_line = Card::type_line(&card);
        assert!(type_line.contains("Creature"));
        assert!(type_line.contains("Wizard"));
    }

    /// Test demonstrating the spawn method
    #[test]
    fn test_card_spawn() {
        // Create a new app for testing
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // System to spawn a test card
        fn spawn_test_card(mut commands: Commands) {
            let _card_entity = Card::spawn(
                &mut commands,
                "Test Card",
                Mana::new(1, 0, 0, 0, 0, 0),
                CardTypes::new_creature(vec!["Wizard".to_string()]),
                CardDetails::new_creature(2, 2),
                "Flying",
            );
        }

        // Run the system to spawn the card
        app.add_systems(Update, spawn_test_card);
        app.update();

        // Check that the entity was created with Card component
        let card_exists = app.world.query::<&Card>().iter(&app.world).count() > 0;
        assert!(card_exists);
    }
}

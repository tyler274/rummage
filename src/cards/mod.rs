// Card module for the game with card-related functionality
//! Cards, rules text, abilities, and other card-related functionality

// Private modules
pub mod abilities;
pub mod builder;
pub mod card;
pub mod components;
pub mod counters;
pub mod details;
pub mod keywords;
pub mod plugin;
pub mod rarity;
pub mod set;
pub mod state;
pub mod systems;
pub mod types; // Making types public so it can be accessed directly

// Test modules
#[cfg(test)]
mod tests;

// Public modules
pub mod hdr; // Historic Definition Records
pub mod mtgjson; // MTG JSON import functionality
pub mod sets; // General set management
pub mod text; // Card text handling

// Re-export types for external use
// Remove glob imports that cause ambiguity
pub use card::Card;
pub use components::card_entity::*;
pub use components::*;
pub use details::*;
pub use keywords::*;
pub use state::*;
pub use systems::*;
pub use types::*;

/// Plugin responsible for registering all card-related systems and resources
pub struct CardPlugin;

impl bevy::prelude::Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Register types with bevy_reflect
        app.register_type::<Card>()
            .register_type::<CardEntity>()
            .register_type::<CardZone>()
            .register_type::<CardOwner>()
            .register_type::<components::CardName>()
            .register_type::<components::CardCost>()
            .register_type::<components::CardTypeInfo>()
            .register_type::<components::CardDetailsComponent>()
            .register_type::<components::CardRulesText>()
            .register_type::<components::CardKeywords>();

        // Add the card systems plugin
        app.add_plugins(systems::CardSystemsPlugin);
    }
}

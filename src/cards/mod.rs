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
// Use a private module to avoid shadowing the components::tests module
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
// Avoid glob imports and be explicit about what's being exported
pub use components::CardCost;
pub use components::CardDetailsComponent;
pub use components::CardEntity;
pub use components::CardKeywords;
pub use components::CardName;
pub use components::CardOwner;
pub use components::CardRulesText;
pub use components::CardTypeInfo;
pub use components::CardZone;
pub use components::Draggable;
pub use components::NoUntapCondition;
pub use components::NoUntapEffect;
pub use components::PermanentState;
pub use details::*;
pub use keywords::*;
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
            // CardTypeInfo contains bitflags which don't fully implement reflection
            // .register_type::<components::CardTypeInfo>()
            .register_type::<components::CardDetailsComponent>()
            .register_type::<components::CardRulesText>()
            .register_type::<components::CardKeywords>();

        // Add the card systems plugin
        app.add_plugins(systems::CardSystemsPlugin);
    }
}

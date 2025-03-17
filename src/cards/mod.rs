// Card module for the game with card-related functionality
//! Cards, rules text, abilities, and other card-related functionality

// Private modules
pub mod abilities;
pub mod builder;
pub mod card;
pub mod components;
pub mod counters;
pub mod details;
pub mod drag;
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

// Re-export from details
pub use details::CardDetails;
pub use details::CreatureCard;
pub use details::SpellCard;
pub use details::SpellType;

// Re-export from types
pub use types::CardTypes;
pub use types::CreatureType;
pub use types::format_type_line;

// Re-export the plugin
pub use plugin::CardPlugin;

// Card module - Handles all card-related functionality and data structures

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
pub use builder::CardBuilder;
pub use card::Card;
pub use components::{
    CardCost, CardDetailsComponent, CardKeywords, CardName, CardRulesText, CardTypeInfo, Draggable,
    NoUntapCondition, NoUntapEffect, PermanentState,
};
pub use details::{CardDetails, CreatureCard, SpellCard, SpellType};
pub use plugin::CardPlugin;
pub use rarity::Rarity;
pub use set::CardSet;
pub use systems::{debug_render_text_positions, handle_card_dragging};
pub use types::{CardTypes, CreatureType, format_type_line};

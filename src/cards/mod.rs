// Card module - Handles all card-related functionality and data structures

// Private modules
mod abilities;
mod builder;
mod card;
mod components;
mod counters;
mod details;
mod keywords;
mod plugin;
mod rarity;
mod set;
mod state;
mod systems;
pub mod types; // Making types public so it can be accessed directly

// Test modules
#[cfg(test)]
mod tests;

// Public modules
pub mod hdr; // Historic Definition Records
pub mod mtgjson; // MTG JSON import functionality
pub mod penacony; // Specific set implementation
pub mod sets; // General set management
pub mod text; // Card text handling

// Re-export types for external use
pub use abilities::*;
pub use builder::CardBuilder;
pub use card::Card;
pub use components::{
    CardCost, CardDetailsComponent, CardKeywords, CardName, CardRulesText, CardTypeInfo, Draggable,
    NoUntapCondition, NoUntapEffect, PermanentState,
};
pub use counters::*;
pub use details::{
    ArtifactCard, CardDetails, CreatureCard, CreatureOnField, EnchantmentCard, LandCard, SpellCard,
    SpellType,
};
pub use keywords::{KeywordAbilities, KeywordAbility};
pub use plugin::CardPlugin;
pub use rarity::Rarity;
pub use set::CardSet;
pub use state::*;
pub use systems::{debug_render_text_positions, handle_card_dragging};
pub use types::{CardTypes, CreatureType, format_type_line};

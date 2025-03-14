//! Text rendering and layout for Magic: The Gathering cards.
//!
//! This module provides:
//! - Card text positioning and layout
//! - Text component management
//! - Font handling and scaling
//! - Debug visualization for text positions
//! - Mana symbol rendering using the Mana font

pub mod components;
pub mod mana_circles;
pub mod mana_symbols;
pub mod systems;
pub mod utils;

pub use components::*;

use bevy::prelude::*;

/// Plugin for text rendering and management
pub struct TextPlugin {
    /// Whether to use the legacy text spawning system
    /// Set to false if all cards are instantiated with text components
    pub use_legacy_text_system: bool,
}

impl Default for TextPlugin {
    fn default() -> Self {
        Self {
            use_legacy_text_system: true,
        }
    }
}

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        // Only add the legacy text spawning system if requested
        // Commenting this out as it's causing compilation errors
        // We will need to properly define this as a system or function later
        /*
        if self.use_legacy_text_system {
            app.add_systems(Update, crate::card::text::spawn_card_text);
        }
        */

        app.add_systems(Update, mana_circles::update_mana_circles);
    }
}

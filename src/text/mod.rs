//! Text rendering and layout for Magic: The Gathering cards.
//!
//! This module provides:
//! - Card text positioning and layout
//! - Text component management
//! - Font handling and scaling
//! - Debug visualization for text positions
//! - Mana symbol rendering using the Mana font

pub mod components;
pub mod layout;
pub mod mana_circles;
// Note: mana_symbols module has been moved to src/mana/render
pub mod systems;
pub mod utils;

pub use components::*;

use bevy::prelude::*;

/// Plugin for text rendering and management
#[derive(Default)]
pub struct TextPlugin {}


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

// Re-export common text components and functions
pub use crate::text::layout::get_battlefield_card_size_multiplier;

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
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crate::card::text::spawn_card_text)
            .add_systems(Update, mana_circles::update_mana_circles);
    }
}

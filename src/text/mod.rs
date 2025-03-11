//! Text rendering and layout for Magic: The Gathering cards.
//!
//! This module provides:
//! - Card text positioning and layout
//! - Text component management
//! - Font handling and scaling
//! - Debug visualization for text positions
//! - Mana symbol rendering using the Mana font

pub mod components;
pub mod systems;
pub mod utils;

pub use components::*;
pub use systems::*;

use bevy::prelude::*;

/// Plugin for text rendering and management
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::card_text::spawn_card_text);
    }
}

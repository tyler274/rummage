//! Text rendering and layout for Magic: The Gathering cards.
//!
//! This module provides:
//! - Card text positioning and layout
//! - Text component management
//! - Font handling and scaling
//! - Debug visualization for text positions
//! - Mana symbol rendering using the Mana font

mod components;
mod systems;
mod utils;

pub use components::*;
pub use systems::*;
pub use utils::*;

use bevy::prelude::*;

/// Plugin for text rendering and management
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugConfig::default())
            .add_systems(Update, systems::spawn_card_text);
    }
}

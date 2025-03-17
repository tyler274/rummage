#![feature(trivial_bounds)]

/// Rummage is a Magic: The Gathering game engine built with Bevy.
///
/// This crate provides functionality for:
/// - Card representation and manipulation
/// - Game state management
/// - Visual card rendering
/// - Player interactions
/// - Rules enforcement
/// - Full Commander game rules implementation
pub mod camera;
pub mod cards;
pub mod deck;
pub mod game_engine;
pub mod mana;
pub mod menu;
pub mod networking;
pub mod player;
pub mod plugins;
pub mod snapshot;
pub mod tests;
pub mod text;
pub mod tracing;
pub mod utils;
pub mod wsl2;

// Re-export for main app
pub use crate::plugins::MainRummagePlugin;

/// Setup reflection for bevy_persist serialization
pub fn setup_reflection(app: &mut bevy::prelude::App) {
    // Register Card types
    app.register_type::<cards::Card>()
        .register_type::<cards::CardEntity>()
        .register_type::<cards::CardZone>()
        .register_type::<cards::CardOwner>();

    // Register Player types
    app.register_type::<player::Player>();

    // Register Permanent types
    app.register_type::<game_engine::permanent::Permanent>()
        .register_type::<game_engine::permanent::PermanentState>()
        .register_type::<game_engine::permanent::PermanentOwner>()
        .register_type::<game_engine::permanent::PermanentController>();
}

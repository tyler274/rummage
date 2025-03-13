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
pub mod card;
pub mod deck;
pub mod docs;
pub mod drag;
pub mod game_engine;
pub mod mana;
pub mod menu;
pub mod player;
pub mod snapshot;
pub mod tests;
pub mod text;
pub mod tracing;
pub mod utils;

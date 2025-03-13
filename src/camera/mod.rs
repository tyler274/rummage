mod config;
mod state;

/// Camera management for the game's 2D view.
///
/// This module provides functionality for:
/// - Camera setup and configuration
/// - Camera movement (WASD/Arrow keys and middle mouse drag)
/// - Mouse wheel zoom with smooth interpolation
/// - Viewport management
/// - Coordinate space transformations
///
/// # Important Note for Bevy 0.15.x Compatibility
/// As of Bevy 0.15.x, Camera2dBundle and other *Bundle types are deprecated.
/// Instead, spawn camera entities with individual components:
pub mod components;
pub mod systems;
pub use config::*;
pub use state::*;

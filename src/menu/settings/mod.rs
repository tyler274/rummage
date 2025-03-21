//! Settings menu module for Rummage
//!
//! This module provides the settings menu functionality, including:
//! - Video settings
//! - Audio settings
//! - Gameplay settings
//! - Control settings

pub mod components;
pub mod plugin;
pub mod state;
pub mod systems;

pub use plugin::SettingsPlugin;
pub use state::*;

//! Settings menu module for Rummage
//!
//! This module provides the settings menu functionality, including:
//! - Video settings
//! - Audio settings
//! - Gameplay settings
//! - Control settings

mod components;
mod plugin;
mod state;
mod systems;

pub use plugin::SettingsPlugin;
pub use state::*;

//! WSL2 compatibility module for Bevy applications
//!
//! This module provides utilities and systems to improve compatibility with
//! Windows Subsystem for Linux 2 (WSL2), which has specific
//! requirements for window management and rendering.

mod plugin;
mod systems;
mod utils;

// Re-exports from the WSL2 compatibility module

// The following imports are unused, so let's comment them out
// pub use plugin::WSL2CompatibilityPlugin;
// pub use plugin::get_wsl2_window_settings;
// pub use utils::detect_wsl2;

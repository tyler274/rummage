//! Module for handling cleanup of menu and game entities

// mod decorative; // Removed declaration for deleted file
mod game;
mod main_menu;
pub mod pause_menu;
pub mod plugin;

// These modules are used internally but not exported
// to avoid unused import warnings

pub use plugin::CleanupPlugin;

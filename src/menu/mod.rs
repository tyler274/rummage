pub mod cleanup;
pub mod components;
pub mod logo;
pub mod main_menu;
pub mod pause_menu;
pub mod plugin;
pub mod settings;
pub mod state;
pub mod styles;
mod systems;

// Add the missing modules
pub mod credits;
pub mod deck;
pub mod main;

pub use plugin::MenuPlugin;
pub use settings::SettingsPlugin;
pub use state::*;

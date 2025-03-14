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

pub use plugin::MenuPlugin;
pub use settings::SettingsPlugin;
pub use state::*;
